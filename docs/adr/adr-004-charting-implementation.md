# ADR-004: Advanced Charting Implementation

**Date:** 2026-02-22  
**Status:** Accepted  
**Context:** Implementation of advanced charting features including real-time data, drawing tools, and multi-symbol comparison

## Decision

BullShift implements advanced charting using a custom Flutter CustomPainter-based solution with the following components:

1. **Real-time Data Integration**
   - MarketDataProvider manages price history and live updates
   - Simulated WebSocket updates via timer (production: connects to Alpaca WebSocket)
   - External price data can be passed to AdvancedChartingWidget

2. **Drawing Tools**
   - DrawingToolManager manages drawing state
   - DrawingObject base class with concrete implementations:
     - Trendline, HorizontalLine, VerticalLine
     - FibonacciRetracement, FibonacciExtension
     - RectangleDrawing, TextAnnotation
   - Color selection and stroke width customization
   - GestureDetector integration for creating drawings on chart

3. **Multi-Symbol Comparison**
   - Normalized percentage-based comparison
   - Support for multiple symbols overlaid on main chart
   - Dynamic add/remove of comparison symbols

## Implementation

```dart
// MarketDataProvider manages real-time data
class MarketDataProvider extends BaseProvider {
  List<PriceData> _priceHistory = [];
  Timer? _updateTimer;
  
  void loadSymbolData(String symbol) async { ... }
  void addRealtimeTick(double price, int volume) { ... }
}

// AdvancedChartingWidget accepts external data
AdvancedChartingWidget(
  symbol: 'AAPL',
  priceData: marketDataProvider.priceHistory,
  useRealtimeData: true,
)

// Drawing tools via toolbar
PopupMenuButton<DrawingToolType> // Tool selection
PopupMenuButton<Color>           // Color picker
```

## Consequences

### Positive
- Real-time price updates without page refresh
- Customizable drawings for technical analysis
- Compare multiple symbols on single chart
- All rendering via CustomPainter for performance

### Negative
- Drawing state not persisted between sessions
- Comparison data is simulated (no live data)
- Complex gesture handling code

## Alternatives Considered

- **TradingView Lightweight Charts**: Rejected - custom implementation gives full control
- **fl_chart**: Rejected - not designed for financial charts
- **Native charting libraries**: Rejected - cross-platform Flutter approach preferred

## Related Files

- `flutter/lib/widgets/advanced_charting_widget.dart`
- `flutter/lib/widgets/drawing_tools.dart`
- `flutter/lib/modules/market_data/market_data_provider.dart`
- `flutter/lib/modules/core_trading/core_trading_view.dart`
