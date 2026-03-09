use async_trait::async_trait;
use reqwest::Client;

use super::{BrokerCapabilities, BrokerStatus};
use crate::error::BullShiftError;
use crate::trading::api::{
    ApiAccount, ApiOrderRequest, ApiOrderResponse, ApiPosition, TradingApi, TradingCredentials,
};

/// Webull brokerage integration via their REST API.
///
/// Uses access_token + device ID (did) authentication.
/// Production: `https://tradeapi.webull.com` (trading), `https://quoteapi.webull.com` (quotes)
///
/// # Setup
/// 1. Obtain access token and device ID via Webull authentication flow
/// 2. Pass the access token as `api_key` and device ID as `api_secret`
pub struct WebullApi {
    client: Client,
    trade_url: String,
    access_token: String,
    device_id: String,
    account_id: String,
}

impl WebullApi {
    pub fn new(credentials: TradingCredentials) -> Self {
        // Webull has no sandbox environment; always use production.
        let trade_url = "https://tradeapi.webull.com".to_string();

        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .connect_timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_else(|_| Client::new()),
            trade_url,
            access_token: credentials.api_key.clone(),
            device_id: credentials.api_secret.clone(),
            account_id: String::new(),
        }
    }

    pub fn capabilities() -> BrokerCapabilities {
        BrokerCapabilities {
            name: "webull".to_string(),
            supports_market_orders: true,
            supports_limit_orders: true,
            supports_stop_orders: true,
            supports_stop_limit_orders: true,
            supports_fractional_shares: true,
            supports_short_selling: false,
            supports_options: true,
            supports_crypto: true,
            supports_extended_hours: true,
            sandbox_available: false,
        }
    }

    pub async fn check_status(&self) -> BrokerStatus {
        let url = format!("{}/api/trade/v2/home/{}", self.trade_url, self.account_id);
        match self
            .client
            .get(&url)
            .header("access_token", &self.access_token)
            .header("did", &self.device_id)
            .header("Accept", "application/json")
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => BrokerStatus::Connected,
            Ok(resp) if resp.status() == reqwest::StatusCode::UNAUTHORIZED => {
                BrokerStatus::Error("Invalid or expired access token".to_string())
            }
            Ok(resp) => BrokerStatus::Error(format!("Unexpected status: {}", resp.status())),
            Err(e) => BrokerStatus::Error(format!("Connection failed: {}", e)),
        }
    }

    fn map_order_type(order_type: &str) -> &'static str {
        match order_type.to_uppercase().as_str() {
            "MARKET" => "MKT",
            "LIMIT" => "LMT",
            "STOP" => "STP",
            "STOP_LIMIT" => "STP LMT",
            other => {
                log::warn!("Unknown order type '{}', defaulting to MKT", other);
                "MKT"
            }
        }
    }

    fn map_time_in_force(tif: Option<&str>) -> &'static str {
        match tif.unwrap_or("DAY").to_uppercase().as_str() {
            "GTC" => "GTC",
            "IOC" => "IOC",
            other => {
                log::warn!("Unknown time-in-force '{}', defaulting to DAY", other);
                "DAY"
            }
        }
    }

    fn parse_position(p: &serde_json::Value) -> ApiPosition {
        let quantity = p["position"].as_f64().unwrap_or(0.0);
        let cost = p["costPrice"].as_f64().unwrap_or(0.0);
        let last = p["lastPrice"].as_f64().unwrap_or(0.0);
        let pnl = p["unrealizedProfitLoss"].as_f64().unwrap_or(0.0);

        ApiPosition {
            symbol: p["ticker"]["symbol"].as_str().unwrap_or("").to_string(),
            quantity,
            entry_price: cost,
            current_price: last,
            unrealized_pnl: pnl,
        }
    }
}

#[async_trait]
impl TradingApi for WebullApi {
    async fn submit_order(
        &self,
        order: ApiOrderRequest,
    ) -> Result<ApiOrderResponse, BullShiftError> {
        let url = format!("{}/api/trade/v2/option/placeOrder", self.trade_url);

        let mut order_body = serde_json::json!({
            "action": order.side.to_uppercase(),
            "orderType": Self::map_order_type(&order.order_type),
            "timeInForce": Self::map_time_in_force(order.time_in_force.as_deref()),
            "quantity": order.quantity.to_string(),
            "ticker": {
                "symbol": order.symbol
            }
        });

        if let Some(price) = order.price {
            order_body["lmtPrice"] = serde_json::json!(price);
        }

        let response = self
            .client
            .post(&url)
            .header("access_token", &self.access_token)
            .header("did", &self.device_id)
            .header("Content-Type", "application/json")
            .json(&order_body)
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;

            let order_id = body["orderId"]
                .as_u64()
                .map(|id| id.to_string())
                .or_else(|| body["orderId"].as_str().map(String::from))
                .unwrap_or_default();

            Ok(ApiOrderResponse {
                order_id,
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
                "Webull order failed ({}): {}",
                status, body
            )))
        }
    }

    async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError> {
        let url = format!("{}/api/trade/v2/option/list", self.trade_url);

        let response = self
            .client
            .get(&url)
            .header("access_token", &self.access_token)
            .header("did", &self.device_id)
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;

            let positions = match &body {
                serde_json::Value::Array(arr) => arr.iter().map(Self::parse_position).collect(),
                _ => {
                    // Some endpoints wrap in a "positions" key
                    match &body["positions"] {
                        serde_json::Value::Array(arr) => {
                            arr.iter().map(Self::parse_position).collect()
                        }
                        _ => Vec::new(),
                    }
                }
            };

            Ok(positions)
        } else {
            Err(BullShiftError::Api(format!(
                "Webull get positions failed: {}",
                response.status()
            )))
        }
    }

    async fn get_account(&self) -> Result<ApiAccount, BullShiftError> {
        let url = format!("{}/api/trade/v2/home/{}", self.trade_url, self.account_id);

        let response = self
            .client
            .get(&url)
            .header("access_token", &self.access_token)
            .header("did", &self.device_id)
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            let account = &body["accountMembers"];

            Ok(ApiAccount {
                balance: account["totalMarketValue"].as_f64().unwrap_or(0.0),
                available: account["dayBuyingPower"].as_f64().unwrap_or(0.0),
                margin_used: account["usedMargin"].as_f64().unwrap_or(0.0),
            })
        } else {
            Err(BullShiftError::Api(format!(
                "Webull get account failed: {}",
                response.status()
            )))
        }
    }

    async fn cancel_order(&self, order_id: String) -> Result<bool, BullShiftError> {
        if !order_id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(BullShiftError::Validation(format!(
                "Invalid order_id format: {}",
                order_id
            )));
        }
        let url = format!(
            "{}/api/trade/v2/option/cancelOrder/{}",
            self.trade_url, order_id
        );

        let response = self
            .client
            .post(&url)
            .header("access_token", &self.access_token)
            .header("did", &self.device_id)
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
            api_key: "test_access_token".to_string(),
            api_secret: "test_device_id".to_string(),
            sandbox: false,
        }
    }

    #[test]
    fn test_new_webull() {
        let api = WebullApi::new(test_credentials());
        assert_eq!(api.trade_url, "https://tradeapi.webull.com");
        assert_eq!(api.access_token, "test_access_token");
        assert_eq!(api.device_id, "test_device_id");
    }

    #[test]
    fn test_new_webull_ignores_sandbox() {
        let mut creds = test_credentials();
        creds.sandbox = true;
        let api = WebullApi::new(creds);
        assert_eq!(api.trade_url, "https://tradeapi.webull.com");
    }

    #[test]
    fn test_capabilities() {
        let caps = WebullApi::capabilities();
        assert_eq!(caps.name, "webull");
        assert!(caps.supports_options);
        assert!(caps.supports_fractional_shares);
        assert!(caps.supports_crypto);
        assert!(caps.supports_extended_hours);
        assert!(!caps.supports_short_selling);
        assert!(!caps.sandbox_available);
    }

    #[test]
    fn test_order_type_mapping() {
        assert_eq!(WebullApi::map_order_type("MARKET"), "MKT");
        assert_eq!(WebullApi::map_order_type("LIMIT"), "LMT");
        assert_eq!(WebullApi::map_order_type("STOP"), "STP");
        assert_eq!(WebullApi::map_order_type("STOP_LIMIT"), "STP LMT");
        assert_eq!(WebullApi::map_order_type("UNKNOWN"), "MKT");
    }

    #[test]
    fn test_time_in_force_mapping() {
        assert_eq!(WebullApi::map_time_in_force(None), "DAY");
        assert_eq!(WebullApi::map_time_in_force(Some("GTC")), "GTC");
        assert_eq!(WebullApi::map_time_in_force(Some("IOC")), "IOC");
        assert_eq!(WebullApi::map_time_in_force(Some("DAY")), "DAY");
    }

    #[test]
    fn test_parse_position() {
        let json = serde_json::json!({
            "ticker": { "symbol": "AAPL" },
            "position": 50.0,
            "costPrice": 150.0,
            "lastPrice": 175.0,
            "unrealizedProfitLoss": 1250.0
        });
        let pos = WebullApi::parse_position(&json);
        assert_eq!(pos.symbol, "AAPL");
        assert_eq!(pos.quantity, 50.0);
        assert_eq!(pos.entry_price, 150.0);
        assert_eq!(pos.current_price, 175.0);
        assert_eq!(pos.unrealized_pnl, 1250.0);
    }

    #[test]
    fn test_parse_position_missing_fields() {
        let json = serde_json::json!({});
        let pos = WebullApi::parse_position(&json);
        assert_eq!(pos.symbol, "");
        assert_eq!(pos.quantity, 0.0);
        assert_eq!(pos.entry_price, 0.0);
    }
}
