import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';

class EditPromptDialog extends StatelessWidget {
  final BearlyManagedProvider provider;
  final Map<String, dynamic> prompt;

  const EditPromptDialog({
    super.key,
    required this.provider,
    required this.prompt,
  });

  @override
  Widget build(BuildContext context) {
    // TODO: Implement edit prompt dialog
    return AlertDialog(
      title: Text('Edit: ${prompt['name']}'),
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
