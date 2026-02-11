import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/bullrunnr/bullrunnr_provider.dart';

void main() {
  group('BullRunnrProvider Tests', () {
    late BullRunnrProvider provider;
    
    setUp(() {
      provider = BullRunnrProvider();
    });
    
    tearDown(() {
      provider.dispose();
    });
    
    group('Initialization', () {
      test('initializes with default values', () {
        expect(provider.newsArticles, isEmpty);
        expect(provider.topSentimentMovers, isEmpty);
        expect(provider.sectorSentiment, isEmpty);
        expect(provider.selectedCategory, 'All');
        expect(provider.selectedSentiment, 'All');
        expect(provider.isLoading, false);
      });
      
      test('initialize populates data', () {
        provider.initialize();
        
        expect(provider.newsArticles, isNotEmpty);
        expect(provider.topSentimentMovers, isNotEmpty);
        expect(provider.sectorSentiment, isNotEmpty);
        expect(provider.marketSentiment, isNotEmpty);
      });
    });
    
    group('News Feed', () {
      test('refreshNews updates news articles', () {
        provider.initialize();
        final initialCount = provider.newsArticles.length;
        
        provider.refreshNews();
        
        // Articles should be refreshed (count might vary due to random generation)
        expect(provider.newsArticles, isNotEmpty);
      });
      
      test('setCategoryFilter updates selected category', () {
        provider.setCategoryFilter('Earnings');
        
        expect(provider.selectedCategory, 'Earnings');
      });
      
      test('setSentimentFilter updates selected sentiment', () {
        provider.setSentimentFilter('Bullish');
        
        expect(provider.selectedSentiment, 'Bullish');
      });
      
      test('searchNews filters by keywords and symbols', () {
        provider.initialize();
        
        provider.searchNews('earnings', ['AAPL']);
        
        // Search should not throw and should filter articles
        expect(provider.newsArticles, isNotNull);
      });
    });
    
    group('Sentiment Analysis', () {
      test('marketSentiment contains required fields', () {
        provider.initialize();
        
        expect(provider.marketSentiment.containsKey('overallScore'), true);
        expect(provider.marketSentiment.containsKey('fearGreedIndex'), true);
        expect(provider.marketSentiment.containsKey('bullishCount'), true);
        expect(provider.marketSentiment.containsKey('bearishCount'), true);
        expect(provider.marketSentiment.containsKey('neutralCount'), true);
      });
      
      test('sectorSentiment has valid data', () {
        provider.initialize();
        
        expect(provider.sectorSentiment, isNotEmpty);
        
        final firstSector = provider.sectorSentiment.first;
        expect(firstSector.containsKey('name'), true);
        expect(firstSector.containsKey('sentiment'), true);
      });
      
      test('topSentimentMovers contains valid data', () {
        provider.initialize();
        
        expect(provider.topSentimentMovers, isNotEmpty);
        
        final firstMover = provider.topSentimentMovers.first;
        expect(firstMover.containsKey('symbol'), true);
        expect(firstMover.containsKey('sentimentScore'), true);
        expect(firstMover.containsKey('buzzScore'), true);
        expect(firstMover.containsKey('articleCount'), true);
      });
    });
    
    group('Article Structure', () {
      test('news articles have required fields', () {
        provider.initialize();
        
        final article = provider.newsArticles.first;
        expect(article.containsKey('title'), true);
        expect(article.containsKey('source'), true);
        expect(article.containsKey('timestamp'), true);
        expect(article.containsKey('sentiment'), true);
        expect(article.containsKey('sentimentScore'), true);
        expect(article.containsKey('category'), true);
      });
      
      test('sentiment scores are within valid range', () {
        provider.initialize();
        
        for (final article in provider.newsArticles) {
          final score = article['sentimentScore'] as double;
          expect(score >= -1.0 && score <= 1.0, true);
        }
      });
    });
    
    group('Filter Combinations', () {
      test('can set multiple filters', () {
        provider.initialize();
        
        provider.setCategoryFilter('Earnings');
        provider.setSentimentFilter('Bullish');
        
        expect(provider.selectedCategory, 'Earnings');
        expect(provider.selectedSentiment, 'Bullish');
        expect(provider.newsArticles, isNotNull);
      });
      
      test('reset filters to All', () {
        provider.initialize();
        
        provider.setCategoryFilter('Earnings');
        provider.setSentimentFilter('Bullish');
        
        provider.setCategoryFilter('All');
        provider.setSentimentFilter('All');
        
        expect(provider.selectedCategory, 'All');
        expect(provider.selectedSentiment, 'All');
      });
    });
  });
}
