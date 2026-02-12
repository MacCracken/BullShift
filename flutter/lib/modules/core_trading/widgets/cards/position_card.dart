import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../../trading_provider.dart';
import '../../../watchlist/watchlist_provider.dart';

class PositionCard extends StatelessWidget {
  final Map<String, dynamic> position;

  const PositionCard({super.key, required this.position});

  @override
  Widget build(BuildContext context) {
    final pnl = position['unrealizedPnl'] as double;
    final pnlColor = pnl >= 0 ? Colors.green : Colors.red;
    final tradingProvider = context.read<TradingProvider>();
    final watchlistProvider = context.read<WatchlistProvider>();

    return Card(
      color: const Color(0xFF37474F),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Text(
                  position['symbol'] as String,
                  style: const TextStyle(
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                    fontSize: 16,
                  ),
                ),
                const Spacer(),
                Text(
                  '${position['quantity']}',
                  style: const TextStyle(color: Colors.white, fontSize: 14),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Text(
                  '\$${position['currentPrice'].toStringAsFixed(2)}',
                  style: const TextStyle(color: Colors.white, fontSize: 14),
                ),
                const Spacer(),
                Text(
                  '\$${pnl.toStringAsFixed(2)}',
                  style: TextStyle(
                    color: pnlColor,
                    fontWeight: FontWeight.bold,
                    fontSize: 14,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            if (tradingProvider.currentSymbol.isNotEmpty)
              Row(
                children: [
                  Expanded(
                    child:
                        watchlistProvider.isInWatchlist(
                          tradingProvider.currentSymbol,
                        )
                        ? ElevatedButton.icon(
                            onPressed: () async {
                              await watchlistProvider.removeFromWatchlist(
                                tradingProvider.currentSymbol,
                              );
                              if (context.mounted) {
                                ScaffoldMessenger.of(context).showSnackBar(
                                  SnackBar(
                                    content: Text(
                                      '${tradingProvider.currentSymbol} removed from watchlist',
                                    ),
                                  ),
                                );
                              }
                            },
                            icon: const Icon(Icons.star, color: Colors.yellow),
                            label: Text(
                              'Watching ${tradingProvider.currentSymbol}',
                            ),
                            style: ElevatedButton.styleFrom(
                              backgroundColor: Colors.orange,
                              foregroundColor: Colors.white,
                            ),
                          )
                        : ElevatedButton.icon(
                            onPressed: () async {
                              final added = await watchlistProvider
                                  .addToWatchlist(
                                    tradingProvider.currentSymbol,
                                  );
                              if (context.mounted) {
                                if (added) {
                                  ScaffoldMessenger.of(context).showSnackBar(
                                    SnackBar(
                                      content: Text(
                                        '${tradingProvider.currentSymbol} added to watchlist',
                                      ),
                                    ),
                                  );
                                } else {
                                  ScaffoldMessenger.of(context).showSnackBar(
                                    const SnackBar(
                                      content: Text(
                                        'Failed to add to watchlist',
                                      ),
                                    ),
                                  );
                                }
                              }
                            },
                            icon: const Icon(Icons.star_border),
                            label: Text(
                              'Add ${tradingProvider.currentSymbol} to Watchlist',
                            ),
                            style: ElevatedButton.styleFrom(
                              backgroundColor: Colors.grey,
                              foregroundColor: Colors.white,
                            ),
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
