import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/core_trading/trading_provider.dart';
import 'package:bullshift/services/rust_trading_engine.dart';
import 'package:mockito/mockito.dart';
import 'package:mockito/annotations.dart';

@GenerateMocks([RustTradingEngine])
import 'trading_provider_error_handling_test.mocks.dart';

void main() {
  group('TradingProvider Error Handling Tests', () {
    late TradingProvider provider;
    late MockRustTradingEngine mockEngine;

    setUp(() {
      mockEngine = MockRustTradingEngine();
      provider = TradingProvider(mockEngine);
    });

    tearDown(() {
      provider.dispose();
    });

    group('Order Validation Errors', () {
      test('rejects market order with empty symbol', () async {
        await provider.submitMarketOrder('BUY');

        expect(provider.hasError, true);
        expect(provider.errorMessage, contains('symbol'));
      });

      test('rejects market order with zero quantity', () async {
        provider.setSymbol('AAPL');
        provider.setQuantity(0.0);

        await provider.submitMarketOrder('BUY');

        expect(provider.hasError, true);
        expect(provider.errorMessage, contains('quantity'));
      });

      test('rejects market order with negative quantity', () async {
        provider.setSymbol('AAPL');
        provider.setQuantity(-10.0);

        await provider.submitMarketOrder('BUY');

        expect(provider.hasError, true);
        expect(provider.errorMessage, contains('quantity'));
      });

      test('rejects limit order without price', () async {
        provider.setSymbol('AAPL');
        provider.setQuantity(100.0);
        provider.setOrderType('LIMIT');
        // Don't set price

        await provider.submitLimitOrder('BUY');

        expect(provider.hasError, true);
        expect(provider.errorMessage, contains('price'));
      });

      test('rejects limit order with zero price', () async {
        provider.setSymbol('AAPL');
        provider.setQuantity(100.0);
        provider.setOrderType('LIMIT');
        provider.setPrice(0.0);

        await provider.submitLimitOrder('BUY');

        expect(provider.hasError, true);
        expect(provider.errorMessage, contains('price'));
      });
    });

    group('Engine Failure Handling', () {
      test('handles engine submission failure', () async {
        provider.setSymbol('AAPL');
        provider.setQuantity(100.0);

        // Mock engine to throw exception
        when(
          mockEngine.submitMarketOrder(any, any, any, any),
        ).thenThrow(Exception('Network error'));

        await provider.submitMarketOrder('BUY');

        expect(provider.hasError, true);
        expect(provider.errorMessage, contains('Network error'));
      });

      test('handles position fetch failure', () async {
        // Mock engine to throw exception when fetching positions
        when(
          mockEngine.getPositions(),
        ).thenThrow(Exception('Connection failed'));

        await provider.refreshPositions();

        expect(provider.hasError, true);
        expect(provider.errorMessage, contains('Connection failed'));
      });

      test('handles balance fetch failure', () async {
        // Mock engine to throw exception when fetching balance
        when(
          mockEngine.getAccountBalance(),
        ).thenThrow(Exception('Authentication failed'));

        await provider.refreshBalance();

        expect(provider.hasError, true);
        expect(provider.errorMessage, contains('Authentication failed'));
      });
    });

    group('Data Validation', () {
      test('validates symbol format', () {
        // Test valid symbols
        provider.setSymbol('AAPL');
        expect(provider.currentSymbol, 'AAPL');

        provider.setSymbol('GOOGL');
        expect(provider.currentSymbol, 'GOOGL');

        provider.setSymbol('msft'); // Should convert to uppercase
        expect(provider.currentSymbol, 'MSFT');
      });

      test('handles special characters in symbol', () {
        provider.setSymbol('BRK.A'); // Test dot notation
        expect(provider.currentSymbol, 'BRK.A');

        provider.setSymbol('BTC-USD'); // Test hyphen
        expect(provider.currentSymbol, 'BTC-USD');
      });

      test('validates quantity bounds', () {
        // Test reasonable bounds
        provider.setQuantity(0.01);
        expect(provider.currentQuantity, 0.01);

        provider.setQuantity(999999.0);
        expect(provider.currentQuantity, 999999.0);

        // Test very small quantity
        provider.setQuantity(0.000001);
        expect(provider.currentQuantity, 0.000001);
      });
    });

    group('State Management', () {
      test('clears error after successful operation', () async {
        // First trigger an error
        await provider.submitMarketOrder('BUY');
        expect(provider.hasError, true);

        // Then perform successful operation
        provider.setSymbol('AAPL');
        provider.setQuantity(100.0);

        when(
          mockEngine.submitMarketOrder(any, any, any, any),
        ).thenAnswer((_) async => {'order_id': '12345'});

        await provider.submitMarketOrder('BUY');

        expect(provider.hasError, false);
        expect(provider.errorMessage, null);
      });

      test('maintains state during operation', () async {
        provider.setSymbol('AAPL');
        provider.setQuantity(100.0);
        provider.setOrderType('LIMIT');
        provider.setPrice(150.0);

        // Mock slow operation
        when(mockEngine.submitLimitOrder(any, any, any, any, any)).thenAnswer((
          _,
        ) async {
          await Future.delayed(Duration(milliseconds: 100));
          return {'order_id': '12345'};
        });

        final orderFuture = provider.submitLimitOrder('BUY');

        // Check that state is preserved during operation
        expect(provider.currentSymbol, 'AAPL');
        expect(provider.currentQuantity, 100.0);
        expect(provider.orderType, 'LIMIT');
        expect(provider.limitPrice, 150.0);
        expect(provider.isLoading, true);

        await orderFuture;

        expect(provider.isLoading, false);
      });
    });

    group('Edge Cases', () {
      test('handles very large symbol names', () {
        const longSymbol = 'VERYLONGSYMBOLNAMETHATEXCEEDSNORMALEXPECTATIONS';
        provider.setSymbol(longSymbol);
        expect(provider.currentSymbol, longSymbol);
      });

      test('handles extreme quantities', () {
        provider.setQuantity(double.maxFinite);
        expect(provider.currentQuantity, double.maxFinite);

        provider.setQuantity(double.minPositive);
        expect(provider.currentQuantity, double.minPositive);
      });

      test('handles special price values', () {
        provider.setPrice(double.maxFinite);
        expect(provider.limitPrice, double.maxFinite);

        provider.setPrice(0.000001);
        expect(provider.limitPrice, 0.000001);
      });
    });
  });
}
