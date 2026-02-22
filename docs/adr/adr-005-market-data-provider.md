# ADR-005: Market Data Provider Architecture

**Date:** 2026-02-22  
**Status:** Accepted  
**Context:** Real-time market data management and distribution to UI components

## Decision

BullShift implements a MarketDataProvider for managing real-time price data:

1. **Price History Management**
   - Stores OHLCV data (Open, High, Low, Close, Volume)
   - Maintains rolling window of price bars
   - Supports historical data fetching

2. **Real-time Updates**
   - Timer-based simulation (2-second intervals)
   - Tick-based updates for live market data
   - OHLCV bar aggregation from ticks

3. **Provider Integration**
   - Extends BaseProvider for state management
   - Notifies listeners on price updates
   - Stop/start streaming controls

## Implementation

```dart
class MarketDataProvider extends BaseProvider {
  List<PriceData> _priceHistory = [];
  Timer? _updateTimer;
  
  Future<void> loadSymbolData(String symbol) async { ... }
  void addRealtimeTick(double price, int volume) { ... }
  void stopStreaming() { ... }
}

class PriceData {
  final DateTime timestamp;
  final double open;
  final double high;
  final double low;
  final double close;
  final int volume;
}
```

## Consequences

### Positive
- Centralized market data management
- Easy to switch between simulated and live data
- Consistent data format across the app

### Negative
- Timer-based updates are not real-time
- No connection to actual market data in current implementation
- Price history not persisted

## Alternatives Considered

- **Direct Rust WebSocket to Flutter**: More complex, deferred for broker integration phase
- **StreamProvider**: Could work but current approach is simpler

## Related Files

- `flutter/lib/modules/market_data/market_data_provider.dart`
