pub mod coinbase;
pub mod interactive_brokers;
pub mod kraken;
pub mod robinhood;
pub mod schwab;
pub mod tradier;
pub mod webull;

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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_capabilities(name: &str) -> BrokerCapabilities {
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
            sandbox_available: true,
        }
    }

    #[test]
    fn test_broker_status_variants() {
        assert_eq!(BrokerStatus::Connected, BrokerStatus::Connected);
        assert_ne!(BrokerStatus::Connected, BrokerStatus::Disconnected);
        assert_ne!(BrokerStatus::Authenticating, BrokerStatus::Connected);

        let err = BrokerStatus::Error("timeout".to_string());
        assert_ne!(err, BrokerStatus::Connected);
    }

    #[test]
    fn test_broker_status_serialization() {
        let status = BrokerStatus::Connected;
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: BrokerStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, BrokerStatus::Connected);
    }

    #[test]
    fn test_broker_status_error_message() {
        let status = BrokerStatus::Error("Invalid API key".to_string());
        if let BrokerStatus::Error(msg) = &status {
            assert_eq!(msg, "Invalid API key");
        } else {
            panic!("Expected Error variant");
        }
    }

    #[test]
    fn test_broker_capabilities_creation() {
        let caps = make_capabilities("test_broker");
        assert_eq!(caps.name, "test_broker");
        assert!(caps.supports_market_orders);
        assert!(caps.supports_limit_orders);
        assert!(!caps.supports_crypto);
        assert!(caps.sandbox_available);
    }

    #[test]
    fn test_broker_capabilities_serialization() {
        let caps = make_capabilities("alpaca");
        let json = serde_json::to_string(&caps).unwrap();
        let deserialized: BrokerCapabilities = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "alpaca");
        assert_eq!(deserialized.supports_market_orders, caps.supports_market_orders);
    }

    #[test]
    fn test_broker_info_construction() {
        let info = BrokerInfo {
            name: "alpaca".to_string(),
            display_name: "Alpaca Markets".to_string(),
            status: BrokerStatus::Connected,
            capabilities: make_capabilities("alpaca"),
        };
        assert_eq!(info.name, "alpaca");
        assert_eq!(info.display_name, "Alpaca Markets");
        assert_eq!(info.status, BrokerStatus::Connected);
    }
}
