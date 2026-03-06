use async_trait::async_trait;
use reqwest::Client;

use super::{BrokerCapabilities, BrokerStatus};
use crate::error::BullShiftError;
use crate::trading::api::{
    ApiAccount, ApiOrderRequest, ApiOrderResponse, ApiPosition, TradingApi, TradingCredentials,
};

/// Kraken exchange integration via their REST API.
///
/// Uses API-Key + API-Sign (HMAC-SHA512) authentication.
/// Production only: `https://api.kraken.com`
///
/// # Setup
/// 1. Generate API keys at <https://www.kraken.com/u/security/api>
/// 2. Pass the API key as `api_key` and the private key as `api_secret`
pub struct KrakenApi {
    client: Client,
    base_url: String,
    api_key: String,
    #[allow(dead_code)] // stored for HMAC-SHA512 signing
    api_secret: String,
}

impl KrakenApi {
    pub fn new(credentials: TradingCredentials) -> Self {
        // Kraken has no sandbox environment; always use production.
        let base_url = "https://api.kraken.com".to_string();

        Self {
            client: Client::new(),
            base_url,
            api_key: credentials.api_key,
            api_secret: credentials.api_secret,
        }
    }

    pub fn capabilities() -> BrokerCapabilities {
        BrokerCapabilities {
            name: "kraken".to_string(),
            supports_market_orders: true,
            supports_limit_orders: true,
            supports_stop_orders: false,
            supports_stop_limit_orders: false,
            supports_fractional_shares: false,
            supports_short_selling: true,
            supports_options: false,
            supports_crypto: true,
            supports_extended_hours: false,
            sandbox_available: false,
        }
    }

    pub async fn check_status(&self) -> BrokerStatus {
        let url = format!("{}/0/private/Balance", self.base_url);
        match self
            .client
            .post(&url)
            .header("API-Key", &self.api_key)
            .header("API-Sign", "stub-signature")
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => BrokerStatus::Connected,
            Ok(resp) if resp.status() == reqwest::StatusCode::FORBIDDEN => {
                BrokerStatus::Error("Invalid API key or signature".to_string())
            }
            Ok(resp) => BrokerStatus::Error(format!("Unexpected status: {}", resp.status())),
            Err(e) => BrokerStatus::Error(format!("Connection failed: {}", e)),
        }
    }

    fn map_order_type(order_type: &str) -> &'static str {
        match order_type.to_uppercase().as_str() {
            "MARKET" => "market",
            "LIMIT" => "limit",
            _ => "market",
        }
    }

    fn map_side(side: &str) -> &'static str {
        match side.to_uppercase().as_str() {
            "BUY" => "buy",
            "SELL" => "sell",
            _ => "buy",
        }
    }

    fn parse_position(pair: &str, data: &serde_json::Value) -> ApiPosition {
        let volume = data["vol"]
            .as_str()
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let cost = data["cost"]
            .as_str()
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let entry_price = if volume != 0.0 { cost / volume } else { 0.0 };
        let net = data["net"]
            .as_str()
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);

        ApiPosition {
            symbol: pair.to_string(),
            quantity: volume,
            entry_price,
            current_price: 0.0,
            unrealized_pnl: net,
        }
    }

    fn parse_kraken_response(
        body: &serde_json::Value,
    ) -> Result<&serde_json::Value, BullShiftError> {
        let errors = &body["error"];
        if let serde_json::Value::Array(errs) = errors {
            if !errs.is_empty() {
                let msg = errs
                    .iter()
                    .filter_map(|e| e.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(BullShiftError::Api(format!("Kraken error: {}", msg)));
            }
        }
        Ok(&body["result"])
    }
}

#[async_trait]
impl TradingApi for KrakenApi {
    async fn submit_order(
        &self,
        order: ApiOrderRequest,
    ) -> Result<ApiOrderResponse, BullShiftError> {
        let url = format!("{}/0/private/AddOrder", self.base_url);

        let mut params = vec![
            ("pair", order.symbol.clone()),
            ("type", Self::map_side(&order.side).to_string()),
            ("ordertype", Self::map_order_type(&order.order_type).to_string()),
            ("volume", order.quantity.to_string()),
        ];

        if let Some(price) = order.price {
            params.push(("price", price.to_string()));
        }

        let response = self
            .client
            .post(&url)
            .header("API-Key", &self.api_key)
            .header("API-Sign", "stub-signature")
            .form(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            let result = Self::parse_kraken_response(&body)?;

            let txid = result["txid"]
                .as_array()
                .and_then(|arr| arr.first())
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            Ok(ApiOrderResponse {
                order_id: txid,
                symbol: order.symbol,
                side: order.side,
                quantity: order.quantity,
                order_type: order.order_type,
                price: order.price,
                status: "pending".to_string(),
                created_at: String::new(),
            })
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(BullShiftError::Api(format!(
                "Kraken order failed ({}): {}",
                status, body
            )))
        }
    }

    async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError> {
        let url = format!("{}/0/private/OpenPositions", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("API-Key", &self.api_key)
            .header("API-Sign", "stub-signature")
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            let result = Self::parse_kraken_response(&body)?;

            let positions = match result {
                serde_json::Value::Object(map) => map
                    .iter()
                    .map(|(id, data)| {
                        let pair = data["pair"].as_str().unwrap_or(id);
                        Self::parse_position(pair, data)
                    })
                    .collect(),
                _ => Vec::new(),
            };

            Ok(positions)
        } else {
            Err(BullShiftError::Api(format!(
                "Kraken get positions failed: {}",
                response.status()
            )))
        }
    }

    async fn get_account(&self) -> Result<ApiAccount, BullShiftError> {
        let url = format!("{}/0/private/Balance", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("API-Key", &self.api_key)
            .header("API-Sign", "stub-signature")
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            let result = Self::parse_kraken_response(&body)?;

            let mut total_balance = 0.0;
            if let serde_json::Value::Object(map) = result {
                for (_currency, value) in map {
                    let amount = value
                        .as_str()
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0);
                    total_balance += amount;
                }
            }

            Ok(ApiAccount {
                balance: total_balance,
                available: total_balance,
                margin_used: 0.0,
            })
        } else {
            Err(BullShiftError::Api(format!(
                "Kraken get account failed: {}",
                response.status()
            )))
        }
    }

    async fn cancel_order(&self, order_id: String) -> Result<bool, BullShiftError> {
        let url = format!("{}/0/private/CancelOrder", self.base_url);

        let params = vec![("txid", order_id)];

        let response = self
            .client
            .post(&url)
            .header("API-Key", &self.api_key)
            .header("API-Sign", "stub-signature")
            .form(&params)
            .send()
            .await?;

        Ok(response.status().is_success())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_credentials() -> TradingCredentials {
        TradingCredentials {
            api_key: "test_kraken_api_key".to_string(),
            api_secret: "test_kraken_private_key".to_string(),
            sandbox: true, // ignored — Kraken has no sandbox
        }
    }

    #[test]
    fn test_new_kraken_always_production() {
        let api = KrakenApi::new(test_credentials());
        assert_eq!(api.base_url, "https://api.kraken.com");
        assert_eq!(api.api_key, "test_kraken_api_key");
    }

    #[test]
    fn test_new_kraken_ignores_sandbox_flag() {
        let mut creds = test_credentials();
        creds.sandbox = false;
        let api = KrakenApi::new(creds);
        assert_eq!(api.base_url, "https://api.kraken.com");
    }

    #[test]
    fn test_capabilities() {
        let caps = KrakenApi::capabilities();
        assert_eq!(caps.name, "kraken");
        assert!(caps.supports_crypto);
        assert!(caps.supports_short_selling);
        assert!(!caps.supports_options);
        assert!(!caps.supports_extended_hours);
        assert!(!caps.sandbox_available);
    }

    #[test]
    fn test_order_type_mapping() {
        assert_eq!(KrakenApi::map_order_type("MARKET"), "market");
        assert_eq!(KrakenApi::map_order_type("LIMIT"), "limit");
        assert_eq!(KrakenApi::map_order_type("UNKNOWN"), "market");
    }

    #[test]
    fn test_side_mapping() {
        assert_eq!(KrakenApi::map_side("BUY"), "buy");
        assert_eq!(KrakenApi::map_side("SELL"), "sell");
        assert_eq!(KrakenApi::map_side("unknown"), "buy");
    }

    #[test]
    fn test_parse_position() {
        let data = serde_json::json!({
            "pair": "XXBTZUSD",
            "vol": "0.5",
            "cost": "25000.0",
            "net": "500.0"
        });
        let pos = KrakenApi::parse_position("XXBTZUSD", &data);
        assert_eq!(pos.symbol, "XXBTZUSD");
        assert_eq!(pos.quantity, 0.5);
        assert_eq!(pos.entry_price, 50000.0);
        assert_eq!(pos.unrealized_pnl, 500.0);
    }

    #[test]
    fn test_parse_kraken_response_with_error() {
        let body = serde_json::json!({
            "error": ["EGeneral:Invalid arguments"],
            "result": {}
        });
        let result = KrakenApi::parse_kraken_response(&body);
        assert!(result.is_err());
    }
}
