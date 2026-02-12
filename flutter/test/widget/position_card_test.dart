import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:bullshift/modules/core_trading/trading_provider.dart';
import 'package:bullshift/modules/watchlist/watchlist_provider.dart';
import 'package:bullshift/modules/core_trading/widgets/cards/position_card.dart';

class MockTradingProvider extends TradingProvider {
  String _currentSymbol = '';

  @override
  String get currentSymbol => _currentSymbol;

  void setCurrentSymbol(String symbol) {
    _currentSymbol = symbol;
    notifyListeners();
  }
}

class MockWatchlistProvider extends WatchlistProvider {
  final Set<String> _watchlist = {};

  @override
  bool isInWatchlist(String symbol) => _watchlist.contains(symbol);

  @override
  Future<bool> addToWatchlist(String symbol) async {
    _watchlist.add(symbol);
    return true;
  }

  @override
  Future<void> removeFromWatchlist(String symbol) async {
    _watchlist.remove(symbol);
  }
}

void main() {
  group('PositionCard Tests', () {
    late MockTradingProvider mockTradingProvider;
    late MockWatchlistProvider mockWatchlistProvider;

    setUp(() {
      mockTradingProvider = MockTradingProvider();
      mockWatchlistProvider = MockWatchlistProvider();
    });

    Widget createWidgetUnderTest(Map<String, dynamic> position) {
      return MaterialApp(
        home: MultiProvider(
          providers: [
            Provider<TradingProvider>.value(value: mockTradingProvider),
            Provider<WatchlistProvider>.value(value: mockWatchlistProvider),
          ],
          child: Scaffold(body: PositionCard(position: position)),
        ),
      );
    }

    testWidgets('displays symbol correctly', (tester) async {
      final position = {
        'symbol': 'AAPL',
        'quantity': 100,
        'currentPrice': 150.0,
        'unrealizedPnl': 500.0,
      };

      await tester.pumpWidget(createWidgetUnderTest(position));

      expect(find.text('AAPL'), findsOneWidget);
    });

    testWidgets('displays quantity', (tester) async {
      final position = {
        'symbol': 'AAPL',
        'quantity': 100,
        'currentPrice': 150.0,
        'unrealizedPnl': 500.0,
      };

      await tester.pumpWidget(createWidgetUnderTest(position));

      expect(find.text('100'), findsOneWidget);
    });

    testWidgets('displays current price formatted', (tester) async {
      final position = {
        'symbol': 'AAPL',
        'quantity': 100,
        'currentPrice': 150.0,
        'unrealizedPnl': 500.0,
      };

      await tester.pumpWidget(createWidgetUnderTest(position));

      expect(find.text('\$150.00'), findsOneWidget);
    });

    testWidgets('displays positive PnL in green', (tester) async {
      final position = {
        'symbol': 'AAPL',
        'quantity': 100,
        'currentPrice': 150.0,
        'unrealizedPnl': 500.0,
      };

      await tester.pumpWidget(createWidgetUnderTest(position));

      expect(find.text('\$500.00'), findsOneWidget);
    });

    testWidgets('displays negative PnL in red', (tester) async {
      final position = {
        'symbol': 'AAPL',
        'quantity': 100,
        'currentPrice': 150.0,
        'unrealizedPnl': -200.0,
      };

      await tester.pumpWidget(createWidgetUnderTest(position));

      expect(find.text('\$-200.00'), findsOneWidget);
    });

    testWidgets('shows Add to Watchlist button when not in watchlist', (
      tester,
    ) async {
      mockTradingProvider.setCurrentSymbol('AAPL');

      final position = {
        'symbol': 'AAPL',
        'quantity': 100,
        'currentPrice': 150.0,
        'unrealizedPnl': 500.0,
      };

      await tester.pumpWidget(createWidgetUnderTest(position));

      expect(find.text('Add AAPL to Watchlist'), findsOneWidget);
    });

    testWidgets('shows Watching button when in watchlist', (tester) async {
      mockTradingProvider.setCurrentSymbol('AAPL');
      await mockWatchlistProvider.addToWatchlist('AAPL');

      final position = {
        'symbol': 'AAPL',
        'quantity': 100,
        'currentPrice': 150.0,
        'unrealizedPnl': 500.0,
      };

      await tester.pumpWidget(createWidgetUnderTest(position));

      expect(find.text('Watching AAPL'), findsOneWidget);
    });

    testWidgets('hides watchlist button when no symbol selected', (
      tester,
    ) async {
      mockTradingProvider.setCurrentSymbol('');

      final position = {
        'symbol': 'AAPL',
        'quantity': 100,
        'currentPrice': 150.0,
        'unrealizedPnl': 500.0,
      };

      await tester.pumpWidget(createWidgetUnderTest(position));

      expect(find.text('Add AAPL to Watchlist'), findsNothing);
      expect(find.text('Watching AAPL'), findsNothing);
    });
  });
}
