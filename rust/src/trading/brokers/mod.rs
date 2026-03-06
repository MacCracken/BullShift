pub mod interactive_brokers;
pub mod tradier;
pub mod robinhood;

use serde::{Deserialize, Serialize};

/// Capabilities that a broker may or may not support.
/// Used by TradingApiManager to query what operations are available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerCapabilities {
    pub name: String,
    pub supports_market_orders: bool,
    pub supports_limit_orders: bool,
    pub supports_stop_orders: bool,
    pub supports_stop_limit_orders: bool,
    pub supports_fractional_shares: bool,
    pub supports_short_selling: bool,
    pub supports_options: bool,
    pub supports_crypto: bool,
    pub supports_extended_hours: bool,
    pub sandbox_available: bool,
}

/// Connection status for a broker.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BrokerStatus {
    Connected,
    Disconnected,
    Authenticating,
    Error(String),
}

/// Metadata returned when registering a broker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerInfo {
    pub name: String,
    pub display_name: String,
    pub status: BrokerStatus,
    pub capabilities: BrokerCapabilities,
}
