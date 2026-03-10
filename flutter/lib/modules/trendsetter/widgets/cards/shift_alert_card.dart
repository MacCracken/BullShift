import 'package:flutter/material.dart';
import '../../../../services/safe_cast.dart';
import '../../../watchlist/watchlist_provider.dart';

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
    final symbol = alert.safeString('symbol');
    final message = alert.safeString('message');
    final alertType = alert.safeString('type');
    final confidence = alert.safeDouble('confidence').clamp(0.0, 1.0);

    Color getAlertColor() {
      switch (alertType) {
        case 'VolumeSpike':
          return Colors.blue;
        case 'PriceBreakout':
          return Colors.green;
        case 'MomentumShift':
          return Colors.orange;
        case 'SocialBuzz':
          return Colors.purple;
        case 'TrendReversal':
          return Colors.red;
        default:
          return Colors.grey;
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
                        const Icon(Icons.star, color: Colors.yellow, size: 16),
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
                    style: const TextStyle(color: Colors.grey, fontSize: 12),
                  ),
                  const SizedBox(height: 4),
                  Row(
                    children: [
                      if (!watchlistProvider.isInWatchlist(symbol))
                        Expanded(
                          child: TextButton.icon(
                            onPressed: () => _addToWatchlist(context, symbol),
                            icon: const Icon(Icons.add, size: 12),
                            label: const Text(
                              'Watch',
                              style: TextStyle(fontSize: 10),
                            ),
                            style: TextButton.styleFrom(
                              foregroundColor: Colors.green,
                              padding: const EdgeInsets.symmetric(
                                horizontal: 8,
                                vertical: 4,
                              ),
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
          ScaffoldMessenger.of(
            context,
          ).showSnackBar(SnackBar(content: Text('$symbol added to watchlist')));
        } else {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('Failed to add to watchlist')),
          );
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
