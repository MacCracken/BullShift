import 'dart:math';
import '../../services/base_provider.dart';
import '../../services/safe_cast.dart';

class BullRunnrProvider extends BaseProvider {
  List<Map<String, dynamic>> _newsArticles = [];
  Map<String, dynamic> _marketSentiment = {};
  List<Map<String, dynamic>> _topSentimentMovers = [];
  List<Map<String, dynamic>> _sectorSentiment = [];
  String _selectedCategory = 'All';
  String _selectedSentiment = 'All';

  // Getters
  List<Map<String, dynamic>> get newsArticles => _newsArticles;
  Map<String, dynamic> get marketSentiment => _marketSentiment;
  List<Map<String, dynamic>> get topSentimentMovers => _topSentimentMovers;
  List<Map<String, dynamic>> get sectorSentiment => _sectorSentiment;
  String get selectedCategory => _selectedCategory;
  String get selectedSentiment => _selectedSentiment;

  // Setters
  void setCategoryFilter(String category) {
    _selectedCategory = category;
    _filterArticles();
    safeNotifyListeners();
  }

  void setSentimentFilter(String sentiment) {
    _selectedSentiment = sentiment;
    _filterArticles();
    safeNotifyListeners();
  }

  // Initialize with sample data
  void initialize() {
    _generateSampleData();
    safeNotifyListeners();
  }

  // Refresh news data
  Future<void> refreshNews() async {
    await executeAsync(
      operation: () async {
        // Simulate API call delay
        await Future.delayed(const Duration(seconds: 2));

        // Generate fresh sample data
        _generateSampleData();
      },
    );
  }

  // Search news
  Future<void> searchNews(String keywords, List<String> symbols) async {
    await executeAsync(
      operation: () async {
        // Simulate search API call
        await Future.delayed(const Duration(seconds: 1));

        // Generate search results
        _generateSearchResults(keywords, symbols);
      },
    );
  }

  // Generate sample data for demonstration
  void _generateSampleData() {
    final random = Random();
    final sources = [
      'Reuters',
      'Bloomberg',
      'CNBC',
      'MarketWatch',
      'Yahoo Finance',
      'Seeking Alpha',
      'The Wall Street Journal'
    ];
    final categories = [
      'Earnings',
      'M&A',
      'Regulatory',
      'Market Analysis',
      'Economic Data',
      'Company News',
      'Sector News',
      'Breaking News'
    ];
    final sentiments = [
      'VeryBullish',
      'Bullish',
      'Neutral',
      'Bearish',
      'VeryBearish'
    ];
    final symbols = [
      'AAPL',
      'GOOGL',
      'MSFT',
      'AMZN',
      'TSLA',
      'META',
      'NVDA',
      'AMD',
      'NFLX',
      'DIS',
      'BA',
      'JPM',
      'V',
      'WMT',
      'PG',
      'JNJ',
      'UNH',
      'HD',
      'MA',
      'PYPL',
      'INTC',
      'CSCO',
      'CMCSA',
      'PEP',
      'COST'
    ];

    // Generate news articles
    _newsArticles = List.generate(50, (index) {
      final symbol = symbols[random.nextInt(symbols.length)];
      final category = categories[random.nextInt(categories.length)];
      final sentiment = sentiments[random.nextInt(sentiments.length)];
      final source = sources[random.nextInt(sources.length)];

      return {
        'id': 'news_$index',
        'title': _generateHeadline(symbol, category, sentiment),
        'source': source,
        'timestamp':
            DateTime.now().subtract(Duration(minutes: random.nextInt(180))),
        'sentiment': sentiment,
        'sentimentScore': _getSentimentScore(sentiment),
        'confidence': 0.6 + random.nextDouble() * 0.4,
        'category': category,
        'symbols': [
          symbol,
          if (random.nextBool()) symbols[random.nextInt(symbols.length)]
        ],
        'aspects': _generateAspects(sentiment),
        'url': 'https://example.com/news/$index',
      };
    });

    // Sort by timestamp (newest first)
    _newsArticles.sort((a, b) =>
        (b['timestamp'] as DateTime).compareTo(a['timestamp'] as DateTime));

    // Generate market sentiment
    _generateMarketSentiment();

    // Generate top sentiment movers
    _generateTopSentimentMovers();

    // Generate sector sentiment
    _generateSectorSentiment();

    // Apply initial filters
    _filterArticles();
  }

  String _generateHeadline(String symbol, String category, String sentiment) {
    final headlines = {
      'Earnings': {
        'VeryBullish': '$symbol Beats Earnings Expectations, Raises Guidance',
        'Bullish': '$symbol Reports Strong Quarterly Earnings',
        'Neutral': '$symbol Meets Earnings Expectations',
        'Bearish': '$symbol Misses Earnings Expectations',
        'VeryBearish': '$symbol Disappoints with Weak Earnings Report',
      },
      'M&A': {
        'VeryBullish': '$symbol Announces Major Acquisition, Stock Soars',
        'Bullish': '$symbol in Advanced Merger Talks',
        'Neutral': '$symbol Confirms Acquisition Discussions',
        'Bearish': '$symbol Acquisition Deal Faces Regulatory Hurdles',
        'VeryBearish': '$symbol Abandons Merger Plans Amid Concerns',
      },
      'Regulatory': {
        'VeryBullish': '$symbol Receives Regulatory Approval for New Product',
        'Bullish': '$symbol Passes Regulatory Review with Flying Colors',
        'Neutral': '$symbol Awaits Regulatory Decision',
        'Bearish': '$symbol Faces Regulatory Scrutiny',
        'VeryBearish':
            '$symbol Hit with Regulatory Fines, Investigation Launched',
      },
      'Market Analysis': {
        'VeryBullish':
            'Analysts Upgrade $symbol to Strong Buy, Price Target Raised',
        'Bullish': 'Bullish Case for $symbol Gains Momentum',
        'Neutral': '$symbol Trading in Range, Analysts Cautious',
        'Bearish': 'Analysts Cut $symbol Price Targets on Concerns',
        'VeryBearish': '$symbol Downgraded as Headwinds Mount',
      },
      'Company News': {
        'VeryBullish': '$symbol Launches Breakthrough Product, Market Excited',
        'Bullish': '$symbol Announces Strong Business Update',
        'Neutral': '$symbol Provides Operational Update',
        'Bearish': '$symbol Faces Business Challenges',
        'VeryBearish': '$symbol Announces Restructuring, Cuts Forecast',
      },
      'Sector News': {
        'VeryBullish': 'Tech Sector Rally Boosts $symbol and Peers',
        'Bullish': 'Sector Trends Favor $symbol Growth Prospects',
        'Neutral': '$symbol Navigates Sector Challenges',
        'Bearish': 'Sector Headwinds Impact $symbol Performance',
        'VeryBearish': 'Sector Downturn Drags $symbol Lower',
      },
      'Breaking News': {
        'VeryBullish': 'BREAKING: $symbol Announces Major Partnership',
        'Bullish': 'Breaking: $symbol Reports Positive Development',
        'Neutral': 'Breaking: $symbol Provides Market Update',
        'Bearish': 'Breaking: $symbol Faces Unexpected Challenge',
        'VeryBearish': 'BREAKING: $symbol Stock Plunges on Negative News',
      },
    };

    final categoryHeadlines = headlines[category] ?? {};
    final sentimentHeadlines =
        categoryHeadlines[sentiment] ?? '$symbol Market Update';

    return sentimentHeadlines;
  }

  double _getSentimentScore(String sentiment) {
    switch (sentiment) {
      case 'VeryBullish':
        return 0.8 + Random().nextDouble() * 0.2;
      case 'Bullish':
        return 0.3 + Random().nextDouble() * 0.5;
      case 'Neutral':
        return -0.2 + Random().nextDouble() * 0.4;
      case 'Bearish':
        return -0.8 + Random().nextDouble() * 0.5;
      case 'VeryBearish':
        return -1.0 + Random().nextDouble() * 0.2;
      default:
        return 0.0;
    }
  }

  Map<String, dynamic> _generateAspects(String overallSentiment) {
    final random = Random();
    final aspects = <String, Map<String, dynamic>>{};

    final aspectTypes = [
      'Revenue',
      'Earnings',
      'Growth',
      'Risk',
      'Innovation',
      'Market Position'
    ];

    for (final aspect in aspectTypes) {
      final aspectSentiment = _getRandomAspectSentiment(overallSentiment);
      aspects[aspect] = {
        'sentiment': aspectSentiment,
        'score': _getSentimentScore(aspectSentiment),
        'confidence': 0.5 + random.nextDouble() * 0.5,
      };
    }

    return aspects;
  }

  String _getRandomAspectSentiment(String overallSentiment) {
    final random = Random();
    final sentiments = [
      'VeryBullish',
      'Bullish',
      'Neutral',
      'Bearish',
      'VeryBearish'
    ];

    // Bias towards overall sentiment but allow variation
    if (overallSentiment.contains('Bullish') && random.nextDouble() > 0.3) {
      return ['VeryBullish', 'Bullish'][random.nextInt(2)];
    } else if (overallSentiment.contains('Bearish') &&
        random.nextDouble() > 0.3) {
      return ['VeryBearish', 'Bearish'][random.nextInt(2)];
    }

    return sentiments[random.nextInt(sentiments.length)];
  }

  void _generateMarketSentiment() {
    final random = Random();
    final bullishCount = 15 + random.nextInt(20);
    final bearishCount = 10 + random.nextInt(15);
    final neutralCount = 5 + random.nextInt(10);
    final totalArticles = bullishCount + bearishCount + neutralCount;

    final overallScore =
        ((bullishCount - bearishCount) / totalArticles).clamp(-1.0, 1.0);
    final fearGreedIndex = ((overallScore + 1.0) * 50.0).clamp(0.0, 100.0);

    _marketSentiment = {
      'overallScore': overallScore,
      'bullishCount': bullishCount,
      'bearishCount': bearishCount,
      'neutralCount': neutralCount,
      'totalArticles': totalArticles,
      'fearGreedIndex': fearGreedIndex,
    };
  }

  void _generateTopSentimentMovers() {
    final random = Random();
    final symbols = [
      'AAPL',
      'TSLA',
      'NVDA',
      'AMD',
      'META',
      'GOOGL',
      'MSFT',
      'AMZN'
    ];

    _topSentimentMovers = symbols.map((symbol) {
      final sentimentScore = -0.8 + random.nextDouble() * 1.6;
      final buzzScore = random.nextDouble();
      final articleCount = 1 + random.nextInt(15);

      return {
        'symbol': symbol,
        'sentimentScore': sentimentScore,
        'buzzScore': buzzScore,
        'articleCount': articleCount,
        'sentimentTrend': _getSentimentTrend(sentimentScore),
      };
    }).toList();

    // Sort by buzz score
    _topSentimentMovers.sort((a, b) =>
        b.safeDouble('buzzScore').compareTo(a.safeDouble('buzzScore')));
  }

  String _getSentimentTrend(double score) {
    if (score > 0.5) return 'Improving';
    if (score < -0.5) return 'Declining';
    return 'Stable';
  }

  void _generateSectorSentiment() {
    final random = Random();
    final sectors = [
      'Technology',
      'Healthcare',
      'Finance',
      'Energy',
      'Consumer',
      'Industrial',
      'Real Estate',
      'Materials',
      'Utilities',
      'Communication'
    ];

    _sectorSentiment = sectors.map((sector) {
      final sentiment = -0.8 + random.nextDouble() * 1.6;

      return {
        'name': sector,
        'sentiment': sentiment,
        'articleCount': 5 + random.nextInt(25),
        'topMover': _getRandomSymbol(),
      };
    }).toList();

    // Sort by sentiment
    _sectorSentiment.sort((a, b) =>
        b.safeDouble('sentiment').compareTo(a.safeDouble('sentiment')));
  }

  String _getRandomSymbol() {
    final symbols = [
      'AAPL',
      'GOOGL',
      'MSFT',
      'AMZN',
      'TSLA',
      'META',
      'NVDA',
      'AMD'
    ];
    return symbols[Random().nextInt(symbols.length)];
  }

  void _generateSearchResults(String keywords, List<String> symbols) {
    final random = Random();
    final sources = ['Reuters', 'Bloomberg', 'CNBC', 'MarketWatch'];
    final sentiments = [
      'VeryBullish',
      'Bullish',
      'Neutral',
      'Bearish',
      'VeryBearish'
    ];

    // Generate search results based on keywords and symbols
    _newsArticles = List.generate(20, (index) {
      final symbol = symbols.isNotEmpty
          ? symbols[random.nextInt(symbols.length)]
          : 'MARKET';
      final sentiment = sentiments[random.nextInt(sentiments.length)];
      final source = sources[random.nextInt(sources.length)];

      return {
        'id': 'search_$index',
        'title': '$keywords: $symbol Market Analysis - $source Report',
        'source': source,
        'timestamp':
            DateTime.now().subtract(Duration(minutes: random.nextInt(60))),
        'sentiment': sentiment,
        'sentimentScore': _getSentimentScore(sentiment),
        'confidence': 0.6 + random.nextDouble() * 0.4,
        'category': 'Search Results',
        'symbols': symbols.isNotEmpty ? [symbol] : [],
        'aspects': _generateAspects(sentiment),
        'url': 'https://example.com/search/$index',
      };
    });

    // Sort by relevance
    _newsArticles.sort((a, b) =>
        b.safeDouble('confidence').compareTo(a.safeDouble('confidence')));

    safeNotifyListeners();
  }

  void _filterArticles() {
    // This would filter the articles based on selected category and sentiment
    // For now, we'll just notify listeners to trigger a rebuild
    safeNotifyListeners();
  }

  // Get news for specific symbol
  List<Map<String, dynamic>> getNewsForSymbol(String symbol) {
    return _newsArticles.where((article) {
      final symbols = article.safeList<String>('symbols');
      return symbols.contains(symbol.toUpperCase());
    }).toList();
  }

  // Get sentiment for specific symbol
  double getSymbolSentiment(String symbol) {
    final symbolArticles = getNewsForSymbol(symbol);
    if (symbolArticles.isEmpty) return 0.0;

    final totalScore = symbolArticles
        .map((article) => article.safeDouble('sentimentScore'))
        .reduce((a, b) => a + b);

    return totalScore / symbolArticles.length;
  }

  // Get market overview statistics
  Map<String, dynamic> getMarketOverview() {
    return {
      'totalArticles': _newsArticles.length,
      'marketSentiment': _marketSentiment,
      'topMovers': _topSentimentMovers.take(5).toList(),
      'sectorSentiment': _sectorSentiment,
    };
  }
}
