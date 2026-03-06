use async_trait::async_trait;
use reqwest::Client;

use super::{BrokerCapabilities, BrokerStatus};
use crate::error::BullShiftError;
use crate::trading::api::{
    ApiAccount, ApiOrderRequest, ApiOrderResponse, ApiPosition, TradingApi, TradingCredentials,
};

/// Robinhood brokerage integration.
///
/// Uses Robinhood's API with OAuth2 bearer token authentication.
/// Note: Robinhood does not publish an official public API; this implementation
/// targets the documented endpoints used by their clients. Use at your own risk.
///
/// # Setup
/// 1. Obtain an OAuth2 access token (via Robinhood login flow)
/// 2. Pass the token as `api_key` in `TradingCredentials`
/// 3. `api_secret` is unused but required (pass any non-empty string)
/// 4. `sandbox` has no effect — Robinhood has no sandbox environment
pub struct RobinhoodApi {
    client: Client,
    base_url: String,
    access_token: String,
}

impl RobinhoodApi {
    pub fn new(credentials: TradingCredentials) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.robinhood.com".to_string(),
            access_token: credentials.api_key,
        }
    }

    pub fn capabilities() -> BrokerCapabilities {
        BrokerCapabilities {
            name: "robinhood".to_string(),
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
        let url = format!("{}/user/", self.base_url);
        match self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => BrokerStatus::Connected,
            Ok(resp) if resp.status() == reqwest::StatusCode::UNAUTHORIZED => {
                BrokerStatus::Error("Token expired or invalid".to_string())
            }
            Ok(resp) => BrokerStatus::Error(format!("Unexpected status: {}", resp.status())),
            Err(e) => BrokerStatus::Error(format!("Connection failed: {}", e)),
        }
    }

    /// Look up Robinhood's internal instrument URL for a ticker symbol.
    async fn resolve_instrument_url(&self, symbol: &str) -> Result<String, BullShiftError> {
        let url = format!("{}/instruments/?symbol={}", self.base_url, symbol);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            body["results"][0]["url"]
                .as_str()
                .map(|s| s.to_string())
                .ok_or_else(|| BullShiftError::Api(format!("Instrument not found: {}", symbol)))
        } else {
            Err(BullShiftError::Api(format!(
                "Instrument lookup failed: {}",
                response.status()
            )))
        }
    }

    /// Resolve a Robinhood instrument URL back to a ticker symbol.
    async fn resolve_symbol(&self, instrument_url: &str) -> Result<String, BullShiftError> {
        let response = self
            .client
            .get(instrument_url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            Ok(body["symbol"].as_str().unwrap_or("???").to_string())
        } else {
            Ok("???".to_string())
        }
    }

    /// Get a current quote for a symbol.
    async fn get_quote(&self, symbol: &str) -> Result<f64, BullShiftError> {
        let url = format!("{}/quotes/{}/", self.base_url, symbol);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            body["last_trade_price"]
                .as_str()
                .and_then(|s| s.parse::<f64>().ok())
                .ok_or_else(|| BullShiftError::Api("Failed to parse quote".to_string()))
        } else {
            Err(BullShiftError::Api(format!(
                "Quote lookup failed: {}",
                response.status()
            )))
        }
    }

    fn map_trigger(order_type: &str) -> &'static str {
        match order_type.to_uppercase().as_str() {
            "STOP" | "STOP_LIMIT" => "stop",
            _ => "immediate",
        }
    }

    fn map_order_type(order_type: &str) -> &'static str {
        match order_type.to_uppercase().as_str() {
            "MARKET" | "STOP" => "market",
            "LIMIT" | "STOP_LIMIT" => "limit",
            _ => "market",
        }
    }

    fn map_time_in_force(tif: Option<&str>) -> &'static str {
        match tif.unwrap_or("GTC").to_uppercase().as_str() {
            "DAY" => "gfd",
            "IOC" => "ioc",
            _ => "gtc",
        }
    }
}

#[async_trait]
impl TradingApi for RobinhoodApi {
    async fn submit_order(
        &self,
        order: ApiOrderRequest,
    ) -> Result<ApiOrderResponse, BullShiftError> {
        let instrument_url = self.resolve_instrument_url(&order.symbol).await?;
        let url = format!("{}/orders/", self.base_url);

        let mut body = serde_json::json!({
            "account": format!("{}/accounts/", self.base_url),
            "instrument": instrument_url,
            "symbol": order.symbol,
            "side": order.side.to_lowercase(),
            "quantity": order.quantity,
            "type": Self::map_order_type(&order.order_type),
            "trigger": Self::map_trigger(&order.order_type),
            "time_in_force": Self::map_time_in_force(order.time_in_force.as_deref()),
        });

        if let Some(price) = order.price {
            body["price"] = serde_json::json!(format!("{:.2}", price));

            if order.order_type.to_uppercase() == "STOP"
                || order.order_type.to_uppercase() == "STOP_LIMIT"
            {
                body["stop_price"] = serde_json::json!(format!("{:.2}", price));
            }
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let resp_body: serde_json::Value = response.json().await?;

            Ok(ApiOrderResponse {
                order_id: resp_body["id"].as_str().unwrap_or("").to_string(),
                symbol: order.symbol,
                side: order.side,
                quantity: order.quantity,
                order_type: order.order_type,
                price: order.price,
                status: resp_body["state"].as_str().unwrap_or("queued").to_string(),
                created_at: resp_body["created_at"].as_str().unwrap_or("").to_string(),
            })
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(BullShiftError::Api(format!(
                "Robinhood order failed ({}): {}",
                status, body
            )))
        }
    }

    async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError> {
        let url = format!("{}/positions/?nonzero=true", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            let results = body["results"].as_array();

            let mut positions = Vec::new();
            if let Some(results) = results {
                for p in results {
                    let quantity: f64 = p["quantity"]
                        .as_str()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0.0);
                    let avg_buy: f64 = p["average_buy_price"]
                        .as_str()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0.0);

                    let instrument_url = p["instrument"].as_str().unwrap_or("");
                    let symbol = self
                        .resolve_symbol(instrument_url)
                        .await
                        .unwrap_or_else(|_| "???".to_string());

                    // Fetch current price
                    let current_price = self.get_quote(&symbol).await.unwrap_or(avg_buy);
                    let unrealized_pnl = (current_price - avg_buy) * quantity;

                    positions.push(ApiPosition {
                        symbol,
                        quantity,
                        entry_price: avg_buy,
                        current_price,
                        unrealized_pnl,
                    });
                }
            }

            Ok(positions)
        } else {
            Err(BullShiftError::Api(format!(
                "Robinhood get positions failed: {}",
                response.status()
            )))
        }
    }

    async fn get_account(&self) -> Result<ApiAccount, BullShiftError> {
        let url = format!("{}/accounts/", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            let account = &body["results"][0];

            let portfolio_cash: f64 = account["portfolio_cash"]
                .as_str()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0);
            let buying_power: f64 = account["buying_power"]
                .as_str()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0);

            Ok(ApiAccount {
                balance: portfolio_cash,
                available: buying_power,
                margin_used: portfolio_cash - buying_power,
            })
        } else {
            Err(BullShiftError::Api(format!(
                "Robinhood get account failed: {}",
                response.status()
            )))
        }
    }

    async fn cancel_order(&self, order_id: String) -> Result<bool, BullShiftError> {
        let url = format!("{}/orders/{}/cancel/", self.base_url, order_id);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
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
            api_secret: "unused".to_string(),
            sandbox: false,
        }
    }

    #[test]
    fn test_new_robinhood() {
        let api = RobinhoodApi::new(test_credentials());
        assert_eq!(api.base_url, "https://api.robinhood.com");
    }

    #[test]
    fn test_capabilities() {
        let caps = RobinhoodApi::capabilities();
        assert_eq!(caps.name, "robinhood");
        assert!(caps.supports_fractional_shares);
        assert!(caps.supports_crypto);
        assert!(!caps.supports_short_selling);
        assert!(!caps.sandbox_available);
    }

    #[test]
    fn test_order_type_mapping() {
        assert_eq!(RobinhoodApi::map_order_type("MARKET"), "market");
        assert_eq!(RobinhoodApi::map_order_type("LIMIT"), "limit");
        assert_eq!(RobinhoodApi::map_order_type("STOP"), "market");
        assert_eq!(RobinhoodApi::map_order_type("STOP_LIMIT"), "limit");
    }

    #[test]
    fn test_trigger_mapping() {
        assert_eq!(RobinhoodApi::map_trigger("MARKET"), "immediate");
        assert_eq!(RobinhoodApi::map_trigger("LIMIT"), "immediate");
        assert_eq!(RobinhoodApi::map_trigger("STOP"), "stop");
        assert_eq!(RobinhoodApi::map_trigger("STOP_LIMIT"), "stop");
    }

    #[test]
    fn test_time_in_force_mapping() {
        assert_eq!(RobinhoodApi::map_time_in_force(None), "gtc");
        assert_eq!(RobinhoodApi::map_time_in_force(Some("DAY")), "gfd");
        assert_eq!(RobinhoodApi::map_time_in_force(Some("GTC")), "gtc");
        assert_eq!(RobinhoodApi::map_time_in_force(Some("IOC")), "ioc");
    }
}
