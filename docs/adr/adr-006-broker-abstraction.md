# ADR-006: Broker Abstraction Layer

**Date:** 2026-03-05
**Status:** Accepted
**Context:** Unified multi-broker support for expanding beyond Alpaca to Interactive Brokers, Tradier, and Robinhood

## Decision

BullShift adopts the `TradingApi` trait as a unified broker abstraction layer, enabling the platform to support multiple brokerages through a single, consistent interface.

### 1. `TradingApi` Trait with `#[async_trait]`

The core abstraction is a dyn-compatible trait annotated with `#[async_trait]`:

```rust
#[async_trait]
pub trait TradingApi {
    async fn submit_order(&self, order: ApiOrderRequest) -> Result<ApiOrderResponse, BullShiftError>;
    async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError>;
    async fn get_account(&self) -> Result<ApiAccount, BullShiftError>;
    async fn cancel_order(&self, order_id: String) -> Result<bool, BullShiftError>;
}
```

Using `#[async_trait]` allows the trait to be used as `dyn TradingApi + Send + Sync`, which is required for runtime broker selection via `TradingApiManager`.

### 2. Shared API Types

Each broker maps its API-specific concepts (order types, auth headers, response formats) to shared types defined in `rust/src/trading/api.rs`:

- **`ApiOrderRequest`** — symbol, side, quantity, order type, optional price and time-in-force
- **`ApiOrderResponse`** — order ID, status, timestamps, filled details
- **`ApiPosition`** — symbol, quantity, entry price, current price, unrealized P&L
- **`ApiAccount`** — balance, available funds, margin used

This normalization means the UI, CLI, and strategy engine never need to know which broker is active.

### 3. `TradingApiManager` as Broker Router

`TradingApiManager` holds a `HashMap<String, Box<dyn TradingApi + Send + Sync>>` alongside a parallel `HashMap<String, BrokerCapabilities>`. It provides:

- **Registration** — `register_broker(name, api, capabilities)` to add a broker at startup or runtime
- **Runtime switching** — `set_default(name)` to change the active broker
- **Capability queries** — `get_capabilities(name)` and `get_broker_info()` for introspection
- **Named routing** — `submit_order_to(broker, order)` to target a specific broker without switching the default
- **Delegated trading** — `submit_order()`, `get_positions()`, `get_account()`, `cancel_order()` all forward to the active broker

### 4. `BrokerCapabilities` Metadata

Each broker implementation exposes a static `capabilities()` method returning a `BrokerCapabilities` struct that declares what the broker supports (fractional shares, options, crypto, short selling, extended hours, sandbox mode, etc.). This metadata is used by the UI to enable/disable controls and by the order validation layer to reject unsupported operations early.

### 5. Per-Broker Implementations

Each broker lives in its own module under `rust/src/trading/brokers/`:

- `rust/src/trading/brokers/interactive_brokers.rs` — IB Client Portal Gateway API; maps order types to IB codes (`MKT`, `LMT`, `STP`, `STP LMT`); uses contract IDs; requires local gateway running
- `rust/src/trading/brokers/tradier.rs` — Tradier REST API with OAuth bearer tokens; form-encoded order submission
- `rust/src/trading/brokers/robinhood.rs` — Robinhood API with OAuth2 tokens; resolves instrument URLs; no sandbox available
- `rust/src/trading/api.rs` — `AlpacaApi` (the original broker) and the `TradingApiManager`

## Consequences

### Positive

- Adding a new broker requires implementing only 4 trait methods (`submit_order`, `get_positions`, `get_account`, `cancel_order`) plus a `capabilities()` function
- The UI and CLI get a consistent interface regardless of which broker is active
- Broker switching at runtime enables multi-account workflows (e.g., execute on IB, check positions on Alpaca)
- Capability metadata prevents invalid operations before they hit the network

### Negative

- Broker-specific features that do not fit the trait require additional helper methods (e.g., `InteractiveBrokersApi::tickle()` for session keep-alive, `RobinhoodApi::resolve_instrument_url()` for instrument lookups, IB contract IDs)
- The shared types are a lowest-common-denominator representation; rich broker-specific response data is discarded during mapping
- `#[async_trait]` introduces a heap allocation per method call due to `Box<dyn Future>`; this is negligible for network-bound trading operations but worth noting

## Alternatives Considered

### Single Monolithic API Client with Broker Enum

A single struct containing a `Broker` enum and match arms for each broker in every method. Rejected because it violates the open-closed principle — every new broker requires modifying the central client — and makes testing harder.

### gRPC Broker Protocol

Define a `.proto` for a broker service and run each broker as a separate process. Rejected as over-engineered for the current scope; the in-process trait approach is simpler, faster, and sufficient for the four planned brokers. Can be reconsidered if third-party or out-of-process broker adapters become necessary.

### Separate Crate Per Broker

Publish each broker as its own Cargo crate with a shared `bullshift-broker-api` interface crate. Rejected because the brokers share significant infrastructure (HTTP client, error types, credential management) and the overhead of multi-crate coordination is not justified at this scale.

## Related Files

- `rust/src/trading/api.rs` — `TradingApi` trait, shared types, `AlpacaApi`, `TradingApiManager`
- `rust/src/trading/brokers/mod.rs` — `BrokerCapabilities`, `BrokerStatus`, `BrokerInfo`
- `rust/src/trading/brokers/interactive_brokers.rs`
- `rust/src/trading/brokers/tradier.rs`
- `rust/src/trading/brokers/robinhood.rs`
