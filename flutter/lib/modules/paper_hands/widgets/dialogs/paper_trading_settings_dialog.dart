import 'package:flutter/material.dart';
import '../../paper_hands_provider.dart';

class PaperTradingSettingsDialog extends StatelessWidget {
  final PaperHandsProvider provider;

  const PaperTradingSettingsDialog({super.key, required this.provider});

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Paper Trading Settings'),
      content: const Text('Settings dialog implementation...'),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Save'),
        ),
      ],
    );
  }
}
