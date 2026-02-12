import 'package:flutter/material.dart';
import '../../../watchlist/watchlist_provider.dart';

class MomentumStockCard extends StatelessWidget {
  final Map<String, dynamic> stock;
  final WatchlistProvider watchlistProvider;
  
  const MomentumStockCard({
    super.key,
    required this.stock,
    required this.watchlistProvider,
  });

  @override
  Widget build(BuildContext context) {
    final symbol = stock['symbol'] as String;
    final score = (stock['score'] as double).clamp(0.0, 1.0);
    final volumeSpike = stock['volumeSpike'] as double;
    final priceMomentum = stock['priceMomentum'] as double;
    final socialSentiment = stock['socialSentiment'] as double;
    final trendStrength = stock['trendStrength'] as String;
    
    Color getScoreColor() {
      if (score >= 0.8) return Colors.red;
      if (score >= 0.6) return Colors.orange;
      if (score >= 0.4) return Colors.yellow;
      return Colors.green;
    }

    Color getTrendColor() {
      switch (trendStrength) {
        case 'Explosive': return Colors.red;
        case 'Strong': return Colors.orange;
        case 'Moderate': return Colors.yellow;
        case 'Weak': return Colors.green;
        default: return Colors.grey;
      }
    }

    return Card(
      color: const Color(0xFF37474F),
      margin: const EdgeInsets.only(bottom: 8),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          children: [
            Row(
              children: [
                Text(
                  symbol,
                  style: const TextStyle(
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                const Spacer(),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: getScoreColor(),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Text(
                    '${(score * 100).toInt()}%',
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                      fontSize: 12,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: getTrendColor(),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Text(
                    trendStrength,
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                      fontSize: 12,
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                _buildMetricBar('Volume', volumeSpike, Colors.blue),
                const SizedBox(width: 12),
                _buildMetricBar('Price', priceMomentum, Colors.green),
                const SizedBox(width: 12),
                _buildMetricBar('Social', socialSentiment, Colors.purple),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Expanded(
                  child: ElevatedButton.icon(
                    onPressed: () => _toggleWatchlist(context, symbol),
                    icon: Icon(
                      watchlistProvider.isInWatchlist(symbol) 
                          ? Icons.star 
                          : Icons.star_border,
                      size: 16,
                    ),
                    label: Text(
                      watchlistProvider.isInWatchlist(symbol) 
                          ? 'Watching' 
                          : 'Watch',
                    ),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: watchlistProvider.isInWatchlist(symbol) 
                          ? Colors.orange 
                          : Colors.grey,
                      foregroundColor: Colors.white,
                      padding: const EdgeInsets.symmetric(vertical: 8),
                    ),
                  ),
                ),
                const SizedBox(width: 4),
                Expanded(
                  child: ElevatedButton(
                    onPressed: () => _showQuickTradeDialog(context, symbol),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.green,
                      padding: const EdgeInsets.symmetric(vertical: 8),
                    ),
                    child: const Text('Trade'),
                  ),
                ),
                const SizedBox(width: 4),
                Expanded(
                  child: ElevatedButton(
                    onPressed: () => _showAnalysisDialog(context, stock),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.blue,
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

  Widget _buildMetricBar(String label, double value, Color color) {
    final normalizedValue = value.clamp(-1.0, 1.0);
    final displayValue = (normalizedValue.abs() * 100).toInt();
    
    return Expanded(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            '$label: $displayValue%',
            style: const TextStyle(
              color: Colors.white,
              fontSize: 12,
            ),
          ),
          const SizedBox(height: 4),
          Container(
            height: 6,
            decoration: BoxDecoration(
              color: Colors.grey.shade700,
              borderRadius: BorderRadius.circular(3),
            ),
            child: FractionallySizedBox(
              alignment: normalizedValue >= 0 ? Alignment.centerLeft : Alignment.centerRight,
              widthFactor: normalizedValue.abs(),
              child: Container(
                decoration: BoxDecoration(
                  color: color,
                  borderRadius: BorderRadius.circular(3),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }

  void _showQuickTradeDialog(BuildContext context, String symbol) {
    showDialog(
      context: context,
      builder: (context) => QuickTradeDialog(symbol: symbol),
    );
  }

  void _showAnalysisDialog(BuildContext context, Map<String, dynamic> stock) {
    showDialog(
      context: context,
      builder: (context) => AnalysisDialog(stock: stock),
    );
  }

  void _toggleWatchlist(BuildContext context, String symbol) async {
    try {
      if (watchlistProvider.isInWatchlist(symbol)) {
        await watchlistProvider.removeFromWatchlist(symbol);
        if (context.mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('$symbol removed from watchlist')),
          );
        }
      } else {
        final added = await watchlistProvider.addToWatchlist(symbol);
        if (context.mounted) {
          if (added) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text('$symbol added to watchlist')),
            );
          } else {
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(content: Text('Failed to add to watchlist')),
            );
          }
        }
      }
    } catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Error: $e')),
        );
      }
    }
  }
}

class QuickTradeDialog extends StatelessWidget {
  final String symbol;
  
  const QuickTradeDialog({
    super.key,
    required this.symbol,
  });

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text('Quick Trade - $symbol'),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          const TextField(
            decoration: InputDecoration(
              labelText: 'Quantity',
              border: OutlineInputBorder(),
            ),
            keyboardType: TextInputType.number,
          ),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(
                child: ElevatedButton(
                  onPressed: () {
                    Navigator.of(context).pop();
                  },
                  style: ElevatedButton.styleFrom(backgroundColor: Colors.green),
                  child: const Text('BUY'),
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: ElevatedButton(
                  onPressed: () {
                    Navigator.of(context).pop();
                  },
                  style: ElevatedButton.styleFrom(backgroundColor: Colors.red),
                  child: const Text('SELL'),
                ),
              ),
            ],
          ),
        ],
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
      ],
    );
  }
}

class AnalysisDialog extends StatelessWidget {
  final Map<String, dynamic> stock;
  
  const AnalysisDialog({
    super.key,
    required this.stock,
  });

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text('Analysis - ${stock['symbol']}'),
      content: SizedBox(
        width: 400,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Overall Score: ${(stock['score'] as double * 100).toInt()}%'),
            const SizedBox(height: 8),
            Text('Volume Spike: ${stock['volumeSpike'].toStringAsFixed(2)}x'),
            const SizedBox(height: 8),
            Text('Price Momentum: ${(stock['priceMomentum'] as double * 100).toInt()}%'),
            const SizedBox(height: 8),
            Text('Social Sentiment: ${(stock['socialSentiment'] as double * 100).toInt()}%'),
            const SizedBox(height: 8),
            Text('Trend Strength: ${stock['trendStrength']}'),
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
}
