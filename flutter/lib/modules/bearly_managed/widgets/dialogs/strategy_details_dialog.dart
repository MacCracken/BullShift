import 'package:flutter/material.dart';

class StrategyDetailsDialog extends StatelessWidget {
  final Map<String, dynamic> strategy;

  const StrategyDetailsDialog({
    super.key,
    required this.strategy,
  });

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(strategy['name'] ?? 'Strategy Details'),
      content: SingleChildScrollView(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            Text('Type: ${strategy['type'] ?? 'Unknown'}'),
            Text('Risk Level: ${strategy['riskLevel'] ?? 'Unknown'}'),
            Text('Win Rate: ${((strategy['winRate'] ?? 0) * 100).toStringAsFixed(1)}%'),
            const SizedBox(height: 16),
            Text(strategy['description'] ?? 'No description available'),
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
