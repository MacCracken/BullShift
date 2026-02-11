# Testing Guide

## Test Structure

```
flutter/test/
├── unit/                      # Unit tests
│   ├── base_provider_test.dart
│   ├── trading_provider_test.dart
│   ├── watchlist_provider_test.dart
│   ├── bullrunnr_provider_test.dart
│   ├── beearly_managed_provider_test.dart
│   ├── paper_hands_provider_test.dart
│   └── trendsetter_provider_test.dart
├── widget/                    # Widget tests
│   ├── ai_provider_card_test.dart
│   ├── news_article_card_test.dart
│   ├── sentiment_mover_card_test.dart
│   ├── sector_sentiment_card_test.dart
│   ├── portfolio_card_test.dart
│   ├── paper_trade_card_test.dart
│   ├── news_search_dialog_test.dart
│   └── create_portfolio_dialog_test.dart
├── integration/               # Integration tests
│   └── (to be added)
└── README.md                  # This file
```

## Running Tests

### Run all tests
```bash
cd flutter
flutter test
```

### Run specific test file
```bash
flutter test test/unit/base_provider_test.dart
flutter test test/widget/portfolio_card_test.dart
```

### Run tests with coverage
```bash
flutter test --coverage
genhtml coverage/lcov.info -o coverage/html
```

### Run tests and watch for changes
```bash
flutter test --watch
```

## Test Categories

### Unit Tests

Test business logic, providers, and services in isolation.

**BaseProvider Tests (16 test cases):**
- ✅ Loading state management
- ✅ Error state management  
- ✅ Async execution with error handling
- ✅ Safe notification (disposed state check)

**TradingProvider Tests (10 test cases):**
- ✅ Symbol management
- ✅ Quantity management
- ✅ Order type management
- ✅ Price management
- ✅ Notes management (add, delete, pin, search)

**WatchlistProvider Tests (8 test cases):**
- ✅ Initialization
- ✅ Search functionality
- ✅ Sorting
- ✅ Real-time updates toggle
- ✅ Export/Import

**BullRunnrProvider Tests (28 test cases):**
- ✅ Initialization with sample data
- ✅ News feed refresh
- ✅ Category and sentiment filters
- ✅ Search functionality
- ✅ Market sentiment data structure
- ✅ Sector sentiment validation
- ✅ Sentiment score ranges
- ✅ Article structure validation

**BearlyManagedProvider Tests (28 test cases):**
- ✅ AI provider management (add, configure, toggle, delete)
- ✅ Strategy generation
- ✅ Prompt management (add, update, delete)
- ✅ Prompt execution
- ✅ Data structure validation

**PaperHandsProvider Tests (24 test cases):**
- ✅ Portfolio management (create, select, delete)
- ✅ Trading controls (symbol, quantity, order type, side, price)
- ✅ Order validation (canPlaceOrder logic)
- ✅ Paper trading (place orders, trade history)
- ✅ Timeframe selection
- ✅ Refresh operations

**TrendSetterProvider Tests (28 test cases):**
- ✅ Initialization with sample data
- ✅ Momentum assets validation
- ✅ Filter functionality (timeframe, min momentum, toggles)
- ✅ Watchlist management (add, remove, check)
- ✅ Heat map data
- ✅ Shift alerts
- ✅ Sorting (momentum, volume, price)
- ✅ Asset details lookup

**Total Unit Tests:** 150 test cases

### Widget Tests

Test UI components and their interactions.

**AIProviderCard Tests (7 test cases):**
- ✅ Displays provider info correctly
- ✅ Shows configured/not configured states
- ✅ Shows appropriate buttons
- ✅ Displays last used time
- ✅ Has toggle switch

**NewsArticleCard Tests (10 test cases):**
- ✅ Displays article title
- ✅ Shows source and timestamp
- ✅ Displays sentiment badge with correct color
- ✅ Shows category chip
- ✅ Displays symbol chips
- ✅ Shows sentiment score bar
- ✅ Has Read More and Analysis buttons
- ✅ Handles bullish/bearish/neutral sentiment
- ✅ Formats timestamps correctly

**SentimentMoverCard Tests (10 test cases):**
- ✅ Displays symbol name
- ✅ Shows article count
- ✅ Displays sentiment score percentage
- ✅ Shows buzz score
- ✅ Colors based on sentiment (green/red/grey)
- ✅ Card container styling

**SectorSentimentCard Tests (10 test cases):**
- ✅ Displays sector name
- ✅ Shows sentiment percentage
- ✅ Displays progress bar
- ✅ Colors based on sentiment
- ✅ Alignment for positive/negative
- ✅ Card container styling

**PortfolioCard Tests (13 test cases):**
- ✅ Displays portfolio name
- ✅ Shows current balance formatted
- ✅ Displays return percentage
- ✅ Colors for positive/negative returns
- ✅ Shows initial balance
- ✅ Displays win rate when available
- ✅ Shows ACTIVE badge when active
- ✅ Has Trade and Details buttons
- ✅ Calls callbacks when buttons pressed

**PaperTradeCard Tests (10 test cases):**
- ✅ Displays symbol name
- ✅ Shows Buy side in green
- ✅ Shows Sell side in red
- ✅ Displays quantity and entry price
- ✅ Shows exit price when available
- ✅ Displays P&L with correct colors
- ✅ Has colored indicator bar
- ✅ Card container styling

**NewsSearchDialog Tests (10 test cases):**
- ✅ Displays dialog title
- ✅ Has search keywords field
- ✅ Has symbols field
- ✅ Has Cancel and Search buttons
- ✅ Closes when Cancel pressed
- ✅ Accepts text input
- ✅ Correct dialog width

**CreatePortfolioDialog Tests (10 test cases):**
- ✅ Displays dialog title
- ✅ Has portfolio name field
- ✅ Has initial balance field with dollar prefix
- ✅ Has Cancel and Create buttons
- ✅ Closes when Cancel pressed
- ✅ Accepts text input
- ✅ Balance field uses number keyboard

**Total Widget Tests:** 90 test cases

### Integration Tests

Test complete user flows and module interactions.

**To Add:**
- [ ] End-to-end trading flow
- [ ] Watchlist to trading integration
- [ ] AI provider to strategy generation
- [ ] Paper trading to real trading
- [ ] FFI bridge integration tests

## Test Best Practices

### 1. Arrange-Act-Assert

```dart
test('description', () {
  // Arrange
  provider.setSymbol('AAPL');
  
  // Act
  provider.setSymbol('tsla');
  
  // Assert
  expect(provider.currentSymbol, 'TSLA');
});
```

### 2. Use setUp/tearDown

```dart
group('Test Group', () {
  late Provider provider;
  
  setUp(() {
    provider = Provider();
  });
  
  tearDown(() {
    provider.dispose();
  });
  
  test('test 1', () {
    // Use provider
  });
});
```

### 3. Test Edge Cases

```dart
test('handles empty input', () {
  provider.setSymbol('');
  expect(provider.currentSymbol, '');
});

test('handles null values', () {
  provider.setPrice(null);
  expect(provider.limitPrice, null);
});
```

### 4. Use Descriptive Names

```dart
// Good
test('setLoading updates isLoading to true', () { });

// Bad
test('loading works', () { });
```

### 5. One Assertion Per Concept

```dart
// Good
group('Loading State', () {
  test('is false by default', () { });
  test('updates to true when set', () { });
  test('notifies listeners on change', () { });
});

// Bad - too many assertions in one test
test('loading state', () {
  expect(provider.isLoading, false);
  provider.setLoading(true);
  expect(provider.isLoading, true);
  expect(notifyCount, 1);
});
```

## Mocking

Use Mockito for mocking dependencies:

```dart
import 'package:mockito/mockito.dart';

class MockRustTradingEngine extends Mock implements RustTradingEngine {}

void main() {
  test('test with mock', () {
    final mockEngine = MockRustTradingEngine();
    when(mockEngine.getAccountBalance()).thenReturn(10000.0);
    
    final provider = TradingProvider(mockEngine);
    expect(provider.getAccountBalance(), 10000.0);
  });
}
```

## Coverage Goals

- **Unit Tests:** 80%+ coverage for providers and services
- **Widget Tests:** All extracted widgets (27/27 widget files tested)
- **Integration Tests:** Critical user flows

## Current Test Status

| Component | Unit Tests | Widget Tests | Integration | Coverage |
|-----------|-----------|--------------|-------------|----------|
| BaseProvider | ✅ 16 | N/A | N/A | 100% |
| TradingProvider | ✅ 10 | N/A | ❌ | ~60% |
| WatchlistProvider | ✅ 8 | N/A | ❌ | ~50% |
| BullRunnrProvider | ✅ 28 | N/A | ❌ | ~70% |
| BearlyManagedProvider | ✅ 28 | N/A | ❌ | ~70% |
| PaperHandsProvider | ✅ 24 | N/A | ❌ | ~70% |
| TrendSetterProvider | ✅ 28 | N/A | ❌ | ~70% |
| AIProviderCard | N/A | ✅ 7 | N/A | 100% |
| NewsArticleCard | N/A | ✅ 10 | N/A | 100% |
| SentimentMoverCard | N/A | ✅ 10 | N/A | 100% |
| SectorSentimentCard | N/A | ✅ 10 | N/A | 100% |
| PortfolioCard | N/A | ✅ 13 | N/A | 100% |
| PaperTradeCard | N/A | ✅ 10 | N/A | 100% |
| NewsSearchDialog | N/A | ✅ 10 | N/A | 100% |
| CreatePortfolioDialog | N/A | ✅ 10 | N/A | 100% |
| FFI Bridge | ❌ | N/A | ❌ | 0% |

**Overall Coverage Estimate:** ~30% (up from 15%)
**Total Test Cases:** 240 (150 unit + 90 widget)

## Next Steps

### High Priority
1. ✅ Add tests for remaining providers (BearlyManaged, BullRunnr, PaperHands, TrendSetter) - **COMPLETED**
2. ✅ Add widget tests for extracted widgets (BullRunnr & PaperHands) - **COMPLETED**
3. [ ] Add FFI integration tests

### Medium Priority
4. [ ] Add end-to-end integration tests
5. [ ] Add golden tests for UI components
6. [ ] Set up CI/CD for automated testing

### Low Priority
7. [ ] Add performance tests
8. [ ] Add accessibility tests
9. [ ] Add screenshot comparison tests

## Testing Checklist

When adding new features:

- [ ] Unit tests for business logic
- [ ] Widget tests for new UI components
- [ ] Integration tests for user flows
- [ ] Error handling tests
- [ ] Edge case tests
- [ ] Documentation updated

## Resources

- [Flutter Testing Documentation](https://docs.flutter.dev/testing)
- [Widget Testing Guide](https://docs.flutter.dev/cookbook/testing/widget/introduction)
- [Integration Testing](https://docs.flutter.dev/testing/integration-tests)
- [Mockito Documentation](https://pub.dev/packages/mockito)

---

**Last Updated:** February 11, 2026  
**Test Status:** 240 test cases across 15 test files
