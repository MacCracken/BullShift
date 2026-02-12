import 'package:flutter/material.dart';
import '../../trendsetter_provider.dart';
import '../../../watchlist/watchlist_provider.dart';
import '../cards/momentum_stock_card.dart';

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
                      child: Text(
                        score,
                        style: const TextStyle(color: Colors.white),
                      ),
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
                      child: Text(
                        strength,
                        style: const TextStyle(color: Colors.white),
                      ),
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
