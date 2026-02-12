import 'package:flutter/material.dart';
import '../../trendsetter_provider.dart';
import '../../../watchlist/watchlist_provider.dart';
import '../cards/heat_map_tile.dart';

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
