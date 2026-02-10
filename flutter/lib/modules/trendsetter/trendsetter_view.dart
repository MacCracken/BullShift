import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../../services/rust_trading_engine.dart';
import '../core_trading/trading_provider.dart';
import '../watchlist/watchlist_provider.dart';
import 'trendsetter_provider.dart';

class TrendSetterView extends StatelessWidget {
  const TrendSetterView({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer2<TrendSetterProvider, WatchlistProvider>(
      builder: (context, trendProvider, watchlistProvider, child) {
        return Row(
          children: [
            // Momentum Scanner
            Expanded(
              flex: 2,
              child: MomentumScanner(
                provider: trendProvider,
                watchlistProvider: watchlistProvider,
              ),
            ),
            // Heat Map
            Expanded(
              flex: 1,
              child: HeatMapPanel(
                provider: trendProvider,
                watchlistProvider: watchlistProvider,
              ),
            ),
            // Shift Alerts
            Expanded(
              flex: 1,
              child: ShiftAlertsPanel(
                provider: trendProvider,
                watchlistProvider: watchlistProvider,
              ),
            ),
          ],
        );
      },
    );
  }
}

class MomentumScanner extends StatelessWidget {
  final TrendSetterProvider provider;
  final WatchlistProvider watchlistProvider;
  
  const MomentumScanner({
    super.key,
    required this.provider,
    required this.watchlistProvider,
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
                '🚀 Momentum Scanner',
                style: TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const Spacer(),
              IconButton(
                icon: const Icon(Icons.refresh, color: Colors.white),
                onPressed: () => provider.refreshMomentumData(),
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
                    labelText: 'Min Score',
                    border: OutlineInputBorder(),
                    labelStyle: TextStyle(color: Colors.white),
                  ),
                  value: provider.minScoreFilter.toString(),
                  items: ['0.5', '0.6', '0.7', '0.8', '0.9'].map((score) {
                    return DropdownMenuItem(
                      value: score,
                      child: Text(score, style: const TextStyle(color: Colors.white)),
                    );
                  }).toList(),
                  onChanged: (value) {
                    if (value != null) {
                      provider.setMinScoreFilter(double.parse(value));
                    }
                  },
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: DropdownButtonFormField<String>(
                  decoration: const InputDecoration(
                    labelText: 'Trend Strength',
                    border: OutlineInputBorder(),
                    labelStyle: TextStyle(color: Colors.white),
                  ),
                  value: provider.trendStrengthFilter,
                  items: ['All', 'Strong', 'Explosive'].map((strength) {
                    return DropdownMenuItem(
                      value: strength,
                      child: Text(strength, style: const TextStyle(color: Colors.white)),
                    );
                  }).toList(),
                  onChanged: (value) {
                    if (value != null) {
                      provider.setTrendStrengthFilter(value);
                    }
                  },
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          // Momentum stocks list
          Expanded(
            child: provider.isLoading
                ? const Center(
                    child: CircularProgressIndicator(color: Colors.white),
                  )
                : ListView.builder(
                    itemCount: provider.momentumStocks.length,
                   itemBuilder: (context, index) {
                     final stock = provider.momentumStocks[index];
                     return MomentumStockCard(
                       stock: stock,
                       watchlistProvider: watchlistProvider,
                     );
                   },
                  ),
          ),
        ],
      ),
    );
  }
}

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

class HeatMapPanel extends StatelessWidget {
  final TrendSetterProvider provider;
  final WatchlistProvider watchlistProvider;
  
  const HeatMapPanel({
    super.key,
    required this.provider,
    required this.watchlistProvider,
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
            '🔥 Market Heat Map',
            style: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 16),
          Expanded(
            child: GridView.builder(
              gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
                crossAxisCount: 3,
                childAspectRatio: 1.5,
                crossAxisSpacing: 8,
                mainAxisSpacing: 8,
              ),
              itemCount: provider.heatMapData.length,
              itemBuilder: (context, index) {
                final data = provider.heatMapData[index];
                return HeatMapTile(
                  data: data,
                  watchlistProvider: watchlistProvider,
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}

class HeatMapTile extends StatelessWidget {
  final Map<String, dynamic> data;
  final WatchlistProvider watchlistProvider;
  
  const HeatMapTile({
    super.key,
    required this.data,
    required this.watchlistProvider,
  });

  @override
  Widget build(BuildContext context) {
    final symbol = data['symbol'] as String;
    final heat = (data['heat'] as double).clamp(0.0, 1.0);
    
    Color getHeatColor() {
      if (heat >= 0.8) return Colors.red.shade900;
      if (heat >= 0.6) return Colors.red.shade700;
      if (heat >= 0.4) return Colors.orange.shade700;
      if (heat >= 0.2) return Colors.yellow.shade700;
      return Colors.green.shade700;
    }

    return InkWell(
      onTap: () => _toggleWatchlist(context, symbol),
      borderRadius: BorderRadius.circular(8),
      child: Container(
        decoration: BoxDecoration(
          color: getHeatColor(),
          borderRadius: BorderRadius.circular(8),
        ),
        child: Stack(
          children: [
            Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text(
                  symbol,
                  style: const TextStyle(
                    color: Colors.white,
                    fontWeight: FontWeight.bold,
                    fontSize: 12,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  '${(heat * 100).toInt()}%',
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 10,
                  ),
                ),
              ],
            ),
            if (watchlistProvider.isInWatchlist(symbol))
              const Positioned(
                top: 2,
                right: 2,
                child: Icon(
                  Icons.star,
                  color: Colors.yellow,
                  size: 12,
                ),
              ),
          ],
        ),
      ),
    );
  }
}

class ShiftAlertsPanel extends StatelessWidget {
  final TrendSetterProvider provider;
  final WatchlistProvider watchlistProvider;
  
  const ShiftAlertsPanel({
    super.key,
    required this.provider,
    required this.watchlistProvider,
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
                '⚡ Shift Alerts',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const Spacer(),
              Container(
                padding: const EdgeInsets.all(4),
                decoration: BoxDecoration(
                  color: Colors.red,
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Text(
                  '${provider.activeAlerts.length}',
                  style: const TextStyle(
                    color: Colors.white,
                    fontWeight: FontWeight.bold,
                    fontSize: 12,
                  ),
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          Expanded(
            child: ListView.builder(
              itemCount: provider.activeAlerts.length,
              itemBuilder: (context, index) {
                final alert = provider.activeAlerts[index];
                return ShiftAlertCard(
                  alert: alert,
                  watchlistProvider: watchlistProvider,
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}

class ShiftAlertCard extends StatelessWidget {
  final Map<String, dynamic> alert;
  final WatchlistProvider watchlistProvider;
  
  const ShiftAlertCard({
    super.key,
    required this.alert,
    required this.watchlistProvider,
  });

  @override
  Widget build(BuildContext context) {
    final symbol = alert['symbol'] as String;
    final message = alert['message'] as String;
    final alertType = alert['type'] as String;
    final confidence = (alert['confidence'] as double).clamp(0.0, 1.0);
    
    Color getAlertColor() {
      switch (alertType) {
        case 'VolumeSpike': return Colors.blue;
        case 'PriceBreakout': return Colors.green;
        case 'MomentumShift': return Colors.orange;
        case 'SocialBuzz': return Colors.purple;
        case 'TrendReversal': return Colors.red;
        default: return Colors.grey;
      }
    }

    return Card(
      color: const Color(0xFF37474F),
      margin: const EdgeInsets.only(bottom: 8),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Row(
          children: [
            Container(
              width: 4,
              height: 40,
              decoration: BoxDecoration(
                color: getAlertColor(),
                borderRadius: BorderRadius.circular(2),
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Text(
                        symbol,
                        style: const TextStyle(
                          color: Colors.white,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      if (watchlistProvider.isInWatchlist(symbol)) ...[
                        const SizedBox(width: 6),
                        const Icon(
                          Icons.star,
                          color: Colors.yellow,
                          size: 16,
                        ),
                      ],
                      const Spacer(),
                      Text(
                        '${(confidence * 100).toInt()}%',
                        style: TextStyle(
                          color: getAlertColor(),
                          fontSize: 12,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 4),
                  Text(
                    message,
                    style: const TextStyle(
                      color: Colors.grey,
                      fontSize: 12,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Row(
                    children: [
                      if (!watchlistProvider.isInWatchlist(symbol))
                        Expanded(
                          child: TextButton.icon(
                            onPressed: () => _addToWatchlist(context, symbol),
                            icon: const Icon(Icons.add, size: 12),
                            label: const Text('Watch', style: TextStyle(fontSize: 10)),
                            style: TextButton.styleFrom(
                              foregroundColor: Colors.green,
                              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                            ),
                          ),
                        ),
                    ],
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  void _addToWatchlist(BuildContext context, String symbol) async {
    try {
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
                    // Implement buy logic
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
                    // Implement sell logic
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