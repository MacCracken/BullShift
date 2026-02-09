import 'package:flutter/foundation.dart';
import '../services/rust_trading_engine.dart';

class TradingProvider extends ChangeNotifier {
  final RustTradingEngine _rustEngine;
  
  String _currentSymbol = '';
  double _currentQuantity = 0.0;
  String _orderType = 'MARKET';
  double? _limitPrice;
  List<Map<String, dynamic>> _positions = [];
  bool _isLoading = false;
  String? _errorMessage;
  
  // Notes functionality
  Map<String, List<Map<String, dynamic>>> _symbolNotes = {};

  TradingProvider(this._rustEngine);

  // Getters
  String get currentSymbol => _currentSymbol;
  double get currentQuantity => _currentQuantity;
  String get orderType => _orderType;
  double? get limitPrice => _limitPrice;
  List<Map<String, dynamic>> get positions => _positions;
  bool get isLoading => _isLoading;
  String? get errorMessage => _errorMessage;

  // Setters
  void setSymbol(String symbol) {
    _currentSymbol = symbol.toUpperCase();
    notifyListeners();
  }

  void setQuantity(double quantity) {
    _currentQuantity = quantity;
    notifyListeners();
  }

  void setOrderType(String orderType) {
    _orderType = orderType;
    notifyListeners();
  }

  void setPrice(double? price) {
    _limitPrice = price;
    notifyListeners();
  }

  void setLoading(bool loading) {
    _isLoading = loading;
    notifyListeners();
  }

  void setError(String? error) {
    _errorMessage = error;
    notifyListeners();
  }

  // Trading Actions
  Future<void> submitMarketOrder(String side) async {
    if (_currentSymbol.isEmpty || _currentQuantity <= 0) {
      setError('Please enter symbol and quantity');
      return;
    }

    setLoading(true);
    setError(null);

    try {
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
        
        notifyListeners();
      } else {
        setError('Order submission failed');
      }
    } catch (e) {
      setError('Order error: $e');
    } finally {
      setLoading(false);
    }
  }

  Future<void> submitLimitOrder(String side) async {
    if (_currentSymbol.isEmpty || _currentQuantity <= 0 || _limitPrice == null) {
      setError('Please enter symbol, quantity, and limit price');
      return;
    }

    setLoading(true);
    setError(null);

    try {
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
        
        notifyListeners();
      } else {
        setError('Order submission failed');
      }
    } catch (e) {
      setError('Order error: $e');
    } finally {
      setLoading(false);
    }
  }

  Future<void> loadPositions() async {
    try {
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
      
      notifyListeners();
    } catch (e) {
      setError('Failed to load positions: $e');
    }
  }

  Future<void> refreshData() async {
    await loadPositions();
  }

  Future<void> connectMarketData(String symbol) async {
    try {
      final success = _rustEngine.connectMarketData(symbol);
      if (!success) {
        setError('Failed to connect to market data');
      }
    } catch (e) {
      setError('Market data connection error: $e');
    }
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
    
    notifyListeners();
  }

  List<Map<String, dynamic>> getNotesForSymbol(String symbol) {
    return _symbolNotes[symbol] ?? [];
  }

  void deleteNote(String noteId) {
    for (final notes in _symbolNotes.values) {
      notes.removeWhere((note) => note['id'] == noteId);
    }
    notifyListeners();
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
    notifyListeners();
  }

  void clearNotesForSymbol(String symbol) {
    _symbolNotes.remove(symbol);
    notifyListeners();
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
    notifyListeners();
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