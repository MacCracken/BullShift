import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';

class ConfigureProviderDialog extends StatefulWidget {
  final BearlyManagedProvider provider;
  final Map<String, dynamic> aiProvider;

  const ConfigureProviderDialog({
    super.key,
    required this.provider,
    required this.aiProvider,
  });

  @override
  State<ConfigureProviderDialog> createState() => _ConfigureProviderDialogState();
}

class _ConfigureProviderDialogState extends State<ConfigureProviderDialog> {
  final _apiKeyController = TextEditingController();
  final _orgIdController = TextEditingController();
  bool _obscureApiKey = true;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text('Configure ${widget.aiProvider['name']}'),
      content: SizedBox(
        width: 400,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Provider Type: ${widget.aiProvider['type']}',
              style: const TextStyle(fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 8),
            Text('Endpoint: ${widget.aiProvider['apiEndpoint']}'),
            Text('Model: ${widget.aiProvider['modelName']}'),
            const SizedBox(height: 24),
            TextField(
              controller: _apiKeyController,
              obscureText: _obscureApiKey,
              decoration: InputDecoration(
                labelText: 'API Key',
                hintText: 'Enter your API key',
                border: const OutlineInputBorder(),
                suffixIcon: IconButton(
                  icon: Icon(_obscureApiKey ? Icons.visibility_off : Icons.visibility),
                  onPressed: () {
                    setState(() {
                      _obscureApiKey = !_obscureApiKey;
                    });
                  },
                ),
              ),
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _orgIdController,
              decoration: const InputDecoration(
                labelText: 'Organization ID (Optional)',
                hintText: 'org-...',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 16),
            if (widget.aiProvider['isConfigured'] == true)
              Container(
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  color: Colors.green.withOpacity(0.1),
                  borderRadius: BorderRadius.circular(8),
                  border: Border.all(color: Colors.green),
                ),
                child: const Row(
                  children: [
                    Icon(Icons.check_circle, color: Colors.green),
                    SizedBox(width: 8),
                    Text('Provider is configured', style: TextStyle(color: Colors.green)),
                  ],
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
        if (widget.aiProvider['isConfigured'] == true)
          ElevatedButton.icon(
            onPressed: _testProvider,
            icon: const Icon(Icons.network_check),
            label: const Text('Test Connection'),
          ),
        ElevatedButton(
          onPressed: _configureProvider,
          child: const Text('Save Configuration'),
        ),
      ],
    );
  }

  void _configureProvider() {
    final apiKey = _apiKeyController.text.trim();
    final orgId = _orgIdController.text.trim();

    if (apiKey.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Please enter an API key')),
      );
      return;
    }

    widget.provider.configureProvider(
      providerId: widget.aiProvider['id'],
      apiKey: apiKey,
      organizationId: orgId.isEmpty ? null : orgId,
    );

    Navigator.of(context).pop();
  }

  void _testProvider() {
    widget.provider.testProvider(widget.aiProvider['id']);
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Testing connection...')),
    );
  }

  @override
  void dispose() {
    _apiKeyController.dispose();
    _orgIdController.dispose();
    super.dispose();
  }
}
