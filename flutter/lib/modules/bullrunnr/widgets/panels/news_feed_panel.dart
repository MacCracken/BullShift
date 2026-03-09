import 'package:flutter/material.dart';
import '../../bullrunnr_provider.dart';
import '../cards/news_article_card.dart';
import '../dialogs/news_search_dialog.dart';

class NewsFeedPanel extends StatelessWidget {
  final BullRunnrProvider provider;

  const NewsFeedPanel({
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
          Row(
            children: [
              const Text(
                '📰 Real-Time News Feed',
                style: TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const Spacer(),
              IconButton(
                icon: const Icon(Icons.refresh, color: Colors.white),
                onPressed: () => provider.refreshNews(),
              ),
              IconButton(
                icon: const Icon(Icons.search, color: Colors.white),
                onPressed: () => _showSearchDialog(context),
              ),
            ],
          ),
          const SizedBox(height: 16),
          // Filter controls
          Row(
            children: [
              Expanded(
                child: DropdownButtonFormField<String>(
                  decoration: const InputDecoration(
                    labelText: 'Category',
                    border: OutlineInputBorder(),
                    labelStyle: TextStyle(color: Colors.white),
                  ),
                  value: provider.selectedCategory,
                  items: [
                    'All',
                    'Earnings',
                    'M&A',
                    'Regulatory',
                    'Market Analysis',
                    'Breaking News'
                  ].map((category) {
                    return DropdownMenuItem(
                      value: category,
                      child: Text(category,
                          style: const TextStyle(color: Colors.white)),
                    );
                  }).toList(),
                  onChanged: (value) {
                    if (value != null) {
                      provider.setCategoryFilter(value);
                    }
                  },
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: DropdownButtonFormField<String>(
                  decoration: const InputDecoration(
                    labelText: 'Sentiment',
                    border: OutlineInputBorder(),
                    labelStyle: TextStyle(color: Colors.white),
                  ),
                  value: provider.selectedSentiment,
                  items:
                      ['All', 'Bullish', 'Bearish', 'Neutral'].map((sentiment) {
                    return DropdownMenuItem(
                      value: sentiment,
                      child: Text(sentiment,
                          style: const TextStyle(color: Colors.white)),
                    );
                  }).toList(),
                  onChanged: (value) {
                    if (value != null) {
                      provider.setSentimentFilter(value);
                    }
                  },
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          // News articles list
          Expanded(
            child: provider.isLoading
                ? const Center(
                    child: CircularProgressIndicator(color: Colors.white),
                  )
                : ListView.builder(
                    itemCount: provider.newsArticles.length,
                    itemBuilder: (context, index) {
                      final article = provider.newsArticles[index];
                      return NewsArticleCard(article: article);
                    },
                  ),
          ),
        ],
      ),
    );
  }

  void _showSearchDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => NewsSearchDialog(provider: provider),
    );
  }
}
