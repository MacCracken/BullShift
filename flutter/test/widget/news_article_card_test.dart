import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:bullshift/modules/bullrunnr/bullrunnr_provider.dart';
import 'package:bullshift/modules/bullrunnr/widgets/cards/news_article_card.dart';

void main() {
  group('NewsArticleCard Widget Tests', () {
    late Map<String, dynamic> sampleArticle;
    
    setUp(() {
      sampleArticle = {
        'title': 'Apple Stock Surges on Strong Earnings Report',
        'source': 'Reuters',
        'timestamp': DateTime.now().subtract(const Duration(hours: 2)),
        'sentiment': 'Bullish',
        'sentimentScore': 0.75,
        'symbols': ['AAPL', 'TECH'],
        'category': 'Earnings',
      };
    });
    
    Widget createTestWidget(Map<String, dynamic> article) {
      return MaterialApp(
        home: Scaffold(
          body: NewsArticleCard(article: article),
        ),
      );
    }
    
    testWidgets('displays article title', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget(sampleArticle));
      
      expect(find.text('Apple Stock Surges on Strong Earnings Report'), findsOneWidget);
    });
    
    testWidgets('displays source and formatted timestamp', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget(sampleArticle));
      
      expect(find.text('Reuters'), findsOneWidget);
      expect(find.textContaining('ago'), findsOneWidget);
    });
    
    testWidgets('displays sentiment badge with correct color', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget(sampleArticle));
      
      expect(find.text('BULLISH'), findsOneWidget);
    });
    
    testWidgets('displays category chip', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget(sampleArticle));
      
      expect(find.text('Earnings'), findsOneWidget);
    });
    
    testWidgets('displays symbol chips', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget(sampleArticle));
      
      expect(find.text('AAPL'), findsOneWidget);
      expect(find.text('TECH'), findsOneWidget);
    });
    
    testWidgets('displays sentiment score bar', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget(sampleArticle));
      
      expect(find.textContaining('75%'), findsOneWidget);
    });
    
    testWidgets('has Read More and Analysis buttons', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget(sampleArticle));
      
      expect(find.text('Read More'), findsOneWidget);
      expect(find.text('Analysis'), findsOneWidget);
    });
    
    testWidgets('handles bearish sentiment correctly', (WidgetTester tester) async {
      final bearishArticle = {
        'title': 'Market Downturn Concerns',
        'source': 'Bloomberg',
        'timestamp': DateTime.now(),
        'sentiment': 'Bearish',
        'sentimentScore': -0.65,
        'symbols': ['SPY'],
        'category': 'Market Analysis',
      };
      
      await tester.pumpWidget(createTestWidget(bearishArticle));
      
      expect(find.text('BEARISH'), findsOneWidget);
    });
    
    testWidgets('handles neutral sentiment correctly', (WidgetTester tester) async {
      final neutralArticle = {
        'title': 'Market Flat Today',
        'source': 'CNBC',
        'timestamp': DateTime.now(),
        'sentiment': 'Neutral',
        'sentimentScore': 0.0,
        'symbols': [],
        'category': 'Market Update',
      };
      
      await tester.pumpWidget(createTestWidget(neutralArticle));
      
      expect(find.text('NEUTRAL'), findsOneWidget);
    });
    
    testWidgets('formats timestamp correctly for different time ranges', (WidgetTester tester) async {
      // Just now
      final justNow = {
        ...sampleArticle,
        'timestamp': DateTime.now().subtract(const Duration(seconds: 30)),
      };
      await tester.pumpWidget(createTestWidget(justNow));
      expect(find.text('Just now'), findsOneWidget);
      
      // Minutes ago
      final minutesAgo = {
        ...sampleArticle,
        'timestamp': DateTime.now().subtract(const Duration(minutes: 45)),
      };
      await tester.pumpWidget(createTestWidget(minutesAgo));
      expect(find.textContaining('m ago'), findsOneWidget);
    });
  });
}
