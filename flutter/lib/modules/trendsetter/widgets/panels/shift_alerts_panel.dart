import 'package:flutter/material.dart';
import '../../trendsetter_provider.dart';
import '../../../watchlist/watchlist_provider.dart';
import '../cards/shift_alert_card.dart';

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
