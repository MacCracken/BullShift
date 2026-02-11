import 'package:flutter/material.dart';
import '../../paper_hands_provider.dart';
import '../cards/paper_trade_card.dart';
import 'performance_overview_card.dart';
import 'trade_history_list.dart';

class PerformanceAnalyticsPanel extends StatelessWidget {
  final PaperHandsProvider provider;
  
  const PerformanceAnalyticsPanel({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
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
              const Icon(Icons.analytics, color: Colors.white, size: 20),
              const SizedBox(width: 8),
              const Text(
                '📊 Performance Analytics',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const Spacer(),
              IconButton(
                icon: const Icon(Icons.refresh, color: Colors.white),
                onPressed: () => provider.refreshAnalytics(),
                tooltip: 'Refresh Analytics',
              ),
            ],
          ),
          const SizedBox(height: 16),
          // Performance Overview
          if (provider.selectedPortfolio != null)
            PerformanceOverviewCard(portfolio: provider.selectedPortfolio!),
          if (provider.selectedPortfolio != null) ...[
            const SizedBox(height: 16),
            // Trade History
            Expanded(
              child: TradeHistoryList(provider: provider),
            ),
          ] else ...[
            const Expanded(
              child: Center(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(
                      Icons.analytics_outlined,
                      color: Colors.grey,
                      size: 48,
                    ),
                    SizedBox(height: 16),
                    Text(
                      'No performance data',
                      style: TextStyle(color: Colors.grey),
                    ),
                    SizedBox(height: 8),
                    Text(
                      'Select a portfolio to view analytics',
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
}
