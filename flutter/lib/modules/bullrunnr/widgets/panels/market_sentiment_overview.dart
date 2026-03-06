import 'package:flutter/material.dart';
import '../../../../services/safe_cast.dart';
import '../../bullrunnr_provider.dart';

class MarketSentimentOverview extends StatelessWidget {
  final BullRunnrProvider provider;
  
  const MarketSentimentOverview({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    final marketSentiment = provider.marketSentiment;
    final overallScore = marketSentiment.safeDouble('overallScore').clamp(-1.0, 1.0);
    final fearGreedIndex = marketSentiment.safeDouble('fearGreedIndex');
    
    Color getSentimentColor() {
      if (overallScore > 0.3) return Colors.green;
      if (overallScore < -0.3) return Colors.red;
      return Colors.grey;
    }

    String getSentimentLabel() {
      if (overallScore > 0.5) return 'Very Bullish';
      if (overallScore > 0.3) return 'Bullish';
      if (overallScore < -0.5) return 'Very Bearish';
      if (overallScore < -0.3) return 'Bearish';
      return 'Neutral';
    }

    return Card(
      color: const Color(0xFF37474F),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            Row(
              children: [
                const Text(
                  'Market Sentiment:',
                  style: TextStyle(color: Colors.white, fontSize: 14),
                ),
                const Spacer(),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: getSentimentColor(),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Text(
                    getSentimentLabel(),
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                      fontSize: 12,
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            // Sentiment score bar
            Container(
              height: 20,
              decoration: BoxDecoration(
                color: Colors.grey.shade700,
                borderRadius: BorderRadius.circular(10),
              ),
              child: FractionallySizedBox(
                alignment: overallScore >= 0 ? Alignment.centerLeft : Alignment.centerRight,
                widthFactor: overallScore.abs(),
                child: Container(
                  decoration: BoxDecoration(
                    color: getSentimentColor(),
                    borderRadius: BorderRadius.circular(10),
                  ),
                ),
              ),
            ),
            const SizedBox(height: 12),
            // Fear & Greed Index
            Row(
              children: [
                const Text(
                  'Fear & Greed:',
                  style: TextStyle(color: Colors.white, fontSize: 14),
                ),
                const Spacer(),
                Text(
                  '${fearGreedIndex.toInt()}',
                  style: TextStyle(
                    color: fearGreedIndex > 50 ? Colors.green : Colors.red,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 8),
            // Article counts
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceAround,
              children: [
                _buildSentimentCount('Bullish', marketSentiment.safeInt('bullishCount'), Colors.green),
                _buildSentimentCount('Neutral', marketSentiment.safeInt('neutralCount'), Colors.grey),
                _buildSentimentCount('Bearish', marketSentiment.safeInt('bearishCount'), Colors.red),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildSentimentCount(String label, int count, Color color) {
    return Column(
      children: [
        Text(
          count.toString(),
          style: TextStyle(
            color: color,
            fontSize: 18,
            fontWeight: FontWeight.bold,
          ),
        ),
        Text(
          label,
          style: const TextStyle(
            color: Colors.grey,
            fontSize: 12,
          ),
        ),
      ],
    );
  }
}
