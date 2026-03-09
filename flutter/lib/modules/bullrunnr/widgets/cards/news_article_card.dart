import 'package:flutter/material.dart';
import '../../../../services/safe_cast.dart';
import '../dialogs/news_analysis_dialog.dart';

class NewsArticleCard extends StatelessWidget {
  final Map<String, dynamic> article;

  const NewsArticleCard({
    super.key,
    required this.article,
  });

  @override
  Widget build(BuildContext context) {
    final title = article.safeString('title');
    final source = article.safeString('source');
    final timestamp = article['timestamp'] as DateTime;
    final sentiment = article.safeString('sentiment');
    final score = article.safeDouble('sentimentScore').clamp(-1.0, 1.0);
    final symbols = (article['symbols'] as List<String>?) ?? [];
    final category = article.safeString('category');

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
                  padding:
                      const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
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
                  padding:
                      const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
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
                    children: symbols
                        .map((symbol) => _buildSymbolChip(symbol))
                        .toList(),
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
                      alignment: score >= 0
                          ? Alignment.centerLeft
                          : Alignment.centerRight,
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
    debugPrint('Opening article: ${article['url']}');
  }

  void _showAnalysis(BuildContext context, Map<String, dynamic> article) {
    showDialog(
      context: context,
      builder: (context) => NewsAnalysisDialog(article: article),
    );
  }
}
