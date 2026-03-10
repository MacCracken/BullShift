import 'package:flutter/material.dart';
import '../../../../services/safe_cast.dart';
import '../../../watchlist/watchlist_provider.dart';

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
    final symbol = data.safeString('symbol');
    final heat = data.safeDouble('heat').clamp(0.0, 1.0);

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
                  style: const TextStyle(color: Colors.white, fontSize: 10),
                ),
              ],
            ),
            if (watchlistProvider.isInWatchlist(symbol))
              const Positioned(
                top: 2,
                right: 2,
                child: Icon(Icons.star, color: Colors.yellow, size: 12),
              ),
          ],
        ),
      ),
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
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text('Error: $e')));
      }
    }
  }
}
