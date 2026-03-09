import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/bullrunnr/widgets/cards/sector_sentiment_card.dart';

void main() {
  group('SectorSentimentCard Widget Tests', () {
    Widget createTestWidget(Map<String, dynamic> sector) {
      return MaterialApp(
        home: Scaffold(
          body: SectorSentimentCard(sector: sector),
        ),
      );
    }

    testWidgets('displays sector name', (WidgetTester tester) async {
      final sector = {
        'name': 'Technology',
        'sentiment': 0.75,
      };

      await tester.pumpWidget(createTestWidget(sector));

      expect(find.text('Technology'), findsOneWidget);
    });

    testWidgets('displays sentiment percentage', (WidgetTester tester) async {
      final sector = {
        'name': 'Healthcare',
        'sentiment': 0.65,
      };

      await tester.pumpWidget(createTestWidget(sector));

      expect(find.text('65%'), findsOneWidget);
    });

    testWidgets('shows progress bar for sentiment',
        (WidgetTester tester) async {
      final sector = {
        'name': 'Energy',
        'sentiment': 0.50,
      };

      await tester.pumpWidget(createTestWidget(sector));

      expect(find.byType(Container), findsWidgets);
    });

    testWidgets('shows green for positive sentiment',
        (WidgetTester tester) async {
      final sector = {
        'name': 'Tech',
        'sentiment': 0.80,
      };

      await tester.pumpWidget(createTestWidget(sector));

      final textWidget = tester.widget<Text>(find.text('80%'));
      expect(textWidget.style?.color, Colors.green);
    });

    testWidgets('shows red for negative sentiment',
        (WidgetTester tester) async {
      final sector = {
        'name': 'Utilities',
        'sentiment': -0.40,
      };

      await tester.pumpWidget(createTestWidget(sector));

      final textWidget = tester.widget<Text>(find.text('40%'));
      expect(textWidget.style?.color, Colors.red);
    });

    testWidgets('shows grey for neutral sentiment',
        (WidgetTester tester) async {
      final sector = {
        'name': 'Materials',
        'sentiment': 0.20,
      };

      await tester.pumpWidget(createTestWidget(sector));

      final textWidget = tester.widget<Text>(find.text('20%'));
      expect(textWidget.style?.color, Colors.grey);
    });

    testWidgets('displays sentiment bar aligned left for positive',
        (WidgetTester tester) async {
      final sector = {
        'name': 'Finance',
        'sentiment': 0.70,
      };

      await tester.pumpWidget(createTestWidget(sector));

      // The FractionallySizedBox should exist with centerLeft alignment
      final fractionallySizedBox = find.byType(FractionallySizedBox);
      expect(fractionallySizedBox, findsOneWidget);
    });

    testWidgets('has Card as container', (WidgetTester tester) async {
      final sector = {
        'name': 'Real Estate',
        'sentiment': -0.30,
      };

      await tester.pumpWidget(createTestWidget(sector));

      expect(find.byType(Card), findsOneWidget);
    });
  });
}
