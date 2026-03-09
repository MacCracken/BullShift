import 'package:flutter/foundation.dart';

/// Base provider class that provides common functionality for all providers.
///
/// This eliminates code duplication across providers for:
/// - Loading state management
/// - Error state management
/// - Common notify patterns
abstract class BaseProvider extends ChangeNotifier {
  bool _isLoading = false;
  String? _errorMessage;
  bool _isDisposed = false;

  bool get isLoading => _isLoading;
  String? get errorMessage => _errorMessage;
  bool get hasError => _errorMessage != null;

  /// Set loading state and notify listeners
  void setLoading(bool loading) {
    if (_isLoading != loading) {
      _isLoading = loading;
      safeNotifyListeners();
    }
  }

  /// Set error message and notify listeners
  void setError(String? error) {
    if (_errorMessage != error) {
      _errorMessage = error;
      safeNotifyListeners();
    }
  }

  /// Clear error message
  void clearError() {
    if (_errorMessage != null) {
      _errorMessage = null;
      safeNotifyListeners();
    }
  }

  /// Execute an async operation with automatic loading/error handling
  Future<T?> executeAsync<T>({
    required Future<T> Function() operation,
    String? loadingMessage,
    bool showLoading = true,
  }) async {
    if (showLoading) {
      setLoading(true);
    }
    clearError();

    try {
      final result = await operation();
      return result;
    } catch (e) {
      setError(e.toString());
      return null;
    } finally {
      if (showLoading) {
        setLoading(false);
      }
    }
  }

  /// Safe notify that checks if disposed
  void safeNotifyListeners() {
    if (!_isDisposed) {
      notifyListeners();
    }
  }

  @override
  void dispose() {
    _isDisposed = true;
    super.dispose();
  }
}
