import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/watchlist/widgets/cards/watchlist_card.dart';

void main() {
  group('WatchlistCard Tests', () {
    Widget createWidgetUnderTest(Map<String, dynamic> item) {
      return MaterialApp(
        home: Scaffold(
          body: WatchlistCard(
            item: item,
            onRemove: () {},
            onTrade: () {},
            onChart: () {},
          ),
        ),
      );
    }

    testWidgets('displays symbol correctly', (tester) async {
      final item = {
        'symbol': 'AAPL',
        'currentPrice': 150.0,
        'dayChange': 2.5,
        'dayChangePercent': 1.5,
        'volume': 50000000,
      };

      await tester.pumpWidget(createWidgetUnderTest(item));

      expect(find.text('AAPL'), findsOneWidget);
    });

    testWidgets('displays formatted price', (tester) async {
      final item = {
        'symbol': 'AAPL',
        'currentPrice': 150.0,
        'dayChange': 2.5,
        'dayChangePercent': 1.5,
        'volume': 50000000,
      };

      await tester.pumpWidget(createWidgetUnderTest(item));

      expect(find.text('\$150.00'), findsOneWidget);
    });

    testWidgets('displays positive day change with up arrow', (tester) async {
      final item = {
        'symbol': 'AAPL',
        'currentPrice': 150.0,
        'dayChange': 2.5,
        'dayChangePercent': 1.5,
        'volume': 50000000,
      };

      await tester.pumpWidget(createWidgetUnderTest(item));

      expect(find.byIcon(Icons.arrow_upward), findsOneWidget);
    });

    testWidgets('displays negative day change with down arrow', (tester) async {
      final item = {
        'symbol': 'AAPL',
        'currentPrice': 150.0,
        'dayChange': -2.5,
        'dayChangePercent': -1.5,
        'volume': 50000000,
      };

      await tester.pumpWidget(createWidgetUnderTest(item));

      expect(find.byIcon(Icons.arrow_downward), findsOneWidget);
    });

    testWidgets('displays volume in millions', (tester) async {
      final item = {
        'symbol': 'AAPL',
        'currentPrice': 150.0,
        'dayChange': 2.5,
        'dayChangePercent': 1.5,
        'volume': 50000000,
      };

      await tester.pumpWidget(createWidgetUnderTest(item));

      expect(find.text('Vol: 50.0M'), findsOneWidget);
    });

    testWidgets('displays volume in thousands', (tester) async {
      final item = {
        'symbol': 'AAPL',
        'currentPrice': 150.0,
        'dayChange': 2.5,
        'dayChangePercent': 1.5,
        'volume': 500000,
      };

      await tester.pumpWidget(createWidgetUnderTest(item));

      expect(find.text('Vol: 500.0K'), findsOneWidget);
    });

    testWidgets('has Chart button', (tester) async {
      final item = {
        'symbol': 'AAPL',
        'currentPrice': 150.0,
        'dayChange': 2.5,
        'dayChangePercent': 1.5,
        'volume': 50000000,
      };

      await tester.pumpWidget(createWidgetUnderTest(item));

      expect(find.text('Chart'), findsOneWidget);
    });

    testWidgets('has Trade button', (tester) async {
      final item = {
        'symbol': 'AAPL',
        'currentPrice': 150.0,
        'dayChange': 2.5,
        'dayChangePercent': 1.5,
        'volume': 50000000,
      };

      await tester.pumpWidget(createWidgetUnderTest(item));

      expect(find.text('Trade'), findsOneWidget);
    });

    testWidgets('has remove button', (tester) async {
      final item = {
        'symbol': 'AAPL',
        'currentPrice': 150.0,
        'dayChange': 2.5,
        'dayChangePercent': 1.5,
        'volume': 50000000,
      };

      await tester.pumpWidget(createWidgetUnderTest(item));

      expect(find.byIcon(Icons.remove_circle), findsOneWidget);
    });

    testWidgets('shows confirmation dialog on remove tap', (tester) async {
      final item = {
        'symbol': 'AAPL',
        'currentPrice': 150.0,
        'dayChange': 2.5,
        'dayChangePercent': 1.5,
        'volume': 50000000,
      };

      await tester.pumpWidget(createWidgetUnderTest(item));

      await tester.tap(find.byIcon(Icons.remove_circle));
      await tester.pumpAndSettle();

      expect(find.text('Remove AAPL?'), findsOneWidget);
    });

    testWidgets('calls onChart when Chart button tapped', (tester) async {
      bool chartTapped = false;

      final item = {
        'symbol': 'AAPL',
        'currentPrice': 150.0,
        'dayChange': 2.5,
        'dayChangePercent': 1.5,
        'volume': 50000000,
      };

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: WatchlistCard(
              item: item,
              onRemove: () {},
              onTrade: () {},
              onChart: () => chartTapped = true,
            ),
          ),
        ),
      );

      await tester.tap(find.text('Chart'));
      await tester.pump();

      expect(chartTapped, isTrue);
    });
  });

  group('SearchResultTile Tests', () {
    Widget createSearchResultTile(Map<String, dynamic> result) {
      return MaterialApp(
        home: Scaffold(
          body: SearchResultTile(result: result, onAdd: () {}),
        ),
      );
    }

    testWidgets('displays symbol', (tester) async {
      final result = {
        'symbol': 'AAPL',
        'name': 'Apple Inc.',
        'exchange': 'NASDAQ',
        'type': 'Stock',
      };

      await tester.pumpWidget(createSearchResultTile(result));

      expect(find.text('AAPL'), findsOneWidget);
    });

    testWidgets('displays company name', (tester) async {
      final result = {
        'symbol': 'AAPL',
        'name': 'Apple Inc.',
        'exchange': 'NASDAQ',
        'type': 'Stock',
      };

      await tester.pumpWidget(createSearchResultTile(result));

      expect(find.text('Apple Inc.'), findsOneWidget);
    });

    testWidgets('displays exchange badge', (tester) async {
      final result = {
        'symbol': 'AAPL',
        'name': 'Apple Inc.',
        'exchange': 'NASDAQ',
        'type': 'Stock',
      };

      await tester.pumpWidget(createSearchResultTile(result));

      expect(find.text('NASDAQ'), findsOneWidget);
    });

    testWidgets('displays type badge', (tester) async {
      final result = {
        'symbol': 'AAPL',
        'name': 'Apple Inc.',
        'exchange': 'NASDAQ',
        'type': 'Stock',
      };

      await tester.pumpWidget(createSearchResultTile(result));

      expect(find.text('Stock'), findsOneWidget);
    });

    testWidgets('has add button', (tester) async {
      final result = {
        'symbol': 'AAPL',
        'name': 'Apple Inc.',
        'exchange': 'NASDAQ',
        'type': 'Stock',
      };

      await tester.pumpWidget(createSearchResultTile(result));

      expect(find.byIcon(Icons.add_circle), findsOneWidget);
    });
  });
}
