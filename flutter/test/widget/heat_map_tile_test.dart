import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:bullshift/modules/watchlist/watchlist_provider.dart';
import 'package:bullshift/modules/trendsetter/widgets/cards/heat_map_tile.dart';

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
  group('HeatMapTile Tests', () {
    late MockWatchlistProvider mockWatchlistProvider;

    setUp(() {
      mockWatchlistProvider = MockWatchlistProvider();
    });

    Widget createWidgetUnderTest(Map<String, dynamic> data) {
      return MaterialApp(
        home: ChangeNotifierProvider<WatchlistProvider>.value(
          value: mockWatchlistProvider,
          child: Scaffold(
            body: SizedBox(
              width: 100,
              height: 100,
              child: HeatMapTile(
                data: data,
                watchlistProvider: mockWatchlistProvider,
              ),
            ),
          ),
        ),
      );
    }

    testWidgets('displays symbol correctly', (tester) async {
      final data = {'symbol': 'AAPL', 'heat': 0.75};

      await tester.pumpWidget(createWidgetUnderTest(data));

      expect(find.text('AAPL'), findsOneWidget);
    });

    testWidgets('displays heat percentage', (tester) async {
      final data = {'symbol': 'AAPL', 'heat': 0.75};

      await tester.pumpWidget(createWidgetUnderTest(data));

      expect(find.text('75%'), findsOneWidget);
    });

    testWidgets('shows star icon when in watchlist', (tester) async {
      await mockWatchlistProvider.addToWatchlist('AAPL');

      final data = {'symbol': 'AAPL', 'heat': 0.75};

      await tester.pumpWidget(createWidgetUnderTest(data));

      expect(find.byIcon(Icons.star), findsOneWidget);
    });

    testWidgets('does not show star icon when not in watchlist', (
      tester,
    ) async {
      final data = {'symbol': 'AAPL', 'heat': 0.75};

      await tester.pumpWidget(createWidgetUnderTest(data));

      expect(find.byIcon(Icons.star), findsNothing);
    });

    testWidgets('clamps heat value to 100%', (tester) async {
      final data = {'symbol': 'AAPL', 'heat': 1.5};

      await tester.pumpWidget(createWidgetUnderTest(data));

      expect(find.text('100%'), findsOneWidget);
    });

    testWidgets('clamps heat value to 0%', (tester) async {
      final data = {'symbol': 'AAPL', 'heat': -0.5};

      await tester.pumpWidget(createWidgetUnderTest(data));

      expect(find.text('0%'), findsOneWidget);
    });
  });
}
