import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';

class AddProviderDialog extends StatelessWidget {
  final BearlyManagedProvider provider;

  const AddProviderDialog({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    // TODO: Implement add provider dialog
    return AlertDialog(
      title: const Text('Add AI Provider'),
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
