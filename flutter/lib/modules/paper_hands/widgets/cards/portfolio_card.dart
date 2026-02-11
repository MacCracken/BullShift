import 'package:flutter/material.dart';

class PortfolioCard extends StatelessWidget {
  final Map<String, dynamic> portfolio;
  final VoidCallback onSelect;
  final VoidCallback onDelete;
  final VoidCallback onViewDetails;

  const PortfolioCard({
    super.key,
    required this.portfolio,
    required this.onSelect,
    required this.onDelete,
    required this.onViewDetails,
  });

  @override
  Widget build(BuildContext context) {
    final name = portfolio['name'] as String;
    final initialBalance = portfolio['initialBalance'] as double;
    final currentBalance = portfolio['currentBalance'] as double;
    final totalReturn = portfolio['totalReturn'] as double;
    final winRate = (portfolio['winRate'] as double?) ?? 0.0;
    final isActive = portfolio['isActive'] as bool;
    
    final returnPercentage = ((currentBalance - initialBalance) / initialBalance * 100);
    final returnColor = returnPercentage >= 0 ? Colors.green : Colors.red;

    return Card(
      color: isActive ? const Color(0xFF37474F) : const Color(0xFF2E2E2E),
      margin: const EdgeInsets.only(bottom: 12),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(
                  child: Text(
                    name,
                    style: TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                      fontSize: 16,
                    ),
                  ),
                ),
                if (isActive)
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                    decoration: BoxDecoration(
                      color: Colors.green.withOpacity(0.3),
                      borderRadius: BorderRadius.circular(8),
                      border: Border.all(color: Colors.green.withOpacity(0.5)),
                    ),
                    child: const Text(
                      'ACTIVE',
                      style: TextStyle(
                        color: Colors.green,
                        fontSize: 10,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Text(
                  '\$${currentBalance.toStringAsFixed(2)}',
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const Spacer(),
                Text(
                  '${returnPercentage.toStringAsFixed(2)}%',
                  style: TextStyle(
                    color: returnColor,
                    fontWeight: FontWeight.bold,
                    fontSize: 16,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Text(
                  'Initial: \$${initialBalance.toStringAsFixed(2)}',
                  style: const TextStyle(
                    color: Colors.grey,
                    fontSize: 12,
                  ),
                ),
                const Spacer(),
                if (winRate > 0)
                  Text(
                    'Win Rate: ${(winRate * 100).toInt()}%',
                    style: TextStyle(
                      color: winRate > 0.5 ? Colors.green : Colors.orange,
                      fontSize: 12,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
              ],
            ),
            const SizedBox(height: 12),
            Row(
              children: [
                Expanded(
                  child: ElevatedButton(
                    onPressed: onSelect,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.blue,
                      padding: const EdgeInsets.symmetric(vertical: 8),
                    ),
                    child: const Text('Trade'),
                  ),
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: ElevatedButton(
                    onPressed: onViewDetails,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.purple,
                      padding: const EdgeInsets.symmetric(vertical: 8),
                    ),
                    child: const Text('Details'),
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
