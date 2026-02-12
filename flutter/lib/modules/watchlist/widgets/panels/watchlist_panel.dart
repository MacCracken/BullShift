import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../../watchlist_provider.dart';
import '../cards/watchlist_card.dart';
import '../dialogs/add_symbol_dialog.dart';

class WatchlistPanel extends StatelessWidget {
  final WatchlistProvider provider;

  const WatchlistPanel({super.key, required this.provider});

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
                '⭐ Watchlist',
                style: TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const Spacer(),
              IconButton(
                icon: const Icon(Icons.refresh, color: Colors.white),
                onPressed: () => provider.refreshWatchlistPrices(),
                tooltip: 'Refresh Prices',
              ),
              IconButton(
                icon: const Icon(Icons.search, color: Colors.white),
                onPressed: () => _showAddSymbolDialog(context),
                tooltip: 'Add Symbol',
              ),
              PopupMenuButton<String>(
                icon: const Icon(Icons.more_vert, color: Colors.white),
                onSelected: (value) => _handleMenuAction(context, value),
                itemBuilder: (context) => [
                  const PopupMenuItem(
                    value: 'import',
                    child: Text('Import Watchlist'),
                  ),
                  const PopupMenuItem(
                    value: 'export',
                    child: Text('Export Watchlist'),
                  ),
                  const PopupMenuItem(value: 'clear', child: Text('Clear All')),
                ],
              ),
            ],
          ),
          const SizedBox(height: 16),
          Row(
            children: [
              const Text(
                'Sort by:',
                style: TextStyle(color: Colors.white, fontSize: 14),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: SingleChildScrollView(
                  scrollDirection: Axis.horizontal,
                  child: Row(
                    children: [
                      _buildSortChip('Symbol', 'symbol', provider),
                      const SizedBox(width: 8),
                      _buildSortChip('Price', 'price', provider),
                      const SizedBox(width: 8),
                      _buildSortChip('Change', 'change', provider),
                      const SizedBox(width: 8),
                      _buildSortChip('Volume', 'volume', provider),
                    ],
                  ),
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
                : provider.watchlist.isEmpty
                ? const Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(Icons.star_border, size: 64, color: Colors.grey),
                        SizedBox(height: 16),
                        Text(
                          'Your watchlist is empty',
                          style: TextStyle(color: Colors.grey, fontSize: 18),
                        ),
                        SizedBox(height: 8),
                        Text(
                          'Click the search icon to add symbols',
                          style: TextStyle(color: Colors.grey, fontSize: 14),
                        ),
                      ],
                    ),
                  )
                : ListView.builder(
                    itemCount: provider.watchlist.length,
                    itemBuilder: (context, index) {
                      final item = provider.watchlist[index];
                      return WatchlistCard(
                        item: item,
                        onRemove: () =>
                            provider.removeFromWatchlist(item['symbol']),
                        onTrade: () => _showTradeDialog(context, item),
                        onChart: () => _showChartDialog(context, item),
                      );
                    },
                  ),
          ),
        ],
      ),
    );
  }

  Widget _buildSortChip(
    String label,
    String value,
    WatchlistProvider provider,
  ) {
    final isSelected = provider.sortBy == value;
    return FilterChip(
      label: Text(
        label,
        style: TextStyle(
          color: isSelected ? Colors.black : Colors.white,
          fontSize: 12,
        ),
      ),
      selected: isSelected,
      onSelected: (_) => provider.setSortBy(value),
      backgroundColor: Colors.grey.shade700,
      selectedColor: Colors.green,
      avatar: isSelected && !provider.sortAscending
          ? const Icon(Icons.arrow_downward, size: 16, color: Colors.black)
          : isSelected && provider.sortAscending
          ? const Icon(Icons.arrow_upward, size: 16, color: Colors.black)
          : null,
    );
  }

  void _showAddSymbolDialog(BuildContext context) {
    showDialog(context: context, builder: (context) => const AddSymbolDialog());
  }

  void _handleMenuAction(BuildContext context, String action) {
    switch (action) {
      case 'import':
        _showImportDialog(context);
        break;
      case 'export':
        _exportWatchlist(context);
        break;
      case 'clear':
        _showClearConfirmation(context);
        break;
    }
  }

  void _showImportDialog(BuildContext context) {
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Import feature coming soon!')),
    );
  }

  void _exportWatchlist(BuildContext context) {
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Export feature coming soon!')),
    );
  }

  void _showClearConfirmation(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Clear Watchlist'),
        content: const Text(
          'Are you sure you want to remove all items from your watchlist?',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              Navigator.of(context).pop();
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(content: Text('Watchlist cleared')),
              );
            },
            child: const Text('Clear', style: TextStyle(color: Colors.red)),
          ),
        ],
      ),
    );
  }

  void _showTradeDialog(BuildContext context, Map<String, dynamic> item) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('Trade dialog for ${item['symbol']} coming soon!'),
      ),
    );
  }

  void _showChartDialog(BuildContext context, Map<String, dynamic> item) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text('Chart for ${item['symbol']} coming soon!')),
    );
  }
}
