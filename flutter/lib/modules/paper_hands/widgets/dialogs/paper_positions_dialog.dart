import 'package:flutter/material.dart';
import '../../paper_hands_provider.dart';

class PaperPositionsDialog extends StatelessWidget {
  final PaperHandsProvider provider;
  
  const PaperPositionsDialog({super.key, required this.provider});

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Current Positions'),
      content: const Text('Positions dialog implementation...'),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Close'),
        ),
      ],
    );
  }
}
