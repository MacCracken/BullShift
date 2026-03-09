import 'package:flutter/material.dart';

class PerformanceOverviewCard extends StatelessWidget {
  final Map<String, dynamic> portfolio;

  const PerformanceOverviewCard({
    super.key,
    required this.portfolio,
  });

  @override
  Widget build(BuildContext context) {
    final totalReturn = portfolio['totalReturn'] as double;
    final totalReturnPercentage = portfolio['totalReturnPercentage'] as double;
    final winRate = (portfolio['winRate'] as double?) ?? 0.0;
    final sharpeRatio = (portfolio['sharpeRatio'] as double?) ?? 0.0;
    final maxDrawdown = (portfolio['maxDrawdown'] as double?) ?? 0.0;
    final totalTrades = portfolio['totalTrades'] as int;

    return Card(
      color: const Color(0xFF37474F),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            Row(
              children: [
                _buildMetricCard(
                    'Total Return',
                    '${totalReturnPercentage.toStringAsFixed(2)}%',
                    totalReturn >= 0 ? Colors.green : Colors.red),
                const SizedBox(width: 8),
                _buildMetricCard('Win Rate', '${(winRate * 100).toInt()}%',
                    winRate > 0.5 ? Colors.green : Colors.orange),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                _buildMetricCard('Sharpe Ratio', sharpeRatio.toStringAsFixed(2),
                    sharpeRatio > 1.0 ? Colors.green : Colors.grey),
                const SizedBox(width: 8),
                _buildMetricCard('Max Drawdown',
                    '${maxDrawdown.toStringAsFixed(2)}%', Colors.red),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                _buildMetricCard(
                    'Total Trades', totalTrades.toString(), Colors.blue),
                const SizedBox(width: 8),
                _buildMetricCard('Active Days', '45', Colors.purple),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildMetricCard(String label, String value, Color color) {
    return Expanded(
      child: Container(
        padding: const EdgeInsets.all(12),
        decoration: BoxDecoration(
          color: color.withOpacity(0.1),
          borderRadius: BorderRadius.circular(8),
          border: Border.all(color: color.withOpacity(0.3)),
        ),
        child: Column(
          children: [
            Text(
              value,
              style: TextStyle(
                color: color,
                fontSize: 16,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 4),
            Text(
              label,
              style: const TextStyle(
                color: Colors.grey,
                fontSize: 12,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
