use crate::error::BullShiftError;
use crate::trading::{Order, OrderSide, OrderStatus};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tokio::sync::broadcast;
use uuid::Uuid;

/// Events emitted by BullShift that SecureYeoman can subscribe to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeEvent {
    pub id: Uuid,
    pub event_type: TradeEventType,
    pub order_id: Uuid,
    pub symbol: String,
    pub side: String,
    pub quantity: f64,
    pub price: Option<f64>,
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeEventType {
    OrderSubmitted,
    OrderFilled,
    OrderPartiallyFilled,
    OrderCancelled,
    OrderRejected,
    PositionOpened,
    PositionClosed,
    PositionUpdated,
    StopLossTriggered,
    TakeProfitTriggered,
}

impl std::fmt::Display for TradeEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OrderSubmitted => write!(f, "order.submitted"),
            Self::OrderFilled => write!(f, "order.filled"),
            Self::OrderPartiallyFilled => write!(f, "order.partially_filled"),
            Self::OrderCancelled => write!(f, "order.cancelled"),
            Self::OrderRejected => write!(f, "order.rejected"),
            Self::PositionOpened => write!(f, "position.opened"),
            Self::PositionClosed => write!(f, "position.closed"),
            Self::PositionUpdated => write!(f, "position.updated"),
            Self::StopLossTriggered => write!(f, "stop_loss.triggered"),
            Self::TakeProfitTriggered => write!(f, "take_profit.triggered"),
        }
    }
}

/// Response from SecureYeoman when submitting an order via its integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureYeomanOrderResponse {
    pub accepted: bool,
    pub order_id: Option<String>,
    pub reason: Option<String>,
}

/// Configuration for the SecureYeoman integration bridge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub event_buffer_size: usize,
    pub auto_emit_events: bool,
    pub subscribe_to_events: bool,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:18789".to_string(),
            api_key: None,
            event_buffer_size: 1000,
            auto_emit_events: true,
            subscribe_to_events: true,
        }
    }
}

/// Bridge between BullShift and SecureYeoman's integration system.
///
/// Provides bidirectional communication:
/// - Emits trade events so SecureYeoman agents can react
/// - Receives autonomous order requests from SecureYeoman agents
/// - Subscribes to SecureYeoman's event bus for coordination
pub struct SecureYeomanBridge {
    client: Client,
    config: IntegrationConfig,
    event_sender: broadcast::Sender<TradeEvent>,
    event_history: VecDeque<TradeEvent>,
    connected: bool,
}

impl SecureYeomanBridge {
    pub fn new(config: IntegrationConfig) -> Self {
        let (event_sender, _) = broadcast::channel(config.event_buffer_size);
        Self {
            client: Client::new(),
            config,
            event_sender,
            event_history: VecDeque::with_capacity(500),
            connected: false,
        }
    }

    /// Subscribe to trade events via a broadcast receiver.
    pub fn subscribe(&self) -> broadcast::Receiver<TradeEvent> {
        self.event_sender.subscribe()
    }

    /// Check connectivity with the SecureYeoman instance.
    pub async fn health_check(&mut self) -> Result<bool, BullShiftError> {
        let url = format!("{}/api/v1/health", self.config.base_url);
        match self.client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                self.connected = true;
                Ok(true)
            }
            Ok(_) => {
                self.connected = false;
                Ok(false)
            }
            Err(e) => {
                self.connected = false;
                Err(BullShiftError::Network(format!(
                    "SecureYeoman health check failed: {}",
                    e
                )))
            }
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Emit a trade event to local subscribers and optionally to SecureYeoman.
    pub async fn emit_event(&mut self, event: TradeEvent) -> Result<(), BullShiftError> {
        // Store locally
        if self.event_history.len() >= 500 {
            self.event_history.pop_front();
        }
        self.event_history.push_back(event.clone());

        // Broadcast to local subscribers
        let _ = self.event_sender.send(event.clone());

        // Forward to SecureYeoman if configured and connected
        if self.config.auto_emit_events && self.connected {
            self.forward_event_to_secureyeoman(&event).await?;
        }

        Ok(())
    }

    /// Create a trade event from an Order and emit it.
    pub async fn emit_order_event(
        &mut self,
        order: &Order,
        event_type: TradeEventType,
    ) -> Result<(), BullShiftError> {
        let side = match order.side {
            OrderSide::Buy => "BUY",
            OrderSide::Sell => "SELL",
        };
        let status = match order.status {
            OrderStatus::Pending => "PENDING",
            OrderStatus::Submitted => "SUBMITTED",
            OrderStatus::PartiallyFilled => "PARTIALLY_FILLED",
            OrderStatus::Filled => "FILLED",
            OrderStatus::Cancelled => "CANCELLED",
            OrderStatus::Rejected => "REJECTED",
        };

        let event = TradeEvent {
            id: Uuid::new_v4(),
            event_type,
            order_id: order.id,
            symbol: order.symbol.clone(),
            side: side.to_string(),
            quantity: order.quantity,
            price: order.price,
            status: status.to_string(),
            timestamp: Utc::now(),
            metadata: std::collections::HashMap::new(),
        };

        self.emit_event(event).await
    }

    async fn forward_event_to_secureyeoman(
        &self,
        event: &TradeEvent,
    ) -> Result<(), BullShiftError> {
        let url = format!(
            "{}/api/v1/integrations/bullshift/events",
            self.config.base_url
        );

        let mut req = self.client.post(&url).json(event);
        if let Some(ref key) = self.config.api_key {
            req = req.header("x-api-key", key);
        }

        match req.send().await {
            Ok(resp) if resp.status().is_success() => Ok(()),
            Ok(resp) => {
                log::warn!(
                    "SecureYeoman event forward returned {}: {}",
                    resp.status(),
                    resp.text().await.unwrap_or_default()
                );
                Ok(()) // Non-fatal — don't block trading on event delivery
            }
            Err(e) => {
                log::warn!("Failed to forward event to SecureYeoman: {}", e);
                Ok(()) // Non-fatal
            }
        }
    }

    /// Submit an order request received from SecureYeoman for validation.
    /// Returns the validated order or an error if the request is invalid.
    pub fn validate_agent_order(
        &self,
        symbol: &str,
        side: &str,
        quantity: f64,
        order_type: &str,
        price: Option<f64>,
    ) -> Result<Order, BullShiftError> {
        if symbol.is_empty() {
            return Err(BullShiftError::Validation(
                "Symbol cannot be empty".to_string(),
            ));
        }
        if quantity <= 0.0 || !quantity.is_finite() {
            return Err(BullShiftError::Validation(
                "Quantity must be a positive finite number".to_string(),
            ));
        }

        let order_side = match side.to_uppercase().as_str() {
            "BUY" => OrderSide::Buy,
            "SELL" => OrderSide::Sell,
            _ => {
                return Err(BullShiftError::Validation(format!(
                    "Invalid side '{}'. Must be BUY or SELL",
                    side
                )))
            }
        };

        let parsed_type = match order_type.to_uppercase().as_str() {
            "MARKET" => crate::trading::OrderType::Market,
            "LIMIT" => crate::trading::OrderType::Limit,
            "STOP" => crate::trading::OrderType::Stop,
            "STOP_LIMIT" => crate::trading::OrderType::StopLimit,
            _ => {
                return Err(BullShiftError::Validation(format!(
                    "Invalid order type '{}'",
                    order_type
                )))
            }
        };

        let now = Utc::now();
        Ok(Order {
            id: Uuid::new_v4(),
            symbol: symbol.to_string(),
            side: order_side,
            quantity,
            order_type: parsed_type,
            price,
            stop_loss: None,
            take_profit: None,
            status: OrderStatus::Pending,
            created_at: now,
            updated_at: now,
        })
    }

    /// Get recent event history.
    pub fn recent_events(&self, limit: usize) -> Vec<&TradeEvent> {
        self.event_history.iter().rev().take(limit).collect()
    }

    /// Register BullShift as an integration with the SecureYeoman instance.
    pub async fn register_integration(&self) -> Result<(), BullShiftError> {
        let url = format!(
            "{}/api/v1/integrations/bullshift/register",
            self.config.base_url
        );

        let payload = serde_json::json!({
            "name": "BullShift",
            "version": env!("CARGO_PKG_VERSION"),
            "capabilities": [
                "trade_events",
                "order_submission",
                "position_queries",
                "audit_trail"
            ],
            "callback_url": format!("http://localhost:{}", std::env::var("BULLSHIFT_PORT").unwrap_or_else(|_| "8787".to_string())),
        });

        let mut req = self.client.post(&url).json(&payload);
        if let Some(ref key) = self.config.api_key {
            req = req.header("x-api-key", key);
        }

        match req.send().await {
            Ok(resp) if resp.status().is_success() => {
                log::info!("Registered BullShift integration with SecureYeoman");
                Ok(())
            }
            Ok(resp) => Err(BullShiftError::Api(format!(
                "Integration registration failed: {}",
                resp.status()
            ))),
            Err(e) => Err(BullShiftError::Network(format!(
                "Failed to register integration: {}",
                e
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = IntegrationConfig::default();
        assert_eq!(config.base_url, "http://localhost:18789");
        assert!(config.auto_emit_events);
        assert!(config.subscribe_to_events);
        assert_eq!(config.event_buffer_size, 1000);
    }

    #[test]
    fn test_bridge_subscribe() {
        let bridge = SecureYeomanBridge::new(IntegrationConfig::default());
        let _rx = bridge.subscribe();
        assert!(!bridge.is_connected());
    }

    #[test]
    fn test_validate_agent_order_valid() {
        let bridge = SecureYeomanBridge::new(IntegrationConfig::default());
        let order = bridge
            .validate_agent_order("AAPL", "BUY", 100.0, "MARKET", None)
            .unwrap();
        assert_eq!(order.symbol, "AAPL");
        assert_eq!(order.quantity, 100.0);
    }

    #[test]
    fn test_validate_agent_order_invalid_side() {
        let bridge = SecureYeomanBridge::new(IntegrationConfig::default());
        let result = bridge.validate_agent_order("AAPL", "HOLD", 100.0, "MARKET", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_agent_order_invalid_quantity() {
        let bridge = SecureYeomanBridge::new(IntegrationConfig::default());
        assert!(bridge
            .validate_agent_order("AAPL", "BUY", -10.0, "MARKET", None)
            .is_err());
        assert!(bridge
            .validate_agent_order("AAPL", "BUY", 0.0, "MARKET", None)
            .is_err());
    }

    #[test]
    fn test_validate_agent_order_empty_symbol() {
        let bridge = SecureYeomanBridge::new(IntegrationConfig::default());
        assert!(bridge
            .validate_agent_order("", "BUY", 100.0, "MARKET", None)
            .is_err());
    }

    #[test]
    fn test_trade_event_type_display() {
        assert_eq!(
            TradeEventType::OrderSubmitted.to_string(),
            "order.submitted"
        );
        assert_eq!(TradeEventType::OrderFilled.to_string(), "order.filled");
        assert_eq!(
            TradeEventType::StopLossTriggered.to_string(),
            "stop_loss.triggered"
        );
    }

    #[tokio::test]
    async fn test_emit_event_stores_history() {
        let mut bridge = SecureYeomanBridge::new(IntegrationConfig {
            auto_emit_events: false, // Don't try to reach SecureYeoman
            ..Default::default()
        });

        let event = TradeEvent {
            id: Uuid::new_v4(),
            event_type: TradeEventType::OrderSubmitted,
            order_id: Uuid::new_v4(),
            symbol: "AAPL".to_string(),
            side: "BUY".to_string(),
            quantity: 100.0,
            price: Some(150.0),
            status: "SUBMITTED".to_string(),
            timestamp: Utc::now(),
            metadata: std::collections::HashMap::new(),
        };

        bridge.emit_event(event).await.unwrap();
        assert_eq!(bridge.recent_events(10).len(), 1);
    }

    #[test]
    fn test_config_custom_values() {
        let config = IntegrationConfig {
            base_url: "https://secureyeoman.example.com:9443".to_string(),
            api_key: Some("my-api-key-xyz".to_string()),
            event_buffer_size: 2000,
            auto_emit_events: false,
            subscribe_to_events: false,
        };
        assert_eq!(config.base_url, "https://secureyeoman.example.com:9443");
        assert_eq!(config.api_key, Some("my-api-key-xyz".to_string()));
        assert_eq!(config.event_buffer_size, 2000);
        assert!(!config.auto_emit_events);
        assert!(!config.subscribe_to_events);
    }

    fn make_test_event(symbol: &str, event_type: TradeEventType) -> TradeEvent {
        TradeEvent {
            id: Uuid::new_v4(),
            event_type,
            order_id: Uuid::new_v4(),
            symbol: symbol.to_string(),
            side: "BUY".to_string(),
            quantity: 10.0,
            price: Some(100.0),
            status: "SUBMITTED".to_string(),
            timestamp: Utc::now(),
            metadata: std::collections::HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_event_history_ordering() {
        let mut bridge = SecureYeomanBridge::new(IntegrationConfig {
            auto_emit_events: false,
            ..Default::default()
        });

        let symbols = ["AAPL", "GOOG", "MSFT"];
        for sym in &symbols {
            bridge
                .emit_event(make_test_event(sym, TradeEventType::OrderSubmitted))
                .await
                .unwrap();
        }

        // recent_events returns newest first (rev iterator)
        let events = bridge.recent_events(10);
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].symbol, "MSFT");
        assert_eq!(events[1].symbol, "GOOG");
        assert_eq!(events[2].symbol, "AAPL");
    }

    #[tokio::test]
    async fn test_event_history_limit() {
        let mut bridge = SecureYeomanBridge::new(IntegrationConfig {
            auto_emit_events: false,
            ..Default::default()
        });

        // The history is capped at 500 entries (see emit_event)
        for i in 0..510 {
            let mut event = make_test_event("TEST", TradeEventType::OrderFilled);
            event.quantity = i as f64;
            bridge.emit_event(event).await.unwrap();
        }

        // History should be bounded to 500
        assert_eq!(bridge.event_history.len(), 500);

        // The oldest entries (0..9) should have been evicted;
        // the first remaining entry should have quantity 10.0
        let oldest = bridge.event_history.front().unwrap();
        assert_eq!(oldest.quantity, 10.0);
    }

    #[test]
    fn test_validate_agent_order_negative_price() {
        let bridge = SecureYeomanBridge::new(IntegrationConfig::default());

        // Negative price in the price field is allowed (it's Option<f64>
        // used for limit orders); validation only checks quantity.
        let order = bridge
            .validate_agent_order("AAPL", "BUY", 50.0, "LIMIT", Some(-5.0))
            .unwrap();
        assert_eq!(order.price, Some(-5.0));

        // But negative quantity must be rejected
        let result = bridge.validate_agent_order("AAPL", "SELL", -1.0, "MARKET", None);
        assert!(result.is_err());

        // NaN quantity must also be rejected
        let result = bridge.validate_agent_order("AAPL", "BUY", f64::NAN, "MARKET", None);
        assert!(result.is_err());

        // Infinity quantity must also be rejected
        let result = bridge.validate_agent_order("AAPL", "BUY", f64::INFINITY, "MARKET", None);
        assert!(result.is_err());
    }
}
