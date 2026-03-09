use async_trait::async_trait;
use reqwest::Client;

use super::{BrokerCapabilities, BrokerStatus};
use crate::error::BullShiftError;
use crate::trading::api::{
    ApiAccount, ApiOrderRequest, ApiOrderResponse, ApiPosition, TradingApi, TradingCredentials,
};

/// Tradier brokerage integration via their REST API.
///
/// Uses OAuth bearer token authentication.
/// Sandbox: `https://sandbox.tradier.com`
/// Production: `https://api.tradier.com`
///
/// # Setup
/// 1. Create a Tradier developer account at <https://developer.tradier.com>
/// 2. Generate an API access token
/// 3. Pass the token as `api_key` and your account number as `api_secret`
pub struct TradierApi {
    client: Client,
    base_url: String,
    access_token: String,
    account_id: String,
}

impl TradierApi {
    pub fn new(credentials: TradingCredentials) -> Self {
        let base_url = if credentials.sandbox {
            "https://sandbox.tradier.com".to_string()
        } else {
            "https://api.tradier.com".to_string()
        };

        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .connect_timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_else(|_| Client::new()),
            base_url,
            access_token: credentials.api_key,
            account_id: credentials.api_secret,
        }
    }

    pub fn capabilities() -> BrokerCapabilities {
        BrokerCapabilities {
            name: "tradier".to_string(),
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
        let url = format!("{}/v1/user/profile", self.base_url);
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
            "MARKET" => "market",
            "LIMIT" => "limit",
            "STOP" => "stop",
            "STOP_LIMIT" => "stop_limit",
            other => {
                log::warn!("Unknown order type '{}', defaulting to market", other);
                "market"
            }
        }
    }

    fn map_time_in_force(tif: Option<&str>) -> &'static str {
        match tif.unwrap_or("DAY").to_uppercase().as_str() {
            "GTC" => "gtc",
            "IOC" => "ioc",
            other => {
                log::warn!("Unknown time-in-force '{}', defaulting to day", other);
                "day"
            }
        }
    }
}

#[async_trait]
impl TradingApi for TradierApi {
    async fn submit_order(
        &self,
        order: ApiOrderRequest,
    ) -> Result<ApiOrderResponse, BullShiftError> {
        let url = format!("{}/v1/accounts/{}/orders", self.base_url, self.account_id);

        let mut params = vec![
            ("class", "equity".to_string()),
            ("symbol", order.symbol.clone()),
            ("side", order.side.to_lowercase()),
            ("quantity", order.quantity.to_string()),
            ("type", Self::map_order_type(&order.order_type).to_string()),
            (
                "duration",
                Self::map_time_in_force(order.time_in_force.as_deref()).to_string(),
            ),
        ];

        if let Some(price) = order.price {
            params.push(("price", price.to_string()));
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Accept", "application/json")
            .form(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            let order_resp = &body["order"];

            Ok(ApiOrderResponse {
                order_id: order_resp["id"]
                    .as_u64()
                    .map(|id| id.to_string())
                    .unwrap_or_default(),
                symbol: order.symbol,
                side: order.side,
                quantity: order.quantity,
                order_type: order.order_type,
                price: order.price,
                status: order_resp["status"]
                    .as_str()
                    .unwrap_or("pending")
                    .to_string(),
                created_at: order_resp["create_date"].as_str().unwrap_or("").to_string(),
            })
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(BullShiftError::Api(format!(
                "Tradier order failed ({}): {}",
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
            let positions_val = &body["positions"]["position"];

            let positions = match positions_val {
                serde_json::Value::Array(arr) => arr.iter().map(Self::parse_position).collect(),
                serde_json::Value::Object(_) => vec![Self::parse_position(positions_val)],
                _ => Vec::new(), // "null" means no positions
            };

            Ok(positions)
        } else {
            Err(BullShiftError::Api(format!(
                "Tradier get positions failed: {}",
                response.status()
            )))
        }
    }

    async fn get_account(&self) -> Result<ApiAccount, BullShiftError> {
        let url = format!("{}/v1/accounts/{}/balances", self.base_url, self.account_id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            let balances = &body["balances"];

            Ok(ApiAccount {
                balance: balances["total_equity"].as_f64().unwrap_or(0.0),
                available: balances["margin"]["stock_buying_power"]
                    .as_f64()
                    .or_else(|| balances["cash"]["cash_available"].as_f64())
                    .unwrap_or(0.0),
                margin_used: balances["margin"]["margin_requirement"]
                    .as_f64()
                    .unwrap_or(0.0),
            })
        } else {
            Err(BullShiftError::Api(format!(
                "Tradier get account failed: {}",
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

impl TradierApi {
    fn parse_position(p: &serde_json::Value) -> ApiPosition {
        let quantity = p["quantity"].as_f64().unwrap_or(0.0);
        let cost_basis = p["cost_basis"].as_f64().unwrap_or(0.0);
        let entry_price = if quantity != 0.0 {
            cost_basis / quantity
        } else {
            0.0
        };

        ApiPosition {
            symbol: p["symbol"].as_str().unwrap_or("").to_string(),
            quantity,
            entry_price,
            current_price: p["last_price"].as_f64().unwrap_or(0.0),
            unrealized_pnl: p["gainloss"].as_f64().unwrap_or(0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_credentials() -> TradingCredentials {
        TradingCredentials {
            api_key: "test_access_token_12345".to_string(),
            api_secret: "VA12345678".to_string(),
            sandbox: true,
        }
    }

    #[test]
    fn test_new_tradier() {
        let api = TradierApi::new(test_credentials());
        assert_eq!(api.base_url, "https://sandbox.tradier.com");
        assert_eq!(api.account_id, "VA12345678");
    }

    #[test]
    fn test_new_tradier_production() {
        let mut creds = test_credentials();
        creds.sandbox = false;
        let api = TradierApi::new(creds);
        assert_eq!(api.base_url, "https://api.tradier.com");
    }

    #[test]
    fn test_capabilities() {
        let caps = TradierApi::capabilities();
        assert_eq!(caps.name, "tradier");
        assert!(caps.supports_options);
        assert!(!caps.supports_crypto);
        assert!(caps.sandbox_available);
    }

    #[test]
    fn test_order_type_mapping() {
        assert_eq!(TradierApi::map_order_type("MARKET"), "market");
        assert_eq!(TradierApi::map_order_type("LIMIT"), "limit");
        assert_eq!(TradierApi::map_order_type("STOP"), "stop");
        assert_eq!(TradierApi::map_order_type("STOP_LIMIT"), "stop_limit");
    }

    #[test]
    fn test_time_in_force_mapping() {
        assert_eq!(TradierApi::map_time_in_force(None), "day");
        assert_eq!(TradierApi::map_time_in_force(Some("GTC")), "gtc");
        assert_eq!(TradierApi::map_time_in_force(Some("DAY")), "day");
    }

    #[test]
    fn test_parse_position() {
        let json = serde_json::json!({
            "symbol": "AAPL",
            "quantity": 100.0,
            "cost_basis": 15000.0,
            "last_price": 175.0,
            "gainloss": 2500.0
        });
        let pos = TradierApi::parse_position(&json);
        assert_eq!(pos.symbol, "AAPL");
        assert_eq!(pos.quantity, 100.0);
        assert_eq!(pos.entry_price, 150.0);
        assert_eq!(pos.current_price, 175.0);
        assert_eq!(pos.unrealized_pnl, 2500.0);
    }

    #[test]
    fn test_parse_position_zero_quantity() {
        let json = serde_json::json!({
            "symbol": "TSLA",
            "quantity": 0.0,
            "cost_basis": 0.0,
            "last_price": 200.0,
            "gainloss": 0.0
        });
        let pos = TradierApi::parse_position(&json);
        assert_eq!(pos.entry_price, 0.0); // No division by zero
    }
}
