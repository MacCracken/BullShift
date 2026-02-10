import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';

class ExecutePromptDialog extends StatelessWidget {
  final BearlyManagedProvider provider;
  final Map<String, dynamic> prompt;

  const ExecutePromptDialog({
    super.key,
    required this.provider,
    required this.prompt,
  });

  @override
  Widget build(BuildContext context) {
    // TODO: Implement execute prompt dialog
    return AlertDialog(
      title: Text('Execute: ${prompt['name']}'),
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
