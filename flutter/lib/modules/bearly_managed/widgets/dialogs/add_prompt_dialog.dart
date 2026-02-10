import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';

class AddPromptDialog extends StatelessWidget {
  final BearlyManagedProvider provider;

  const AddPromptDialog({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    // TODO: Implement add prompt dialog
    return AlertDialog(
      title: const Text('Add AI Prompt'),
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
