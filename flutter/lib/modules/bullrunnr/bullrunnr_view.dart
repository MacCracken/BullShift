import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'bullrunnr_provider.dart';
import 'widgets/panels/news_feed_panel.dart';
import 'widgets/panels/sentiment_analysis_panel.dart';
import 'widgets/panels/market_sentiment_panel.dart';

class BullRunnrView extends StatelessWidget {
  const BullRunnrView({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer<BullRunnrProvider>(
      builder: (context, provider, child) {
        return Row(
          children: [
            // News Feed
            Expanded(
              flex: 2,
              child: NewsFeedPanel(provider: provider),
            ),
            // Sentiment Analysis
            Expanded(
              flex: 1,
              child: SentimentAnalysisPanel(provider: provider),
            ),
            // Market Sentiment
            Expanded(
              flex: 1,
              child: MarketSentimentPanel(provider: provider),
            ),
          ],
        );
      },
    );
  }
}
