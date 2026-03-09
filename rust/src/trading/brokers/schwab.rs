use async_trait::async_trait;
use reqwest::Client;

use super::{BrokerCapabilities, BrokerStatus};
use crate::error::BullShiftError;
use crate::trading::api::{
    ApiAccount, ApiOrderRequest, ApiOrderResponse, ApiPosition, TradingApi, TradingCredentials,
};

/// Charles Schwab brokerage integration (formerly TD Ameritrade).
///
/// Uses OAuth2 Bearer token authentication.
/// Sandbox: `https://sandbox.schwabapi.com`
/// Production: `https://api.schwabapi.com`
///
/// # Setup
/// 1. Register at <https://developer.schwab.com>
/// 2. Obtain an OAuth2 access token
/// 3. Pass the token as `api_key` and your account ID as `api_secret`
pub struct SchwabApi {
    client: Client,
    base_url: String,
    access_token: String,
    account_id: String,
}

impl SchwabApi {
    pub fn new(credentials: TradingCredentials) -> Self {
        let base_url = if credentials.sandbox {
            "https://sandbox.schwabapi.com".to_string()
        } else {
            "https://api.schwabapi.com".to_string()
        };

        Self {
            client: Client::new(),
            base_url,
            access_token: credentials.api_key,
            account_id: credentials.api_secret,
        }
    }

    pub fn capabilities() -> BrokerCapabilities {
        BrokerCapabilities {
            name: "schwab".to_string(),
            supports_market_orders: true,
            supports_limit_orders: true,
            supports_stop_orders: true,
            supports_stop_limit_orders: true,
            supports_fractional_shares: false,
            supports_short_selling: true,
            supports_options: true,
            supports_crypto: false,
            supports_extended_hours: true,
            sandbox_available: true,
        }
    }

    pub async fn check_status(&self) -> BrokerStatus {
        let url = format!("{}/v1/accounts/{}", self.base_url, self.account_id);
        match self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
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
            "MARKET" => "MARKET",
            "LIMIT" => "LIMIT",
            "STOP" => "STOP",
            "STOP_LIMIT" => "STOP_LIMIT",
            _ => "MARKET",
        }
    }

    fn map_time_in_force(tif: Option<&str>) -> &'static str {
        match tif.unwrap_or("DAY").to_uppercase().as_str() {
            "GTC" => "GOOD_TILL_CANCEL",
            "IOC" => "IMMEDIATE_OR_CANCEL",
            _ => "DAY",
        }
    }

    fn parse_position(p: &serde_json::Value) -> ApiPosition {
        let long_qty = p["longQuantity"].as_f64().unwrap_or(0.0);
        let short_qty = p["shortQuantity"].as_f64().unwrap_or(0.0);
        let quantity = if long_qty > 0.0 { long_qty } else { -short_qty };

        ApiPosition {
            symbol: p["instrument"]["symbol"].as_str().unwrap_or("").to_string(),
            quantity,
            entry_price: p["averagePrice"].as_f64().unwrap_or(0.0),
            current_price: p["lastPrice"].as_f64().unwrap_or(0.0),
            unrealized_pnl: p["currentDayProfitLoss"].as_f64().unwrap_or(0.0),
        }
    }
}

#[async_trait]
impl TradingApi for SchwabApi {
    async fn submit_order(
        &self,
        order: ApiOrderRequest,
    ) -> Result<ApiOrderResponse, BullShiftError> {
        let url = format!("{}/v1/accounts/{}/orders", self.base_url, self.account_id);

        let side_instruction = if order.side.to_uppercase() == "BUY" {
            "BUY"
        } else {
            "SELL"
        };

        let mut order_body = serde_json::json!({
            "orderType": Self::map_order_type(&order.order_type),
            "session": "NORMAL",
            "duration": Self::map_time_in_force(order.time_in_force.as_deref()),
            "orderLegCollection": [
                {
                    "instruction": side_instruction,
                    "quantity": order.quantity,
                    "instrument": {
                        "symbol": order.symbol,
                        "assetType": "EQUITY"
                    }
                }
            ]
        });

        if let Some(price) = order.price {
            order_body["price"] = serde_json::json!(price);
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .json(&order_body)
            .send()
            .await?;

        if response.status().is_success() {
            // Schwab returns the order ID in the Location header
            let order_id = response
                .headers()
                .get("Location")
                .and_then(|v| v.to_str().ok())
                .and_then(|loc| loc.rsplit('/').next())
                .unwrap_or("")
                .to_string();

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
                "Schwab order failed ({}): {}",
                status, body
            )))
        }
    }

    async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError> {
        let url = format!(
            "{}/v1/accounts/{}/positions",
            self.base_url, self.account_id
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            let positions_val = &body["securitiesAccount"]["positions"];

            let positions = match positions_val {
                serde_json::Value::Array(arr) => arr.iter().map(Self::parse_position).collect(),
                serde_json::Value::Object(_) => vec![Self::parse_position(positions_val)],
                _ => Vec::new(),
            };

            Ok(positions)
        } else {
            Err(BullShiftError::Api(format!(
                "Schwab get positions failed: {}",
                response.status()
            )))
        }
    }

    async fn get_account(&self) -> Result<ApiAccount, BullShiftError> {
        let url = format!("{}/v1/accounts/{}", self.base_url, self.account_id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            let balances = &body["securitiesAccount"]["currentBalances"];

            Ok(ApiAccount {
                balance: balances["liquidationValue"].as_f64().unwrap_or(0.0),
                available: balances["availableFunds"].as_f64().unwrap_or(0.0),
                margin_used: balances["marginBalance"].as_f64().unwrap_or(0.0),
            })
        } else {
            Err(BullShiftError::Api(format!(
                "Schwab get account failed: {}",
                response.status()
            )))
        }
    }

    async fn cancel_order(&self, order_id: String) -> Result<bool, BullShiftError> {
        let url = format!(
            "{}/v1/accounts/{}/orders/{}",
            self.base_url, self.account_id, order_id
        );

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Accept", "application/json")
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
            api_key: "test_oauth_token_12345".to_string(),
            api_secret: "ACCT12345678".to_string(),
            sandbox: true,
        }
    }

    #[test]
    fn test_new_schwab_sandbox() {
        let api = SchwabApi::new(test_credentials());
        assert_eq!(api.base_url, "https://sandbox.schwabapi.com");
        assert_eq!(api.account_id, "ACCT12345678");
    }

    #[test]
    fn test_new_schwab_production() {
        let mut creds = test_credentials();
        creds.sandbox = false;
        let api = SchwabApi::new(creds);
        assert_eq!(api.base_url, "https://api.schwabapi.com");
    }

    #[test]
    fn test_capabilities() {
        let caps = SchwabApi::capabilities();
        assert_eq!(caps.name, "schwab");
        assert!(caps.supports_options);
        assert!(caps.supports_short_selling);
        assert!(!caps.supports_crypto);
        assert!(!caps.supports_fractional_shares);
        assert!(caps.sandbox_available);
    }

    #[test]
    fn test_order_type_mapping() {
        assert_eq!(SchwabApi::map_order_type("MARKET"), "MARKET");
        assert_eq!(SchwabApi::map_order_type("LIMIT"), "LIMIT");
        assert_eq!(SchwabApi::map_order_type("STOP"), "STOP");
        assert_eq!(SchwabApi::map_order_type("STOP_LIMIT"), "STOP_LIMIT");
        assert_eq!(SchwabApi::map_order_type("UNKNOWN"), "MARKET");
    }

    #[test]
    fn test_time_in_force_mapping() {
        assert_eq!(SchwabApi::map_time_in_force(None), "DAY");
        assert_eq!(
            SchwabApi::map_time_in_force(Some("GTC")),
            "GOOD_TILL_CANCEL"
        );
        assert_eq!(
            SchwabApi::map_time_in_force(Some("IOC")),
            "IMMEDIATE_OR_CANCEL"
        );
        assert_eq!(SchwabApi::map_time_in_force(Some("DAY")), "DAY");
    }

    #[test]
    fn test_parse_position_long() {
        let json = serde_json::json!({
            "instrument": { "symbol": "AAPL", "assetType": "EQUITY" },
            "longQuantity": 100.0,
            "shortQuantity": 0.0,
            "averagePrice": 150.0,
            "lastPrice": 175.0,
            "currentDayProfitLoss": 2500.0
        });
        let pos = SchwabApi::parse_position(&json);
        assert_eq!(pos.symbol, "AAPL");
        assert_eq!(pos.quantity, 100.0);
        assert_eq!(pos.entry_price, 150.0);
        assert_eq!(pos.current_price, 175.0);
        assert_eq!(pos.unrealized_pnl, 2500.0);
    }

    #[test]
    fn test_parse_position_short() {
        let json = serde_json::json!({
            "instrument": { "symbol": "TSLA", "assetType": "EQUITY" },
            "longQuantity": 0.0,
            "shortQuantity": 50.0,
            "averagePrice": 200.0,
            "lastPrice": 180.0,
            "currentDayProfitLoss": 1000.0
        });
        let pos = SchwabApi::parse_position(&json);
        assert_eq!(pos.symbol, "TSLA");
        assert_eq!(pos.quantity, -50.0);
    }
}
