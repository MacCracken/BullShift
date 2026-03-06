use async_trait::async_trait;
use reqwest::Client;

use super::{BrokerCapabilities, BrokerStatus};
use crate::error::BullShiftError;
use crate::trading::api::{
    ApiAccount, ApiOrderRequest, ApiOrderResponse, ApiPosition, TradingApi, TradingCredentials,
};

/// Coinbase Advanced Trade API integration.
///
/// Uses Bearer token authentication.
/// Sandbox: `https://api-sandbox.coinbase.com`
/// Production: `https://api.coinbase.com`
///
/// # Setup
/// 1. Create API keys at <https://www.coinbase.com/settings/api>
/// 2. Pass the API key as `api_key` and leave `api_secret` as your account identifier
pub struct CoinbaseApi {
    client: Client,
    base_url: String,
    api_key: String,
    #[allow(dead_code)]
    account_id: String,
}

impl CoinbaseApi {
    pub fn new(credentials: TradingCredentials) -> Self {
        let base_url = if credentials.sandbox {
            "https://api-sandbox.coinbase.com".to_string()
        } else {
            "https://api.coinbase.com".to_string()
        };

        Self {
            client: Client::new(),
            base_url,
            api_key: credentials.api_key,
            account_id: credentials.api_secret,
        }
    }

    pub fn capabilities() -> BrokerCapabilities {
        BrokerCapabilities {
            name: "coinbase".to_string(),
            supports_market_orders: true,
            supports_limit_orders: true,
            supports_stop_orders: false,
            supports_stop_limit_orders: false,
            supports_fractional_shares: false,
            supports_short_selling: false,
            supports_options: false,
            supports_crypto: true,
            supports_extended_hours: false,
            sandbox_available: true,
        }
    }

    pub async fn check_status(&self) -> BrokerStatus {
        let url = format!("{}/api/v3/brokerage/accounts", self.base_url);
        match self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Accept", "application/json")
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => BrokerStatus::Connected,
            Ok(resp) if resp.status() == reqwest::StatusCode::UNAUTHORIZED => {
                BrokerStatus::Error("Invalid or expired API key".to_string())
            }
            Ok(resp) => BrokerStatus::Error(format!("Unexpected status: {}", resp.status())),
            Err(e) => BrokerStatus::Error(format!("Connection failed: {}", e)),
        }
    }

    fn map_order_type(order_type: &str) -> &'static str {
        match order_type.to_uppercase().as_str() {
            "MARKET" => "market_market_ioc",
            "LIMIT" => "limit_limit_gtc",
            _ => "market_market_ioc",
        }
    }

    fn build_order_configuration(
        order_type: &str,
        quantity: f64,
        price: Option<f64>,
    ) -> serde_json::Value {
        let config_type = Self::map_order_type(order_type);
        match config_type {
            "limit_limit_gtc" => {
                serde_json::json!({
                    "limit_limit_gtc": {
                        "base_size": quantity.to_string(),
                        "limit_price": price.unwrap_or(0.0).to_string()
                    }
                })
            }
            _ => {
                serde_json::json!({
                    "market_market_ioc": {
                        "quote_size": quantity.to_string()
                    }
                })
            }
        }
    }

    fn parse_account_as_position(account: &serde_json::Value) -> Option<ApiPosition> {
        let currency = account["currency"].as_str().unwrap_or("");
        let balance = account["available_balance"]["value"]
            .as_str()
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);

        if balance > 0.0 && currency != "USD" {
            Some(ApiPosition {
                symbol: currency.to_string(),
                quantity: balance,
                entry_price: 0.0,
                current_price: 0.0,
                unrealized_pnl: 0.0,
            })
        } else {
            None
        }
    }
}

#[async_trait]
impl TradingApi for CoinbaseApi {
    async fn submit_order(
        &self,
        order: ApiOrderRequest,
    ) -> Result<ApiOrderResponse, BullShiftError> {
        let url = format!("{}/api/v3/brokerage/orders", self.base_url);

        let order_config =
            Self::build_order_configuration(&order.order_type, order.quantity, order.price);

        let order_body = serde_json::json!({
            "client_order_id": uuid::Uuid::new_v4().to_string(),
            "product_id": order.symbol,
            "side": order.side.to_uppercase(),
            "order_configuration": order_config
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&order_body)
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            let result = &body["success_response"];

            Ok(ApiOrderResponse {
                order_id: result["order_id"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
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
                "Coinbase order failed ({}): {}",
                status, body
            )))
        }
    }

    async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError> {
        let url = format!("{}/api/v3/brokerage/accounts", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            let accounts = &body["accounts"];

            let positions = match accounts {
                serde_json::Value::Array(arr) => arr
                    .iter()
                    .filter_map(Self::parse_account_as_position)
                    .collect(),
                _ => Vec::new(),
            };

            Ok(positions)
        } else {
            Err(BullShiftError::Api(format!(
                "Coinbase get positions failed: {}",
                response.status()
            )))
        }
    }

    async fn get_account(&self) -> Result<ApiAccount, BullShiftError> {
        let url = format!("{}/api/v3/brokerage/accounts", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            let accounts = &body["accounts"];

            let mut total_balance = 0.0;
            if let serde_json::Value::Array(arr) = accounts {
                for acct in arr {
                    let val = acct["available_balance"]["value"]
                        .as_str()
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0);
                    total_balance += val;
                }
            }

            Ok(ApiAccount {
                balance: total_balance,
                available: total_balance,
                margin_used: 0.0,
            })
        } else {
            Err(BullShiftError::Api(format!(
                "Coinbase get account failed: {}",
                response.status()
            )))
        }
    }

    async fn cancel_order(&self, order_id: String) -> Result<bool, BullShiftError> {
        let url = format!("{}/api/v3/brokerage/orders/batch_cancel", self.base_url);

        let body = serde_json::json!({
            "order_ids": [order_id]
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
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
            api_key: "test_coinbase_api_key".to_string(),
            api_secret: "test_account_id".to_string(),
            sandbox: true,
        }
    }

    #[test]
    fn test_new_coinbase_sandbox() {
        let api = CoinbaseApi::new(test_credentials());
        assert_eq!(api.base_url, "https://api-sandbox.coinbase.com");
        assert_eq!(api.api_key, "test_coinbase_api_key");
    }

    #[test]
    fn test_new_coinbase_production() {
        let mut creds = test_credentials();
        creds.sandbox = false;
        let api = CoinbaseApi::new(creds);
        assert_eq!(api.base_url, "https://api.coinbase.com");
    }

    #[test]
    fn test_capabilities() {
        let caps = CoinbaseApi::capabilities();
        assert_eq!(caps.name, "coinbase");
        assert!(caps.supports_crypto);
        assert!(!caps.supports_options);
        assert!(!caps.supports_short_selling);
        assert!(!caps.supports_stop_orders);
        assert!(caps.sandbox_available);
    }

    #[test]
    fn test_order_type_mapping() {
        assert_eq!(CoinbaseApi::map_order_type("MARKET"), "market_market_ioc");
        assert_eq!(CoinbaseApi::map_order_type("LIMIT"), "limit_limit_gtc");
        assert_eq!(CoinbaseApi::map_order_type("UNKNOWN"), "market_market_ioc");
    }

    #[test]
    fn test_build_order_config_market() {
        let config = CoinbaseApi::build_order_configuration("MARKET", 100.0, None);
        assert!(config["market_market_ioc"].is_object());
        assert_eq!(config["market_market_ioc"]["quote_size"], "100");
    }

    #[test]
    fn test_build_order_config_limit() {
        let config = CoinbaseApi::build_order_configuration("LIMIT", 0.5, Some(50000.0));
        assert!(config["limit_limit_gtc"].is_object());
        assert_eq!(config["limit_limit_gtc"]["base_size"], "0.5");
        assert_eq!(config["limit_limit_gtc"]["limit_price"], "50000");
    }

    #[test]
    fn test_parse_account_as_position() {
        let btc_account = serde_json::json!({
            "currency": "BTC",
            "available_balance": { "value": "1.5", "currency": "BTC" }
        });
        let pos = CoinbaseApi::parse_account_as_position(&btc_account);
        assert!(pos.is_some());
        let pos = pos.unwrap();
        assert_eq!(pos.symbol, "BTC");
        assert_eq!(pos.quantity, 1.5);

        // USD accounts should be filtered out
        let usd_account = serde_json::json!({
            "currency": "USD",
            "available_balance": { "value": "10000.0", "currency": "USD" }
        });
        let pos = CoinbaseApi::parse_account_as_position(&usd_account);
        assert!(pos.is_none());
    }
}
