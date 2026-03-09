import 'package:flutter/material.dart';
import '../../../../widgets/advanced_charting_widget.dart';
import '../../paper_hands_provider.dart';
import 'paper_trading_controls.dart';
import '../dialogs/paper_trading_settings_dialog.dart';

class PaperTradingPanel extends StatelessWidget {
  final PaperHandsProvider provider;

  const PaperTradingPanel({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    final selectedPortfolio = provider.selectedPortfolio;

    return Container(
      margin: const EdgeInsets.all(8),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF263238),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              const Icon(Icons.trending_up, color: Colors.white, size: 20),
              const SizedBox(width: 8),
              Text(
                selectedPortfolio != null
                    ? '📈 Paper Trading - ${selectedPortfolio['name']}'
                    : '📈 Paper Trading',
                style: const TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const Spacer(),
              if (selectedPortfolio != null) ...[
                IconButton(
                  icon: const Icon(Icons.refresh, color: Colors.white),
                  onPressed: () => provider.refreshPortfolioData(),
                  tooltip: 'Refresh Data',
                ),
                IconButton(
                  icon: const Icon(Icons.settings, color: Colors.white),
                  onPressed: () => _showSettingsDialog(context),
                  tooltip: 'Settings',
                ),
              ],
            ],
          ),
          const SizedBox(height: 16),
          if (selectedPortfolio != null) ...[
            // Advanced Charting
            Expanded(
              flex: 2,
              child: AdvancedChartingWidget(
                symbol: provider.currentSymbol.isEmpty
                    ? 'AAPL'
                    : provider.currentSymbol,
                timeframe: provider.selectedTimeframe,
              ),
            ),
            const SizedBox(height: 8),
            // Paper Trading Controls
            Expanded(
              flex: 1,
              child: PaperTradingControls(provider: provider),
            ),
          ] else ...[
            // No portfolio selected
            const Expanded(
              child: Center(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(
                      Icons.trending_up_outlined,
                      color: Colors.grey,
                      size: 48,
                    ),
                    SizedBox(height: 16),
                    Text(
                      'Select a portfolio to start paper trading',
                      style: TextStyle(color: Colors.grey),
                    ),
                    SizedBox(height: 8),
                    Text(
                      'Create a portfolio from the left panel',
                      style: TextStyle(
                        color: Colors.grey,
                        fontSize: 12,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ],
      ),
    );
  }

  void _showSettingsDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => PaperTradingSettingsDialog(provider: provider),
    );
  }
}
