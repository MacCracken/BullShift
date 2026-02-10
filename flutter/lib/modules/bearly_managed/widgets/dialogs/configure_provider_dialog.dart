import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';

class ConfigureProviderDialog extends StatelessWidget {
  final BearlyManagedProvider provider;
  final Map<String, dynamic> aiProvider;

  const ConfigureProviderDialog({
    super.key,
    required this.provider,
    required this.aiProvider,
  });

  @override
  Widget build(BuildContext context) {
    // TODO: Implement configure provider dialog
    return AlertDialog(
      title: Text('Configure ${aiProvider['name']}'),
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
