import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'paper_hands_provider.dart';
import 'widgets/panels/portfolio_management_panel.dart';
import 'widgets/panels/paper_trading_panel.dart';
import 'widgets/panels/performance_analytics_panel.dart';

class PaperHandsView extends StatelessWidget {
  const PaperHandsView({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer<PaperHandsProvider>(
      builder: (context, provider, child) {
        return Row(
          children: [
            // Portfolio Management
            Expanded(
              flex: 1,
              child: PortfolioManagementPanel(provider: provider),
            ),
            // Paper Trading with Advanced Charting
            Expanded(
              flex: 2,
              child: PaperTradingPanel(provider: provider),
            ),
            // Performance Analytics
            Expanded(
              flex: 1,
              child: PerformanceAnalyticsPanel(provider: provider),
            ),
          ],
        );
      },
    );
  }
}
