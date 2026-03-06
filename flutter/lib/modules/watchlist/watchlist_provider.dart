import 'dart:math';
import 'dart:async';
import 'package:flutter/foundation.dart';
import '../../services/base_provider.dart';
import '../../services/rust_trading_engine.dart';

class WatchlistProvider extends BaseProvider {
  final RustTradingEngine _rustEngine;
  List<Map<String, dynamic>> _watchlist = [];
  List<Map<String, dynamic>> _searchResults = [];
  String _searchQuery = '';
  String _sortBy = 'symbol'; // symbol, price, change, volume
  bool _sortAscending = true;
  Timer? _priceUpdateTimer;
  bool _realTimeUpdatesEnabled = true;

  WatchlistProvider(this._rustEngine);

  // Getters
  List<Map<String, dynamic>> get watchlist => _watchlist;
  List<Map<String, dynamic>> get searchResults => _searchResults;
  String get searchQuery => _searchQuery;
  String get sortBy => _sortBy;
  bool get sortAscending => _sortAscending;
  bool get realTimeUpdatesEnabled => _realTimeUpdatesEnabled;

  // Setters
  void setSearchQuery(String query) {
    _searchQuery = query;
    safeNotifyListeners();
  }

  void setSortBy(String sortBy) {
    if (_sortBy == sortBy) {
      _sortAscending = !_sortAscending;
    } else {
      _sortBy = sortBy;
      _sortAscending = true;
    }
    _sortWatchlist();
    safeNotifyListeners();
  }

  void setRealTimeUpdatesEnabled(bool enabled) {
    _realTimeUpdatesEnabled = enabled;
    if (enabled) {
      _startRealTimeUpdates();
    } else {
      _stopRealTimeUpdates();
    }
    safeNotifyListeners();
  }

  // Initialize watchlist with sample data
  void initialize() {
    _generateSampleWatchlist();
    _startRealTimeUpdates();
    safeNotifyListeners();
  }

  // Add symbol to watchlist
  Future<bool> addToWatchlist(String symbol) async {
    if (_watchlist.any((item) => item['symbol'] == symbol.toUpperCase())) {
      return false; // Already exists
    }

    return await executeAsync(
      operation: () async {
        final stockData = await _fetchSymbolData(symbol.toUpperCase());
        if (stockData != null) {
          _watchlist.add(stockData);
          _sortWatchlist();
          return true;
        }
        return false;
      },
      showLoading: false,
    ) ?? false;
  }

  // Remove symbol from watchlist
  Future<void> removeFromWatchlist(String symbol) async {
    _watchlist.removeWhere((item) => item['symbol'] == symbol.toUpperCase());
    safeNotifyListeners();
  }

  // Search for symbols
  Future<void> searchSymbols(String query) async {
    _searchQuery = query;
    await executeAsync(
      operation: () async {
        await Future.delayed(const Duration(milliseconds: 300)); // Debounce

        if (query.isEmpty) {
          _searchResults = [];
        } else {
          _searchResults = await _searchSymbolsAPI(query);
        }
      },
      showLoading: true,
    );
  }

  // Refresh all watchlist prices
  Future<void> refreshWatchlistPrices() async {
    await executeAsync(
      operation: () async {
        await Future.delayed(const Duration(seconds: 1)); // Simulate API call

        for (int i = 0; i < _watchlist.length; i++) {
          final symbol = _watchlist[i]['symbol'] as String;
          final updatedData = await _fetchSymbolData(symbol);
          if (updatedData != null) {
            // Preserve the previous price for change calculation
            final previousPrice = _watchlist[i]['currentPrice'] as double;
            updatedData['previousPrice'] = previousPrice;
            _watchlist[i] = updatedData;
          }
        }

        _sortWatchlist();
      },
    );
  }

  // Get watchlist statistics
  Map<String, dynamic> getWatchlistStats() {
    if (_watchlist.isEmpty) {
      return {
        'totalSymbols': 0,
        'totalValue': 0.0,
        'dayChange': 0.0,
        'dayChangePercent': 0.0,
        'topGainer': null,
        'topLoser': null,
      };
    }

    double totalValue = 0.0;
    double totalChange = 0.0;
    Map<String, dynamic>? topGainer;
    Map<String, dynamic>? topLoser;

    for (final item in _watchlist) {
      final price = item['currentPrice'] as double;
      final change = item['dayChange'] as double;
      totalValue += price;
      totalChange += change;

      if (topGainer == null || change > (topGainer['dayChange'] as double)) {
        topGainer = item;
      }

      if (topLoser == null || change < (topLoser['dayChange'] as double)) {
        topLoser = item;
      }
    }

    final totalChangePercent = totalValue > 0 ? (totalChange / totalValue) * 100 : 0.0;

    return {
      'totalSymbols': _watchlist.length,
      'totalValue': totalValue,
      'dayChange': totalChange,
      'dayChangePercent': totalChangePercent,
      'topGainer': topGainer,
      'topLoser': topLoser,
    };
  }

  // Check if symbol is in watchlist
  bool isInWatchlist(String symbol) {
    return _watchlist.any((item) => item['symbol'] == symbol.toUpperCase());
  }

  // Get symbol data from watchlist
  Map<String, dynamic>? getSymbolData(String symbol) {
    try {
      return _watchlist.firstWhere((item) => item['symbol'] == symbol.toUpperCase());
    } catch (e) {
      return null;
    }
  }

  // Private methods

  void _generateSampleWatchlist() {
    final random = Random();
    final sampleSymbols = ['AAPL', 'GOOGL', 'MSFT', 'TSLA', 'NVDA'];

    _watchlist = sampleSymbols.map((symbol) {
      final price = 50.0 + random.nextDouble() * 450.0;
      final dayChange = (-20.0 + random.nextDouble() * 40.0);
      final volume = 1000000 + random.nextInt(50000000);
      
      return {
        'symbol': symbol,
        'currentPrice': price,
        'dayChange': dayChange,
        'dayChangePercent': (dayChange / price) * 100,
        'volume': volume,
        'marketCap': price * (1000000000 + random.nextInt(9000000000)),
        'timestamp': DateTime.now(),
        'previousPrice': price - dayChange,
      };
    }).toList();

    _sortWatchlist();
  }

  Future<Map<String, dynamic>?> _fetchSymbolData(String symbol) async {
    try {
      // Simulate API call to fetch symbol data
      await Future.delayed(const Duration(milliseconds: 100));
      
      final random = Random();
      final price = 50.0 + random.nextDouble() * 450.0;
      final dayChange = (-20.0 + random.nextDouble() * 40.0);
      final volume = 1000000 + random.nextInt(50000000);

      return {
        'symbol': symbol,
        'currentPrice': price,
        'dayChange': dayChange,
        'dayChangePercent': (dayChange / price) * 100,
        'volume': volume,
        'marketCap': price * (1000000000 + random.nextInt(9000000000)),
        'timestamp': DateTime.now(),
        'previousPrice': price - dayChange,
      };
    } catch (e) {
      debugPrint('Error fetching symbol data for $symbol: $e');
      return null;
    }
  }

  Future<List<Map<String, dynamic>>> _searchSymbolsAPI(String query) async {
    // Simulate symbol search API
    await Future.delayed(const Duration(milliseconds: 200));
    
    final allSymbols = [
      'AAPL', 'GOOGL', 'MSFT', 'AMZN', 'TSLA', 'META', 'NVDA', 'AMD',
      'NFLX', 'DIS', 'BA', 'JPM', 'V', 'WMT', 'PG', 'JNJ', 'UNH',
      'HD', 'MA', 'PYPL', 'INTC', 'CSCO', 'CMCSA', 'PEP', 'COST',
      'ADBE', 'CRM', 'XOM', 'CVX', 'LLY', 'ABBV', 'DHR', 'MDT',
      'ABT', 'T', 'VZ', 'KO', 'NKE', 'MRK', 'HON', 'TXN', 'NEE',
      'AMGN', 'UPS', 'LIN', 'PLD', 'EL', 'LMT', 'CAT', 'GS', 'RTX',
      'SPY', 'QQQ', 'IWM', 'DIA', 'VTI', 'VOO', 'GLD', 'SLV', 'BTC',
      'ETH', 'DOGE', 'ADA', 'DOT', 'LINK', 'UNI', 'AAVE', 'COMP'
    ];

    final random = Random();
    final filtered = allSymbols
        .where((symbol) => symbol.toLowerCase().contains(query.toLowerCase()))
        .take(10)
        .toList();

    return filtered.map((symbol) {
      final price = 50.0 + random.nextDouble() * 450.0;
      return {
        'symbol': symbol,
        'price': price,
        'name': '${symbol} Inc.',
        'exchange': random.nextBool() ? 'NASDAQ' : 'NYSE',
        'type': symbol.length == 3 ? 'Stock' : (symbol.contains('BTC') || symbol.contains('ETH') ? 'Crypto' : 'ETF'),
      };
    }).toList();
  }

  void _sortWatchlist() {
    switch (_sortBy) {
      case 'symbol':
        _watchlist.sort((a, b) => _sortAscending 
            ? (a['symbol'] as String).compareTo(b['symbol'] as String)
            : (b['symbol'] as String).compareTo(a['symbol'] as String));
        break;
      case 'price':
        _watchlist.sort((a, b) => _sortAscending 
            ? (a['currentPrice'] as double).compareTo(b['currentPrice'] as double)
            : (b['currentPrice'] as double).compareTo(a['currentPrice'] as double));
        break;
      case 'change':
        _watchlist.sort((a, b) => _sortAscending 
            ? (a['dayChange'] as double).compareTo(b['dayChange'] as double)
            : (b['dayChange'] as double).compareTo(a['dayChange'] as double));
        break;
      case 'volume':
        _watchlist.sort((a, b) => _sortAscending 
            ? (a['volume'] as int).compareTo(b['volume'] as int)
            : (b['volume'] as int).compareTo(a['volume'] as int));
        break;
    }
  }

  // Clear search results
  void clearSearchResults() {
    _searchResults = [];
    _searchQuery = '';
    safeNotifyListeners();
  }

  // Add multiple symbols at once
  Future<int> addMultipleToWatchlist(List<String> symbols) async {
    int added = 0;
    
    for (final symbol in symbols) {
      if (await addToWatchlist(symbol)) {
        added++;
      }
    }
    
    return added;
  }

  // Export watchlist to JSON
  Map<String, dynamic> exportWatchlist() {
    return {
      'watchlist': _watchlist,
      'exportDate': DateTime.now().toIso8601String(),
      'version': '1.0',
    };
  }

  // Import watchlist from JSON
  Future<bool> importWatchlist(Map<String, dynamic> data) async {
    try {
      if (data['watchlist'] is List) {
        final importedWatchlist = data['watchlist'] as List;
        int added = 0;
        
        for (final item in importedWatchlist) {
          if (item is Map && item['symbol'] is String) {
            if (await addToWatchlist(item['symbol'] as String)) {
              added++;
            }
          }
        }
        
        debugPrint('Imported $added symbols to watchlist');
        return true;
      }
    } catch (e) {
      debugPrint('Error importing watchlist: $e');
    }
    return false;
  }

  // Real-time updates management
  void _startRealTimeUpdates() {
    if (_priceUpdateTimer != null) return;
    
    _priceUpdateTimer = Timer.periodic(const Duration(seconds: 5), (timer) {
      if (_realTimeUpdatesEnabled && _watchlist.isNotEmpty) {
        _updateWatchlistPrices();
      }
    });
  }

  void _stopRealTimeUpdates() {
    _priceUpdateTimer?.cancel();
    _priceUpdateTimer = null;
  }

  Future<void> _updateWatchlistPrices() async {
    try {
      // Simulate real-time price updates from WebSocket
      // In a real implementation, this would receive data from the Rust WebSocket stream
      for (int i = 0; i < _watchlist.length; i++) {
        final item = _watchlist[i];
        final symbol = item['symbol'] as String;
        
        // Simulate price update
        final random = Random();
        final priceChange = (-2.0 + random.nextDouble() * 4.0); // -2% to +2%
        final currentPrice = item['currentPrice'] as double;
        final newPrice = currentPrice * (1 + priceChange / 100);
        final dayChange = item['dayChange'] as double + (newPrice - currentPrice);
        final previousPrice = item['previousPrice'] as double;
        final dayChangePercent = ((newPrice - previousPrice) / previousPrice) * 100;
        
        // Update the item
        _watchlist[i] = {
          ...item,
          'currentPrice': newPrice,
          'dayChange': dayChange,
          'dayChangePercent': dayChangePercent,
          'previousPrice': currentPrice,
          'timestamp': DateTime.now(),
          'volume': (item['volume'] as int) + random.nextInt(10000),
        };
      }
      
      _sortWatchlist();
      safeNotifyListeners();
    } catch (e) {
      debugPrint('Error updating watchlist prices: $e');
    }
  }

  // Connect to WebSocket for real-time data
  Future<void> connectToRealTimeData() async {
    try {
      // This would connect to the Rust WebSocket stream
      // For now, we'll simulate with timer-based updates
      final symbols = _watchlist.map((item) => item['symbol'] as String).toList();
      
      // In real implementation:
      // await _rustEngine.connectToMarketData(symbols);
      debugPrint('Connecting to real-time data for symbols: $symbols');
      
    } catch (e) {
      debugPrint('Error connecting to real-time data: $e');
    }
  }

  // Handle incoming WebSocket data
  void handleMarketDataUpdate(Map<String, dynamic> data) {
    try {
      final symbol = data['symbol'] as String?;
      if (symbol == null) return;
      
      // Find the item in watchlist
      final index = _watchlist.indexWhere((item) => item['symbol'] == symbol);
      if (index == -1) return;
      
      final item = _watchlist[index];
      final currentPrice = item['currentPrice'] as double;
      final newPrice = (data['price'] as num?)?.toDouble() ?? currentPrice;
      final volume = (data['volume'] as num?)?.toInt() ?? item['volume'] as int;
      final previousPrice = item['previousPrice'] as double;
      final dayChange = newPrice - previousPrice;
      final dayChangePercent = (dayChange / previousPrice) * 100;
      
      _watchlist[index] = {
        ...item,
        'currentPrice': newPrice,
        'dayChange': dayChange,
        'dayChangePercent': dayChangePercent,
        'volume': volume,
        'timestamp': DateTime.now(),
      };
      
      _sortWatchlist();
      safeNotifyListeners();
      
    } catch (e) {
      debugPrint('Error handling market data update: $e');
    }
  }

  @override
  void dispose() {
    _stopRealTimeUpdates();
    super.dispose();
  }
}