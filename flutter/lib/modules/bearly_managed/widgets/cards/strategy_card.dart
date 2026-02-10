import 'package:flutter/material.dart';

class StrategyCard extends StatelessWidget {
  final Map<String, dynamic> strategy;
  final VoidCallback onView;
  final VoidCallback onActivate;
  final VoidCallback onDelete;

  const StrategyCard({
    super.key,
    required this.strategy,
    required this.onView,
    required this.onActivate,
    required this.onDelete,
  });

  @override
  Widget build(BuildContext context) {
    final name = strategy['name'] as String;
    final description = strategy['description'] as String;
    final strategyType = strategy['type'] as String;
    final riskLevel = strategy['riskLevel'] as String;
    final isActive = strategy['isActive'] as bool;
    final winRate = (strategy['winRate'] as double?) ?? 0.0;
    
    Color getStrategyColor() {
      switch (strategyType.toLowerCase()) {
        case 'momentum': return Colors.blue;
        case 'meanreversion': return Colors.green;
        case 'breakout': return Colors.orange;
        case 'sentiment': return Colors.purple;
        default: return Colors.grey;
      }
    }

    Color getRiskColor() {
      switch (riskLevel.toLowerCase()) {
        case 'conservative': return Colors.green;
        case 'moderate': return Colors.yellow;
        case 'aggressive': return Colors.orange;
        case 'veryaggressive': return Colors.red;
        default: return Colors.grey;
      }
    }

    return Card(
      color: const Color(0xFF37474F),
      margin: const EdgeInsets.only(bottom: 12),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: getStrategyColor(),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Text(
                    strategyType.toUpperCase(),
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                      fontSize: 10,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    name,
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                      fontSize: 16,
                    ),
                  ),
                ),
                Switch(
                  value: isActive,
                  onChanged: (_) => onActivate(),
                  activeColor: getStrategyColor(),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Text(
              description,
              style: const TextStyle(
                color: Colors.grey,
                fontSize: 12,
              ),
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                  decoration: BoxDecoration(
                    color: getRiskColor().withOpacity(0.3),
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: getRiskColor().withOpacity(0.5)),
                  ),
                  child: Text(
                    riskLevel,
                    style: TextStyle(
                      color: getRiskColor(),
                      fontSize: 10,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                if (winRate > 0) ...[
                  Text(
                    'Win Rate: ${(winRate * 100).toInt()}%',
                    style: TextStyle(
                      color: winRate > 0.5 ? Colors.green : Colors.red,
                      fontSize: 12,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ],
                const Spacer(),
                IconButton(
                  icon: const Icon(Icons.info_outline, color: Colors.white, size: 16),
                  onPressed: onView,
                  tooltip: 'View Details',
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
