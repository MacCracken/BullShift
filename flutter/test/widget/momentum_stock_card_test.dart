import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/watchlist/watchlist_provider.dart';
import 'package:bullshift/modules/trendsetter/widgets/cards/momentum_stock_card.dart';

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
  group('MomentumStockCard Tests', () {
    late MockWatchlistProvider mockWatchlistProvider;

    setUp(() {
      mockWatchlistProvider = MockWatchlistProvider();
    });

    Widget createWidgetUnderTest(Map<String, dynamic> stock) {
      return MaterialApp(
        home: Scaffold(
          body: MomentumStockCard(
            stock: stock,
            watchlistProvider: mockWatchlistProvider,
          ),
        ),
      );
    }

    testWidgets('displays symbol correctly', (tester) async {
      final stock = {
        'symbol': 'AAPL',
        'score': 0.85,
        'volumeSpike': 2.5,
        'priceMomentum': 0.75,
        'socialSentiment': 0.6,
        'trendStrength': 'Strong',
      };

      await tester.pumpWidget(createWidgetUnderTest(stock));

      expect(find.text('AAPL'), findsOneWidget);
    });

    testWidgets('displays score percentage', (tester) async {
      final stock = {
        'symbol': 'AAPL',
        'score': 0.85,
        'volumeSpike': 2.5,
        'priceMomentum': 0.75,
        'socialSentiment': 0.6,
        'trendStrength': 'Strong',
      };

      await tester.pumpWidget(createWidgetUnderTest(stock));

      expect(find.text('85%'), findsOneWidget);
    });

    testWidgets('displays trend strength badge', (tester) async {
      final stock = {
        'symbol': 'AAPL',
        'score': 0.85,
        'volumeSpike': 2.5,
        'priceMomentum': 0.75,
        'socialSentiment': 0.6,
        'trendStrength': 'Explosive',
      };

      await tester.pumpWidget(createWidgetUnderTest(stock));

      expect(find.text('Explosive'), findsOneWidget);
    });

    testWidgets('shows Watch button when not in watchlist', (tester) async {
      final stock = {
        'symbol': 'AAPL',
        'score': 0.85,
        'volumeSpike': 2.5,
        'priceMomentum': 0.75,
        'socialSentiment': 0.6,
        'trendStrength': 'Strong',
      };

      await tester.pumpWidget(createWidgetUnderTest(stock));

      expect(find.text('Watch'), findsOneWidget);
    });

    testWidgets('shows Trade button', (tester) async {
      final stock = {
        'symbol': 'AAPL',
        'score': 0.85,
        'volumeSpike': 2.5,
        'priceMomentum': 0.75,
        'socialSentiment': 0.6,
        'trendStrength': 'Strong',
      };

      await tester.pumpWidget(createWidgetUnderTest(stock));

      expect(find.text('Trade'), findsOneWidget);
    });

    testWidgets('shows Analysis button', (tester) async {
      final stock = {
        'symbol': 'AAPL',
        'score': 0.85,
        'volumeSpike': 2.5,
        'priceMomentum': 0.75,
        'socialSentiment': 0.6,
        'trendStrength': 'Strong',
      };

      await tester.pumpWidget(createWidgetUnderTest(stock));

      expect(find.text('Analysis'), findsOneWidget);
    });

    testWidgets('opens QuickTradeDialog when Trade button tapped', (
      tester,
    ) async {
      final stock = {
        'symbol': 'AAPL',
        'score': 0.85,
        'volumeSpike': 2.5,
        'priceMomentum': 0.75,
        'socialSentiment': 0.6,
        'trendStrength': 'Strong',
      };

      await tester.pumpWidget(createWidgetUnderTest(stock));

      await tester.tap(find.text('Trade'));
      await tester.pumpAndSettle();

      expect(find.text('Quick Trade - AAPL'), findsOneWidget);
    });

    testWidgets('opens AnalysisDialog when Analysis button tapped', (
      tester,
    ) async {
      final stock = {
        'symbol': 'AAPL',
        'score': 0.85,
        'volumeSpike': 2.5,
        'priceMomentum': 0.75,
        'socialSentiment': 0.6,
        'trendStrength': 'Strong',
      };

      await tester.pumpWidget(createWidgetUnderTest(stock));

      await tester.tap(find.text('Analysis'));
      await tester.pumpAndSettle();

      expect(find.text('Analysis - AAPL'), findsOneWidget);
    });

    testWidgets('displays metric bars', (tester) async {
      final stock = {
        'symbol': 'AAPL',
        'score': 0.85,
        'volumeSpike': 0.5,
        'priceMomentum': 0.75,
        'socialSentiment': 0.3,
        'trendStrength': 'Strong',
      };

      await tester.pumpWidget(createWidgetUnderTest(stock));

      expect(find.text('Volume: 50%'), findsOneWidget);
      expect(find.text('Price: 75%'), findsOneWidget);
      expect(find.text('Social: 30%'), findsOneWidget);
    });
  });
}
