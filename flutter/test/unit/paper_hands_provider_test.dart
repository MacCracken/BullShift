import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/paper_hands/paper_hands_provider.dart';

void main() {
  group('PaperHandsProvider Tests', () {
    late PaperHandsProvider provider;

    setUp(() {
      provider = PaperHandsProvider();
    });

    tearDown(() {
      provider.dispose();
    });

    group('Initialization', () {
      test('initializes with default values', () {
        expect(provider.paperPortfolios, isEmpty);
        expect(provider.selectedPortfolio, isNull);
        expect(provider.currentSymbol, isEmpty);
        expect(provider.selectedTimeframe, '1D');
        expect(provider.selectedOrderType, 'Market');
        expect(provider.selectedSide, 'Buy');
        expect(provider.quantity, 0.0);
        expect(provider.price, isNull);
        expect(provider.recentTrades, isEmpty);
        expect(provider.isLoading, false);
      });
    });

    group('Portfolio Management', () {
      test('createPortfolio adds a new portfolio', () async {
        await provider.createPortfolio('Test Portfolio', 10000.0);

        expect(provider.paperPortfolios.length, 1);
        expect(provider.paperPortfolios.first['name'], 'Test Portfolio');
        expect(provider.paperPortfolios.first['initialBalance'], 10000.0);
        expect(provider.paperPortfolios.first['currentBalance'], 10000.0);
      });

      test('selectPortfolio sets active portfolio', () async {
        await provider.createPortfolio('Test Portfolio', 10000.0);
        final portfolioId = provider.paperPortfolios.first['id'];

        provider.selectPortfolio(portfolioId);

        expect(provider.selectedPortfolio, isNotNull);
        expect(provider.selectedPortfolio!['id'], portfolioId);
      });

      test('deletePortfolio removes portfolio', () async {
        await provider.createPortfolio('Test Portfolio', 10000.0);
        final portfolioId = provider.paperPortfolios.first['id'];

        provider.deletePortfolio(portfolioId);

        expect(provider.paperPortfolios, isEmpty);
        expect(provider.selectedPortfolio, isNull);
      });

      test('selecting deleted portfolio clears selection', () async {
        await provider.createPortfolio('Test Portfolio', 10000.0);
        final portfolioId = provider.paperPortfolios.first['id'];

        provider.selectPortfolio(portfolioId);
        expect(provider.selectedPortfolio, isNotNull);

        provider.deletePortfolio(portfolioId);
        expect(provider.selectedPortfolio, isNull);
      });
    });

    group('Trading Controls', () {
      test('setSymbol updates current symbol', () {
        provider.setSymbol('AAPL');

        expect(provider.currentSymbol, 'AAPL');
      });

      test('setQuantity updates quantity', () {
        provider.setQuantity(100.0);

        expect(provider.quantity, 100.0);
      });

      test('setOrderType updates order type', () {
        provider.setOrderType('Limit');

        expect(provider.selectedOrderType, 'Limit');
      });

      test('setSide updates side', () {
        provider.setSide('Sell');

        expect(provider.selectedSide, 'Sell');
      });

      test('setPrice updates price', () {
        provider.setPrice(150.0);

        expect(provider.price, 150.0);
      });

      test('canPlaceOrder validates order requirements', () async {
        // Initially should not be able to place order (no portfolio, no symbol, no quantity)
        expect(provider.canPlaceOrder, false);

        // Create portfolio and select it
        await provider.createPortfolio('Test Portfolio', 10000.0);
        provider.selectPortfolio(provider.paperPortfolios.first['id']);

        // Still can't place order (no symbol or quantity)
        expect(provider.canPlaceOrder, false);

        // Set symbol and quantity
        provider.setSymbol('AAPL');
        provider.setQuantity(10.0);

        // Now should be able to place order
        expect(provider.canPlaceOrder, true);
      });

      test('canPlaceOrder requires price for limit orders', () async {
        await provider.createPortfolio('Test Portfolio', 10000.0);
        provider.selectPortfolio(provider.paperPortfolios.first['id']);
        provider.setSymbol('AAPL');
        provider.setQuantity(10.0);
        provider.setOrderType('Limit');

        // Can't place limit order without price
        expect(provider.canPlaceOrder, false);

        // Set price
        provider.setPrice(150.0);
        expect(provider.canPlaceOrder, true);
      });
    });

    group('Paper Trading', () {
      test('placePaperOrder creates a trade', () async {
        await provider.createPortfolio('Test Portfolio', 10000.0);
        provider.selectPortfolio(provider.paperPortfolios.first['id']);
        provider.setSymbol('AAPL');
        provider.setQuantity(10.0);
        provider.setOrderType('Market');
        provider.setSide('Buy');

        await provider.placePaperOrder();

        expect(provider.recentTrades, isNotEmpty);
        expect(provider.recentTrades.first['symbol'], 'AAPL');
        expect(provider.recentTrades.first['side'], 'Buy');
        expect(provider.recentTrades.first['quantity'], 10.0);
      });

      test('trade affects portfolio balance', () async {
        await provider.createPortfolio('Test Portfolio', 10000.0);
        final initialBalance = provider.paperPortfolios.first['currentBalance'];

        provider.selectPortfolio(provider.paperPortfolios.first['id']);
        provider.setSymbol('AAPL');
        provider.setQuantity(10.0);

        await provider.placePaperOrder();

        // Balance should have changed
        final currentBalance = provider.paperPortfolios.first['currentBalance'];
        expect(currentBalance, isNot(equals(initialBalance)));
      });
    });

    group('Timeframe Selection', () {
      test('setTimeframe updates selected timeframe', () {
        provider.setTimeframe('4h');

        expect(provider.selectedTimeframe, '4h');
      });

      test('timeframe accepts valid values', () {
        final validTimeframes = ['1m', '5m', '15m', '1h', '4h', '1D', '1W'];

        for (final tf in validTimeframes) {
          provider.setTimeframe(tf);
          expect(provider.selectedTimeframe, tf);
        }
      });
    });

    group('Refresh Operations', () {
      test('refreshPortfolioData updates portfolio data', () async {
        await provider.createPortfolio('Test Portfolio', 10000.0);
        provider.selectPortfolio(provider.paperPortfolios.first['id']);

        // Should not throw
        await provider.refreshPortfolioData();

        expect(provider.selectedPortfolio, isNotNull);
      });

      test('refreshAnalytics updates analytics', () async {
        await provider.createPortfolio('Test Portfolio', 10000.0);
        provider.selectPortfolio(provider.paperPortfolios.first['id']);

        // Should not throw
        await provider.refreshAnalytics();
      });
    });
  });
}
