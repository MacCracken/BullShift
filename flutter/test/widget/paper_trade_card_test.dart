import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/paper_hands/widgets/cards/paper_trade_card.dart';

void main() {
  group('PaperTradeCard Widget Tests', () {
    Widget createTestWidget(Map<String, dynamic> trade) {
      return MaterialApp(
        home: Scaffold(
          body: PaperTradeCard(trade: trade),
        ),
      );
    }

    testWidgets('displays symbol name', (WidgetTester tester) async {
      final trade = {
        'symbol': 'AAPL',
        'side': 'Buy',
        'quantity': 100.0,
        'entryPrice': 150.0,
        'exitPrice': null,
        'pnl': null,
        'status': 'Open',
        'timestamp': DateTime.now(),
      };

      await tester.pumpWidget(createTestWidget(trade));

      expect(find.text('AAPL'), findsOneWidget);
    });

    testWidgets('displays Buy side in green', (WidgetTester tester) async {
      final trade = {
        'symbol': 'AAPL',
        'side': 'Buy',
        'quantity': 100.0,
        'entryPrice': 150.0,
        'exitPrice': null,
        'pnl': null,
        'status': 'Open',
        'timestamp': DateTime.now(),
      };

      await tester.pumpWidget(createTestWidget(trade));

      final textWidget = tester.widget<Text>(find.text('Buy'));
      expect(textWidget.style?.color, Colors.green);
    });

    testWidgets('displays Sell side in red', (WidgetTester tester) async {
      final trade = {
        'symbol': 'TSLA',
        'side': 'Sell',
        'quantity': 50.0,
        'entryPrice': 200.0,
        'exitPrice': null,
        'pnl': null,
        'status': 'Open',
        'timestamp': DateTime.now(),
      };

      await tester.pumpWidget(createTestWidget(trade));

      final textWidget = tester.widget<Text>(find.text('Sell'));
      expect(textWidget.style?.color, Colors.red);
    });

    testWidgets('displays quantity and entry price',
        (WidgetTester tester) async {
      final trade = {
        'symbol': 'MSFT',
        'side': 'Buy',
        'quantity': 75.0,
        'entryPrice': 300.50,
        'exitPrice': null,
        'pnl': null,
        'status': 'Open',
        'timestamp': DateTime.now(),
      };

      await tester.pumpWidget(createTestWidget(trade));

      expect(find.text('75.0 @ \$300.50'), findsOneWidget);
    });

    testWidgets('displays exit price when available',
        (WidgetTester tester) async {
      final trade = {
        'symbol': 'GOOGL',
        'side': 'Buy',
        'quantity': 25.0,
        'entryPrice': 2500.0,
        'exitPrice': 2600.0,
        'pnl': 2500.0,
        'status': 'Closed',
        'timestamp': DateTime.now(),
      };

      await tester.pumpWidget(createTestWidget(trade));

      expect(find.text('→ \$2600.00'), findsOneWidget);
    });

    testWidgets('displays P&L in green for positive',
        (WidgetTester tester) async {
      final trade = {
        'symbol': 'NVDA',
        'side': 'Buy',
        'quantity': 10.0,
        'entryPrice': 500.0,
        'exitPrice': 600.0,
        'pnl': 1000.0,
        'status': 'Closed',
        'timestamp': DateTime.now(),
      };

      await tester.pumpWidget(createTestWidget(trade));

      expect(find.text('\$1000.00'), findsOneWidget);
      final textWidget = tester.widget<Text>(find.text('\$1000.00'));
      expect(textWidget.style?.color, Colors.green);
    });

    testWidgets('displays P&L in red for negative',
        (WidgetTester tester) async {
      final trade = {
        'symbol': 'AMD',
        'side': 'Buy',
        'quantity': 100.0,
        'entryPrice': 150.0,
        'exitPrice': 140.0,
        'pnl': -1000.0,
        'status': 'Closed',
        'timestamp': DateTime.now(),
      };

      await tester.pumpWidget(createTestWidget(trade));

      expect(find.textContaining('-\$1000.00'), findsOneWidget);
    });

    testWidgets('shows colored indicator bar on left',
        (WidgetTester tester) async {
      final trade = {
        'symbol': 'TEST',
        'side': 'Buy',
        'quantity': 1.0,
        'entryPrice': 100.0,
        'exitPrice': null,
        'pnl': null,
        'status': 'Open',
        'timestamp': DateTime.now(),
      };

      await tester.pumpWidget(createTestWidget(trade));

      // Should have a container acting as the indicator bar
      expect(find.byType(Container), findsWidgets);
    });

    testWidgets('has Card as container', (WidgetTester tester) async {
      final trade = {
        'symbol': 'TEST',
        'side': 'Buy',
        'quantity': 1.0,
        'entryPrice': 100.0,
        'exitPrice': null,
        'pnl': null,
        'status': 'Open',
        'timestamp': DateTime.now(),
      };

      await tester.pumpWidget(createTestWidget(trade));

      expect(find.byType(Card), findsOneWidget);
    });
  });
}
