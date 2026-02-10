import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';

class GenerateStrategyDialog extends StatelessWidget {
  final BearlyManagedProvider provider;

  const GenerateStrategyDialog({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    // TODO: Implement generate strategy dialog
    return AlertDialog(
      title: const Text('Generate AI Strategy'),
      content: const Text('Dialog implementation pending'),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Close'),
        ),
      ],
    );
  }
}
