import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../../services/rust_trading_engine.dart';
import '../core_trading/trading_provider.dart';
import 'bullrunnr_provider.dart';

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
                  items: ['All', 'Earnings', 'M&A', 'Regulatory', 'Market Analysis', 'Breaking News'].map((category) {
                    return DropdownMenuItem(
                      value: category,
                      child: Text(category, style: const TextStyle(color: Colors.white)),
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
                  items: ['All', 'Bullish', 'Bearish', 'Neutral'].map((sentiment) {
                    return DropdownMenuItem(
                      value: sentiment,
                      child: Text(sentiment, style: const TextStyle(color: Colors.white)),
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

class NewsArticleCard extends StatelessWidget {
  final Map<String, dynamic> article;
  
  const NewsArticleCard({
    super.key,
    required this.article,
  });

  @override
  Widget build(BuildContext context) {
    final title = article['title'] as String;
    final source = article['source'] as String;
    final timestamp = article['timestamp'] as DateTime;
    final sentiment = article['sentiment'] as String;
    final score = (article['sentimentScore'] as double).clamp(-1.0, 1.0);
    final symbols = (article['symbols'] as List<String>?) ?? [];
    final category = article['category'] as String;
    
    Color getSentimentColor() {
      switch (sentiment.toLowerCase()) {
        case 'verybullish':
        case 'bullish':
          return Colors.green;
        case 'verybearish':
        case 'bearish':
          return Colors.red;
        default:
          return Colors.grey;
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
            // Header with source and timestamp
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: getSentimentColor(),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Text(
                    sentiment.toUpperCase(),
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                      fontSize: 10,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                Text(
                  source,
                  style: const TextStyle(
                    color: Colors.grey,
                    fontSize: 12,
                  ),
                ),
                const Spacer(),
                Text(
                  _formatTimestamp(timestamp),
                  style: const TextStyle(
                    color: Colors.grey,
                    fontSize: 12,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 8),
            // Title
            Text(
              title,
              style: const TextStyle(
                color: Colors.white,
                fontSize: 16,
                fontWeight: FontWeight.bold,
              ),
              maxLines: 3,
              overflow: TextOverflow.ellipsis,
            ),
            const SizedBox(height: 8),
            // Category and symbols
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                  decoration: BoxDecoration(
                    color: Colors.blue.withOpacity(0.3),
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: Colors.blue.withOpacity(0.5)),
                  ),
                  child: Text(
                    category,
                    style: const TextStyle(
                      color: Colors.blue,
                      fontSize: 10,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: Wrap(
                    spacing: 4,
                    children: symbols.map((symbol) => _buildSymbolChip(symbol)).toList(),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 8),
            // Sentiment score bar
            Row(
              children: [
                const Text(
                  'Sentiment: ',
                  style: TextStyle(color: Colors.grey, fontSize: 12),
                ),
                Expanded(
                  child: Container(
                    height: 6,
                    decoration: BoxDecoration(
                      color: Colors.grey.shade700,
                      borderRadius: BorderRadius.circular(3),
                    ),
                    child: FractionallySizedBox(
                      alignment: score >= 0 ? Alignment.centerLeft : Alignment.centerRight,
                      widthFactor: score.abs(),
                      child: Container(
                        decoration: BoxDecoration(
                          color: getSentimentColor(),
                          borderRadius: BorderRadius.circular(3),
                        ),
                      ),
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                Text(
                  '${(score.abs() * 100).toInt()}%',
                  style: TextStyle(
                    color: getSentimentColor(),
                    fontSize: 12,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 8),
            // Action buttons
            Row(
              children: [
                Expanded(
                  child: ElevatedButton(
                    onPressed: () => _openArticle(article),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.blue,
                      padding: const EdgeInsets.symmetric(vertical: 8),
                    ),
                    child: const Text('Read More'),
                  ),
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: ElevatedButton(
                    onPressed: () => _showAnalysis(context, article),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.purple,
                      padding: const EdgeInsets.symmetric(vertical: 8),
                    ),
                    child: const Text('Analysis'),
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildSymbolChip(String symbol) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
      decoration: BoxDecoration(
        color: Colors.green.withOpacity(0.3),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.green.withOpacity(0.5)),
      ),
      child: Text(
        symbol,
        style: const TextStyle(
          color: Colors.green,
          fontSize: 10,
          fontWeight: FontWeight.bold,
        ),
      ),
    );
  }

  String _formatTimestamp(DateTime timestamp) {
    final now = DateTime.now();
    final difference = now.difference(timestamp);
    
    if (difference.inMinutes < 1) {
      return 'Just now';
    } else if (difference.inHours < 1) {
      return '${difference.inMinutes}m ago';
    } else if (difference.inDays < 1) {
      return '${difference.inHours}h ago';
    } else {
      return '${difference.inDays}d ago';
    }
  }

  void _openArticle(Map<String, dynamic> article) {
    // Open article in browser or web view
    debugPrint('Opening article: ${article['url']}');
  }

  void _showAnalysis(BuildContext context, Map<String, dynamic> article) {
    showDialog(
      context: context,
      builder: (context) => NewsAnalysisDialog(article: article),
    );
  }
}

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

class MarketSentimentOverview extends StatelessWidget {
  final BullRunnrProvider provider;
  
  const MarketSentimentOverview({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    final marketSentiment = provider.marketSentiment;
    final overallScore = (marketSentiment['overallScore'] as double).clamp(-1.0, 1.0);
    final fearGreedIndex = marketSentiment['fearGreedIndex'] as double;
    
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
                _buildSentimentCount('Bullish', marketSentiment['bullishCount'] as int, Colors.green),
                _buildSentimentCount('Neutral', marketSentiment['neutralCount'] as int, Colors.grey),
                _buildSentimentCount('Bearish', marketSentiment['bearishCount'] as int, Colors.red),
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

class FearGreedGauge extends StatelessWidget {
  final BullRunnrProvider provider;
  
  const FearGreedGauge({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    final fearGreedIndex = provider.marketSentiment['fearGreedIndex'] as double;
    
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
                  colors: [Colors.red, Colors.orange, Colors.grey, Colors.green],
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

class NewsSearchDialog extends StatefulWidget {
  final BullRunnrProvider provider;
  
  const NewsSearchDialog({
    super.key,
    required this.provider,
  });

  @override
  State<NewsSearchDialog> createState() => _NewsSearchDialogState();
}

class _NewsSearchDialogState extends State<NewsSearchDialog> {
  final TextEditingController _searchController = TextEditingController();
  final TextEditingController _symbolController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Search News'),
      content: SizedBox(
        width: 400,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: _searchController,
              decoration: const InputDecoration(
                labelText: 'Search keywords',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _symbolController,
              decoration: const InputDecoration(
                labelText: 'Symbols (comma-separated)',
                border: OutlineInputBorder(),
              ),
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () {
            final keywords = _searchController.text.trim();
            final symbols = _symbolController.text
                .split(',')
                .map((s) => s.trim().toUpperCase())
                .where((s) => s.isNotEmpty)
                .toList();
            
            widget.provider.searchNews(keywords, symbols);
            Navigator.of(context).pop();
          },
          child: const Text('Search'),
        ),
      ],
    );
  }

  @override
  void dispose() {
    _searchController.dispose();
    _symbolController.dispose();
    super.dispose();
  }
}

class NewsAnalysisDialog extends StatelessWidget {
  final Map<String, dynamic> article;
  
  const NewsAnalysisDialog({
    super.key,
    required this.article,
  });

  @override
  Widget build(BuildContext context) {
    final title = article['title'] as String;
    final sentiment = article['sentiment'] as String;
    final score = (article['sentimentScore'] as double).clamp(-1.0, 1.0);
    final confidence = (article['confidence'] as double).clamp(0.0, 1.0);
    final aspects = article['aspects'] as Map<String, dynamic>? ?? {};

    return AlertDialog(
      title: Text('Sentiment Analysis - ${article['symbol'] ?? 'General'}'),
      content: SizedBox(
        width: 400,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              title,
              style: const TextStyle(fontWeight: FontWeight.bold),
              maxLines: 3,
              overflow: TextOverflow.ellipsis,
            ),
            const SizedBox(height: 16),
            _buildAnalysisRow('Overall Sentiment', sentiment, score),
            _buildAnalysisRow('Confidence', '${(confidence * 100).toInt()}%', confidence),
            const SizedBox(height: 12),
            const Text(
              'Aspect Analysis:',
              style: TextStyle(fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 8),
            ...aspects.entries.map((entry) => 
              _buildAspectRow(entry.key, entry.value as Map<String, dynamic>),
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Close'),
        ),
      ],
    );
  }

  Widget _buildAnalysisRow(String label, String value, double score) {
    Color getColor() {
      if (score > 0.3) return Colors.green;
      if (score < -0.3) return Colors.red;
      return Colors.grey;
    }

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        children: [
          SizedBox(
            width: 120,
            child: Text(
              label,
              style: const TextStyle(fontWeight: FontWeight.bold),
            ),
          ),
          Text(
            value,
            style: TextStyle(
              color: getColor(),
              fontWeight: FontWeight.bold,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildAspectRow(String aspect, Map<String, dynamic> data) {
    final sentiment = data['sentiment'] as String;
    final score = (data['score'] as double).clamp(-1.0, 1.0);
    
    Color getColor() {
      if (score > 0.3) return Colors.green;
      if (score < -0.3) return Colors.red;
      return Colors.grey;
    }

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 2, horizontal: 16),
      child: Row(
        children: [
          Text(
            '$aspect:',
            style: const TextStyle(fontSize: 12),
          ),
          const Spacer(),
          Text(
            '$sentiment (${(score.abs() * 100).toInt()}%)',
            style: TextStyle(
              color: getColor(),
              fontSize: 12,
              fontWeight: FontWeight.bold,
            ),
          ),
        ],
      ),
    );
  }
}