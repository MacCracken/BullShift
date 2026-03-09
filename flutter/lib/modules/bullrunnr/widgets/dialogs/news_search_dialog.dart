import 'package:flutter/material.dart';
import '../../bullrunnr_provider.dart';

class NewsSearchDialog extends StatefulWidget {
  final BullRunnrProvider provider;

  const NewsSearchDialog({
    super.key,
    required this.provider,
  });

  @override
  State<NewsSearchDialog> createState() => _NewsSearchDialogState();
}

class _NewsSearchDialogState extends State<NewsSearchDialog> {
  final TextEditingController _searchController = TextEditingController();
  final TextEditingController _symbolController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Search News'),
      content: SizedBox(
        width: 400,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: _searchController,
              decoration: const InputDecoration(
                labelText: 'Search keywords',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _symbolController,
              decoration: const InputDecoration(
                labelText: 'Symbols (comma-separated)',
                border: OutlineInputBorder(),
              ),
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () {
            final keywords = _searchController.text.trim();
            final symbols = _symbolController.text
                .split(',')
                .map((s) => s.trim().toUpperCase())
                .where((s) => s.isNotEmpty)
                .toList();

            widget.provider.searchNews(keywords, symbols);
            Navigator.of(context).pop();
          },
          child: const Text('Search'),
        ),
      ],
    );
  }

  @override
  void dispose() {
    _searchController.dispose();
    _symbolController.dispose();
    super.dispose();
  }
}
