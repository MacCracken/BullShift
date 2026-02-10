import '../../services/base_provider.dart';
import '../../services/rust_trading_engine.dart';

class TradingProvider extends BaseProvider {
  final RustTradingEngine _rustEngine;
  
  String _currentSymbol = '';
  double _currentQuantity = 0.0;
  String _orderType = 'MARKET';
  double? _limitPrice;
  List<Map<String, dynamic>> _positions = [];
  
  // Notes functionality
  Map<String, List<Map<String, dynamic>>> _symbolNotes = {};

  TradingProvider(this._rustEngine);

  // Getters
  String get currentSymbol => _currentSymbol;
  double get currentQuantity => _currentQuantity;
  String get orderType => _orderType;
  double? get limitPrice => _limitPrice;
  List<Map<String, dynamic>> get positions => _positions;

  // Setters
  void setSymbol(String symbol) {
    _currentSymbol = symbol.toUpperCase();
    safeNotifyListeners();
  }

  void setQuantity(double quantity) {
    _currentQuantity = quantity;
    safeNotifyListeners();
  }

  void setOrderType(String orderType) {
    _orderType = orderType;
    safeNotifyListeners();
  }

  void setPrice(double? price) {
    _limitPrice = price;
    safeNotifyListeners();
  }

  // Trading Actions
  Future<void> submitMarketOrder(String side) async {
    if (_currentSymbol.isEmpty || _currentQuantity <= 0) {
      setError('Please enter symbol and quantity');
      return;
    }

    await executeAsync(
      operation: () async {
        final success = _rustEngine.submitOrder(
          symbol: _currentSymbol,
          side: side,
          quantity: _currentQuantity,
          orderType: 'MARKET',
        );

        if (success) {
          // Clear form
          _currentSymbol = '';
          _currentQuantity = 0.0;
          _limitPrice = null;
          
          // Refresh positions
          await loadPositions();
          
          return success;
        } else {
          throw Exception('Order submission failed');
        }
      },
    );
  }

  Future<void> submitLimitOrder(String side) async {
    if (_currentSymbol.isEmpty || _currentQuantity <= 0 || _limitPrice == null) {
      setError('Please enter symbol, quantity, and limit price');
      return;
    }

    await executeAsync(
      operation: () async {
        final success = _rustEngine.submitOrder(
          symbol: _currentSymbol,
          side: side,
          quantity: _currentQuantity,
          orderType: 'LIMIT',
          price: _limitPrice,
        );

        if (success) {
          // Clear form
          _currentSymbol = '';
          _currentQuantity = 0.0;
          _limitPrice = null;
          
          // Refresh positions
          await loadPositions();
          
          return success;
        } else {
          throw Exception('Order submission failed');
        }
      },
    );
  }

  Future<void> loadPositions() async {
    await executeAsync(
      operation: () async {
        final positionsPtr = _rustEngine.getPositions();
        
        // For now, create sample data
        _positions = [
          {
            'symbol': 'AAPL',
            'quantity': 100,
            'entryPrice': 150.25,
            'currentPrice': 152.80,
            'unrealizedPnl': 255.0,
          },
          {
            'symbol': 'TSLA',
            'quantity': 50,
            'entryPrice': 245.50,
            'currentPrice': 242.30,
            'unrealizedPnl': -160.0,
          },
        ];
      },
      showLoading: false,
    );
  }

  Future<void> refreshData() async {
    await loadPositions();
  }

  Future<void> connectMarketData(String symbol) async {
    await executeAsync(
      operation: () async {
        final success = _rustEngine.connectMarketData(symbol);
        if (!success) {
          throw Exception('Failed to connect to market data');
        }
        return success;
      },
      showLoading: false,
    );
  }

  double getAccountBalance() {
    return _rustEngine.getAccountBalance();
  }

  // Notes functionality
  void addNote({
    required String symbol,
    required String note,
    List<String> tags = const [],
    bool isPinned = false,
  }) {
    final noteData = {
      'id': DateTime.now().millisecondsSinceEpoch.toString(),
      'note': note,
      'timestamp': DateTime.now(),
      'tags': tags,
      'isPinned': isPinned,
    };
    
    _symbolNotes.putIfAbsent(symbol, () => []).add(noteData);
    
    // Sort notes: pinned first, then by timestamp (newest first)
    _symbolNotes[symbol]!.sort((a, b) {
      if (a['isPinned'] == true && b['isPinned'] != true) return -1;
      if (a['isPinned'] != true && b['isPinned'] == true) return 1;
      return (b['timestamp'] as DateTime).compareTo(a['timestamp'] as DateTime);
    });
    
    safeNotifyListeners();
  }

  List<Map<String, dynamic>> getNotesForSymbol(String symbol) {
    return _symbolNotes[symbol] ?? [];
  }

  void deleteNote(String noteId) {
    for (final notes in _symbolNotes.values) {
      notes.removeWhere((note) => note['id'] == noteId);
    }
    safeNotifyListeners();
  }

  void updateNote({
    required String noteId,
    required String newNote,
    List<String>? tags,
    bool? isPinned,
  }) {
    for (final notes in _symbolNotes.values) {
      for (final note in notes) {
        if (note['id'] == noteId) {
          note['note'] = newNote;
          note['timestamp'] = DateTime.now();
          if (tags != null) note['tags'] = tags;
          if (isPinned != null) note['isPinned'] = isPinned;
          break;
        }
      }
    }
    safeNotifyListeners();
  }

  void clearNotesForSymbol(String symbol) {
    _symbolNotes.remove(symbol);
    safeNotifyListeners();
  }

  void pinNote(String noteId) {
    for (final notes in _symbolNotes.values) {
      for (final note in notes) {
        if (note['id'] == noteId) {
          note['isPinned'] = !(note['isPinned'] ?? false);
          break;
        }
      }
    }
    safeNotifyListeners();
  }

  List<Map<String, dynamic>> getAllNotes() {
    final allNotes = <Map<String, dynamic>>[];
    for (final entry in _symbolNotes.entries) {
      for (final note in entry.value) {
        allNotes.add({
          ...note,
          'symbol': entry.key,
        });
      }
    }
    
    // Sort by timestamp (newest first)
    allNotes.sort((a, b) => (b['timestamp'] as DateTime).compareTo(a['timestamp'] as DateTime));
    return allNotes;
  }

  List<String> getAllTags() {
    final allTags = <String>{};
    for (final notes in _symbolNotes.values) {
      for (final note in notes) {
        final tags = (note['tags'] as List<String>?) ?? [];
        allTags.addAll(tags);
      }
    }
    return allTags.toList()..sort();
  }

  List<Map<String, dynamic>> searchNotes(String query) {
    final results = <Map<String, dynamic>>[];
    final lowerQuery = query.toLowerCase();
    
    for (final entry in _symbolNotes.entries) {
      for (final note in entry.value) {
        final noteText = (note['note'] as String).toLowerCase();
        final tags = (note['tags'] as List<String>?) ?? [];
        
        if (noteText.contains(lowerQuery) || 
            tags.any((tag) => tag.toLowerCase().contains(lowerQuery))) {
          results.add({
            ...note,
            'symbol': entry.key,
          });
        }
      }
    }
    
    return results;
  }
}