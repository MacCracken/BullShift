import 'package:flutter/material.dart';
import '../../../../services/safe_cast.dart';

class NewsAnalysisDialog extends StatelessWidget {
  final Map<String, dynamic> article;
  
  const NewsAnalysisDialog({
    super.key,
    required this.article,
  });

  @override
  Widget build(BuildContext context) {
    final title = article.safeString('title');
    final sentiment = article.safeString('sentiment');
    final score = article.safeDouble('sentimentScore').clamp(-1.0, 1.0);
    final confidence = article.safeDouble('confidence').clamp(0.0, 1.0);
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
    final sentiment = data.safeString('sentiment');
    final score = data.safeDouble('score').clamp(-1.0, 1.0);
    
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
