import 'dart:async';
import 'dart:math';
import '../../services/base_provider.dart';

class MarketDataProvider extends BaseProvider {
  String _currentSymbol = '';
  List<PriceData> _priceHistory = [];
  PriceData? _latestPrice;
  Timer? _updateTimer;
  bool _isStreaming = false;

  MarketDataProvider(dynamic rustEngine);

  String get currentSymbol => _currentSymbol;
  List<PriceData> get priceHistory => _priceHistory;
  PriceData? get latestPrice => _latestPrice;
  bool get isStreaming => _isStreaming;

  Future<void> loadSymbolData(String symbol) async {
    _currentSymbol = symbol;

    setLoading(true);
    try {
      await _fetchHistoricalData(symbol);
      _startRealtimeUpdates();
    } finally {
      setLoading(false);
    }
  }

  Future<void> _fetchHistoricalData(String symbol) async {
    final random = Random();
    final data = <PriceData>[];
    final now = DateTime.now();

    double currentPrice = 150.0 + random.nextDouble() * 50;

    for (int i = 0; i < 100; i++) {
      final change = (random.nextDouble() - 0.5) * 3.0;
      currentPrice += change;
      currentPrice = currentPrice.clamp(50.0, 300.0);

      final open = currentPrice;
      final close = currentPrice + (random.nextDouble() - 0.5) * 1.5;
      final high = max(open, close) + random.nextDouble() * 0.5;
      final low = min(open, close) - random.nextDouble() * 0.5;
      final volume = 1000000 + random.nextInt(2000000);

      data.add(
        PriceData(
          timestamp: now.subtract(Duration(minutes: (100 - i) * 5)),
          open: open,
          high: high,
          low: low,
          close: close,
          volume: volume,
        ),
      );
    }

    _priceHistory = data;
    _latestPrice = data.last;
    safeNotifyListeners();
  }

  void _startRealtimeUpdates() {
    _stopRealtimeUpdates();
    _isStreaming = true;

    _updateTimer = Timer.periodic(const Duration(seconds: 2), (_) {
      _simulatePriceUpdate();
    });
  }

  void _stopRealtimeUpdates() {
    _updateTimer?.cancel();
    _updateTimer = null;
    _isStreaming = false;
  }

  void _simulatePriceUpdate() {
    if (_priceHistory.isEmpty) return;

    final random = Random();
    final lastPrice = _priceHistory.last;

    final change = (random.nextDouble() - 0.5) * 0.5;
    final newClose = lastPrice.close + change;
    final newHigh = max(lastPrice.high, newClose);
    final newLow = min(lastPrice.low, newClose);

    final newPriceData = PriceData(
      timestamp: DateTime.now(),
      open: lastPrice.close,
      high: newHigh,
      low: newLow,
      close: newClose,
      volume: lastPrice.volume + random.nextInt(10000),
    );

    _priceHistory = [..._priceHistory.sublist(1), newPriceData];
    _latestPrice = newPriceData;
    safeNotifyListeners();
  }

  void addRealtimeTick(double price, int volume) {
    final now = DateTime.now();
    final lastPrice = _priceHistory.isNotEmpty ? _priceHistory.last : null;

    if (lastPrice != null &&
        now.difference(lastPrice.timestamp).inMinutes < 5) {
      final updatedLast = PriceData(
        timestamp: lastPrice.timestamp,
        open: lastPrice.open,
        high: max(lastPrice.high, price),
        low: min(lastPrice.low, price),
        close: price,
        volume: lastPrice.volume + volume,
      );
      _priceHistory = [
        ..._priceHistory.sublist(0, _priceHistory.length - 1),
        updatedLast,
      ];
      _latestPrice = updatedLast;
    } else {
      final newBar = PriceData(
        timestamp: now,
        open: price,
        high: price,
        low: price,
        close: price,
        volume: volume,
      );
      _priceHistory = [..._priceHistory.sublist(1), newBar];
      _latestPrice = newBar;
    }

    safeNotifyListeners();
  }

  void stopStreaming() {
    _stopRealtimeUpdates();
    safeNotifyListeners();
  }

  @override
  void dispose() {
    _stopRealtimeUpdates();
    super.dispose();
  }
}

class PriceData {
  final DateTime timestamp;
  final double open;
  final double high;
  final double low;
  final double close;
  final int volume;

  PriceData({
    required this.timestamp,
    required this.open,
    required this.high,
    required this.low,
    required this.close,
    required this.volume,
  });
}
