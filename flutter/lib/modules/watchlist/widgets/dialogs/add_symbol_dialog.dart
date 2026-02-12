import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../../watchlist_provider.dart';
import '../cards/search_result_tile.dart';

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
                  const Text(
                    'Search Results:',
                    style: TextStyle(fontWeight: FontWeight.bold),
                  ),
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
                                  final added = await provider.addToWatchlist(
                                    result['symbol'],
                                  );
                                  if (added) {
                                    Navigator.of(context).pop();
                                    ScaffoldMessenger.of(context).showSnackBar(
                                      SnackBar(
                                        content: Text(
                                          '${result['symbol']} added to watchlist',
                                        ),
                                      ),
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
                onPressed: _isLoading
                    ? null
                    : () async {
                        setState(() => _isLoading = true);
                        final added = await provider.addToWatchlist(
                          _controller.text.toUpperCase(),
                        );
                        setState(() => _isLoading = false);

                        if (added) {
                          Navigator.of(context).pop();
                          ScaffoldMessenger.of(context).showSnackBar(
                            SnackBar(
                              content: Text(
                                '${_controller.text.toUpperCase()} added to watchlist',
                              ),
                            ),
                          );
                        } else {
                          ScaffoldMessenger.of(context).showSnackBar(
                            const SnackBar(
                              content: Text(
                                'Symbol already in watchlist or not found',
                              ),
                            ),
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
