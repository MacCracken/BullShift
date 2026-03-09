use crate::error::BullShiftError;
use crate::integration::TradeEvent;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Supported webhook payload formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookFormat {
    /// Standard BullShift JSON payload.
    Json,
    /// Slack incoming webhook format.
    Slack,
    /// Discord webhook format.
    Discord,
    /// Generic form-encoded.
    FormEncoded,
}

impl std::fmt::Display for WebhookFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Slack => write!(f, "slack"),
            Self::Discord => write!(f, "discord"),
            Self::FormEncoded => write!(f, "form"),
        }
    }
}

/// Events that can trigger a webhook notification.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WebhookTrigger {
    OrderSubmitted,
    OrderFilled,
    OrderCancelled,
    OrderRejected,
    PositionOpened,
    PositionClosed,
    StopLossTriggered,
    TakeProfitTriggered,
    SentimentAlert,
    PriceAlert,
    SystemError,
    AuditEvent,
}

impl std::fmt::Display for WebhookTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OrderSubmitted => write!(f, "order.submitted"),
            Self::OrderFilled => write!(f, "order.filled"),
            Self::OrderCancelled => write!(f, "order.cancelled"),
            Self::OrderRejected => write!(f, "order.rejected"),
            Self::PositionOpened => write!(f, "position.opened"),
            Self::PositionClosed => write!(f, "position.closed"),
            Self::StopLossTriggered => write!(f, "stop_loss.triggered"),
            Self::TakeProfitTriggered => write!(f, "take_profit.triggered"),
            Self::SentimentAlert => write!(f, "sentiment.alert"),
            Self::PriceAlert => write!(f, "price.alert"),
            Self::SystemError => write!(f, "system.error"),
            Self::AuditEvent => write!(f, "audit.event"),
        }
    }
}

/// Configuration for a registered webhook endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub format: WebhookFormat,
    pub triggers: Vec<WebhookTrigger>,
    pub secret: Option<String>,
    pub headers: HashMap<String, String>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub retry_count: u32,
    pub timeout_ms: u64,
}

/// Delivery record for a webhook invocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDelivery {
    pub id: Uuid,
    pub webhook_id: Uuid,
    pub trigger: WebhookTrigger,
    pub status_code: Option<u16>,
    pub success: bool,
    pub error: Option<String>,
    pub response_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

/// Manages webhook registrations and dispatches notifications.
pub struct WebhookManager {
    webhooks: HashMap<Uuid, WebhookConfig>,
    deliveries: std::collections::VecDeque<WebhookDelivery>,
    client: Client,
    max_deliveries: usize,
}

impl Default for WebhookManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WebhookManager {
    pub fn new() -> Self {
        Self {
            webhooks: HashMap::new(),
            deliveries: std::collections::VecDeque::with_capacity(1000),
            client: Client::new(),
            max_deliveries: 1000,
        }
    }

    /// Register a new webhook endpoint.
    pub fn register(&mut self, config: WebhookConfig) -> Uuid {
        let id = config.id;
        self.webhooks.insert(id, config);
        id
    }

    /// Create and register a webhook with minimal config.
    pub fn add_webhook(
        &mut self,
        name: &str,
        url: &str,
        format: WebhookFormat,
        triggers: Vec<WebhookTrigger>,
    ) -> Uuid {
        let config = WebhookConfig {
            id: Uuid::new_v4(),
            name: name.to_string(),
            url: url.to_string(),
            format,
            triggers,
            secret: None,
            headers: HashMap::new(),
            enabled: true,
            created_at: Utc::now(),
            retry_count: 2,
            timeout_ms: 5000,
        };
        self.register(config)
    }

    /// Remove a webhook.
    pub fn remove(&mut self, id: &Uuid) -> bool {
        self.webhooks.remove(id).is_some()
    }

    /// Enable or disable a webhook.
    pub fn set_enabled(&mut self, id: &Uuid, enabled: bool) -> bool {
        if let Some(wh) = self.webhooks.get_mut(id) {
            wh.enabled = enabled;
            true
        } else {
            false
        }
    }

    /// List all registered webhooks.
    pub fn list(&self) -> Vec<&WebhookConfig> {
        self.webhooks.values().collect()
    }

    /// Get a webhook by ID.
    pub fn get(&self, id: &Uuid) -> Option<&WebhookConfig> {
        self.webhooks.get(id)
    }

    /// Get webhooks that match a specific trigger.
    pub fn webhooks_for_trigger(&self, trigger: &WebhookTrigger) -> Vec<&WebhookConfig> {
        self.webhooks
            .values()
            .filter(|wh| wh.enabled && wh.triggers.contains(trigger))
            .collect()
    }

    /// Dispatch a notification to all matching webhooks for a trade event.
    pub async fn notify_trade_event(&mut self, event: &TradeEvent) -> Vec<WebhookDelivery> {
        let trigger = trade_event_to_trigger(event);
        let matching: Vec<WebhookConfig> = self
            .webhooks
            .values()
            .filter(|wh| wh.enabled && wh.triggers.contains(&trigger))
            .cloned()
            .collect();

        let mut results = Vec::new();
        for webhook in &matching {
            let delivery = self.dispatch(webhook, &trigger, event).await;
            results.push(delivery);
        }
        results
    }

    /// Send a custom notification to all webhooks matching a trigger.
    pub async fn notify(
        &mut self,
        trigger: &WebhookTrigger,
        payload: &serde_json::Value,
    ) -> Vec<WebhookDelivery> {
        let matching: Vec<WebhookConfig> = self
            .webhooks
            .values()
            .filter(|wh| wh.enabled && wh.triggers.contains(trigger))
            .cloned()
            .collect();

        let mut results = Vec::new();
        for webhook in &matching {
            let delivery = self.dispatch(webhook, trigger, payload).await;
            results.push(delivery);
        }
        results
    }

    async fn dispatch<T: Serialize>(
        &mut self,
        webhook: &WebhookConfig,
        trigger: &WebhookTrigger,
        payload: &T,
    ) -> WebhookDelivery {
        let start = std::time::Instant::now();
        let mut last_error = None;
        let mut status_code = None;

        for attempt in 0..=webhook.retry_count {
            if attempt > 0 {
                tokio::time::sleep(std::time::Duration::from_millis(500 * 2u64.pow(attempt - 1))).await;
            }

            match self.send_request(webhook, trigger, payload).await {
                Ok(code) => {
                    status_code = Some(code);
                    if (200..300).contains(&code) {
                        let delivery = WebhookDelivery {
                            id: Uuid::new_v4(),
                            webhook_id: webhook.id,
                            trigger: trigger.clone(),
                            status_code: Some(code),
                            success: true,
                            error: None,
                            response_time_ms: start.elapsed().as_millis() as u64,
                            timestamp: Utc::now(),
                        };
                        self.record_delivery(delivery.clone());
                        return delivery;
                    }
                    last_error = Some(format!("HTTP {}", code));
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                }
            }
        }

        let delivery = WebhookDelivery {
            id: Uuid::new_v4(),
            webhook_id: webhook.id,
            trigger: trigger.clone(),
            status_code,
            success: false,
            error: last_error,
            response_time_ms: start.elapsed().as_millis() as u64,
            timestamp: Utc::now(),
        };
        self.record_delivery(delivery.clone());
        delivery
    }

    async fn send_request<T: Serialize>(
        &self,
        webhook: &WebhookConfig,
        trigger: &WebhookTrigger,
        payload: &T,
    ) -> Result<u16, BullShiftError> {
        let body = match webhook.format {
            WebhookFormat::Json => {
                let wrapper = serde_json::json!({
                    "event": trigger.to_string(),
                    "timestamp": Utc::now().to_rfc3339(),
                    "data": serde_json::to_value(payload).unwrap_or(serde_json::Value::Null),
                });
                serde_json::to_string(&wrapper)
                    .map_err(|e| BullShiftError::Api(format!("Serialize failed: {}", e)))?
            }
            WebhookFormat::Slack => {
                let text = format!(
                    "*BullShift Alert*\n`{}`\n```{}```",
                    trigger,
                    serde_json::to_string_pretty(payload).unwrap_or_default()
                );
                serde_json::json!({"text": text}).to_string()
            }
            WebhookFormat::Discord => {
                let content = format!(
                    "**BullShift Alert** — `{}`\n```json\n{}\n```",
                    trigger,
                    serde_json::to_string_pretty(payload).unwrap_or_default()
                );
                serde_json::json!({"content": content}).to_string()
            }
            WebhookFormat::FormEncoded => {
                let data = serde_json::to_string(payload).unwrap_or_default();
                format!("event={}&data={}", trigger, urlencoding::encode(&data))
            }
        };

        let content_type = match webhook.format {
            WebhookFormat::FormEncoded => "application/x-www-form-urlencoded",
            _ => "application/json",
        };

        let mut req = self
            .client
            .post(&webhook.url)
            .header("Content-Type", content_type)
            .header("User-Agent", "BullShift/2026.3.6")
            .header("X-BullShift-Event", trigger.to_string())
            .timeout(std::time::Duration::from_millis(webhook.timeout_ms));

        // Add HMAC signature if secret is configured
        if let Some(ref secret) = webhook.secret {
            let signature = compute_hmac_signature(secret, &body);
            req = req.header("X-BullShift-Signature", signature);
        }

        // Add custom headers
        for (key, value) in &webhook.headers {
            req = req.header(key.as_str(), value.as_str());
        }

        req = req.body(body);

        let resp = req
            .send()
            .await
            .map_err(|e| BullShiftError::Network(format!("Webhook delivery failed: {}", e)))?;

        Ok(resp.status().as_u16())
    }

    fn record_delivery(&mut self, delivery: WebhookDelivery) {
        if self.deliveries.len() >= self.max_deliveries {
            self.deliveries.pop_front();
        }
        self.deliveries.push_back(delivery);
    }

    /// Get recent delivery history.
    pub fn recent_deliveries(&self, limit: usize) -> Vec<&WebhookDelivery> {
        self.deliveries.iter().rev().take(limit).collect()
    }

    /// Get deliveries for a specific webhook.
    pub fn deliveries_for(&self, webhook_id: &Uuid) -> Vec<&WebhookDelivery> {
        self.deliveries
            .iter()
            .filter(|d| d.webhook_id == *webhook_id)
            .collect()
    }
}

fn trade_event_to_trigger(event: &TradeEvent) -> WebhookTrigger {
    use crate::integration::TradeEventType;
    match event.event_type {
        TradeEventType::OrderSubmitted => WebhookTrigger::OrderSubmitted,
        TradeEventType::OrderFilled => WebhookTrigger::OrderFilled,
        TradeEventType::OrderPartiallyFilled => WebhookTrigger::OrderFilled,
        TradeEventType::OrderCancelled => WebhookTrigger::OrderCancelled,
        TradeEventType::OrderRejected => WebhookTrigger::OrderRejected,
        TradeEventType::PositionOpened => WebhookTrigger::PositionOpened,
        TradeEventType::PositionClosed => WebhookTrigger::PositionClosed,
        TradeEventType::PositionUpdated => WebhookTrigger::PositionOpened,
        TradeEventType::StopLossTriggered => WebhookTrigger::StopLossTriggered,
        TradeEventType::TakeProfitTriggered => WebhookTrigger::TakeProfitTriggered,
    }
}

/// Compute HMAC-SHA256 signature for webhook payload verification.
fn compute_hmac_signature(secret: &str, body: &str) -> String {
    use ring::hmac;
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let tag = hmac::sign(&key, body.as_bytes());
    format!("sha256={}", hex::encode(tag.as_ref()))
}

/// URL-encode helper (minimal, for form-encoded payloads).
mod urlencoding {
    pub fn encode(input: &str) -> String {
        let mut result = String::with_capacity(input.len() * 3);
        for byte in input.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    result.push(byte as char);
                }
                _ => {
                    result.push('%');
                    result.push_str(&format!("{:02X}", byte));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_list_webhooks() {
        let mut mgr = WebhookManager::new();
        let id = mgr.add_webhook(
            "Slack alerts",
            "https://hooks.slack.com/xxx",
            WebhookFormat::Slack,
            vec![
                WebhookTrigger::OrderFilled,
                WebhookTrigger::StopLossTriggered,
            ],
        );

        assert_eq!(mgr.list().len(), 1);
        let wh = mgr.get(&id).unwrap();
        assert_eq!(wh.name, "Slack alerts");
        assert!(wh.enabled);
    }

    #[test]
    fn test_remove_webhook() {
        let mut mgr = WebhookManager::new();
        let id = mgr.add_webhook(
            "test",
            "https://example.com",
            WebhookFormat::Json,
            vec![WebhookTrigger::OrderSubmitted],
        );
        assert!(mgr.remove(&id));
        assert_eq!(mgr.list().len(), 0);
        assert!(!mgr.remove(&id));
    }

    #[test]
    fn test_enable_disable() {
        let mut mgr = WebhookManager::new();
        let id = mgr.add_webhook(
            "test",
            "https://example.com",
            WebhookFormat::Json,
            vec![WebhookTrigger::OrderFilled],
        );

        assert!(mgr.get(&id).unwrap().enabled);
        mgr.set_enabled(&id, false);
        assert!(!mgr.get(&id).unwrap().enabled);
    }

    #[test]
    fn test_webhooks_for_trigger() {
        let mut mgr = WebhookManager::new();
        mgr.add_webhook(
            "fills",
            "https://a.com",
            WebhookFormat::Json,
            vec![WebhookTrigger::OrderFilled],
        );
        mgr.add_webhook(
            "all orders",
            "https://b.com",
            WebhookFormat::Slack,
            vec![
                WebhookTrigger::OrderSubmitted,
                WebhookTrigger::OrderFilled,
                WebhookTrigger::OrderCancelled,
            ],
        );
        mgr.add_webhook(
            "sentiment",
            "https://c.com",
            WebhookFormat::Discord,
            vec![WebhookTrigger::SentimentAlert],
        );

        let fill_hooks = mgr.webhooks_for_trigger(&WebhookTrigger::OrderFilled);
        assert_eq!(fill_hooks.len(), 2);

        let sentiment_hooks = mgr.webhooks_for_trigger(&WebhookTrigger::SentimentAlert);
        assert_eq!(sentiment_hooks.len(), 1);

        let price_hooks = mgr.webhooks_for_trigger(&WebhookTrigger::PriceAlert);
        assert_eq!(price_hooks.len(), 0);
    }

    #[test]
    fn test_disabled_webhook_not_matched() {
        let mut mgr = WebhookManager::new();
        let id = mgr.add_webhook(
            "test",
            "https://example.com",
            WebhookFormat::Json,
            vec![WebhookTrigger::OrderFilled],
        );
        mgr.set_enabled(&id, false);

        assert_eq!(
            mgr.webhooks_for_trigger(&WebhookTrigger::OrderFilled).len(),
            0
        );
    }

    #[test]
    fn test_hmac_signature() {
        let sig = compute_hmac_signature("my-secret", r#"{"event":"test"}"#);
        assert!(sig.starts_with("sha256="));
        assert_eq!(sig.len(), 7 + 64); // "sha256=" + 64 hex chars
    }

    #[test]
    fn test_webhook_format_display() {
        assert_eq!(WebhookFormat::Json.to_string(), "json");
        assert_eq!(WebhookFormat::Slack.to_string(), "slack");
        assert_eq!(WebhookFormat::Discord.to_string(), "discord");
    }

    #[test]
    fn test_webhook_trigger_display() {
        assert_eq!(WebhookTrigger::OrderFilled.to_string(), "order.filled");
        assert_eq!(
            WebhookTrigger::SentimentAlert.to_string(),
            "sentiment.alert"
        );
    }

    #[test]
    fn test_url_encoding() {
        assert_eq!(urlencoding::encode("hello world"), "hello%20world");
        assert_eq!(urlencoding::encode("a=b&c=d"), "a%3Db%26c%3Dd");
        assert_eq!(
            urlencoding::encode("safe-text_here.ok~"),
            "safe-text_here.ok~"
        );
    }

    #[test]
    fn test_delivery_tracking() {
        let mut mgr = WebhookManager::new();
        let webhook_id = mgr.add_webhook(
            "test",
            "https://example.com",
            WebhookFormat::Json,
            vec![WebhookTrigger::OrderFilled],
        );

        // Manually record a delivery
        let delivery = WebhookDelivery {
            id: Uuid::new_v4(),
            webhook_id,
            trigger: WebhookTrigger::OrderFilled,
            status_code: Some(200),
            success: true,
            error: None,
            response_time_ms: 42,
            timestamp: Utc::now(),
        };
        mgr.record_delivery(delivery.clone());

        let recent = mgr.recent_deliveries(10);
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].webhook_id, webhook_id);
        assert!(recent[0].success);
        assert_eq!(recent[0].status_code, Some(200));

        let for_hook = mgr.deliveries_for(&webhook_id);
        assert_eq!(for_hook.len(), 1);

        // Deliveries for a different webhook should be empty
        let other_id = Uuid::new_v4();
        assert_eq!(mgr.deliveries_for(&other_id).len(), 0);
    }

    #[test]
    fn test_multiple_triggers_same_webhook() {
        let mut mgr = WebhookManager::new();
        let id = mgr.add_webhook(
            "multi-trigger",
            "https://example.com/hook",
            WebhookFormat::Json,
            vec![
                WebhookTrigger::OrderSubmitted,
                WebhookTrigger::OrderFilled,
                WebhookTrigger::OrderCancelled,
                WebhookTrigger::PositionOpened,
            ],
        );

        // Should match each registered trigger
        for trigger in &[
            WebhookTrigger::OrderSubmitted,
            WebhookTrigger::OrderFilled,
            WebhookTrigger::OrderCancelled,
            WebhookTrigger::PositionOpened,
        ] {
            let matched = mgr.webhooks_for_trigger(trigger);
            assert_eq!(matched.len(), 1, "Should match trigger {:?}", trigger);
            assert_eq!(matched[0].id, id);
        }

        // Should NOT match unregistered triggers
        assert_eq!(
            mgr.webhooks_for_trigger(&WebhookTrigger::SystemError).len(),
            0
        );
        assert_eq!(
            mgr.webhooks_for_trigger(&WebhookTrigger::PriceAlert).len(),
            0
        );
    }

    #[test]
    fn test_webhook_secret_stored() {
        let mut mgr = WebhookManager::new();
        let config = WebhookConfig {
            id: Uuid::new_v4(),
            name: "signed-hook".to_string(),
            url: "https://example.com/signed".to_string(),
            format: WebhookFormat::Json,
            triggers: vec![WebhookTrigger::OrderFilled],
            secret: Some("super-secret-key-123".to_string()),
            headers: HashMap::new(),
            enabled: true,
            created_at: Utc::now(),
            retry_count: 3,
            timeout_ms: 10000,
        };
        let id = mgr.register(config);

        let wh = mgr.get(&id).unwrap();
        assert_eq!(wh.secret, Some("super-secret-key-123".to_string()));
        assert_eq!(wh.retry_count, 3);
        assert_eq!(wh.timeout_ms, 10000);

        // Webhook without secret should have None
        let id2 = mgr.add_webhook(
            "no-secret",
            "https://example.com",
            WebhookFormat::Json,
            vec![WebhookTrigger::OrderFilled],
        );
        assert_eq!(mgr.get(&id2).unwrap().secret, None);
    }

    #[test]
    fn test_format_payload_json() {
        // Verify the JSON wrapper structure produced by the webhook format
        let payload = serde_json::json!({"symbol": "AAPL", "price": 150.0});
        let trigger = WebhookTrigger::OrderFilled;
        let wrapper = serde_json::json!({
            "event": trigger.to_string(),
            "timestamp": Utc::now().to_rfc3339(),
            "data": payload,
        });
        let body = serde_json::to_string(&wrapper).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();

        assert_eq!(parsed["event"], "order.filled");
        assert!(parsed["timestamp"].is_string());
        assert_eq!(parsed["data"]["symbol"], "AAPL");
        assert_eq!(parsed["data"]["price"], 150.0);
    }

    #[test]
    fn test_format_payload_slack() {
        // Verify Slack format produces {"text": "..."} with expected content
        let payload = serde_json::json!({"symbol": "TSLA", "action": "buy"});
        let trigger = WebhookTrigger::StopLossTriggered;
        let text = format!(
            "*BullShift Alert*\n`{}`\n```{}```",
            trigger,
            serde_json::to_string_pretty(&payload).unwrap_or_default()
        );
        let slack_body = serde_json::json!({"text": text});
        let body_str = slack_body.to_string();
        let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        assert!(parsed["text"].is_string());
        let text_val = parsed["text"].as_str().unwrap();
        assert!(text_val.contains("*BullShift Alert*"));
        assert!(text_val.contains("stop_loss.triggered"));
        assert!(text_val.contains("TSLA"));
    }

    #[test]
    fn test_all_trigger_display_values() {
        assert_eq!(WebhookTrigger::OrderFilled.to_string(), "order.filled");
        assert_eq!(WebhookTrigger::OrderCancelled.to_string(), "order.cancelled");
        assert_eq!(WebhookTrigger::PriceAlert.to_string(), "price.alert");
        assert_eq!(
            WebhookTrigger::StopLossTriggered.to_string(),
            "stop_loss.triggered"
        );
        assert_eq!(
            WebhookTrigger::TakeProfitTriggered.to_string(),
            "take_profit.triggered"
        );
    }

    #[test]
    fn test_webhook_format_variants() {
        let json = WebhookFormat::Json;
        let slack = WebhookFormat::Slack;
        let form = WebhookFormat::FormEncoded;
        assert_ne!(format!("{:?}", json), format!("{:?}", slack));
        assert_ne!(format!("{:?}", slack), format!("{:?}", form));
    }

    #[test]
    fn test_register_and_list_webhooks() {
        let mut mgr = WebhookManager::new();
        let id1 = mgr.add_webhook(
            "hook1",
            "https://example.com/1",
            WebhookFormat::Json,
            vec![WebhookTrigger::OrderFilled],
        );
        let id2 = mgr.add_webhook(
            "hook2",
            "https://example.com/2",
            WebhookFormat::Slack,
            vec![WebhookTrigger::PriceAlert],
        );
        assert_eq!(mgr.list().len(), 2);
        assert!(mgr.get(&id1).is_some());
        assert!(mgr.get(&id2).is_some());
    }

    #[test]
    fn test_remove_and_verify_webhook() {
        let mut mgr = WebhookManager::new();
        let id = mgr.add_webhook(
            "to-remove",
            "https://example.com",
            WebhookFormat::Json,
            vec![WebhookTrigger::OrderFilled],
        );
        assert_eq!(mgr.list().len(), 1);
        assert!(mgr.remove(&id));
        assert!(mgr.list().is_empty());
        assert!(!mgr.remove(&id)); // already removed
    }

    #[test]
    fn test_get_nonexistent_webhook() {
        let mgr = WebhookManager::new();
        assert!(mgr.get(&Uuid::new_v4()).is_none());
    }

    #[test]
    fn test_webhook_delivery_struct() {
        let delivery = WebhookDelivery {
            id: Uuid::new_v4(),
            webhook_id: Uuid::new_v4(),
            trigger: WebhookTrigger::OrderFilled,
            status_code: Some(200),
            success: true,
            error: None,
            response_time_ms: 42,
            timestamp: Utc::now(),
        };
        assert!(delivery.success);
        assert_eq!(delivery.status_code, Some(200));
        assert!(delivery.error.is_none());
    }

    #[test]
    fn test_webhook_delivery_failure() {
        let delivery = WebhookDelivery {
            id: Uuid::new_v4(),
            webhook_id: Uuid::new_v4(),
            trigger: WebhookTrigger::PriceAlert,
            status_code: Some(500),
            success: false,
            error: Some("Internal Server Error".to_string()),
            response_time_ms: 1500,
            timestamp: Utc::now(),
        };
        assert!(!delivery.success);
        assert_eq!(delivery.error.as_deref(), Some("Internal Server Error"));
    }
}
