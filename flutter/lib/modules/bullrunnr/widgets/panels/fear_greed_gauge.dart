import 'package:flutter/material.dart';
import '../../../../services/safe_cast.dart';
import '../../bullrunnr_provider.dart';

class FearGreedGauge extends StatelessWidget {
  final BullRunnrProvider provider;

  const FearGreedGauge({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    final fearGreedIndex =
        provider.marketSentiment.safeDouble('fearGreedIndex');

    String getFearGreedLabel() {
      if (fearGreedIndex >= 75) return 'Extreme Greed';
      if (fearGreedIndex >= 55) return 'Greed';
      if (fearGreedIndex >= 45) return 'Neutral';
      if (fearGreedIndex >= 25) return 'Fear';
      return 'Extreme Fear';
    }

    Color getFearGreedColor() {
      if (fearGreedIndex >= 55) return Colors.green;
      if (fearGreedIndex >= 45) return Colors.grey;
      return Colors.red;
    }

    return Card(
      color: const Color(0xFF37474F),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            const Text(
              'Fear & Greed Index',
              style: TextStyle(
                color: Colors.white,
                fontSize: 16,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 12),
            // Gauge visualization
            Container(
              height: 30,
              decoration: BoxDecoration(
                gradient: const LinearGradient(
                  colors: [
                    Colors.red,
                    Colors.orange,
                    Colors.grey,
                    Colors.green
                  ],
                  stops: [0.0, 0.25, 0.5, 1.0],
                ),
                borderRadius: BorderRadius.circular(15),
              ),
              child: Stack(
                children: [
                  Center(
                    child: Text(
                      '${fearGreedIndex.toInt()}',
                      style: const TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.bold,
                        fontSize: 16,
                      ),
                    ),
                  ),
                  Positioned(
                    left: '${(fearGreedIndex / 100.0) * 100}%',
                    top: 0,
                    bottom: 0,
                    child: Container(
                      width: 4,
                      decoration: BoxDecoration(
                        color: Colors.white,
                        borderRadius: BorderRadius.circular(2),
                      ),
                    ),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 8),
            Text(
              getFearGreedLabel(),
              style: TextStyle(
                color: getFearGreedColor(),
                fontWeight: FontWeight.bold,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
