import 'dart:math';
import '../../services/base_provider.dart';

class PaperHandsProvider extends BaseProvider {
  List<Map<String, dynamic>> _paperPortfolios = [];
  Map<String, dynamic>? _selectedPortfolio;
  List<Map<String, dynamic>> _recentTrades = [];
  List<Map<String, dynamic>> _backtestResults = [];

  // Trading state
  String _currentSymbol = '';
  String _selectedTimeframe = '1D';
  String _selectedOrderType = 'Market';
  String _selectedSide = 'Buy';
  double _currentQuantity = 0.0;
  double _currentPrice = 0.0;

  // Getters
  List<Map<String, dynamic>> get paperPortfolios => _paperPortfolios;
  Map<String, dynamic>? get selectedPortfolio => _selectedPortfolio;
  List<Map<String, dynamic>> get recentTrades => _recentTrades;
  List<Map<String, dynamic>> get backtestResults => _backtestResults;
  String get currentSymbol => _currentSymbol;
  String get selectedTimeframe => _selectedTimeframe;
  String get selectedOrderType => _selectedOrderType;
  String get selectedSide => _selectedSide;
  double get currentQuantity => _currentQuantity;
  double get currentPrice => _currentPrice;
  bool get canPlaceOrder =>
      _currentSymbol.isNotEmpty &&
      _currentQuantity > 0 &&
      _selectedPortfolio != null;

  // Setters
  void setSymbol(String symbol) {
    _currentSymbol = symbol.toUpperCase();
    safeNotifyListeners();
  }

  void setTimeframe(String timeframe) {
    _selectedTimeframe = timeframe;
    safeNotifyListeners();
  }

  void setOrderType(String orderType) {
    _selectedOrderType = orderType;
    safeNotifyListeners();
  }

  void setSide(String side) {
    _selectedSide = side;
    safeNotifyListeners();
  }

  void setQuantity(double quantity) {
    _currentQuantity = quantity;
    safeNotifyListeners();
  }

  void setPrice(double price) {
    _currentPrice = price;
    safeNotifyListeners();
  }

  // Initialize with sample data
  void initialize() {
    _generateSampleData();
    safeNotifyListeners();
  }

  // Portfolio Management
  Future<void> createPortfolio(String name, double initialBalance) async {
    await executeAsync(
      operation: () async {
        final portfolio = {
          'id': _generateId(),
          'name': name,
          'initialBalance': initialBalance,
          'currentBalance': initialBalance,
          'allocatedBalance': 0.0,
          'availableBalance': initialBalance,
          'positions': <Map<String, dynamic>>[],
          'trades': <Map<String, dynamic>>[],
          'totalReturn': 0.0,
          'totalReturnPercentage': 0.0,
          'winRate': 0.0,
          'sharpeRatio': 0.0,
          'maxDrawdown': 0.0,
          'totalTrades': 0,
          'winningTrades': 0,
          'losingTrades': 0,
          'isActive': false,
          'createdAt': DateTime.now(),
          'lastUpdated': DateTime.now(),
        };

        _paperPortfolios.add(portfolio);
        safeNotifyListeners();
      },
      loadingMessage: 'Creating portfolio...',
    );
  }

  void selectPortfolio(String portfolioId) {
    _selectedPortfolio =
        _paperPortfolios.firstWhere((p) => p['id'] == portfolioId);

    // Mark as active and deactivate others
    for (final portfolio in _paperPortfolios) {
      portfolio['isActive'] = portfolio['id'] == portfolioId;
    }

    // Load portfolio trades
    _loadPortfolioTrades();

    safeNotifyListeners();
  }

  void deletePortfolio(String portfolioId) {
    _paperPortfolios.removeWhere((p) => p['id'] == portfolioId);

    if (_selectedPortfolio?['id'] == portfolioId) {
      _selectedPortfolio = null;
      _recentTrades.clear();
    }

    safeNotifyListeners();
  }

  // Paper Trading
  Future<void> placePaperOrder() async {
    if (_selectedPortfolio == null || !canPlaceOrder) {
      setError('Cannot place order: missing required fields');
      return;
    }

    await executeAsync(
      operation: () async {
        final portfolio = _selectedPortfolio!;

        // Simulate order execution
        await Future.delayed(const Duration(milliseconds: 500));

        // Create trade record
        final trade = {
          'id': _generateId(),
          'portfolioId': portfolio['id'],
          'symbol': _currentSymbol,
          'side': _selectedSide,
          'orderType': _selectedOrderType,
          'quantity': _currentQuantity,
          'entryPrice': _currentPrice > 0
              ? _currentPrice
              : _getCurrentPrice(_currentSymbol),
          'exitPrice': null,
          'status': 'Open',
          'pnl': null,
          'pnlPercentage': null,
          'fees': _calculateFees(_currentQuantity, _currentPrice),
          'timestamp': DateTime.now(),
          'notes': null,
        };

        // Update portfolio
        _updatePortfolioAfterTrade(portfolio, trade);

        // Add to recent trades
        _recentTrades.insert(0, trade);
        if (_recentTrades.length > 50) {
          _recentTrades.removeLast();
        }

        safeNotifyListeners();
      },
      loadingMessage: 'Placing order...',
    );
  }

  Future<void> closePosition(String symbol, double exitPrice) async {
    if (_selectedPortfolio == null) return;

    await executeAsync(
      operation: () async {
        final portfolio = _selectedPortfolio!;

        // Find open position
        final positions = portfolio['positions'] as List;
        final positionIndex = positions.indexWhere(
          (p) => p['symbol'] == symbol && p['status'] == 'Open',
        );

        if (positionIndex == -1) {
          throw Exception('No open position found for $symbol');
        }
        final position = positions[positionIndex];

        // Create closing trade
        final closingTrade = {
          'id': _generateId(),
          'portfolioId': portfolio['id'],
          'symbol': symbol,
          'side': position['side'] == 'Buy' ? 'Sell' : 'Buy',
          'orderType': 'Market',
          'quantity': position['quantity'],
          'entryPrice': position['entryPrice'],
          'exitPrice': exitPrice,
          'status': 'Closed',
          'pnl': _calculatePnl(position, exitPrice),
          'pnlPercentage': _calculatePnlPercentage(position, exitPrice),
          'fees': _calculateFees(position['quantity'], exitPrice),
          'timestamp': DateTime.now(),
          'notes': null,
        };

        // Update portfolio
        _updatePortfolioAfterClose(portfolio, position, closingTrade);

        // Add to recent trades
        _recentTrades.insert(0, closingTrade);
        if (_recentTrades.length > 50) {
          _recentTrades.removeLast();
        }

        safeNotifyListeners();
      },
      loadingMessage: 'Closing position...',
    );
  }

  // Backtesting
  Future<void> runBacktest({
    required String strategyName,
    required String symbol,
    required String timeframe,
    required DateTime startDate,
    required DateTime endDate,
    required double initialBalance,
  }) async {
    await executeAsync(
      operation: () async {
        // Simulate backtest execution
        await Future.delayed(const Duration(seconds: 3));

        final backtestResult = {
          'id': _generateId(),
          'strategyName': strategyName,
          'symbol': symbol,
          'timeframe': timeframe,
          'startDate': startDate,
          'endDate': endDate,
          'initialBalance': initialBalance,
          'finalBalance': initialBalance *
              (0.9 + Random().nextDouble() * 0.3), // -10% to +20% return
          'totalReturn': 0.0,
          'totalReturnPercentage': 0.0,
          'winRate': 0.45 + Random().nextDouble() * 0.3, // 45-75%
          'sharpeRatio': -0.5 + Random().nextDouble() * 2.0, // -0.5 to 1.5
          'maxDrawdown': -0.05 - Random().nextDouble() * 0.15, // -5% to -20%
          'totalTrades': 50 + Random().nextInt(200),
          'winningTrades': 0,
          'losingTrades': 0,
          'averageWin': 0.0,
          'averageLoss': 0.0,
          'profitFactor': 0.0,
          'equityCurve': <Map<String, dynamic>>[],
          'createdAt': DateTime.now(),
        };

        // Calculate derived metrics
        final totalReturn = backtestResult['finalBalance'] - initialBalance;
        backtestResult['totalReturn'] = totalReturn;
        backtestResult['totalReturnPercentage'] =
            (totalReturn / initialBalance) * 100;

        final totalTrades = backtestResult['totalTrades'] as int;
        final winRate = backtestResult['winRate'] as double;
        backtestResult['winningTrades'] = (totalTrades * winRate).round();
        backtestResult['losingTrades'] =
            totalTrades - backtestResult['winningTrades'];

        // Generate equity curve
        backtestResult['equityCurve'] = _generateEquityCurve(
            initialBalance, backtestResult['finalBalance'], startDate, endDate);

        _backtestResults.add(backtestResult);
        safeNotifyListeners();
      },
      loadingMessage: 'Running backtest...',
    );
  }

  // Analytics
  void refreshPortfolioData() {
    if (_selectedPortfolio != null) {
      _updatePortfolioMetrics(_selectedPortfolio!);
      safeNotifyListeners();
    }
  }

  void refreshAnalytics() {
    // Refresh analytics for selected portfolio
    refreshPortfolioData();
  }

  // Helper methods
  void _generateSampleData() {
    // Generate sample portfolios
    _paperPortfolios = [
      {
        'id': _generateId(),
        'name': 'Aggressive Growth',
        'initialBalance': 50000.0,
        'currentBalance': 58750.0,
        'allocatedBalance': 45000.0,
        'availableBalance': 13750.0,
        'positions': [
          {
            'symbol': 'AAPL',
            'side': 'Buy',
            'quantity': 100,
            'entryPrice': 145.50,
            'currentPrice': 152.30,
            'status': 'Open',
            'pnl': 680.0,
            'pnlPercentage': 0.047,
            'timestamp': DateTime.now().subtract(const Duration(hours: 4)),
          },
          {
            'symbol': 'TSLA',
            'side': 'Buy',
            'quantity': 50,
            'entryPrice': 245.80,
            'currentPrice': 238.90,
            'status': 'Open',
            'pnl': -345.0,
            'pnlPercentage': -0.028,
            'timestamp': DateTime.now().subtract(const Duration(hours: 2)),
          },
        ],
        'trades': <Map<String, dynamic>>[],
        'totalReturn': 8750.0,
        'totalReturnPercentage': 17.5,
        'winRate': 0.62,
        'sharpeRatio': 1.35,
        'maxDrawdown': -8.2,
        'totalTrades': 47,
        'winningTrades': 29,
        'losingTrades': 18,
        'isActive': true,
        'createdAt': DateTime.now().subtract(const Duration(days: 30)),
        'lastUpdated': DateTime.now(),
      },
      {
        'id': _generateId(),
        'name': 'Conservative Income',
        'initialBalance': 100000.0,
        'currentBalance': 103250.0,
        'allocatedBalance': 80000.0,
        'availableBalance': 23250.0,
        'positions': [
          {
            'symbol': 'JNJ',
            'side': 'Buy',
            'quantity': 200,
            'entryPrice': 155.25,
            'currentPrice': 157.80,
            'status': 'Open',
            'pnl': 510.0,
            'pnlPercentage': 0.016,
            'timestamp': DateTime.now().subtract(const Duration(days: 1)),
          },
          {
            'symbol': 'KO',
            'side': 'Buy',
            'quantity': 300,
            'entryPrice': 58.90,
            'currentPrice': 59.45,
            'status': 'Open',
            'pnl': 165.0,
            'pnlPercentage': 0.009,
            'timestamp': DateTime.now().subtract(const Duration(hours: 6)),
          },
        ],
        'trades': <Map<String, dynamic>>[],
        'totalReturn': 3250.0,
        'totalReturnPercentage': 3.25,
        'winRate': 0.71,
        'sharpeRatio': 0.89,
        'maxDrawdown': -3.1,
        'totalTrades': 24,
        'winningTrades': 17,
        'losingTrades': 7,
        'isActive': false,
        'createdAt': DateTime.now().subtract(const Duration(days: 60)),
        'lastUpdated': DateTime.now(),
      },
    ];

    // Generate sample trades
    _generateSampleTrades();

    // Generate sample backtest results
    _generateSampleBacktestResults();
  }

  void _generateSampleTrades() {
    final random = Random();
    final symbols = [
      'AAPL',
      'GOOGL',
      'MSFT',
      'TSLA',
      'AMZN',
      'NVDA',
      'META',
      'AMD'
    ];
    final sides = ['Buy', 'Sell'];

    for (final portfolio in _paperPortfolios) {
      for (int i = 0; i < 20; i++) {
        final symbol = symbols[random.nextInt(symbols.length)];
        final side = sides[random.nextInt(sides.length)];
        final quantity = 50 + random.nextInt(200);
        final entryPrice = 50.0 + random.nextDouble() * 200.0;
        final exitPrice = entryPrice * (0.95 + random.nextDouble() * 0.1);
        final isClosed = random.nextBool();

        final trade = {
          'id': _generateId(),
          'portfolioId': portfolio['id'],
          'symbol': symbol,
          'side': side,
          'orderType': 'Market',
          'quantity': quantity,
          'entryPrice': entryPrice,
          'exitPrice': isClosed ? exitPrice : null,
          'status': isClosed ? 'Closed' : 'Open',
          'pnl': isClosed
              ? _calculateSimplePnl(side, quantity, entryPrice, exitPrice)
              : null,
          'pnlPercentage': isClosed
              ? _calculateSimplePnlPercentage(side, entryPrice, exitPrice)
              : null,
          'fees': 5.0 + quantity * 0.005,
          'timestamp':
              DateTime.now().subtract(Duration(hours: random.nextInt(168))),
          'notes': null,
        };

        portfolio['trades'].add(trade);

        if (isClosed) {
          _recentTrades.add(trade);
        }
      }
    }

    // Sort recent trades
    _recentTrades.sort((a, b) =>
        (b['timestamp'] as DateTime).compareTo(a['timestamp'] as DateTime));
    if (_recentTrades.length > 50) {
      _recentTrades = _recentTrades.take(50).toList();
    }
  }

  void _generateSampleBacktestResults() {
    final random = Random();
    final strategies = [
      'Momentum Strategy',
      'Mean Reversion',
      'Breakout Scanner',
      'RSI Strategy'
    ];
    final symbols = ['AAPL', 'SPY', 'QQQ', 'IWM'];

    for (int i = 0; i < 5; i++) {
      final strategyName = strategies[random.nextInt(strategies.length)];
      final symbol = symbols[random.nextInt(symbols.length)];
      final initialBalance = 10000.0 + random.nextInt(40000).toDouble();

      final backtest = {
        'id': _generateId(),
        'strategyName': strategyName,
        'symbol': symbol,
        'timeframe': '1D',
        'startDate': DateTime.now().subtract(const Duration(days: 365)),
        'endDate': DateTime.now(),
        'initialBalance': initialBalance,
        'finalBalance': initialBalance * (0.8 + random.nextDouble() * 0.4),
        'totalReturn': 0.0,
        'totalReturnPercentage': 0.0,
        'winRate': 0.4 + random.nextDouble() * 0.4,
        'sharpeRatio': -1.0 + random.nextDouble() * 2.5,
        'maxDrawdown': -0.2 - random.nextDouble() * 0.15,
        'totalTrades': 25 + random.nextInt(150),
        'winningTrades': 0,
        'losingTrades': 0,
        'averageWin': 0.0,
        'averageLoss': 0.0,
        'profitFactor': 0.0,
        'equityCurve': <Map<String, dynamic>>[],
        'createdAt':
            DateTime.now().subtract(Duration(days: random.nextInt(30))),
      };

      // Calculate derived metrics
      final totalReturn = backtest['finalBalance'] - initialBalance;
      backtest['totalReturn'] = totalReturn;
      backtest['totalReturnPercentage'] = (totalReturn / initialBalance) * 100;

      final totalTrades = backtest['totalTrades'] as int;
      final winRate = backtest['winRate'] as double;
      backtest['winningTrades'] = (totalTrades * winRate).round();
      backtest['losingTrades'] = totalTrades - backtest['winningTrades'];

      _backtestResults.add(backtest);
    }

    // Sort by creation date
    _backtestResults.sort((a, b) =>
        (b['createdAt'] as DateTime).compareTo(a['createdAt'] as DateTime));
  }

  void _loadPortfolioTrades() {
    if (_selectedPortfolio != null) {
      _recentTrades = _selectedPortfolio!['trades']
          .where((t) => t['status'] == 'Closed')
          .toList();
      _recentTrades.sort((a, b) =>
          (b['timestamp'] as DateTime).compareTo(a['timestamp'] as DateTime));
    }
  }

  void _updatePortfolioAfterTrade(
      Map<String, dynamic> portfolio, Map<String, dynamic> trade) {
    // Update balances
    final tradeCost = trade['quantity'] * trade['entryPrice'] + trade['fees'];
    portfolio['currentBalance'] = portfolio['currentBalance'] - tradeCost;
    portfolio['allocatedBalance'] += tradeCost;
    portfolio['availableBalance'] =
        portfolio['currentBalance'] - portfolio['allocatedBalance'];

    // Add position
    portfolio['positions'].add(trade);
    portfolio['trades'].add(trade);
    portfolio['totalTrades'] = portfolio['totalTrades'] + 1;
    portfolio['lastUpdated'] = DateTime.now();

    // Update metrics
    _updatePortfolioMetrics(portfolio);
  }

  void _updatePortfolioAfterClose(Map<String, dynamic> portfolio,
      Map<String, dynamic> position, Map<String, dynamic> closingTrade) {
    // Update position
    position['status'] = 'Closed';
    position['exitPrice'] = closingTrade['exitPrice'];
    position['pnl'] = closingTrade['pnl'];
    position['pnlPercentage'] = closingTrade['pnlPercentage'];

    // Update portfolio balance
    final exitValue = position['quantity'] * closingTrade['exitPrice'];
    final fees = closingTrade['fees'];
    portfolio['currentBalance'] += exitValue - fees;
    portfolio['allocatedBalance'] -=
        position['quantity'] * position['entryPrice'];
    portfolio['availableBalance'] =
        portfolio['currentBalance'] - portfolio['allocatedBalance'];

    // Remove from active positions
    portfolio['positions'].removeWhere((p) => p['id'] == position['id']);

    // Add closing trade
    portfolio['trades'].add(closingTrade);

    // Update metrics
    _updatePortfolioMetrics(portfolio);
  }

  void _updatePortfolioMetrics(Map<String, dynamic> portfolio) {
    final closedTrades =
        portfolio['trades'].where((t) => t['status'] == 'Closed').toList();

    if (closedTrades.isNotEmpty) {
      // Calculate win rate
      final winningTrades =
          closedTrades.where((t) => (t['pnl'] ?? 0.0) > 0.0).length;
      portfolio['winRate'] = winningTrades / closedTrades.length;
      portfolio['winningTrades'] = winningTrades;
      portfolio['losingTrades'] = closedTrades.length - winningTrades;

      // Calculate total return
      portfolio['totalReturn'] =
          portfolio['currentBalance'] - portfolio['initialBalance'];
      portfolio['totalReturnPercentage'] =
          (portfolio['totalReturn'] / portfolio['initialBalance']) * 100;

      // Calculate other metrics (simplified)
      portfolio['sharpeRatio'] = 0.5 + Random().nextDouble(); // Placeholder
      portfolio['maxDrawdown'] =
          -0.05 - Random().nextDouble() * 0.1; // Placeholder
    }
  }

  double _calculateFees(double quantity, double price) {
    return 5.0 + (quantity * price * 0.005);
  }

  double _getCurrentPrice(String symbol) {
    // Simulate getting current price
    return 100.0 + Random().nextDouble() * 100.0;
  }

  double _calculatePnl(Map<String, dynamic> position, double exitPrice) {
    final side = position['side'] as String;
    final quantity = position['quantity'] as double;
    final entryPrice = position['entryPrice'] as double;

    return _calculateSimplePnl(side, quantity, entryPrice, exitPrice);
  }

  double _calculatePnlPercentage(
      Map<String, dynamic> position, double exitPrice) {
    final side = position['side'] as String;
    final entryPrice = position['entryPrice'] as double;

    return _calculateSimplePnlPercentage(side, entryPrice, exitPrice);
  }

  double _calculateSimplePnl(
      String side, double quantity, double entryPrice, double exitPrice) {
    final priceDiff = exitPrice - entryPrice;
    return side == 'Buy' ? (quantity * priceDiff) : (-quantity * priceDiff);
  }

  double _calculateSimplePnlPercentage(
      String side, double entryPrice, double exitPrice) {
    final priceDiff = exitPrice - entryPrice;
    return side == 'Buy' ? (priceDiff / entryPrice) : (-priceDiff / entryPrice);
  }

  List<Map<String, dynamic>> _generateEquityCurve(double startValue,
      double endValue, DateTime startDate, DateTime endDate) {
    final equityCurve = <Map<String, dynamic>>[];
    final days = endDate.difference(startDate).inDays;
    final random = Random();

    for (int i = 0; i <= days; i++) {
      final progress = i / days;
      final targetValue = startValue + (endValue - startValue) * progress;
      final randomVariation =
          (random.nextDouble() - 0.5) * targetValue * 0.02; // ±2% variation
      final value = targetValue + randomVariation;

      equityCurve.add({
        'date': startDate.add(Duration(days: i)),
        'value': value,
        'drawdown': 0.0, // Would calculate actual drawdown
      });
    }

    return equityCurve;
  }

  String _generateId() {
    return DateTime.now().millisecondsSinceEpoch.toString() +
        Random().nextInt(1000).toString();
  }

  // Public interface methods
  Map<String, dynamic>? getPortfolio(String portfolioId) {
    return _paperPortfolios.firstWhere((p) => p['id'] == portfolioId);
  }

  List<Map<String, dynamic>> getPortfolioPositions(String portfolioId) {
    final portfolio = getPortfolio(portfolioId);
    return portfolio?['positions'] ?? <Map<String, dynamic>>[];
  }

  Map<String, dynamic>? getBacktestResult(String backtestId) {
    return _backtestResults.firstWhere((b) => b['id'] == backtestId);
  }
}
