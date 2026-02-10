# Testing Guide

## Test Structure

```
flutter/test/
├── unit/                      # Unit tests
│   ├── base_provider_test.dart
│   ├── trading_provider_test.dart
│   └── watchlist_provider_test.dart
├── widget/                    # Widget tests
│   └── ai_provider_card_test.dart
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

**BaseProvider Tests:**
- ✅ Loading state management
- ✅ Error state management  
- ✅ Async execution with error handling
- ✅ Safe notification (disposed state check)

**TradingProvider Tests:**
- ✅ Symbol management
- ✅ Quantity management
- ✅ Order type management
- ✅ Price management
- ✅ Notes management (add, delete, pin, search)

**WatchlistProvider Tests:**
- ✅ Initialization
- ✅ Search functionality
- ✅ Sorting
- ✅ Real-time updates toggle
- ✅ Export/Import

### Widget Tests

Test UI components and their interactions.

**AIProviderCard Tests:**
- ✅ Displays provider info correctly
- ✅ Shows configured/not configured states
- ✅ Shows appropriate buttons
- ✅ Displays last used time
- ✅ Has toggle switch

**Tests To Add:**
- [ ] StrategyCard widget tests
- [ ] PromptCard widget tests
- [ ] ProviderSetupPanel widget tests
- [ ] StrategyGenerationPanel widget tests
- [ ] PromptManagementPanel widget tests
- [ ] All dialog widget tests
- [ ] NewsFeedPanel widget tests (BullRunnr)
- [ ] PortfolioManagementPanel widget tests (PaperHands)

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
- **Widget Tests:** All extracted widgets
- **Integration Tests:** Critical user flows

## Current Test Status

| Component | Unit Tests | Widget Tests | Integration | Coverage |
|-----------|-----------|--------------|-------------|----------|
| BaseProvider | ✅ | N/A | N/A | 100% |
| TradingProvider | ✅ | N/A | ❌ | ~60% |
| WatchlistProvider | ✅ | N/A | ❌ | ~50% |
| BearlyManagedProvider | ❌ | N/A | ❌ | 0% |
| AIProviderCard | N/A | ✅ | N/A | 100% |
| StrategyCard | N/A | ❌ | N/A | 0% |
| PromptCard | N/A | ❌ | N/A | 0% |
| FFI Bridge | ❌ | N/A | ❌ | 0% |

**Overall Coverage Estimate:** ~15%

## Next Steps

### High Priority
1. [ ] Add tests for remaining providers (BearlyManaged, BullRunnr, PaperHands, TrendSetter)
2. [ ] Add widget tests for all extracted widgets
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
