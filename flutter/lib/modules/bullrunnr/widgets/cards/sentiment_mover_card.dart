import 'package:flutter/material.dart';

class SentimentMoverCard extends StatelessWidget {
  final Map<String, dynamic> mover;
  
  const SentimentMoverCard({
    super.key,
    required this.mover,
  });

  @override
  Widget build(BuildContext context) {
    final symbol = mover['symbol'] as String;
    final sentimentScore = (mover['sentimentScore'] as double).clamp(-1.0, 1.0);
    final buzzScore = (mover['buzzScore'] as double).clamp(0.0, 1.0);
    final articleCount = mover['articleCount'] as int;
    
    Color getSentimentColor() {
      if (sentimentScore > 0.3) return Colors.green;
      if (sentimentScore < -0.3) return Colors.red;
      return Colors.grey;
    }

    return Card(
      color: const Color(0xFF37474F),
      margin: const EdgeInsets.only(bottom: 8),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Row(
          children: [
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    symbol,
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    '$articleCount articles',
                    style: const TextStyle(
                      color: Colors.grey,
                      fontSize: 12,
                    ),
                  ),
                ],
              ),
            ),
            Column(
              crossAxisAlignment: CrossAxisAlignment.end,
              children: [
                Text(
                  '${(sentimentScore.abs() * 100).toInt()}%',
                  style: TextStyle(
                    color: getSentimentColor(),
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  'Buzz: ${(buzzScore * 100).toInt()}%',
                  style: const TextStyle(
                    color: Colors.orange,
                    fontSize: 12,
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
