# View File Refactoring Guide

## Current State

Three view files exceed 1,000 lines and need to be broken down:

1. **bearly_managed_view.dart** - 1,121 lines (14 classes)
2. **bullrunnr_view.dart** - 1,065 lines  
3. **paper_hands_view.dart** - 1,057 lines

## Refactoring Strategy

### Directory Structure

```
modules/[module_name]/
├── [module_name]_view.dart          # Main view (simplified)
├── widgets/
│   ├── panels/                       # Main panel widgets
│   │   ├── provider_setup_panel.dart
│   │   ├── strategy_generation_panel.dart
│   │   └── prompt_management_panel.dart
│   ├── cards/                        # Card widgets
│   │   ├── ai_provider_card.dart
│   │   ├── strategy_card.dart
│   │   └── prompt_card.dart
│   └── dialogs/                      # Dialog widgets
│       ├── add_provider_dialog.dart
│       ├── configure_provider_dialog.dart
│       └── generate_strategy_dialog.dart
```

### Target File Sizes

- Main view file: < 100 lines
- Panel widgets: < 200 lines each
- Card widgets: < 150 lines each
- Dialog widgets: < 150 lines each

## Extraction Process

### Step 1: Create Directory Structure

```bash
mkdir -p modules/bearly_managed/widgets/{panels,cards,dialogs}
mkdir -p modules/bullrunnr/widgets/{panels,cards,dialogs}
mkdir -p modules/paper_hands/widgets/{panels,cards,dialogs}
```

### Step 2: Identify Components

Use grep to find all classes:
```bash
grep -n "^class " modules/bearly_managed/bearly_managed_view.dart
```

### Step 3: Extract Each Component

1. Copy the class definition and its methods
2. Add proper imports
3. Update references in the main file
4. Export from widgets directory

### Step 4: Update Main View

Replace inline widgets with imports:

**Before:**
```dart
class BearlyManagedView extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(child: ProviderSetupPanel()), // Defined in same file
      ],
    );
  }
}

class ProviderSetupPanel extends StatelessWidget {
  // 150 lines of code
}
```

**After:**
```dart
import 'widgets/panels/provider_setup_panel.dart';

class BearlyManagedView extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(child: ProviderSetupPanel()), // Imported from separate file
      ],
    );
  }
}
```

## Component Inventory

### BearlyManaged Module

**Panels (3):**
- [x] ProviderSetupPanel (lines 38-174)
- [x] StrategyGenerationPanel (lines 336-464)
- [ ] PromptManagementPanel (lines 607-744)

**Cards (3):**
- [x] AIProviderCard (lines 176-334)
- [x] StrategyCard (lines 466-605)
- [ ] PromptCard (lines 746-912)

**Dialogs (6):**
- [ ] AddProviderDialog (lines 914-940)
- [ ] ConfigureProviderDialog (lines 941-972)
- [ ] GenerateStrategyDialog (lines 973-999)
- [ ] StrategyDetailsDialog (lines 1000-1032)
- [ ] AddPromptDialog (lines 1033-1059)
- [ ] ExecutePromptDialog (lines 1060-1091)
- [ ] EditPromptDialog (lines 1092-1121)

### BullRunnr Module

**Panels (3):**
- [ ] NewsFeedPanel
- [ ] SentimentAnalysisPanel
- [ ] MarketSentimentPanel

**Cards:**
- [ ] NewsArticleCard
- [ ] SentimentMoverCard
- [ ] SectorSentimentCard

**Dialogs:**
- [ ] SearchNewsDialog
- [ ] ArticleDetailsDialog

### PaperHands Module

**Panels (3):**
- [ ] PortfolioManagementPanel
- [ ] PaperTradingPanel
- [ ] PerformanceAnalyticsPanel

**Cards:**
- [ ] PortfolioCard
- [ ] TradeCard
- [ ] BacktestResultCard

**Dialogs:**
- [ ] CreatePortfolioDialog
- [ ] PlaceOrderDialog
- [ ] RunBacktestDialog

## Import Pattern

Each extracted file should follow this pattern:

```dart
import 'package:flutter/material.dart';
// Provider import (relative path)
import '../../bearly_managed_provider.dart';
// Child widget imports (relative path)
import '../cards/ai_provider_card.dart';
import '../dialogs/add_provider_dialog.dart';

class ProviderSetupPanel extends StatelessWidget {
  final BearlyManagedProvider provider;
  
  const ProviderSetupPanel({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    // Widget implementation
  }
}
```

## Export Pattern

Create `widgets.dart` barrel file:

```dart
// panels
export 'panels/provider_setup_panel.dart';
export 'panels/strategy_generation_panel.dart';
export 'panels/prompt_management_panel.dart';

// cards
export 'cards/ai_provider_card.dart';
export 'cards/strategy_card.dart';
export 'cards/prompt_card.dart';

// dialogs
export 'dialogs/add_provider_dialog.dart';
export 'dialogs/configure_provider_dialog.dart';
// ... etc
```

Then import all at once:
```dart
import 'widgets/widgets.dart';
```

## Progress Tracking

### Phase 1: BearlyManaged Module ✅ COMPLETE
- [x] Created directory structure
- [x] Extracted ProviderSetupPanel
- [x] Extracted AIProviderCard
- [x] Extracted StrategyGenerationPanel
- [x] Extracted StrategyCard
- [x] Extracted PromptManagementPanel
- [x] Extracted PromptCard
- [x] Extracted all Dialog classes
- [x] Updated main view file

### Phase 2: BullRunnr Module ✅ COMPLETE
- [x] Created directory structure
- [x] Extracted NewsFeedPanel
- [x] Extracted SentimentAnalysisPanel
- [x] Extracted MarketSentimentPanel
- [x] Extracted all cards (NewsArticleCard, SentimentMoverCard, SectorSentimentCard)
- [x] Extracted all dialogs (NewsSearchDialog, NewsAnalysisDialog)
- [x] Extracted helper widgets (MarketSentimentOverview, FearGreedGauge)
- [x] Updated main view file (1,065 → 40 lines)

### Phase 3: PaperHands Module ✅ COMPLETE
- [x] Created directory structure
- [x] Extracted PortfolioManagementPanel
- [x] Extracted PaperTradingPanel
- [x] Extracted PerformanceAnalyticsPanel
- [x] Extracted all cards (PortfolioCard, PaperTradeCard)
- [x] Extracted all dialogs (CreatePortfolioDialog, PortfolioDetailsDialog, PaperTradingSettingsDialog, PaperPositionsDialog)
- [x] Extracted helper widgets (PaperTradingControls, PerformanceOverviewCard, TradeHistoryList)
- [x] Updated main view file (1,057 → 36 lines)

## Expected Results

After complete refactoring:
- **bearly_managed_view.dart**: 1,121 lines → ~50 lines
- **bullrunnr_view.dart**: 1,065 lines → ~50 lines
- **paper_hands_view.dart**: 1,057 lines → ~50 lines

**Total files created**: ~40 new widget files
**Average file size**: < 150 lines
**Maintainability**: Significantly improved

## Next Steps

1. Complete BearlyManaged extraction (continue with remaining 9 components)
2. Repeat process for BullRunnr module
3. Repeat process for PaperHands module
4. Update all imports in the main view files
5. Test that everything still compiles and works
