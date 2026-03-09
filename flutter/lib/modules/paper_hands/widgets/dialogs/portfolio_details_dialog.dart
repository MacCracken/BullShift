import 'package:flutter/material.dart';

class PortfolioDetailsDialog extends StatelessWidget {
  final Map<String, dynamic> portfolio;

  const PortfolioDetailsDialog({super.key, required this.portfolio});

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(portfolio['name']),
      content: SizedBox(
        width: 500,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Initial Balance: \$${portfolio['initialBalance']}'),
            const SizedBox(height: 8),
            Text('Current Balance: \$${portfolio['currentBalance']}'),
            const SizedBox(height: 8),
            Text('Total Return: ${portfolio['totalReturnPercentage']}%'),
            const SizedBox(height: 8),
            Text('Win Rate: ${((portfolio['winRate'] ?? 0.0) * 100).toInt()}%'),
            const SizedBox(height: 8),
            Text('Total Trades: ${portfolio['totalTrades']}'),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Close'),
        ),
      ],
    );
  }
}
