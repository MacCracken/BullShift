# Broker Integration Guide

This guide covers how BullShift connects to multiple brokerages through its unified `TradingApi` trait, how to configure each supported broker, and how to add new ones.

## Supported Brokers

BullShift supports four brokerages. Each broker implements the same `TradingApi` trait, so the UI and CLI work identically regardless of which broker is active.

| Capability              | Alpaca | Interactive Brokers | Tradier | Robinhood |
|-------------------------|--------|---------------------|---------|-----------|
| Market orders           | Yes    | Yes                 | Yes     | Yes       |
| Limit orders            | Yes    | Yes                 | Yes     | Yes       |
| Stop orders             | Yes    | Yes                 | Yes     | Yes       |
| Stop-limit orders       | Yes    | Yes                 | Yes     | Yes       |
| Fractional shares       | Yes    | No                  | No      | Yes       |
| Short selling           | Yes    | Yes                 | Yes     | No        |
| Options                 | No     | Yes                 | Yes     | Yes       |
| Crypto                  | Yes    | Yes                 | No      | Yes       |
| Extended hours          | Yes    | Yes                 | Yes     | Yes       |
| Sandbox / paper trading | Yes    | Yes                 | Yes     | No        |

## Broker Setup

All brokers are configured via the `TradingCredentials` struct defined in `rust/src/trading/api.rs`:

```rust
pub struct TradingCredentials {
    pub api_key: String,
    pub api_secret: String,
    pub sandbox: bool,
}
```

The meaning of each field varies by broker.

### Alpaca

1. Create an account at [alpaca.markets](https://alpaca.markets).
2. Generate an API key pair from the dashboard (paper or live).
3. Configure credentials:

```rust
use bullshift::trading::api::{AlpacaApi, TradingCredentials};

let credentials = TradingCredentials {
    api_key: "AKXXXXXXXXXXXXXXXXXX".to_string(),
    api_secret: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
    sandbox: true, // true = paper-api.alpaca.markets, false = api.alpaca.markets
};
let alpaca = AlpacaApi::new(credentials);
```

### Interactive Brokers

IB requires the Client Portal Gateway (or TWS) running locally. BullShift communicates with it via its REST API.

1. Download IB Gateway from [interactivebrokers.com](https://www.interactivebrokers.com/en/trading/ibgateway-stable.php).
2. Start the gateway and authenticate through its web UI.
3. Configure credentials — `api_key` is the gateway URL, `api_secret` is your account ID:

```rust
use bullshift::trading::brokers::interactive_brokers::InteractiveBrokersApi;

let credentials = TradingCredentials {
    api_key: "https://localhost:5000".to_string(),  // Gateway URL
    api_secret: "DU12345".to_string(),               // Account ID
    sandbox: true,                                    // Paper trading account
};
let ib = InteractiveBrokersApi::new(credentials);
```

Note: The gateway uses self-signed TLS certificates. The IB client is configured to accept them.

IB-specific helpers:
- `ib.tickle().await` — keep the gateway session alive (call periodically)
- `ib.check_status().await` — verify gateway connectivity

### Tradier

1. Create a developer account at [developer.tradier.com](https://developer.tradier.com).
2. Generate an API access token.
3. Configure credentials — `api_key` is the access token, `api_secret` is your account number:

```rust
use bullshift::trading::brokers::tradier::TradierApi;

let credentials = TradingCredentials {
    api_key: "your_access_token".to_string(),   // OAuth bearer token
    api_secret: "VA12345678".to_string(),        // Account number
    sandbox: true, // true = sandbox.tradier.com, false = api.tradier.com
};
let tradier = TradierApi::new(credentials);
```

### Robinhood

Robinhood does not publish an official public API. This integration targets the documented endpoints used by their clients. Use at your own risk.

1. Obtain an OAuth2 access token through the Robinhood login flow.
2. Configure credentials — `api_key` is the OAuth2 token, `api_secret` is unused:

```rust
use bullshift::trading::brokers::robinhood::RobinhoodApi;

let credentials = TradingCredentials {
    api_key: "your_oauth2_token".to_string(),
    api_secret: "unused".to_string(),  // Required but not used
    sandbox: false,                     // Robinhood has no sandbox
};
let robinhood = RobinhoodApi::new(credentials);
```

Robinhood-specific behavior:
- Instrument URLs are resolved internally via `resolve_instrument_url()`
- Positions require per-symbol quote lookups to compute current price and unrealized P&L
- No sandbox environment is available; all operations are live

## Registering Brokers with TradingApiManager

`TradingApiManager` is the central router that holds all broker connections and forwards requests to the active one. It is defined in `rust/src/trading/api.rs`.

```rust
use bullshift::trading::api::{AlpacaApi, TradingApiManager, TradingCredentials};
use bullshift::trading::brokers::interactive_brokers::InteractiveBrokersApi;
use bullshift::trading::brokers::tradier::TradierApi;
use bullshift::trading::brokers::robinhood::RobinhoodApi;

let mut manager = TradingApiManager::new();

// Register Alpaca
let alpaca_creds = TradingCredentials {
    api_key: "AK...".to_string(),
    api_secret: "secret...".to_string(),
    sandbox: true,
};
manager.register_broker(
    "alpaca",
    Box::new(AlpacaApi::new(alpaca_creds)),
    AlpacaApi::capabilities(),
);

// Register Interactive Brokers
let ib_creds = TradingCredentials {
    api_key: "https://localhost:5000".to_string(),
    api_secret: "DU12345".to_string(),
    sandbox: true,
};
manager.register_broker(
    "interactive_brokers",
    Box::new(InteractiveBrokersApi::new(ib_creds)),
    InteractiveBrokersApi::capabilities(),
);

// Register Tradier
let tradier_creds = TradingCredentials {
    api_key: "token...".to_string(),
    api_secret: "VA12345678".to_string(),
    sandbox: true,
};
manager.register_broker(
    "tradier",
    Box::new(TradierApi::new(tradier_creds)),
    TradierApi::capabilities(),
);

// Register Robinhood
let rh_creds = TradingCredentials {
    api_key: "oauth_token...".to_string(),
    api_secret: "unused".to_string(),
    sandbox: false,
};
manager.register_broker(
    "robinhood",
    Box::new(RobinhoodApi::new(rh_creds)),
    RobinhoodApi::capabilities(),
);
```

## Switching the Active Broker at Runtime

The manager defaults to `"alpaca"`. Use `set_default()` to switch:

```rust
// Switch to Interactive Brokers
let switched = manager.set_default("interactive_brokers".to_string());
assert!(switched); // true if the broker is registered

// Check which broker is active
println!("Active broker: {}", manager.active_broker());

// You can also send an order to a specific broker without switching the default
let order = ApiOrderRequest {
    symbol: "AAPL".to_string(),
    side: "BUY".to_string(),
    quantity: 10.0,
    order_type: "LIMIT".to_string(),
    price: Some(150.0),
    time_in_force: Some("DAY".to_string()),
};
let response = manager.submit_order_to("tradier", order).await?;
```

All standard methods (`submit_order`, `get_positions`, `get_account`, `cancel_order`) are forwarded to whichever broker is currently set as the default.

## Querying Broker Capabilities

Before submitting an order, you can check whether the active broker supports the operation:

```rust
// Get capabilities for a specific broker
if let Some(caps) = manager.get_capabilities("robinhood") {
    if !caps.supports_short_selling {
        println!("Robinhood does not support short selling");
    }
    if caps.supports_fractional_shares {
        println!("Fractional share orders are supported");
    }
}

// List all registered brokers
let brokers = manager.list_brokers();
println!("Registered brokers: {:?}", brokers);

// Get detailed info for all brokers (name, display name, status, capabilities)
let info = manager.get_broker_info();
for broker in &info {
    println!("{} ({}) - {:?}", broker.display_name, broker.name, broker.status);
}
```

## Adding a New Custom Broker

To add a broker that is not yet supported, follow these steps.

### 1. Create the Module

Add a new file under `rust/src/trading/brokers/`, for example `rust/src/trading/brokers/my_broker.rs`.

### 2. Implement the `TradingApi` Trait

Your struct needs to implement four async methods and a static `capabilities()` function:

```rust
use async_trait::async_trait;
use reqwest::Client;

use crate::error::BullShiftError;
use crate::trading::api::{
    ApiAccount, ApiOrderRequest, ApiOrderResponse, ApiPosition, TradingApi, TradingCredentials,
};
use super::BrokerCapabilities;

pub struct MyBrokerApi {
    client: Client,
    base_url: String,
    api_key: String,
}

impl MyBrokerApi {
    pub fn new(credentials: TradingCredentials) -> Self {
        let base_url = if credentials.sandbox {
            "https://sandbox.mybroker.com".to_string()
        } else {
            "https://api.mybroker.com".to_string()
        };

        Self {
            client: Client::new(),
            base_url,
            api_key: credentials.api_key,
        }
    }

    pub fn capabilities() -> BrokerCapabilities {
        BrokerCapabilities {
            name: "my_broker".to_string(),
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
}

#[async_trait]
impl TradingApi for MyBrokerApi {
    async fn submit_order(&self, order: ApiOrderRequest) -> Result<ApiOrderResponse, BullShiftError> {
        // Map ApiOrderRequest fields to your broker's API format.
        // Send the HTTP request and map the response back to ApiOrderResponse.
        todo!()
    }

    async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError> {
        // Fetch positions and map each to ApiPosition.
        todo!()
    }

    async fn get_account(&self) -> Result<ApiAccount, BullShiftError> {
        // Fetch account balances and map to ApiAccount.
        todo!()
    }

    async fn cancel_order(&self, order_id: String) -> Result<bool, BullShiftError> {
        // Cancel the order and return true on success.
        todo!()
    }
}
```

### 3. Register the Module

Add your module to `rust/src/trading/brokers/mod.rs`:

```rust
pub mod interactive_brokers;
pub mod tradier;
pub mod robinhood;
pub mod my_broker;  // Add this line
```

### 4. Register with the Manager

```rust
use bullshift::trading::brokers::my_broker::MyBrokerApi;

let creds = TradingCredentials {
    api_key: "your_key".to_string(),
    api_secret: "your_secret".to_string(),
    sandbox: true,
};
manager.register_broker(
    "my_broker",
    Box::new(MyBrokerApi::new(creds)),
    MyBrokerApi::capabilities(),
);
```

### 5. Add Display Name (Optional)

To give your broker a human-readable name in the UI, update the `display_name()` match in `rust/src/trading/api.rs`:

```rust
fn display_name(name: &str) -> String {
    match name {
        "alpaca" => "Alpaca Markets".to_string(),
        "interactive_brokers" => "Interactive Brokers".to_string(),
        "tradier" => "Tradier".to_string(),
        "robinhood" => "Robinhood".to_string(),
        "my_broker" => "My Broker".to_string(),  // Add this line
        other => other.to_string(),
    }
}
```

## Related Files

- `rust/src/trading/api.rs` — `TradingApi` trait, shared types, `AlpacaApi`, `TradingApiManager`
- `rust/src/trading/brokers/mod.rs` — `BrokerCapabilities`, `BrokerStatus`, `BrokerInfo`
- `rust/src/trading/brokers/interactive_brokers.rs` — Interactive Brokers implementation
- `rust/src/trading/brokers/tradier.rs` — Tradier implementation
- `rust/src/trading/brokers/robinhood.rs` — Robinhood implementation
