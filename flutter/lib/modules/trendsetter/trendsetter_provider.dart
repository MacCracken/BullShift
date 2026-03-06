import 'dart:math';
import '../../services/base_provider.dart';
import '../../services/safe_cast.dart';

class TrendSetterProvider extends BaseProvider {
  List<Map<String, dynamic>> _momentumStocks = [];
  List<Map<String, dynamic>> _heatMapData = [];
  List<Map<String, dynamic>> _activeAlerts = [];
  double _minScoreFilter = 0.6;
  String _trendStrengthFilter = 'All';

  // Getters
  List<Map<String, dynamic>> get momentumStocks => _momentumStocks;
  List<Map<String, dynamic>> get heatMapData => _heatMapData;
  List<Map<String, dynamic>> get activeAlerts => _activeAlerts;
  double get minScoreFilter => _minScoreFilter;
  String get trendStrengthFilter => _trendStrengthFilter;

  // Setters
  void setMinScoreFilter(double value) {
    _minScoreFilter = value;
    _filterStocks();
    safeNotifyListeners();
  }

  void setTrendStrengthFilter(String value) {
    _trendStrengthFilter = value;
    _filterStocks();
    safeNotifyListeners();
  }

  // Initialize with sample data
  void initialize() {
    _generateSampleData();
    safeNotifyListeners();
  }

  // Refresh momentum data
  Future<void> refreshMomentumData() async {
    await executeAsync(
      operation: () async {
        // Simulate API call delay
        await Future.delayed(const Duration(seconds: 1));
        
        // Generate fresh sample data
        _generateSampleData();
        
        // Clear expired alerts
        _clearExpiredAlerts();
      },
    );
  }

  // Generate sample data for demonstration
  void _generateSampleData() {
    final random = Random();
    final symbols = [
      'AAPL', 'GOOGL', 'MSFT', 'AMZN', 'TSLA', 'META', 'NVDA', 'AMD',
      'NFLX', 'DIS', 'BA', 'JPM', 'V', 'WMT', 'PG', 'JNJ', 'UNH',
      'HD', 'MA', 'PYPL', 'INTC', 'CSCO', 'CMCSA', 'PEP', 'COST',
      'ADBE', 'CRM', 'XOM', 'CVX', 'LLY', 'ABBV', 'DHR', 'MDT',
      'ABT', 'T', 'VZ', 'KO', 'NKE', 'MRK', 'HON', 'TXN', 'NEE',
      'AMGN', 'UPS', 'LIN', 'PLD', 'EL', 'LMT', 'CAT', 'GS', 'RTX',
      'BLK', 'GE', 'MMM', 'DE', 'CI', 'TMO', 'SNY', 'AZN', 'QCOM',
      'TM', 'HMC', 'BABA', 'PDD', 'BIDU', 'NIO', 'XPEV', 'LI', 'RIVN',
      'LCID', 'RBLX', 'SNAP', 'TWTR', 'SPOT', 'ROKU', 'ZM', 'DOCU',
      'SQ', 'SHOP', 'MELI', 'SE', 'BNGO', 'GME', 'AMC', 'BB', 'NOK'
    ];

    // Generate momentum stocks
    _momentumStocks = symbols.map((symbol) {
      final score = random.nextDouble();
      final volumeSpike = 0.5 + random.nextDouble() * 4.5; // 0.5x to 5x
      final priceMomentum = -1.0 + random.nextDouble() * 2.0; // -1 to 1
      final socialSentiment = -1.0 + random.nextDouble() * 2.0; // -1 to 1
      
      final trendStrength = _getTrendStrength(score);
      
      return {
        'symbol': symbol,
        'score': score,
        'volumeSpike': volumeSpike,
        'priceMomentum': priceMomentum,
        'socialSentiment': socialSentiment,
        'trendStrength': trendStrength,
        'timestamp': DateTime.now().subtract(Duration(minutes: random.nextInt(60))),
      };
    }).toList();

    // Sort by score (highest first)
    _momentumStocks.sort((a, b) => b.safeDouble('score').compareTo(a.safeDouble('score')));

    // Generate heat map data (subset of stocks)
    _heatMapData = _momentumStocks.take(15).map((stock) {
      return {
        'symbol': stock['symbol'],
        'heat': stock['score'],
      };
    }).toList();

    // Generate alerts
    _generateAlerts();

    // Apply initial filters
    _filterStocks();
  }

  String _getTrendStrength(double score) {
    if (score >= 0.8) return 'Explosive';
    if (score >= 0.6) return 'Strong';
    if (score >= 0.4) return 'Moderate';
    return 'Weak';
  }

  void _generateAlerts() {
    final random = Random();
    _activeAlerts = [];

    for (final stock in _momentumStocks.take(20)) {
      final symbol = stock.safeString('symbol');
      final score = stock.safeDouble('score');
      final volumeSpike = stock.safeDouble('volumeSpike');
      final socialSentiment = stock.safeDouble('socialSentiment');

      // Volume spike alerts
      if (volumeSpike > 3.0 && random.nextBool()) {
        _activeAlerts.add({
          'symbol': symbol,
          'type': 'VolumeSpike',
          'message': 'Unusual volume spike detected: ${volumeSpike.toStringAsFixed(1)}x average',
          'confidence': 0.8 + random.nextDouble() * 0.2,
          'timestamp': DateTime.now().subtract(Duration(minutes: random.nextInt(30))),
        });
      }

      // Momentum shift alerts
      if (score > 0.7 && random.nextBool()) {
        _activeAlerts.add({
          'symbol': symbol,
          'type': 'MomentumShift',
          'message': 'Strong momentum detected: ${(score * 100).toInt()}%',
          'confidence': score,
          'timestamp': DateTime.now().subtract(Duration(minutes: random.nextInt(45))),
        });
      }

      // Social buzz alerts
      if (socialSentiment > 0.6 && random.nextBool()) {
        _activeAlerts.add({
          'symbol': symbol,
          'type': 'SocialBuzz',
          'message': 'High social sentiment: ${(socialSentiment * 100).toInt()}%',
          'confidence': 0.7 + random.nextDouble() * 0.3,
          'timestamp': DateTime.now().subtract(Duration(minutes: random.nextInt(60))),
        });
      }

      // Price breakout alerts
      if (stock.safeDouble('priceMomentum') > 0.5 && random.nextBool()) {
        _activeAlerts.add({
          'symbol': symbol,
          'type': 'PriceBreakout',
          'message': 'Price breakout detected',
          'confidence': 0.6 + random.nextDouble() * 0.4,
          'timestamp': DateTime.now().subtract(Duration(minutes: random.nextInt(20))),
        });
      }
    }

    // Sort alerts by timestamp (newest first)
    _activeAlerts.sort((a, b) => (b['timestamp'] as DateTime).compareTo(a['timestamp'] as DateTime));

    // Limit to 20 most recent alerts
    _activeAlerts = _activeAlerts.take(20).toList();
  }

  void _filterStocks() {
    List<Map<String, dynamic>> filtered = List.from(_momentumStocks);

    // Filter by minimum score
    filtered = filtered.where((stock) {
      return stock.safeDouble('score') >= _minScoreFilter;
    }).toList();

    // Filter by trend strength
    if (_trendStrengthFilter != 'All') {
      filtered = filtered.where((stock) {
        return stock['trendStrength'] == _trendStrengthFilter;
      }).toList();
    }

    _momentumStocks = filtered;
  }

  void _clearExpiredAlerts() {
    final now = DateTime.now();
    _activeAlerts = _activeAlerts.where((alert) {
      final timestamp = alert['timestamp'] as DateTime;
      return now.difference(timestamp).inHours < 4; // Keep alerts less than 4 hours old
    }).toList();
  }

  // Get top momentum stocks
  List<Map<String, dynamic>> getTopMomentumStocks(int limit) {
    return _momentumStocks.take(limit).toList();
  }

  // Get alerts for specific symbol
  List<Map<String, dynamic>> getAlertsForSymbol(String symbol) {
    return _activeAlerts.where((alert) => alert['symbol'] == symbol).toList();
  }

  // Search stocks by symbol
  List<Map<String, dynamic>> searchStocks(String query) {
    if (query.isEmpty) return _momentumStocks;
    
    return _momentumStocks.where((stock) {
      final symbol = stock.safeString('symbol');
      return symbol.toLowerCase().contains(query.toLowerCase());
    }).toList();
  }

  // Get market overview statistics
  Map<String, dynamic> getMarketOverview() {
    if (_momentumStocks.isEmpty) {
      return {
        'totalStocks': 0,
        'avgScore': 0.0,
        'topStocks': 0,
        'activeAlerts': 0,
      };
    }

    final totalStocks = _momentumStocks.length;
    final avgScore = _momentumStocks
        .map((stock) => stock.safeDouble('score'))
        .reduce((a, b) => a + b) / totalStocks;
    final topStocks = _momentumStocks
        .where((stock) => stock.safeDouble('score') > 0.7)
        .length;

    return {
      'totalStocks': totalStocks,
      'avgScore': avgScore,
      'topStocks': topStocks,
      'activeAlerts': _activeAlerts.length,
    };
  }
}