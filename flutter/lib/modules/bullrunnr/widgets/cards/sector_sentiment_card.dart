import 'package:flutter/material.dart';

class SectorSentimentCard extends StatelessWidget {
  final Map<String, dynamic> sector;
  
  const SectorSentimentCard({
    super.key,
    required this.sector,
  });

  @override
  Widget build(BuildContext context) {
    final name = sector['name'] as String;
    final sentiment = (sector['sentiment'] as double).clamp(-1.0, 1.0);
    
    Color getSentimentColor() {
      if (sentiment > 0.3) return Colors.green;
      if (sentiment < -0.3) return Colors.red;
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
              child: Text(
                name,
                style: const TextStyle(
                  color: Colors.white,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
            Container(
              width: 100,
              height: 8,
              decoration: BoxDecoration(
                color: Colors.grey.shade700,
                borderRadius: BorderRadius.circular(4),
              ),
              child: FractionallySizedBox(
                alignment: sentiment >= 0 ? Alignment.centerLeft : Alignment.centerRight,
                widthFactor: sentiment.abs(),
                child: Container(
                  decoration: BoxDecoration(
                    color: getSentimentColor(),
                    borderRadius: BorderRadius.circular(4),
                  ),
                ),
              ),
            ),
            const SizedBox(width: 8),
            Text(
              '${(sentiment.abs() * 100).toInt()}%',
              style: TextStyle(
                color: getSentimentColor(),
                fontWeight: FontWeight.bold,
                fontSize: 12,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
