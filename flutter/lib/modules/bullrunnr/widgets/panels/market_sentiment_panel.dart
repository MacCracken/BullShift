import 'package:flutter/material.dart';
import '../../bullrunnr_provider.dart';
import '../cards/sector_sentiment_card.dart';
import 'fear_greed_gauge.dart';

class MarketSentimentPanel extends StatelessWidget {
  final BullRunnrProvider provider;
  
  const MarketSentimentPanel({
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
          const Text(
            '🌍 Market Overview',
            style: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 16),
          // Fear & Greed gauge
          FearGreedGauge(provider: provider),
          const SizedBox(height: 16),
          // Sector sentiment
          const Text(
            'Sector Sentiment',
            style: TextStyle(
              fontSize: 14,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 8),
          Expanded(
            child: ListView.builder(
              itemCount: provider.sectorSentiment.length,
              itemBuilder: (context, index) {
                final sector = provider.sectorSentiment[index];
                return SectorSentimentCard(sector: sector);
              },
            ),
          ),
        ],
      ),
    );
  }
}
