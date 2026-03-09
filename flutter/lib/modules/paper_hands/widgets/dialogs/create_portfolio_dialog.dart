import 'package:flutter/material.dart';
import '../../paper_hands_provider.dart';

class CreatePortfolioDialog extends StatefulWidget {
  final PaperHandsProvider provider;

  const CreatePortfolioDialog({super.key, required this.provider});

  @override
  State<CreatePortfolioDialog> createState() => _CreatePortfolioDialogState();
}

class _CreatePortfolioDialogState extends State<CreatePortfolioDialog> {
  final _nameController = TextEditingController();
  final _balanceController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Create Paper Portfolio'),
      content: SizedBox(
        width: 400,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: _nameController,
              decoration: const InputDecoration(
                labelText: 'Portfolio Name',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _balanceController,
              decoration: const InputDecoration(
                labelText: 'Initial Balance',
                border: OutlineInputBorder(),
                prefixText: '\$',
              ),
              keyboardType: TextInputType.number,
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
            final name = _nameController.text.trim();
            final balance = double.tryParse(_balanceController.text) ?? 10000.0;

            if (name.isNotEmpty) {
              widget.provider.createPortfolio(name, balance);
              Navigator.of(context).pop();
            }
          },
          child: const Text('Create'),
        ),
      ],
    );
  }

  @override
  void dispose() {
    _nameController.dispose();
    _balanceController.dispose();
    super.dispose();
  }
}
