import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../../services/rust_trading_engine.dart';
import 'watchlist_provider.dart';

class WatchlistView extends StatelessWidget {
  const WatchlistView({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer<WatchlistProvider>(
      builder: (context, provider, child) {
        return Row(
          children: [
            // Main Watchlist Panel
            Expanded(
              flex: 3,
              child: WatchlistPanel(provider: provider),
            ),
            // Search and Stats Panel
            Expanded(
              flex: 1,
              child: SearchAndStatsPanel(provider: provider),
            ),
          ],
        );
      },
    );
  }
}

class WatchlistPanel extends StatelessWidget {
  final WatchlistProvider provider;
  
  const WatchlistPanel({
    super.key,
    required this.provider,
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
          // Header
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
                  const PopupMenuItem(
                    value: 'clear',
                    child: Text('Clear All'),
                  ),
                ],
              ),
            ],
          ),
          const SizedBox(height: 16),
          
          // Sort controls
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
          
          // Watchlist items
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
                            Icon(
                              Icons.star_border,
                              size: 64,
                              color: Colors.grey,
                            ),
                            SizedBox(height: 16),
                            Text(
                              'Your watchlist is empty',
                              style: TextStyle(
                                color: Colors.grey,
                                fontSize: 18,
                              ),
                            ),
                            SizedBox(height: 8),
                            Text(
                              'Click the search icon to add symbols',
                              style: TextStyle(
                                color: Colors.grey,
                                fontSize: 14,
                              ),
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
                            onRemove: () => provider.removeFromWatchlist(item['symbol']),
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

  Widget _buildSortChip(String label, String value, WatchlistProvider provider) {
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
    showDialog(
      context: context,
      builder: (context) => const AddSymbolDialog(),
    );
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
    // Implementation for importing watchlist
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Import feature coming soon!')),
    );
  }

  void _exportWatchlist(BuildContext context) {
    // Implementation for exporting watchlist
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Export feature coming soon!')),
    );
  }

  void _showClearConfirmation(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Clear Watchlist'),
        content: const Text('Are you sure you want to remove all items from your watchlist?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              // Clear all items logic would go here
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
    // Implementation for trade dialog
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text('Trade dialog for ${item['symbol']} coming soon!')),
    );
  }

  void _showChartDialog(BuildContext context, Map<String, dynamic> item) {
    // Implementation for chart dialog
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text('Chart for ${item['symbol']} coming soon!')),
    );
  }
}

class WatchlistCard extends StatelessWidget {
  final Map<String, dynamic> item;
  final VoidCallback onRemove;
  final VoidCallback onTrade;
  final VoidCallback onChart;

  const WatchlistCard({
    super.key,
    required this.item,
    required this.onRemove,
    required this.onTrade,
    required this.onChart,
  });

  @override
  Widget build(BuildContext context) {
    final symbol = item['symbol'] as String;
    final currentPrice = item['currentPrice'] as double;
    final dayChange = item['dayChange'] as double;
    final dayChangePercent = item['dayChangePercent'] as double;
    final volume = item['volume'] as int;
    final isPositive = dayChange >= 0;

    return Card(
      color: const Color(0xFF37474F),
      margin: const EdgeInsets.only(bottom: 8),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            Row(
              children: [
                // Symbol and basic info
                Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      symbol,
                      style: const TextStyle(
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                        color: Colors.white,
                      ),
                    ),
                    Text(
                      'Vol: ${_formatVolume(volume)}',
                      style: const TextStyle(
                        color: Colors.grey,
                        fontSize: 12,
                      ),
                    ),
                  ],
                ),
                const Spacer(),
                // Price info
                Column(
                  crossAxisAlignment: CrossAxisAlignment.end,
                  children: [
                    Text(
                      '\$${currentPrice.toStringAsFixed(2)}',
                      style: const TextStyle(
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                        color: Colors.white,
                      ),
                    ),
                    Row(
                      children: [
                        Icon(
                          isPositive ? Icons.arrow_upward : Icons.arrow_downward,
                          size: 16,
                          color: isPositive ? Colors.green : Colors.red,
                        ),
                        const SizedBox(width: 4),
                        Text(
                          '${isPositive ? '+' : ''}${dayChange.toStringAsFixed(2)} (${dayChangePercent.toStringAsFixed(2)}%)',
                          style: TextStyle(
                            color: isPositive ? Colors.green : Colors.red,
                            fontSize: 14,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ],
                    ),
                  ],
                ),
              ],
            ),
            const SizedBox(height: 12),
            // Action buttons
            Row(
              children: [
                Expanded(
                  child: ElevatedButton.icon(
                    onPressed: onChart,
                    icon: const Icon(Icons.show_chart, size: 16),
                    label: const Text('Chart'),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.blue,
                      foregroundColor: Colors.white,
                      padding: const EdgeInsets.symmetric(vertical: 8),
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: ElevatedButton.icon(
                    onPressed: onTrade,
                    icon: const Icon(Icons.trending_up, size: 16),
                    label: const Text('Trade'),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.green,
                      foregroundColor: Colors.white,
                      padding: const EdgeInsets.symmetric(vertical: 8),
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                IconButton(
                  onPressed: () => _showRemoveConfirm(context),
                  icon: const Icon(Icons.remove_circle, color: Colors.red),
                  tooltip: 'Remove from watchlist',
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  String _formatVolume(int volume) {
    if (volume >= 1000000) {
      return '${(volume / 1000000).toStringAsFixed(1)}M';
    } else if (volume >= 1000) {
      return '${(volume / 1000).toStringAsFixed(1)}K';
    }
    return volume.toString();
  }

  void _showRemoveConfirm(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text('Remove ${item['symbol']}?'),
        content: Text('Are you sure you want to remove ${item['symbol']} from your watchlist?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              onRemove();
              Navigator.of(context).pop();
            },
            child: const Text('Remove', style: TextStyle(color: Colors.red)),
          ),
        ],
      ),
    );
  }
}

class SearchAndStatsPanel extends StatelessWidget {
  final WatchlistProvider provider;
  
  const SearchAndStatsPanel({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        // Search Section
        Expanded(
          flex: 1,
          child: SearchSection(provider: provider),
        ),
        const SizedBox(height: 8),
        // Stats Section
        Expanded(
          flex: 1,
          child: StatsSection(provider: provider),
        ),
      ],
    );
  }
}

class SearchSection extends StatelessWidget {
  final WatchlistProvider provider;
  
  const SearchSection({
    super.key,
    required this.provider,
  });

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
                ? const Center(child: CircularProgressIndicator(color: Colors.white))
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
    final added = await Provider.of<WatchlistProvider>(context, listen: false)
        .addToWatchlist(symbol);
    
    if (added) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('$symbol added to watchlist')),
      );
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('$symbol is already in watchlist')),
      );
    }
  }
}

class SearchResultTile extends StatelessWidget {
  final Map<String, dynamic> result;
  final VoidCallback onAdd;

  const SearchResultTile({
    super.key,
    required this.result,
    required this.onAdd,
  });

  @override
  Widget build(BuildContext context) {
    final symbol = result['symbol'] as String;
    final name = result['name'] as String? ?? '$symbol Inc.';
    final exchange = result['exchange'] as String? ?? 'NASDAQ';
    final type = result['type'] as String? ?? 'Stock';

    return Card(
      color: const Color(0xFF37474F),
      margin: const EdgeInsets.only(bottom: 8),
      child: ListTile(
        contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
        title: Text(
          symbol,
          style: const TextStyle(
            color: Colors.white,
            fontWeight: FontWeight.bold,
          ),
        ),
        subtitle: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              name,
              style: const TextStyle(color: Colors.grey, fontSize: 12),
            ),
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                  decoration: BoxDecoration(
                    color: Colors.blue.shade700,
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Text(
                    exchange,
                    style: const TextStyle(
                      color: Colors.white,
                      fontSize: 10,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
                const SizedBox(width: 6),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                  decoration: BoxDecoration(
                    color: Colors.purple.shade700,
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Text(
                    type,
                    style: const TextStyle(
                      color: Colors.white,
                      fontSize: 10,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
              ],
            ),
          ],
        ),
        trailing: IconButton(
          onPressed: onAdd,
          icon: const Icon(Icons.add_circle, color: Colors.green),
          tooltip: 'Add to watchlist',
        ),
      ),
    );
  }
}

class StatsSection extends StatelessWidget {
  final WatchlistProvider provider;
  
  const StatsSection({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    final stats = provider.getWatchlistStats();
    final topGainer = stats['topGainer'] as Map<String, dynamic>?;
    final topLoser = stats['topLoser'] as Map<String, dynamic>?;

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
                _buildStatRow('Day Change', '${stats['dayChangePercent'].toStringAsFixed(2)}%'),
                _buildStatRow('Total Value', '\$${stats['totalValue'].toStringAsFixed(2)}'),
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
          Text(
            label,
            style: const TextStyle(color: Colors.grey, fontSize: 14),
          ),
          Text(
            value,
            style: const TextStyle(color: Colors.white, fontSize: 14, fontWeight: FontWeight.bold),
          ),
        ],
      ),
    );
  }

  Widget _buildGainerLoserRow(Map<String, dynamic> item, bool isGainer) {
    final symbol = item['symbol'] as String;
    final changePercent = item['dayChangePercent'] as double;

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

class AddSymbolDialog extends StatefulWidget {
  const AddSymbolDialog({super.key});

  @override
  State<AddSymbolDialog> createState() => _AddSymbolDialogState();
}

class _AddSymbolDialogState extends State<AddSymbolDialog> {
  final _controller = TextEditingController();
  bool _isLoading = false;

  @override
  Widget build(BuildContext context) {
    return Consumer<WatchlistProvider>(
      builder: (context, provider, child) {
        return AlertDialog(
          title: const Text('Add to Watchlist'),
          content: SizedBox(
            width: 400,
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                TextField(
                  controller: _controller,
                  decoration: const InputDecoration(
                    labelText: 'Symbol',
                    hintText: 'e.g., AAPL, GOOGL, TSLA',
                    border: OutlineInputBorder(),
                  ),
                  autofocus: true,
                  onChanged: (value) => provider.searchSymbols(value),
                ),
                const SizedBox(height: 16),
                if (_controller.text.isNotEmpty) ...[
                  const Text('Search Results:', style: TextStyle(fontWeight: FontWeight.bold)),
                  const SizedBox(height: 8),
                  SizedBox(
                    height: 200,
                    child: _isLoading
                        ? const Center(child: CircularProgressIndicator())
                        : ListView.builder(
                            itemCount: provider.searchResults.length,
                            itemBuilder: (context, index) {
                              final result = provider.searchResults[index];
                              return SearchResultTile(
                                result: result,
                                onAdd: () async {
                                  final added = await provider.addToWatchlist(result['symbol']);
                                  if (added) {
                                    Navigator.of(context).pop();
                                    ScaffoldMessenger.of(context).showSnackBar(
                                      SnackBar(content: Text('${result['symbol']} added to watchlist')),
                                    );
                                  }
                                },
                              );
                            },
                          ),
                  ),
                ],
              ],
            ),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.of(context).pop(),
              child: const Text('Cancel'),
            ),
            if (_controller.text.isNotEmpty)
              TextButton(
                onPressed: _isLoading ? null : () async {
                  setState(() => _isLoading = true);
                  final added = await provider.addToWatchlist(_controller.text.toUpperCase());
                  setState(() => _isLoading = false);
                  
                  if (added) {
                    Navigator.of(context).pop();
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(content: Text('${_controller.text.toUpperCase()} added to watchlist')),
                    );
                  } else {
                    ScaffoldMessenger.of(context).showSnackBar(
                      const SnackBar(content: Text('Symbol already in watchlist or not found')),
                    );
                  }
                },
                child: _isLoading 
                    ? const SizedBox(
                        width: 16,
                        height: 16,
                        child: CircularProgressIndicator(strokeWidth: 2),
                      )
                    : const Text('Add'),
              ),
          ],
        );
      },
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }
}