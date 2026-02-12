# ADR-002: Provider Pattern for State Management

**Date:** 2026-02-12  
**Status:** Accepted  
**Context:** Flutter state management approach

## Decision

BullShift uses the Provider package with a custom BaseProvider class for state management.

## Implementation

```dart
class BaseProvider extends ChangeNotifier {
  bool _isLoading = false;
  String? _error;
  
  // Common provider functionality
}

class TradingProvider extends BaseProvider {
  // Trading-specific state and methods
}
```

## Consequences

### Positive
- Simple and straightforward state management
- Easy to understand and maintain
- Good separation of concerns
- Built-in support for dependency injection

### Negative
- No built-in state persistence
- Can lead to prop drilling if not careful

## Alternatives Considered

- **Riverpod**: More powerful but steeper learning curve
- **Bloc**: Good for complex state but more boilerplate
- **GetX**: Popular but less official/maintained
- **setState**: Too simple for app-wide state
