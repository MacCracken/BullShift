import 'package:flutter/material.dart';
import '../../../../services/safe_cast.dart';
import '../../watchlist_provider.dart';
import '../cards/search_result_tile.dart';

class SearchAndStatsPanel extends StatelessWidget {
  final WatchlistProvider provider;

  const SearchAndStatsPanel({super.key, required this.provider});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(flex: 1, child: SearchSection(provider: provider)),
        const SizedBox(height: 8),
        Expanded(flex: 1, child: StatsSection(provider: provider)),
      ],
    );
  }
}

class SearchSection extends StatelessWidget {
  final WatchlistProvider provider;

  const SearchSection({super.key, required this.provider});

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(left: 8, right: 8, top: 8),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF263238),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            '🔍 Search Symbols',
            style: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 16),
          TextField(
            onChanged: (value) => provider.searchSymbols(value),
            decoration: const InputDecoration(
              hintText: 'Enter symbol name...',
              border: OutlineInputBorder(),
              prefixIcon: Icon(Icons.search, color: Colors.white),
              hintStyle: TextStyle(color: Colors.grey),
            ),
            style: const TextStyle(color: Colors.white),
          ),
          const SizedBox(height: 16),
          Expanded(
            child: provider.isLoading
                ? const Center(
                    child: CircularProgressIndicator(color: Colors.white),
                  )
                : ListView.builder(
                    itemCount: provider.searchResults.length,
                    itemBuilder: (context, index) {
                      final result = provider.searchResults[index];
                      return SearchResultTile(
                        result: result,
                        onAdd: () => _addToWatchlist(context, result['symbol']),
                      );
                    },
                  ),
          ),
        ],
      ),
    );
  }

  void _addToWatchlist(BuildContext context, String symbol) async {
    final added = await Provider.of<WatchlistProvider>(
      context,
      listen: false,
    ).addToWatchlist(symbol);

    if (added) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('$symbol added to watchlist')));
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('$symbol is already in watchlist')),
      );
    }
  }
}

class StatsSection extends StatelessWidget {
  final WatchlistProvider provider;

  const StatsSection({super.key, required this.provider});

  @override
  Widget build(BuildContext context) {
    final stats = provider.getWatchlistStats();
    final topGainer = stats['topGainer'] != null ? stats.safeMap('topGainer') : null;
    final topLoser = stats['topLoser'] != null ? stats.safeMap('topLoser') : null;

    return Container(
      margin: const EdgeInsets.only(left: 8, right: 8, bottom: 8),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF263238),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            '📊 Watchlist Stats',
            style: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 16),
          Expanded(
            child: Column(
              children: [
                _buildStatRow('Total Symbols', '${stats['totalSymbols']}'),
                _buildStatRow(
                  'Day Change',
                  '${stats['dayChangePercent'].toStringAsFixed(2)}%',
                ),
                _buildStatRow(
                  'Total Value',
                  '\$${stats['totalValue'].toStringAsFixed(2)}',
                ),
                const Divider(color: Colors.grey),
                if (topGainer != null) _buildGainerLoserRow(topGainer, true),
                if (topLoser != null) _buildGainerLoserRow(topLoser, false),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildStatRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(label, style: const TextStyle(color: Colors.grey, fontSize: 14)),
          Text(
            value,
            style: const TextStyle(
              color: Colors.white,
              fontSize: 14,
              fontWeight: FontWeight.bold,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildGainerLoserRow(Map<String, dynamic> item, bool isGainer) {
    final symbol = item.safeString('symbol');
    final changePercent = item.safeDouble('dayChangePercent');

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(
            isGainer ? '🚀 Top Gainer' : '📉 Top Loser',
            style: const TextStyle(color: Colors.grey, fontSize: 12),
          ),
          Row(
            children: [
              Text(
                symbol,
                style: TextStyle(
                  color: isGainer ? Colors.green : Colors.red,
                  fontSize: 12,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(width: 8),
              Text(
                '${changePercent.toStringAsFixed(2)}%',
                style: TextStyle(
                  color: isGainer ? Colors.green : Colors.red,
                  fontSize: 12,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
