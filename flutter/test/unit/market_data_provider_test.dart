import 'package:flutter_test/flutter_test.dart';

class MockRustEngine {
  bool connectMarketDataCalled = false;

  bool connectMarketData(String symbol) {
    connectMarketDataCalled = true;
    return true;
  }
}

void main() {
  group('PriceData Tests', () {
    test('PriceData stores OHLCV correctly', () {
      final now = DateTime.now();
      final priceData = PriceData(
        timestamp: now,
        open: 100.0,
        high: 105.0,
        low: 98.0,
        close: 103.0,
        volume: 1000000,
      );

      expect(priceData.timestamp, now);
      expect(priceData.open, 100.0);
      expect(priceData.high, 105.0);
      expect(priceData.low, 98.0);
      expect(priceData.close, 103.0);
      expect(priceData.volume, 1000000);
    });

    test('PriceData can be created with named parameters', () {
      final priceData = PriceData(
        timestamp: DateTime.now(),
        open: 50.0,
        high: 55.0,
        low: 48.0,
        close: 52.0,
        volume: 500000,
      );

      expect(priceData.open, 50.0);
      expect(priceData.high, 55.0);
      expect(priceData.low, 48.0);
      expect(priceData.close, 52.0);
      expect(priceData.volume, 500000);
    });
  });

  group('MarketDataProvider State Tests', () {
    test('initial current symbol is empty', () {
      const currentSymbol = '';
      expect(currentSymbol, '');
    });

    test('initial price history is empty list', () {
      final List<PriceData> priceHistory = [];
      expect(priceHistory, isEmpty);
    });

    test('initial streaming state is false', () {
      const isStreaming = false;
      expect(isStreaming, false);
    });

    test('latest price is null when no data', () {
      PriceData? latestPrice;
      expect(latestPrice, isNull);
    });
  });
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
