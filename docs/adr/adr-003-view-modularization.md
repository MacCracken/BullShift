# ADR-003: View File Modularization Pattern

**Date:** 2026-02-12  
**Status:** Accepted  
**Context:** Flutter view file organization

## Decision

Large view files are refactored into a modular structure with panels, cards, and dialogs:

```
modules/[module_name]/
├── [module_name]_view.dart          # Main view (~50 lines)
├── widgets/
│   ├── widgets.dart                 # Barrel file
│   ├── panels/
│   │   ├── some_panel.dart
│   │   └── other_panel.dart
│   ├── cards/
│   │   ├── some_card.dart
│   │   └── other_card.dart
│   └── dialogs/
│       ├── some_dialog.dart
│       └── other_dialog.dart
```

## Target Sizes

- Main view: < 100 lines
- Panel: < 200 lines
- Card: < 150 lines
- Dialog: < 150 lines

## Consequences

### Positive
- Easier to maintain and navigate
- Better code reuse
- Easier to test individual components
- Clearer separation of concerns

### Negative
- More files to manage
- Requires consistent naming conventions

## Migration Complete

| Module | Before | After | Reduction |
|--------|--------|-------|-----------|
| bearly_managed | 1,121 lines | ~50 lines | 96% |
| bullrunnr | 1,065 lines | ~40 lines | 96% |
| paper_hands | 1,057 lines | ~36 lines | 97% |
| trendsetter | 824 lines | 42 lines | 95% |
| watchlist | 838 lines | 22 lines | 97% |
| core_trading | 398 lines | 63 lines | 84% |
