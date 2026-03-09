use async_trait::async_trait;
use reqwest::Client;

use super::{BrokerCapabilities, BrokerStatus};
use crate::error::BullShiftError;
use crate::trading::api::{
    ApiAccount, ApiOrderRequest, ApiOrderResponse, ApiPosition, TradingApi, TradingCredentials,
};

/// Interactive Brokers integration via the Client Portal Gateway API.
///
/// Requires the IB Gateway or TWS running locally (default `https://localhost:5000`).
/// The gateway handles OAuth/session management; this client sends REST requests to it.
///
/// # Setup
/// 1. Download IB Gateway from <https://www.interactivebrokers.com/en/trading/ibgateway-stable.php>
/// 2. Start the gateway and authenticate via its web UI
/// 3. Pass the gateway URL as `api_key` in `TradingCredentials` (e.g. `https://localhost:5000`)
///    and your account ID as `api_secret`
pub struct InteractiveBrokersApi {
    client: Client,
    gateway_url: String,
    account_id: String,
}

impl InteractiveBrokersApi {
    pub fn new(credentials: TradingCredentials) -> Self {
        let gateway_url = if credentials.sandbox {
            // Paper trading gateway
            credentials.api_key.clone()
        } else {
            credentials.api_key.clone()
        };

        Self {
            client: Client::builder()
                .danger_accept_invalid_certs(true) // IB Gateway uses self-signed certs
                .build()
                .unwrap_or_else(|_| Client::new()),
            gateway_url,
            account_id: credentials.api_secret,
        }
    }

    pub fn capabilities() -> BrokerCapabilities {
        BrokerCapabilities {
            name: "interactive_brokers".to_string(),
            supports_market_orders: true,
            supports_limit_orders: true,
            supports_stop_orders: true,
            supports_stop_limit_orders: true,
            supports_fractional_shares: false,
            supports_short_selling: true,
            supports_options: true,
            supports_crypto: true,
            supports_extended_hours: true,
            sandbox_available: true,
        }
    }

    pub async fn check_status(&self) -> BrokerStatus {
        let url = format!("{}/v1/api/iserver/auth/status", self.gateway_url);
        match self.client.post(&url).send().await {
            Ok(resp) if resp.status().is_success() => BrokerStatus::Connected,
            Ok(_) => BrokerStatus::Authenticating,
            Err(e) => BrokerStatus::Error(format!("Gateway unreachable: {}", e)),
        }
    }

    /// Keep the IB gateway session alive (must be called periodically).
    pub async fn tickle(&self) -> Result<(), BullShiftError> {
        let url = format!("{}/v1/api/tickle", self.gateway_url);
        self.client.post(&url).send().await?;
        Ok(())
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

    fn map_side(side: &str) -> &'static str {
        match side.to_uppercase().as_str() {
            "BUY" => "BUY",
            "SELL" => "SELL",
            other => {
                log::warn!("Unknown side '{}', defaulting to BUY", other);
                "BUY"
            }
        }
    }
}

#[async_trait]
impl TradingApi for InteractiveBrokersApi {
    async fn submit_order(
        &self,
        order: ApiOrderRequest,
    ) -> Result<ApiOrderResponse, BullShiftError> {
        let url = format!(
            "{}/v1/api/iserver/account/{}/orders",
            self.gateway_url, self.account_id
        );

        let ib_order = serde_json::json!({
            "orders": [{
                "conid": order.symbol, // IB uses contract IDs; symbol lookup needed in production
                "orderType": Self::map_order_type(&order.order_type),
                "side": Self::map_side(&order.side),
                "quantity": order.quantity,
                "price": order.price,
                "tif": order.time_in_force.as_deref().unwrap_or("DAY"),
            }]
        });

        let response = self.client.post(&url).json(&ib_order).send().await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            // IB returns an array; first element has order_id
            let order_id = body[0]["order_id"]
                .as_str()
                .or_else(|| body[0]["orderId"].as_str())
                .unwrap_or("unknown")
                .to_string();

            Ok(ApiOrderResponse {
                order_id,
                symbol: order.symbol,
                side: order.side,
                quantity: order.quantity,
                order_type: order.order_type,
                price: order.price,
                status: "Submitted".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
            })
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(BullShiftError::Api(format!(
                "IB order failed ({}): {}",
                status, body
            )))
        }
    }

    async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError> {
        let url = format!(
            "{}/v1/api/portfolio/{}/positions/0",
            self.gateway_url, self.account_id
        );

        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let body: Vec<serde_json::Value> = response.json().await?;
            let positions = body
                .iter()
                .map(|p| ApiPosition {
                    symbol: p["contractDesc"].as_str().unwrap_or("").to_string(),
                    quantity: p["position"].as_f64().unwrap_or(0.0),
                    entry_price: p["avgCost"].as_f64().unwrap_or(0.0),
                    current_price: p["mktPrice"].as_f64().unwrap_or(0.0),
                    unrealized_pnl: p["unrealizedPnl"].as_f64().unwrap_or(0.0),
                })
                .collect();
            Ok(positions)
        } else {
            Err(BullShiftError::Api(format!(
                "IB get positions failed: {}",
                response.status()
            )))
        }
    }

    async fn get_account(&self) -> Result<ApiAccount, BullShiftError> {
        let url = format!(
            "{}/v1/api/portfolio/{}/summary",
            self.gateway_url, self.account_id
        );

        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            Ok(ApiAccount {
                balance: body["totalcashvalue"]["amount"].as_f64().unwrap_or(0.0),
                available: body["availablefunds"]["amount"].as_f64().unwrap_or(0.0),
                margin_used: body["maintmarginreq"]["amount"].as_f64().unwrap_or(0.0),
            })
        } else {
            Err(BullShiftError::Api(format!(
                "IB get account failed: {}",
                response.status()
            )))
        }
    }

    async fn cancel_order(&self, order_id: String) -> Result<bool, BullShiftError> {
        let url = format!(
            "{}/v1/api/iserver/account/{}/order/{}",
            self.gateway_url, self.account_id, order_id
        );

        let response = self.client.delete(&url).send().await?;
        Ok(response.status().is_success())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_credentials() -> TradingCredentials {
        TradingCredentials {
            api_key: "https://localhost:5000".to_string(),
            api_secret: "DU12345".to_string(),
            sandbox: true,
        }
    }

    #[test]
    fn test_new_interactive_brokers() {
        let api = InteractiveBrokersApi::new(test_credentials());
        assert_eq!(api.gateway_url, "https://localhost:5000");
        assert_eq!(api.account_id, "DU12345");
    }

    #[test]
    fn test_capabilities() {
        let caps = InteractiveBrokersApi::capabilities();
        assert_eq!(caps.name, "interactive_brokers");
        assert!(caps.supports_options);
        assert!(caps.supports_short_selling);
        assert!(caps.supports_stop_limit_orders);
        assert!(!caps.supports_fractional_shares);
    }

    #[test]
    fn test_order_type_mapping() {
        assert_eq!(InteractiveBrokersApi::map_order_type("MARKET"), "MKT");
        assert_eq!(InteractiveBrokersApi::map_order_type("LIMIT"), "LMT");
        assert_eq!(InteractiveBrokersApi::map_order_type("STOP"), "STP");
        assert_eq!(
            InteractiveBrokersApi::map_order_type("STOP_LIMIT"),
            "STP LMT"
        );
    }

    #[test]
    fn test_side_mapping() {
        assert_eq!(InteractiveBrokersApi::map_side("BUY"), "BUY");
        assert_eq!(InteractiveBrokersApi::map_side("SELL"), "SELL");
        assert_eq!(InteractiveBrokersApi::map_side("buy"), "BUY");
    }
}
