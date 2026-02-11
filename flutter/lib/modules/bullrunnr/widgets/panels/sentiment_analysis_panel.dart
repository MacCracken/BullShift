import 'package:flutter/material.dart';
import '../../bullrunnr_provider.dart';
import '../cards/sentiment_mover_card.dart';
import 'market_sentiment_overview.dart';

class SentimentAnalysisPanel extends StatelessWidget {
  final BullRunnrProvider provider;
  
  const SentimentAnalysisPanel({
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
            '📊 Sentiment Analysis',
            style: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 16),
          // Market sentiment overview
          MarketSentimentOverview(provider: provider),
          const SizedBox(height: 16),
          // Top sentiment movers
          const Text(
            'Top Sentiment Movers',
            style: TextStyle(
              fontSize: 14,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 8),
          Expanded(
            child: ListView.builder(
              itemCount: provider.topSentimentMovers.length,
              itemBuilder: (context, index) {
                final mover = provider.topSentimentMovers[index];
                return SentimentMoverCard(mover: mover);
              },
            ),
          ),
        ],
      ),
    );
  }
}
