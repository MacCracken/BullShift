# API Reference Documentation

## Overview

BullShift provides comprehensive APIs for trading integration, data access, and AI functionality. This document covers the core APIs available in both Rust and Flutter.

## Table of Contents

1. [REST API (api_server)](#rest-api-api_server)
2. [Rust Backend API](#rust-backend-api)
3. [Flutter/Dart API](#flutterdart-api)
4. [WebSocket API](#websocket-api)
5. [FFI Interface](#ffi-interface)
6. [Error Handling](#error-handling)
7. [Examples](#examples)

---

## REST API (api_server)

The `api_server` binary exposes BullShift functionality over HTTP on port `8787` (configurable via `BULLSHIFT_PORT`). Used by SecureYeoman MCP tools and external integrations.

### Trading

| Method | Path | Description |
|--------|------|-------------|
| POST | `/v1/orders` | Submit a trading order |
| GET | `/v1/positions` | List open positions |
| GET | `/v1/account` | Get account details (balance, margin) |
| DELETE | `/v1/orders/:id` | Cancel an open order |

### Market Data

| Method | Path | Description |
|--------|------|-------------|
| GET | `/v1/market/:symbol` | Get real-time quote (price, bid/ask, volume, OHLC, change) |

### Algo Strategies

| Method | Path | Description |
|--------|------|-------------|
| GET | `/v1/algo/strategies` | List all strategies with performance metrics |
| POST | `/v1/algo/strategies` | Create a new strategy |
| GET | `/v1/algo/strategies/:id` | Get a single strategy |
| GET | `/v1/algo/signals` | Get recent algo signals (`?limit=N`) |

### Sentiment

| Method | Path | Description |
|--------|------|-------------|
| GET | `/v1/sentiment` | Overview: sources + recent signals (or `?symbol=X` for aggregate) |
| GET | `/v1/sentiment/:symbol` | Per-symbol aggregate sentiment + signal history |
| GET | `/v1/sentiment/signals` | Raw sentiment signals (`?limit=N`) |

### Alerts

| Method | Path | Description |
|--------|------|-------------|
| GET | `/v1/alerts` | List active (unresolved) alerts |
| POST | `/v1/alerts` | Create an alert rule |
| GET | `/v1/alerts/rules` | List all alert rules |
| DELETE | `/v1/alerts/rules/:id` | Delete an alert rule |

### AI Providers

| Method | Path | Description |
|--------|------|-------------|
| GET | `/v1/ai/providers` | List configured AI providers |
| POST | `/v1/ai/providers` | Add a new AI provider |
| POST | `/v1/ai/providers/:id/configure` | Store API key for provider |
| POST | `/v1/ai/providers/:id/test` | Test provider connectivity |
| POST | `/v1/ai/chat` | Send a chat request to a provider |

### Health

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Service health check |

---

## Rust Backend API

### Core Trading Module

#### Order Execution

```rust
use bullshift_trading::{OrderRequest, OrderType, OrderSide};

// Submit market order
pub async fn submit_market_order(
    symbol: &str,
    quantity: f64,
    side: OrderSide,
) -> Result<OrderResponse, TradingError>

// Submit limit order
pub async fn submit_limit_order(
    symbol: &str,
    quantity: f64,
    price: f64,
    side: OrderSide,
) -> Result<OrderResponse, TradingError>

// Submit stop order
pub async fn submit_stop_order(
    symbol: &str,
    quantity: f64,
    stop_price: f64,
    side: OrderSide,
) -> Result<OrderResponse, TradingError>

// Submit stop-limit order
pub async fn submit_stop_limit_order(
    symbol: &str,
    quantity: f64,
    stop_price: f64,
    limit_price: f64,
    side: OrderSide,
) -> Result<OrderResponse, TradingError>
```

#### Position Management

```rust
use bullshift_trading::{Position, Portfolio};

// Get all positions
pub async fn get_positions() -> Result<Vec<Position>, TradingError>

// Get positions by symbol
pub async fn get_position(symbol: &str) -> Result<Option<Position>, TradingError>

// Close position
pub async fn close_position(
    symbol: &str,
    quantity: Option<f64>, // None for all
) -> Result<OrderResponse, TradingError>

// Get portfolio summary
pub async fn get_portfolio() -> Result<Portfolio, TradingError>
```

#### Account Information

```rust
use bullshift_trading::{Account, AccountBalances};

// Get account details
pub async fn get_account() -> Result<Account, TradingError>

// Get account balances
pub async fn get_balances() -> Result<AccountBalances, TradingError>

// Get buying power
pub async fn get_buying_power() -> Result<f64, TradingError>

// Get day trading count
pub async fn get_day_trade_count() -> Result<u32, TradingError>
```

### Broker Abstraction Layer

#### TradingApi Trait

All brokers implement the same interface:

```rust
use bullshift_core::trading::api::*;
use bullshift_core::error::BullShiftError;

#[async_trait]
pub trait TradingApi {
    async fn submit_order(&self, order: ApiOrderRequest) -> Result<ApiOrderResponse, BullShiftError>;
    async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError>;
    async fn get_account(&self) -> Result<ApiAccount, BullShiftError>;
    async fn cancel_order(&self, order_id: String) -> Result<bool, BullShiftError>;
}
```

#### Supported Brokers

| Broker | Module | Auth | Sandbox |
|--------|--------|------|---------|
| Alpaca | `trading::api::AlpacaApi` | API key + secret headers | Yes |
| Interactive Brokers | `trading::brokers::interactive_brokers::InteractiveBrokersApi` | Client Portal Gateway | Yes |
| Tradier | `trading::brokers::tradier::TradierApi` | OAuth bearer token | Yes |
| Robinhood | `trading::brokers::robinhood::RobinhoodApi` | OAuth2 bearer token | No |

#### TradingApiManager

```rust
use bullshift_core::trading::api::TradingApiManager;
use bullshift_core::trading::brokers::BrokerCapabilities;

let mut manager = TradingApiManager::new();

// Register brokers with capabilities
manager.register_broker("alpaca", Box::new(alpaca), AlpacaApi::capabilities());
manager.register_broker("tradier", Box::new(tradier), TradierApi::capabilities());

// Query capabilities
let caps = manager.get_capabilities("alpaca");

// List all registered brokers
let brokers = manager.list_brokers();

// Switch active broker
manager.set_default("tradier".to_string());

// Submit order to active broker
let response = manager.submit_order(order).await?;

// Submit to a specific broker (not the default)
let response = manager.submit_order_to("alpaca", order).await?;
```

---

### Market Data Module

#### Real-time Data

```rust
use bullshift_data_stream::{Quote, Trade, Bar};

// Get current quote
pub async fn get_quote(symbol: &str) -> Result<Quote, DataError>

// Get historical bars
pub async fn get_historical_bars(
    symbol: &str,
    timeframe: TimeFrame,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<Bar>, DataError>

// Subscribe to real-time data
pub fn subscribe_quotes(
    symbols: Vec<String>,
    callback: Box<dyn FnMut(Quote) + Send>,
) -> Result<SubscriptionId, DataError>

// Unsubscribe from data
pub fn unsubscribe_quotes(subscription_id: SubscriptionId) -> Result<(), DataError>
```

#### Market Scanners

```rust
use bullshift_data_stream::{Scanner, ScannerResult};

// Create scanner
pub fn create_scanner(criteria: ScannerCriteria) -> Result<Scanner, DataError>

// Run scanner
pub async fn run_scanner(scanner: Scanner) -> Result<Vec<ScannerResult>, DataError>

// Get top movers
pub async fn get_top_movers(
    sort_by: MoverSortType,
    limit: usize,
) -> Result<Vec<ScannerResult>, DataError>
```

### Security Module

#### Credential Management

```rust
use bullshift_security::{CredentialStore, SecureCredential};

// Store credentials securely
pub async fn store_credential(
    service: &str,
    username: &str,
    credential: &SecureCredential,
) -> Result<(), SecurityError>

// Retrieve credentials
pub async fn get_credential(
    service: &str,
    username: &str,
) -> Result<Option<SecureCredential>, SecurityError>

// Delete credentials
pub async fn delete_credential(
    service: &str,
    username: &str,
) -> Result<(), SecurityError>

// List all stored services
pub async fn list_services() -> Result<Vec<String>, SecurityError>
```

#### Encryption

```rust
use bullshift_security::{Encryptor, Decryptor};

// Encrypt data
pub fn encrypt_data(data: &[u8], key: &[u8]) -> Result<Vec<u8>, SecurityError>

// Decrypt data
pub fn decrypt_data(encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, SecurityError>

// Generate secure random
pub fn generate_secure_random(length: usize) -> Result<Vec<u8>, SecurityError>

// Derive key from password
pub fn derive_key(password: &str, salt: &[u8]) -> Result<Vec<u8>, SecurityError>
```

### AI Bridge Module

#### Provider Integration

```rust
use bullshift_ai_bridge::{AIProvider, AIRequest, AIResponse};

// Add AI provider
pub async fn add_provider(config: ProviderConfig) -> Result<ProviderId, AIError>

// Send request to AI
pub async fn send_ai_request(
    provider_id: ProviderId,
    request: AIRequest,
) -> Result<AIResponse, AIError>

// Test provider connection
pub async fn test_provider(provider_id: ProviderId) -> Result<bool, AIError>

// List available providers
pub async fn list_providers() -> Result<Vec<AIProvider>, AIError>
```

#### Strategy Generation

```rust
use bullshift_ai_bridge::{StrategyRequest, TradingStrategy};

// Generate trading strategy
pub async fn generate_strategy(
    request: StrategyRequest,
    provider_id: ProviderId,
) -> Result<TradingStrategy, AIError>

// Validate strategy
pub async fn validate_strategy(strategy: &TradingStrategy) -> Result<ValidationResult, AIError>

// Optimize strategy parameters
pub async fn optimize_strategy(
    strategy: &TradingStrategy,
    historical_data: &[Bar],
) -> Result<TradingStrategy, AIError>
```

---

## Flutter/Dart API

### Trading Provider

```dart
import 'package:bullshift/modules/core_trading/trading_provider.dart';

class TradingProvider extends BaseProvider {
  // Submit market order
  Future<OrderResponse> submitMarketOrder({
    required String symbol,
    required double quantity,
    required OrderSide side,
  });

  // Submit limit order
  Future<OrderResponse> submitLimitOrder({
    required String symbol,
    required double quantity,
    required double price,
    required OrderSide side,
  });

  // Get positions
  Future<List<Position>> getPositions();

  // Get specific position
  Future<Position?> getPosition(String symbol);

  // Close position
  Future<OrderResponse> closePosition({
    required String symbol,
    double? quantity,
  });

  // Get portfolio summary
  Future<Portfolio> getPortfolio();

  // Add trading note
  Future<void> addNote({
    required String symbol,
    required String note,
    List<String> tags = const [],
  });

  // Get trading notes
  Future<List<TradingNote>> getNotes(String symbol);

  // Search symbols
  Future<List<SymbolInfo>> searchSymbols(String query);
}
```

### Market Data Provider

```dart
import 'package:bullshift/services/market_data_provider.dart';

class MarketDataProvider extends BaseProvider {
  // Get current quote
  Future<Quote> getQuote(String symbol);

  // Get historical bars
  Future<List<Bar>> getHistoricalBars({
    required String symbol,
    required TimeFrame timeframe,
    required DateTime start,
    required DateTime end,
  });

  // Subscribe to real-time quotes
  Stream<Quote> subscribeToQuotes(List<String> symbols);

  // Get top movers
  Future<List<Mover>> getTopMovers({
    MoverSortType sortBy = MoverSortType.percentChange,
    int limit = 20,
  });

  // Get market hours
  Future<MarketHours> getMarketHours();

  // Check if market is open
  bool isMarketOpen();
}
```

### AI Provider

```dart
import 'package:bullshift/modules/bearly_managed/bearly_managed_provider.dart';

class BearlyManagedProvider extends BaseProvider {
  // Add AI provider
  Future<AIProvider> addProvider({
    required String name,
    required String type,
    required String apiEndpoint,
    required String modelName,
  });

  // Configure provider with API key
  Future<void> configureProvider({
    required String providerId,
    required String apiKey,
  });

  // Send prompt to AI
  Future<AIResponse> sendPrompt({
    required String providerId,
    required String prompt,
    Map<String, dynamic>? variables,
  });

  // Generate trading strategy
  Future<TradingStrategy> generateStrategy({
    required String name,
    required String type,
    required List<String> symbols,
    required String timeframe,
    required String riskLevel,
    required String providerId,
  });

  // Get providers
  Future<List<AIProvider>> getProviders();

  // Test provider connection
  Future<bool> testProvider(String providerId);

  // Get usage statistics
  Future<UsageStats> getUsageStats(String providerId);
}
```

### Security Manager

```dart
import 'package:bullshift/services/security_manager.dart';

class SecurityManager {
  // Store trading credentials
  static Future<void> storeTradingCredentials({
    required String broker,
    required String apiKey,
    required String apiSecret,
  });

  // Get trading credentials
  static Future<Map<String, String>?> getTradingCredentials(String broker);

  // Store AI credentials
  static Future<void> storeAICredentials({
    required String provider,
    required String apiKey,
  });

  // Get AI credentials
  static Future<String?> getAICredentials(String provider);

  // Delete credentials
  static Future<void> deleteCredentials({
    required String service,
    String? username,
  });

  // Generate secure random
  static String generateSecureToken(int length);

  // Encrypt data
  static Future<String> encrypt(String data, String key);

  // Decrypt data
  static Future<String> decrypt(String encryptedData, String key);
}
```

---

## WebSocket API

### Connection

```javascript
// Connect to WebSocket
const ws = new WebSocket('wss://api.bullshift.io/ws');

// Authentication
ws.send(JSON.stringify({
  type: 'auth',
  token: 'your-api-token'
}));
```

### Message Types

#### Quote Updates
```json
{
  "type": "quote",
  "symbol": "AAPL",
  "bid": 150.25,
  "ask": 150.26,
  "last": 150.255,
  "volume": 1000000,
  "timestamp": "2026-02-10T15:30:00Z"
}
```

#### Trade Updates
```json
{
  "type": "trade",
  "symbol": "AAPL",
  "price": 150.25,
  "size": 100,
  "timestamp": "2026-02-10T15:30:00Z"
}
```

#### Order Updates
```json
{
  "type": "order_update",
  "orderId": "order_12345",
  "status": "filled",
  "symbol": "AAPL",
  "side": "buy",
  "quantity": 100,
  "filled_quantity": 100,
  "average_price": 150.25
}
```

#### Position Updates
```json
{
  "type": "position_update",
  "symbol": "AAPL",
  "quantity": 100,
  "market_value": 15025.00,
  "cost_basis": 15000.00,
  "unrealized_pnl": 25.00,
  "unrealized_pnl_percent": 0.17
}
```

### Subscriptions

```javascript
// Subscribe to quotes
ws.send(JSON.stringify({
  type: 'subscribe',
  channel: 'quotes',
  symbols: ['AAPL', 'GOOGL', 'MSFT']
}));

// Subscribe to orders
ws.send(JSON.stringify({
  type: 'subscribe',
  channel: 'orders'
}));

// Subscribe to positions
ws.send(JSON.stringify({
  type: 'subscribe',
  channel: 'positions'
}));

// Unsubscribe
ws.send(JSON.stringify({
  type: 'unsubscribe',
  channel: 'quotes',
  symbols: ['AAPL']
}));
```

---

## FFI Interface

### Rust to Flutter Bridge

```rust
// FFI exports
#[no_mangle]
pub extern "C" fn rust_submit_order(
    symbol: *const c_char,
    quantity: f64,
    side: i32,
    order_type: i32,
    price: f64,
    stop_price: f64,
) -> *mut c_char {
    // Implementation
}

#[no_mangle]
pub extern "C" fn rust_get_positions() -> *mut c_char {
    // Implementation
}

#[no_mangle]
pub extern "C" fn rust_store_credential(
    service: *const c_char,
    username: *const c_char,
    credential: *const c_char,
) -> i32 {
    // Implementation
}
```

### Flutter Side

```dart
import 'dart:ffi';
import 'package:ffi/ffi.dart';

typedef NativeSubmitOrderFunc = Pointer<Utf8> Function(
  Pointer<Utf8> symbol,
  Double quantity,
  Int32 side,
  Int32 orderType,
  Double price,
  Double stopPrice,
);

typedef SubmitOrderFunc = Pointer<Utf8> Function(
  Pointer<Utf8> symbol,
  double quantity,
  int side,
  int orderType,
  double price,
  double stopPrice,
);

class RustTradingEngine {
  late DynamicLibrary _lib;
  late SubmitOrderFunc _submitOrder;

  RustTradingEngine() {
    _lib = DynamicLibrary.open('libbullshift.so');
    _submitOrder = _lib.lookupFunction<
      NativeSubmitOrderFunc,
      SubmitOrderFunc>('rust_submit_order');
  }

  OrderResponse submitOrder(OrderRequest request) {
    final symbol = request.symbol.toNativeUtf8().cast<Utf8>();
    
    final result = _submitOrder(
      symbol,
      request.quantity,
      request.side.index,
      request.type.index,
      request.price ?? 0.0,
      request.stopPrice ?? 0.0,
    );
    
    final responseStr = result.toDartString();
    return OrderResponse.fromJson(jsonDecode(responseStr));
  }
}
```

---

## Error Handling

### Rust Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum TradingError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("API error: {message} (code: {code})")]
    Api { message: String, code: u16 },
    
    #[error("Insufficient buying power")]
    InsufficientBuyingPower,
    
    #[error("Order rejected: {reason}")]
    OrderRejected { reason: String },
    
    #[error("Symbol not found: {symbol}")]
    SymbolNotFound { symbol: String },
    
    #[error("Invalid order parameters")]
    InvalidParameters,
}
```

### Flutter Error Types

```dart
abstract class BullShiftException implements Exception {
  final String message;
  final String? code;
  final dynamic details;
  
  const BullShiftException(this.message, {this.code, this.details});
}

class TradingException extends BullShiftException {
  const TradingException(String message, {String? code, dynamic details})
      : super(message, code: code, details: details);
}

class NetworkException extends BullShiftException {
  const NetworkException(String message, {String? code, dynamic details})
      : super(message, code: code, details: details);
}

class APIException extends BullShiftException {
  const APIException(String message, {String? code, dynamic details})
      : super(message, code: code, details: details);
}
```

### Error Response Format

```json
{
  "error": {
    "type": "TradingException",
    "message": "Insufficient buying power",
    "code": "INSUFFICIENT_BP",
    "details": {
      "required": 10000.0,
      "available": 5000.0
    }
  }
}
```

---

## Examples

### Basic Trading (Rust)

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize trading engine
    let engine = TradingEngine::new().await?;
    
    // Submit market order
    let order = engine.submit_market_order(
        "AAPL",
        100.0,
        OrderSide::Buy,
    ).await?;
    
    println!("Order submitted: {:?}", order.id);
    
    // Get positions
    let positions = engine.get_positions().await?;
    for position in positions {
        println!("{}: {} shares", position.symbol, position.quantity);
    }
    
    Ok(())
}
```

### Market Data (Flutter)

```dart
import 'package:bullshift/services/market_data_provider.dart';

Future<void> exampleMarketData() async {
  final provider = MarketDataProvider();
  
  // Get current quote
  final quote = await provider.getQuote('AAPL');
  print('AAPL: ${quote.lastPrice}');
  
  // Get historical data
  final bars = await provider.getHistoricalBars(
    symbol: 'AAPL',
    timeframe: TimeFrame.oneDay,
    start: DateTime.now().subtract(Duration(days: 30)),
    end: DateTime.now(),
  );
  
  // Subscribe to real-time data
  provider.subscribeToQuotes(['AAPL', 'GOOGL']).listen((quote) {
    print('${quote.symbol}: ${quote.lastPrice}');
  });
}
```

### AI Integration (Dart)

```dart
import 'package:bullshift/modules/bearly_managed/bearly_managed_provider.dart';

Future<void> exampleAI() async {
  final provider = BearlyManagedProvider();
  
  // Add OpenAI provider
  final openAI = await provider.addProvider(
    name: 'OpenAI GPT-4',
    type: 'OpenAI',
    apiEndpoint: 'https://api.openai.com/v1',
    modelName: 'gpt-4',
  );
  
  // Configure with API key
  await provider.configureProvider(
    providerId: openAI.id,
    apiKey: 'your-openai-api-key',
  );
  
  // Generate trading strategy
  final strategy = await provider.generateStrategy(
    name: 'AI Momentum Strategy',
    type: 'Momentum',
    symbols: ['AAPL', 'GOOGL', 'MSFT'],
    timeframe: '1h',
    riskLevel: 'Moderate',
    providerId: openAI.id,
  );
  
  print('Generated strategy: ${strategy.name}');
}
```

### WebSocket Client (JavaScript)

```javascript
class BullShiftWS {
  constructor(token) {
    this.ws = new WebSocket('wss://api.bullshift.io/ws');
    this.token = token;
    this.setupEventHandlers();
  }
  
  setupEventHandlers() {
    this.ws.onopen = () => {
      // Authenticate
      this.ws.send(JSON.stringify({
        type: 'auth',
        token: this.token
      }));
      
      // Subscribe to quotes
      this.subscribeToQuotes(['AAPL', 'GOOGL']);
    };
    
    this.ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      this.handleMessage(data);
    };
  }
  
  subscribeToQuotes(symbols) {
    this.ws.send(JSON.stringify({
      type: 'subscribe',
      channel: 'quotes',
      symbols: symbols
    }));
  }
  
  handleMessage(data) {
    switch (data.type) {
      case 'quote':
        console.log(`${data.symbol}: ${data.last}`);
        break;
      case 'order_update':
        console.log(`Order ${data.orderId}: ${data.status}`);
        break;
      case 'position_update':
        console.log(`${data.symbol}: ${data.unrealized_pnl}`);
        break;
    }
  }
}

// Usage
const client = new BullShiftWS('your-api-token');
```

---

## API Versioning

### Version Format
BullShift uses CalVer: `YYYY.M.D` (year, month, day). Same-day patches use
`YYYY.M.D-N` suffix (e.g., `2026.3.5-1`).

### Current Version: 2026.3.9

### Supported Versions
- **2026.3.x** - Current stable version

### Version Headers
```http
API-Version: 2026.3.9
Content-Type: application/json
Authorization: Bearer your-token
```

---

**API Reference Version: 2026.3.9**
**Last Updated: March 9, 2026**

For additional examples and integration guides, see the [Examples](docs/examples/) directory.