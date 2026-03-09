import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/bullrunnr/widgets/cards/sentiment_mover_card.dart';

void main() {
  group('SentimentMoverCard Widget Tests', () {
    Widget createTestWidget(Map<String, dynamic> mover) {
      return MaterialApp(
        home: Scaffold(
          body: SentimentMoverCard(mover: mover),
        ),
      );
    }

    testWidgets('displays symbol name', (WidgetTester tester) async {
      final mover = {
        'symbol': 'AAPL',
        'sentimentScore': 0.75,
        'buzzScore': 0.85,
        'articleCount': 42,
      };

      await tester.pumpWidget(createTestWidget(mover));

      expect(find.text('AAPL'), findsOneWidget);
    });

    testWidgets('displays article count', (WidgetTester tester) async {
      final mover = {
        'symbol': 'TSLA',
        'sentimentScore': 0.60,
        'buzzScore': 0.70,
        'articleCount': 15,
      };

      await tester.pumpWidget(createTestWidget(mover));

      expect(find.text('15 articles'), findsOneWidget);
    });

    testWidgets('displays sentiment score percentage',
        (WidgetTester tester) async {
      final mover = {
        'symbol': 'NVDA',
        'sentimentScore': 0.82,
        'buzzScore': 0.90,
        'articleCount': 28,
      };

      await tester.pumpWidget(createTestWidget(mover));

      expect(find.text('82%'), findsOneWidget);
    });

    testWidgets('displays buzz score', (WidgetTester tester) async {
      final mover = {
        'symbol': 'MSFT',
        'sentimentScore': 0.45,
        'buzzScore': 0.65,
        'articleCount': 20,
      };

      await tester.pumpWidget(createTestWidget(mover));

      expect(find.text('Buzz: 65%'), findsOneWidget);
    });

    testWidgets('shows green color for positive sentiment',
        (WidgetTester tester) async {
      final mover = {
        'symbol': 'BULL',
        'sentimentScore': 0.75,
        'buzzScore': 0.80,
        'articleCount': 10,
      };

      await tester.pumpWidget(createTestWidget(mover));

      final textWidget = tester.widget<Text>(find.text('75%'));
      expect(textWidget.style?.color, Colors.green);
    });

    testWidgets('shows red color for negative sentiment',
        (WidgetTester tester) async {
      final mover = {
        'symbol': 'BEAR',
        'sentimentScore': -0.75,
        'buzzScore': 0.60,
        'articleCount': 8,
      };

      await tester.pumpWidget(createTestWidget(mover));

      final textWidget = tester.widget<Text>(find.text('75%'));
      expect(textWidget.style?.color, Colors.red);
    });

    testWidgets('shows grey color for neutral sentiment',
        (WidgetTester tester) async {
      final mover = {
        'symbol': 'NEUT',
        'sentimentScore': 0.10,
        'buzzScore': 0.50,
        'articleCount': 5,
      };

      await tester.pumpWidget(createTestWidget(mover));

      final textWidget = tester.widget<Text>(find.text('10%'));
      expect(textWidget.style?.color, Colors.grey);
    });

    testWidgets('has Card widget as container', (WidgetTester tester) async {
      final mover = {
        'symbol': 'TEST',
        'sentimentScore': 0.50,
        'buzzScore': 0.50,
        'articleCount': 1,
      };

      await tester.pumpWidget(createTestWidget(mover));

      expect(find.byType(Card), findsOneWidget);
    });
  });
}
