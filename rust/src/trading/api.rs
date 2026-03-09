use crate::error::BullShiftError;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingCredentials {
    pub api_key: String,
    pub api_secret: String,
    pub sandbox: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiOrderRequest {
    pub symbol: String,
    pub side: String,
    pub quantity: f64,
    pub order_type: String,
    pub price: Option<f64>,
    pub time_in_force: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiOrderResponse {
    pub order_id: String,
    pub symbol: String,
    pub side: String,
    pub quantity: f64,
    pub order_type: String,
    pub price: Option<f64>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPosition {
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub unrealized_pnl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAccount {
    pub balance: f64,
    pub available: f64,
    pub margin_used: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiQuote {
    pub symbol: String,
    pub last_price: f64,
    pub bid: f64,
    pub ask: f64,
    pub volume: u64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub prev_close: f64,
    pub change: f64,
    pub change_pct: f64,
    pub timestamp: String,
}

#[async_trait]
pub trait TradingApi {
    async fn submit_order(
        &self,
        order: ApiOrderRequest,
    ) -> Result<ApiOrderResponse, BullShiftError>;
    async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError>;
    async fn get_account(&self) -> Result<ApiAccount, BullShiftError>;
    async fn cancel_order(&self, order_id: String) -> Result<bool, BullShiftError>;
}

pub struct AlpacaApi {
    client: Client,
    base_url: String,
    api_key: String,
    api_secret: String,
}

impl AlpacaApi {
    pub fn new(credentials: TradingCredentials) -> Self {
        let base_url = if credentials.sandbox {
            "https://paper-api.alpaca.markets".to_string()
        } else {
            "https://api.alpaca.markets".to_string()
        };

        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .connect_timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_else(|_| Client::new()),
            api_key: credentials.api_key,
            api_secret: credentials.api_secret,
            base_url,
        }
    }

    pub fn data_url(&self) -> &str {
        "https://data.alpaca.markets"
    }

    pub async fn get_quote(&self, symbol: &str) -> Result<ApiQuote, BullShiftError> {
        let url = format!(
            "{}/v2/stocks/{}/snapshot",
            self.data_url(),
            symbol.to_uppercase()
        );

        let response = self
            .client
            .get(&url)
            .header("APCA-API-KEY-ID", &self.api_key)
            .header("APCA-API-SECRET-KEY", &self.api_secret)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BullShiftError::Api(format!(
                "Failed to get quote for {}: {}",
                symbol,
                response.status()
            )));
        }

        let data: serde_json::Value = response.json().await?;

        let latest_trade_price = data["latestTrade"]["p"].as_f64().unwrap_or(0.0);
        if latest_trade_price == 0.0 {
            log::warn!(
                "Alpaca returned zero last_trade_price for {}; snapshot may be stale or missing",
                symbol
            );
        }
        let latest_quote_bid = data["latestQuote"]["bp"].as_f64().unwrap_or(0.0);
        let latest_quote_ask = data["latestQuote"]["ap"].as_f64().unwrap_or(0.0);
        let daily_bar = &data["dailyBar"];
        let prev_daily_bar = &data["prevDailyBar"];
        let prev_close = prev_daily_bar["c"].as_f64().unwrap_or(0.0);
        let change = if prev_close > 0.0 {
            latest_trade_price - prev_close
        } else {
            0.0
        };
        let change_pct = if prev_close > 0.0 {
            (change / prev_close) * 100.0
        } else {
            0.0
        };

        Ok(ApiQuote {
            symbol: symbol.to_uppercase(),
            last_price: latest_trade_price,
            bid: latest_quote_bid,
            ask: latest_quote_ask,
            volume: daily_bar["v"].as_u64().unwrap_or(0),
            high: daily_bar["h"].as_f64().unwrap_or(0.0),
            low: daily_bar["l"].as_f64().unwrap_or(0.0),
            open: daily_bar["o"].as_f64().unwrap_or(0.0),
            prev_close,
            change,
            change_pct,
            timestamp: data["latestTrade"]["t"].as_str().unwrap_or("").to_string(),
        })
    }
}

#[async_trait]
impl TradingApi for AlpacaApi {
    async fn submit_order(
        &self,
        order: ApiOrderRequest,
    ) -> Result<ApiOrderResponse, BullShiftError> {
        let url = format!("{}/v2/orders", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("APCA-API-KEY-ID", &self.api_key)
            .header("APCA-API-SECRET-KEY", &self.api_secret)
            .header("Content-Type", "application/json")
            .json(&order)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<ApiOrderResponse>().await?)
        } else {
            Err(BullShiftError::Api(format!(
                "Order submission failed: {}",
                response.status()
            )))
        }
    }

    async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError> {
        let url = format!("{}/v2/positions", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("APCA-API-KEY-ID", &self.api_key)
            .header("APCA-API-SECRET-KEY", &self.api_secret)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<Vec<ApiPosition>>().await?)
        } else {
            Err(BullShiftError::Api(format!(
                "Failed to get positions: {}",
                response.status()
            )))
        }
    }

    async fn get_account(&self) -> Result<ApiAccount, BullShiftError> {
        let url = format!("{}/v2/account", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("APCA-API-KEY-ID", &self.api_key)
            .header("APCA-API-SECRET-KEY", &self.api_secret)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<ApiAccount>().await?)
        } else {
            Err(BullShiftError::Api(format!(
                "Failed to get account: {}",
                response.status()
            )))
        }
    }

    async fn cancel_order(&self, order_id: String) -> Result<bool, BullShiftError> {
        if !order_id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(BullShiftError::Validation(format!(
                "Invalid order_id format: {}",
                order_id
            )));
        }
        let url = format!("{}/v2/orders/{}", self.base_url, order_id);

        let response = self
            .client
            .delete(&url)
            .header("APCA-API-KEY-ID", &self.api_key)
            .header("APCA-API-SECRET-KEY", &self.api_secret)
            .send()
            .await?;

        Ok(response.status().is_success())
    }
}

use super::brokers::{BrokerCapabilities, BrokerInfo, BrokerStatus};

/// Manages multiple broker connections and routes requests to the active broker.
///
/// Provides a unified interface for submitting orders, querying positions,
/// and checking account details regardless of which broker is active.
///
/// # Example
/// ```ignore
/// let mut manager = TradingApiManager::new();
///
/// // Register brokers
/// manager.register_broker("alpaca", Box::new(alpaca_api), AlpacaApi::capabilities());
/// manager.register_broker("tradier", Box::new(tradier_api), TradierApi::capabilities());
///
/// // Use the default broker
/// let positions = manager.get_positions().await?;
///
/// // Switch brokers at runtime
/// manager.set_default("tradier".to_string());
/// ```
pub struct TradingApiManager {
    apis: HashMap<String, Box<dyn TradingApi + Send + Sync>>,
    capabilities: HashMap<String, BrokerCapabilities>,
    default_api: String,
}

impl Default for TradingApiManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TradingApiManager {
    pub fn new() -> Self {
        Self {
            apis: HashMap::new(),
            capabilities: HashMap::new(),
            default_api: "alpaca".to_string(),
        }
    }

    /// Register a broker with its API implementation and capabilities.
    pub fn register_broker(
        &mut self,
        name: &str,
        api: Box<dyn TradingApi + Send + Sync>,
        capabilities: BrokerCapabilities,
    ) {
        self.apis.insert(name.to_string(), api);
        self.capabilities.insert(name.to_string(), capabilities);
    }

    /// Legacy method — registers without capabilities metadata.
    pub fn add_api(&mut self, name: String, api: Box<dyn TradingApi + Send + Sync>) {
        self.apis.insert(name, api);
    }

    /// Set the active broker. Returns false if the broker isn't registered.
    pub fn set_default(&mut self, name: String) -> bool {
        if self.apis.contains_key(&name) {
            self.default_api = name;
            true
        } else {
            false
        }
    }

    /// Get the name of the currently active broker.
    pub fn active_broker(&self) -> &str {
        &self.default_api
    }

    /// List all registered broker names.
    pub fn list_brokers(&self) -> Vec<String> {
        self.apis.keys().cloned().collect()
    }

    /// Get capabilities for a specific broker.
    pub fn get_capabilities(&self, name: &str) -> Option<&BrokerCapabilities> {
        self.capabilities.get(name)
    }

    /// Get info for all registered brokers.
    pub fn get_broker_info(&self) -> Vec<BrokerInfo> {
        self.apis
            .keys()
            .map(|name| BrokerInfo {
                name: name.clone(),
                display_name: Self::display_name(name),
                status: if name == &self.default_api {
                    BrokerStatus::Connected
                } else {
                    BrokerStatus::Disconnected
                },
                capabilities: self
                    .capabilities
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| Self::default_capabilities(name)),
            })
            .collect()
    }

    /// Submit an order to the active broker.
    pub async fn submit_order(
        &self,
        order: ApiOrderRequest,
    ) -> Result<ApiOrderResponse, BullShiftError> {
        self.get_active_api()?.submit_order(order).await
    }

    /// Get positions from the active broker.
    pub async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError> {
        self.get_active_api()?.get_positions().await
    }

    /// Get account details from the active broker.
    pub async fn get_account(&self) -> Result<ApiAccount, BullShiftError> {
        self.get_active_api()?.get_account().await
    }

    /// Cancel an order on the active broker.
    pub async fn cancel_order(&self, order_id: String) -> Result<bool, BullShiftError> {
        self.get_active_api()?.cancel_order(order_id).await
    }

    /// Submit an order to a specific named broker (not necessarily the default).
    pub async fn submit_order_to(
        &self,
        broker: &str,
        order: ApiOrderRequest,
    ) -> Result<ApiOrderResponse, BullShiftError> {
        self.get_api(broker)?.submit_order(order).await
    }

    fn get_active_api(&self) -> Result<&(dyn TradingApi + Send + Sync), BullShiftError> {
        self.apis
            .get(&self.default_api)
            .map(|b| b.as_ref())
            .ok_or_else(|| BullShiftError::Configuration("No trading API configured".to_string()))
    }

    fn get_api(&self, name: &str) -> Result<&(dyn TradingApi + Send + Sync), BullShiftError> {
        self.apis.get(name).map(|b| b.as_ref()).ok_or_else(|| {
            BullShiftError::Configuration(format!("Broker '{}' not registered", name))
        })
    }

    fn display_name(name: &str) -> String {
        match name {
            "alpaca" => "Alpaca Markets".to_string(),
            "interactive_brokers" => "Interactive Brokers".to_string(),
            "tradier" => "Tradier".to_string(),
            "robinhood" => "Robinhood".to_string(),
            "schwab" => "Charles Schwab".to_string(),
            "coinbase" => "Coinbase".to_string(),
            "kraken" => "Kraken".to_string(),
            "webull" => "Webull".to_string(),
            other => other.to_string(),
        }
    }

    fn default_capabilities(name: &str) -> BrokerCapabilities {
        BrokerCapabilities {
            name: name.to_string(),
            supports_market_orders: true,
            supports_limit_orders: true,
            supports_stop_orders: false,
            supports_stop_limit_orders: false,
            supports_fractional_shares: false,
            supports_short_selling: false,
            supports_options: false,
            supports_crypto: false,
            supports_extended_hours: false,
            sandbox_available: false,
        }
    }
}

impl AlpacaApi {
    pub fn capabilities() -> BrokerCapabilities {
        BrokerCapabilities {
            name: "alpaca".to_string(),
            supports_market_orders: true,
            supports_limit_orders: true,
            supports_stop_orders: true,
            supports_stop_limit_orders: true,
            supports_fractional_shares: true,
            supports_short_selling: true,
            supports_options: false,
            supports_crypto: true,
            supports_extended_hours: true,
            sandbox_available: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_register_and_list() {
        let mut mgr = TradingApiManager::new();
        let creds = TradingCredentials {
            api_key: "key".to_string(),
            api_secret: "secret".to_string(),
            sandbox: true,
        };
        mgr.register_broker(
            "alpaca",
            Box::new(AlpacaApi::new(creds)),
            AlpacaApi::capabilities(),
        );
        let brokers = mgr.list_brokers();
        assert!(brokers.contains(&"alpaca".to_string()));
    }

    #[test]
    fn test_manager_set_default() {
        let mut mgr = TradingApiManager::new();
        assert!(!mgr.set_default("nonexistent".to_string()));

        let creds = TradingCredentials {
            api_key: "k".to_string(),
            api_secret: "s".to_string(),
            sandbox: true,
        };
        mgr.register_broker(
            "tradier",
            Box::new(AlpacaApi::new(creds)),
            AlpacaApi::capabilities(),
        );
        assert!(mgr.set_default("tradier".to_string()));
        assert_eq!(mgr.active_broker(), "tradier");
    }

    #[test]
    fn test_manager_capabilities() {
        let mut mgr = TradingApiManager::new();
        let creds = TradingCredentials {
            api_key: "k".to_string(),
            api_secret: "s".to_string(),
            sandbox: true,
        };
        mgr.register_broker(
            "alpaca",
            Box::new(AlpacaApi::new(creds)),
            AlpacaApi::capabilities(),
        );
        let caps = mgr.get_capabilities("alpaca").unwrap();
        assert!(caps.supports_crypto);
        assert!(caps.supports_fractional_shares);
        assert!(!caps.supports_options);
    }

    #[test]
    fn test_manager_broker_info() {
        let mut mgr = TradingApiManager::new();
        let creds = TradingCredentials {
            api_key: "k".to_string(),
            api_secret: "s".to_string(),
            sandbox: true,
        };
        mgr.register_broker(
            "alpaca",
            Box::new(AlpacaApi::new(creds)),
            AlpacaApi::capabilities(),
        );
        mgr.set_default("alpaca".to_string());
        let info = mgr.get_broker_info();
        assert_eq!(info.len(), 1);
        assert_eq!(info[0].display_name, "Alpaca Markets");
        assert_eq!(info[0].status, BrokerStatus::Connected);
    }

    #[test]
    fn test_display_names() {
        assert_eq!(TradingApiManager::display_name("alpaca"), "Alpaca Markets");
        assert_eq!(
            TradingApiManager::display_name("interactive_brokers"),
            "Interactive Brokers"
        );
        assert_eq!(TradingApiManager::display_name("tradier"), "Tradier");
        assert_eq!(TradingApiManager::display_name("robinhood"), "Robinhood");
        assert_eq!(TradingApiManager::display_name("custom"), "custom");
    }

    #[test]
    fn test_alpaca_api_sandbox_url() {
        let creds = TradingCredentials {
            api_key: "pk_test".to_string(),
            api_secret: "sk_test".to_string(),
            sandbox: true,
        };
        let api = AlpacaApi::new(creds);
        assert_eq!(api.base_url, "https://paper-api.alpaca.markets");
    }

    #[test]
    fn test_alpaca_api_production_url() {
        let creds = TradingCredentials {
            api_key: "pk_live".to_string(),
            api_secret: "sk_live".to_string(),
            sandbox: false,
        };
        let api = AlpacaApi::new(creds);
        assert_eq!(api.base_url, "https://api.alpaca.markets");
    }

    #[test]
    fn test_alpaca_data_url() {
        let creds = TradingCredentials {
            api_key: "k".to_string(),
            api_secret: "s".to_string(),
            sandbox: true,
        };
        let api = AlpacaApi::new(creds);
        assert_eq!(api.data_url(), "https://data.alpaca.markets");
    }

    #[test]
    fn test_alpaca_capabilities() {
        let caps = AlpacaApi::capabilities();
        assert_eq!(caps.name, "alpaca");
        assert!(caps.supports_market_orders);
        assert!(caps.supports_limit_orders);
        assert!(caps.supports_stop_orders);
        assert!(caps.supports_stop_limit_orders);
        assert!(caps.supports_fractional_shares);
        assert!(caps.supports_short_selling);
        assert!(!caps.supports_options);
        assert!(caps.supports_crypto);
        assert!(caps.supports_extended_hours);
        assert!(caps.sandbox_available);
    }

    #[test]
    fn test_manager_default_active_broker() {
        let mgr = TradingApiManager::new();
        assert_eq!(mgr.active_broker(), "alpaca");
    }

    #[test]
    fn test_manager_empty_list_brokers() {
        let mgr = TradingApiManager::new();
        assert!(mgr.list_brokers().is_empty());
    }

    #[test]
    fn test_manager_no_capabilities_for_unknown() {
        let mgr = TradingApiManager::new();
        assert!(mgr.get_capabilities("nonexistent").is_none());
    }

    #[tokio::test]
    async fn test_manager_submit_order_no_api() {
        let mgr = TradingApiManager::new();
        let order = ApiOrderRequest {
            symbol: "AAPL".to_string(),
            side: "buy".to_string(),
            quantity: 10.0,
            order_type: "market".to_string(),
            price: None,
            time_in_force: None,
        };
        let result = mgr.submit_order(order).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_manager_get_positions_no_api() {
        let mgr = TradingApiManager::new();
        assert!(mgr.get_positions().await.is_err());
    }

    #[tokio::test]
    async fn test_manager_get_account_no_api() {
        let mgr = TradingApiManager::new();
        assert!(mgr.get_account().await.is_err());
    }

    #[tokio::test]
    async fn test_manager_cancel_order_no_api() {
        let mgr = TradingApiManager::new();
        assert!(mgr.cancel_order("o-1".to_string()).await.is_err());
    }

    #[tokio::test]
    async fn test_manager_submit_order_to_unknown_broker() {
        let mgr = TradingApiManager::new();
        let order = ApiOrderRequest {
            symbol: "TSLA".to_string(),
            side: "sell".to_string(),
            quantity: 5.0,
            order_type: "limit".to_string(),
            price: Some(250.0),
            time_in_force: None,
        };
        let result = mgr.submit_order_to("fake_broker", order).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_manager_add_api_legacy() {
        let mut mgr = TradingApiManager::new();
        let creds = TradingCredentials {
            api_key: "k".to_string(),
            api_secret: "s".to_string(),
            sandbox: true,
        };
        mgr.add_api("legacy_broker".to_string(), Box::new(AlpacaApi::new(creds)));
        assert!(mgr.list_brokers().contains(&"legacy_broker".to_string()));
        // No capabilities registered via add_api
        assert!(mgr.get_capabilities("legacy_broker").is_none());
    }

    #[test]
    fn test_manager_multiple_brokers() {
        let mut mgr = TradingApiManager::new();
        for name in &["alpaca", "tradier", "schwab"] {
            let creds = TradingCredentials {
                api_key: format!("{}_key", name),
                api_secret: format!("{}_secret", name),
                sandbox: true,
            };
            mgr.register_broker(
                name,
                Box::new(AlpacaApi::new(creds)),
                AlpacaApi::capabilities(),
            );
        }
        assert_eq!(mgr.list_brokers().len(), 3);
        let info = mgr.get_broker_info();
        assert_eq!(info.len(), 3);
    }

    #[test]
    fn test_manager_default_capabilities_unknown() {
        let caps = TradingApiManager::default_capabilities("unknown");
        assert_eq!(caps.name, "unknown");
        assert!(caps.supports_market_orders);
        assert!(caps.supports_limit_orders);
        assert!(!caps.supports_stop_orders);
        assert!(!caps.supports_fractional_shares);
        assert!(!caps.supports_crypto);
    }

    #[test]
    fn test_display_names_all_brokers() {
        assert_eq!(TradingApiManager::display_name("schwab"), "Charles Schwab");
        assert_eq!(TradingApiManager::display_name("coinbase"), "Coinbase");
        assert_eq!(TradingApiManager::display_name("kraken"), "Kraken");
        assert_eq!(TradingApiManager::display_name("webull"), "Webull");
    }

    #[test]
    fn test_broker_info_status() {
        let mut mgr = TradingApiManager::new();
        let creds1 = TradingCredentials {
            api_key: "k".to_string(),
            api_secret: "s".to_string(),
            sandbox: true,
        };
        let creds2 = TradingCredentials {
            api_key: "k2".to_string(),
            api_secret: "s2".to_string(),
            sandbox: true,
        };
        mgr.register_broker(
            "alpaca",
            Box::new(AlpacaApi::new(creds1)),
            AlpacaApi::capabilities(),
        );
        mgr.register_broker(
            "tradier",
            Box::new(AlpacaApi::new(creds2)),
            AlpacaApi::capabilities(),
        );
        mgr.set_default("alpaca".to_string());

        let info = mgr.get_broker_info();
        let alpaca_info = info.iter().find(|i| i.name == "alpaca").unwrap();
        let tradier_info = info.iter().find(|i| i.name == "tradier").unwrap();
        assert_eq!(alpaca_info.status, BrokerStatus::Connected);
        assert_eq!(tradier_info.status, BrokerStatus::Disconnected);
    }

    #[test]
    fn test_api_quote_struct() {
        let quote = ApiQuote {
            symbol: "AAPL".to_string(),
            last_price: 150.25,
            bid: 150.20,
            ask: 150.30,
            volume: 50_000_000,
            high: 152.00,
            low: 149.00,
            open: 150.00,
            prev_close: 149.50,
            change: 0.75,
            change_pct: 0.5,
            timestamp: "2026-03-09T14:30:00Z".to_string(),
        };
        assert_eq!(quote.symbol, "AAPL");
        assert!((quote.change - 0.75).abs() < f64::EPSILON);
        assert!((quote.change_pct - 0.5).abs() < f64::EPSILON);
    }
}
