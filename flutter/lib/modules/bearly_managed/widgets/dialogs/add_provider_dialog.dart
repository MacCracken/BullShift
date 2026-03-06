import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';

class AddProviderDialog extends StatefulWidget {
  final BearlyManagedProvider provider;

  const AddProviderDialog({
    super.key,
    required this.provider,
  });

  @override
  State<AddProviderDialog> createState() => _AddProviderDialogState();
}

class _AddProviderDialogState extends State<AddProviderDialog> {
  final _nameController = TextEditingController();
  final _apiEndpointController = TextEditingController();
  final _modelNameController = TextEditingController();
  String _selectedType = 'OpenAI';

  final List<String> _providerTypes = [
    'OpenAI',
    'Anthropic',
    'Ollama',
    'SecureYeoman',
    'Local LLM',
    'Custom',
  ];

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Add AI Provider'),
      content: SizedBox(
        width: 400,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: _nameController,
              decoration: const InputDecoration(
                labelText: 'Provider Name',
                hintText: 'e.g., My OpenAI Account',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 16),
            DropdownButtonFormField<String>(
              value: _selectedType,
              decoration: const InputDecoration(
                labelText: 'Provider Type',
                border: OutlineInputBorder(),
              ),
              items: _providerTypes.map((type) {
                return DropdownMenuItem(
                  value: type,
                  child: Text(type),
                );
              }).toList(),
              onChanged: (value) {
                if (value != null) {
                  setState(() {
                    _selectedType = value;
                    _updateDefaultEndpoint();
                  });
                }
              },
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _apiEndpointController,
              decoration: const InputDecoration(
                labelText: 'API Endpoint',
                hintText: 'https://api.openai.com/v1',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _modelNameController,
              decoration: const InputDecoration(
                labelText: 'Model Name',
                hintText: 'gpt-4',
                border: OutlineInputBorder(),
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
        ElevatedButton(
          onPressed: _addProvider,
          child: const Text('Add Provider'),
        ),
      ],
    );
  }

  void _updateDefaultEndpoint() {
    switch (_selectedType) {
      case 'OpenAI':
        _apiEndpointController.text = 'https://api.openai.com/v1';
        _modelNameController.text = 'gpt-4';
        break;
      case 'Anthropic':
        _apiEndpointController.text = 'https://api.anthropic.com/v1';
        _modelNameController.text = 'claude-sonnet-4-6';
        break;
      case 'Ollama':
        _apiEndpointController.text = 'http://localhost:11434';
        _modelNameController.text = 'llama2';
        break;
      case 'SecureYeoman':
        _apiEndpointController.text = 'http://localhost:18789';
        _modelNameController.text = 'auto';
        break;
      case 'Local LLM':
        _apiEndpointController.text = 'http://localhost:8080';
        _modelNameController.text = 'local-model';
        break;
      case 'Custom':
        _apiEndpointController.text = '';
        _modelNameController.text = '';
        break;
    }
  }

  void _addProvider() {
    final name = _nameController.text.trim();
    final apiEndpoint = _apiEndpointController.text.trim();
    final modelName = _modelNameController.text.trim();

    if (name.isEmpty || apiEndpoint.isEmpty || modelName.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Please fill in all fields')),
      );
      return;
    }

    widget.provider.addProvider(
      name: name,
      type: _selectedType,
      apiEndpoint: apiEndpoint,
      modelName: modelName,
    );

    Navigator.of(context).pop();
  }

  @override
  void dispose() {
    _nameController.dispose();
    _apiEndpointController.dispose();
    _modelNameController.dispose();
    super.dispose();
  }
}
