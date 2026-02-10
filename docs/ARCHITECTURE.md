# BullShift Codebase Documentation

**Last Updated:** February 10, 2026  
**Status:** Phase 1 Complete - BaseProvider Refactoring

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Provider Pattern](#provider-pattern)
3. [Refactoring Summary](#refactoring-summary)
4. [TODO Registry](#todo-registry)
5. [Code Quality Guidelines](#code-quality-guidelines)

---

## Architecture Overview

### Project Structure

```
bullshift/
├── flutter/
│   ├── lib/
│   │   ├── main.dart                    # App entry point
│   │   ├── modules/                     # Feature modules
│   │   │   ├── core_trading/            # Live trading module
│   │   │   ├── trendsetter/             # Market analytics
│   │   │   ├── bullrunnr/               # News sentiment
│   │   │   ├── bearly_managed/          # AI integration
│   │   │   ├── paper_hands/             # Paper trading
│   │   │   └── watchlist/               # Watchlist management
│   │   ├── services/                    # Shared services
│   │   │   ├── base_provider.dart       # Base provider class
│   │   │   ├── rust_trading_engine.dart # FFI bridge
│   │   │   └── security_manager.dart    # Secure storage
│   │   └── widgets/                     # Reusable widgets
│   └── pubspec.yaml
├── rust/
│   └── src/                             # Rust backend
└── docs/                                # Documentation
```

### Module Descriptions

#### Core Trading Module
- **Purpose:** Live order execution and position management
- **Provider:** `TradingProvider`
- **Key Features:**
  - Market/Limit order submission
  - Real-time position tracking
  - Trading notes with tags
  - Symbol-specific note management

#### TrendSetter Module
- **Purpose:** Market momentum detection and analytics
- **Provider:** `TrendSetterProvider`
- **Key Features:**
  - Momentum scoring algorithm
  - Volume spike detection
  - Heat map visualization
  - Real-time alerts

#### BullRunnr Module
- **Purpose:** News sentiment analysis
- **Provider:** `BullRunnrProvider`
- **Key Features:**
  - Multi-source news aggregation
  - NLP sentiment classification
  - Fear & Greed index
  - Sector sentiment tracking

#### BearlyManaged Module
- **Purpose:** AI provider integration
- **Provider:** `BearlyManagedProvider`
- **Key Features:**
  - Multi-LLM support (OpenAI, Anthropic, Ollama)
  - Strategy generation
  - Prompt management
  - Usage tracking

#### PaperHands Module
- **Purpose:** Paper trading simulation
- **Provider:** `PaperHandsProvider`
- **Key Features:**
  - Virtual portfolio management
  - Strategy backtesting
  - Performance analytics
  - Trade simulation

#### Watchlist Module
- **Purpose:** Symbol tracking and monitoring
- **Provider:** `WatchlistProvider`
- **Key Features:**
  - Real-time price updates
  - Search and filter
  - Import/Export
  - Cross-module integration

---

## Provider Pattern

### BaseProvider

All providers extend `BaseProvider` which provides:

```dart
abstract class BaseProvider extends ChangeNotifier {
  // State management
  bool get isLoading;
  String? get errorMessage;
  bool get hasError;
  
  // State setters
  void setLoading(bool loading);
  void setError(String? error);
  void clearError();
  
  // Async execution with automatic error handling
  Future<T?> executeAsync<T>({
    required Future<T> Function() operation,
    String? loadingMessage,
    bool showLoading = true,
  });
  
  // Safe notification (checks disposed state)
  void safeNotifyListeners();
}
```

### Usage Example

**Before (Manual Error Handling):**
```dart
Future<void> loadData() async {
  setLoading(true);
  setError(null);
  
  try {
    final data = await api.fetchData();
    _data = data;
    notifyListeners();
  } catch (e) {
    setError('Failed to load: $e');
  } finally {
    setLoading(false);
  }
}
```

**After (Using BaseProvider):**
```dart
Future<void> loadData() async {
  await executeAsync(
    operation: () async {
      final data = await api.fetchData();
      _data = data;
    },
    loadingMessage: 'Loading data...',
  );
}
```

### Benefits

1. **Eliminates Code Duplication:** ~200+ lines removed across providers
2. **Consistent Error Handling:** All errors handled uniformly
3. **Memory Safety:** Prevents notification on disposed providers
4. **Cleaner Code:** Focus on business logic, not boilerplate

---

## Refactoring Summary

### Phase 1: BaseProvider Implementation ✅ COMPLETE

#### Files Modified

1. **Created:** `flutter/lib/services/base_provider.dart` (NEW)
   - 82 lines of reusable provider functionality

2. **Modified Providers:**
   - `trading_provider.dart` - 134 lines → cleaner code
   - `trendsetter_provider.dart` - Refactored to use BaseProvider
   - `bullrunnr_provider.dart` - Refactored to use BaseProvider
   - `bearly_managed_provider.dart` - Refactored to use BaseProvider
   - `paper_hands_provider.dart` - Refactored to use BaseProvider
   - `watchlist_provider.dart` - Refactored to use BaseProvider

#### Statistics

- **Total Lines Reduced:** ~200+ lines of boilerplate removed
- **Duplication Eliminated:** 6 providers now share common code
- **Remaining Manual Calls:** 3 (early validation errors only)
- **Code Quality:** Improved maintainability and consistency

#### Pattern Changes

| Pattern | Before | After |
|---------|--------|-------|
| Extends | `ChangeNotifier` | `BaseProvider` |
| Loading | Manual setLoading() | Automatic in executeAsync() |
| Errors | Manual try/catch | Automatic error handling |
| Notification | notifyListeners() | safeNotifyListeners() |
| Async Operations | Manual boilerplate | executeAsync() wrapper |

---

## TODO Registry

### High Priority (User-Facing Features)

#### Flutter Frontend

**Advanced Charting Widget** (`flutter/lib/widgets/advanced_charting_widget.dart`)
- [ ] Line 185: Update timeframe functionality
- [ ] Line 451: Implement actual chart rendering
- [ ] Line 520: Implement volume chart rendering
- [ ] Line 563: Implement indicator chart rendering

**BearlyManaged Module** (`flutter/lib/modules/bearly_managed/bearly_managed_view.dart`)
- [ ] Line 931: Implement add provider dialog
- [ ] Line 963: Implement provider configuration UI
- [ ] Line 990: Implement strategy generation UI
- [ ] Line 1050: Implement add prompt dialog
- [ ] Line 1082: Implement execute prompt UI
- [ ] Line 1114: Implement edit prompt UI

#### Rust Backend

**AI Bridge** (`rust/src/ai_bridge/mod.rs`)
- [ ] Line 435: Implement Anthropic connection test
- [ ] Line 440: Implement Anthropic API requests
- [ ] Line 445: Implement Ollama connection test
- [ ] Line 450: Implement Ollama API requests
- [ ] Line 455: Implement Local LLM connection test
- [ ] Line 460: Implement Local LLM requests
- [ ] Line 465: Implement Custom provider connection test
- [ ] Line 470: Implement Custom provider requests
- [ ] Line 516: Implement encryption using security manager

**Paper Hands** (`rust/src/paper_hands/mod.rs`)
- [ ] Line 596: Implement actual trading simulation
- [ ] Line 615: Implement Monte Carlo simulation
- [ ] Line 626: Implement Monte Carlo statistics calculation
- [ ] Line 638: Implement correlation calculation
- [ ] Line 643: Get current price from market data

**Data Stream** (`rust/src/data_stream/mod.rs`)
- [ ] Line 100: Load credentials from secure storage (production)

### Medium Priority (Technical Debt)

- [ ] Refactor large view files (>1000 lines)
  - `bearly_managed_view.dart` (1,122 lines)
  - `bullrunnr_view.dart` (1,065 lines)
  - `paper_hands_view.dart` (1,057 lines)
- [ ] Add comprehensive unit tests
- [ ] Add integration tests for FFI boundary
- [ ] Implement proper error types in Rust (replace `Result<T, String>`)
- [ ] Add structured logging

### Low Priority (Nice to Have)

- [ ] Add doc comments to all public APIs
- [ ] Implement options trading support
- [ ] Add algorithmic trading execution
- [ ] Support additional brokers (Interactive Brokers, Tradier)
- [ ] Add comprehensive API documentation

---

## Code Quality Guidelines

### Provider Best Practices

1. **Always extend BaseProvider:**
   ```dart
   class MyProvider extends BaseProvider {
     // Implementation
   }
   ```

2. **Use executeAsync for async operations:**
   ```dart
   Future<void> fetchData() async {
     await executeAsync(
       operation: () async {
         // Your async logic here
       },
       loadingMessage: 'Optional loading message',
     );
   }
   ```

3. **Use safeNotifyListeners for manual notifications:**
   ```dart
   void updateState() {
     _state = newState;
     safeNotifyListeners(); // Checks if disposed
   }
   ```

4. **Avoid manual try/catch in providers:**
   ```dart
   // ❌ Don't do this
   try {
     await operation();
   } catch (e) {
     setError(e.toString());
   }
   
   // ✅ Do this instead
   await executeAsync(operation: () async {
     await operation();
   });
   ```

### File Organization

1. **Maximum file size:** 500 lines (refactor if larger)
2. **Widget extraction:** Break down large widgets into smaller components
3. **Private methods:** Use underscore prefix for private methods
4. **Constants:** Extract magic numbers to named constants

### Error Handling

1. **Use specific error types when possible**
2. **Provide user-friendly error messages**
3. **Log technical details for debugging**
4. **Always clear errors before new operations**

### Testing

1. **Unit tests:** Test provider business logic
2. **Widget tests:** Test UI components
3. **Integration tests:** Test FFI boundary
4. **Golden tests:** Test visual regressions

---

## Next Steps

1. ✅ **Phase 1: BaseProvider Refactoring** - COMPLETE
2. ⏳ **Phase 2: Address TODO Comments** - IN PROGRESS
3. ⏳ **Phase 3: Break Down Large Files**
4. ⏳ **Phase 4: Add Comprehensive Tests**
5. ⏳ **Phase 5: Verify Build**

---

## Maintenance Notes

### Regular Tasks

- [ ] Review TODO registry monthly
- [ ] Update dependencies quarterly
- [ ] Run security audit semi-annually
- [ ] Performance profiling annually

### Documentation Updates

- Update this document after each major refactoring
- Document architectural decisions in ADRs
- Keep CHANGELOG.md updated

---

## Contributing

When contributing to this codebase:

1. Follow the Provider Pattern guidelines
2. Update this documentation for architectural changes
3. Add/update tests for new features
4. Address TODO items when working on related code
5. Keep files under 500 lines

---

**Questions?** Refer to the audit summary or contact the development team.
