import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';

class ExecutePromptDialog extends StatefulWidget {
  final BearlyManagedProvider provider;
  final Map<String, dynamic> prompt;

  const ExecutePromptDialog({
    super.key,
    required this.provider,
    required this.prompt,
  });

  @override
  State<ExecutePromptDialog> createState() => _ExecutePromptDialogState();
}

class _ExecutePromptDialogState extends State<ExecutePromptDialog> {
  final _symbolController = TextEditingController();
  String? _selectedProviderId;
  bool _isExecuting = false;

  @override
  void initState() {
    super.initState();
    // Select first active provider as default
    final activeProviders = widget.provider.aiProviders
        .where((p) => p['isActive'] == true)
        .toList();
    if (activeProviders.isNotEmpty) {
      _selectedProviderId = activeProviders.first['id'];
    }
  }

  @override
  Widget build(BuildContext context) {
    final activeProviders = widget.provider.aiProviders
        .where((p) => p['isActive'] == true)
        .toList();

    return AlertDialog(
      title: Text('Execute: ${widget.prompt['name']}'),
      content: SizedBox(
        width: 400,
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Prompt details
              Container(
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  color: Colors.blue.withOpacity(0.1),
                  borderRadius: BorderRadius.circular(8),
                  border: Border.all(color: Colors.blue.withOpacity(0.3)),
                ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'Category: ${widget.prompt['category']}',
                      style: const TextStyle(fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      widget.prompt['description'] ?? 'No description',
                      style: const TextStyle(color: Colors.grey),
                    ),
                  ],
                ),
              ),
              const SizedBox(height: 16),
              // Symbol input
              TextField(
                controller: _symbolController,
                decoration: const InputDecoration(
                  labelText: 'Symbol',
                  hintText: 'e.g., AAPL',
                  border: OutlineInputBorder(),
                  prefixIcon: Icon(Icons.show_chart),
                ),
                textCapitalization: TextCapitalization.characters,
              ),
              const SizedBox(height: 16),
              // Provider selection
              if (activeProviders.isEmpty)
                Container(
                  padding: const EdgeInsets.all(12),
                  decoration: BoxDecoration(
                    color: Colors.orange.withOpacity(0.1),
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: Colors.orange),
                  ),
                  child: const Row(
                    children: [
                      Icon(Icons.warning, color: Colors.orange),
                      SizedBox(width: 8),
                      Expanded(
                        child: Text(
                          'No active AI providers. Please configure and activate a provider first.',
                          style: TextStyle(color: Colors.orange),
                        ),
                      ),
                    ],
                  ),
                )
              else
                DropdownButtonFormField<String>(
                  value: _selectedProviderId,
                  decoration: const InputDecoration(
                    labelText: 'AI Provider',
                    border: OutlineInputBorder(),
                  ),
                  items: activeProviders.map((provider) {
                    return DropdownMenuItem(
                      value: provider['id'] as String,
                      child: Text(provider['name'] as String),
                    );
                  }).toList(),
                  onChanged: (value) {
                    if (value != null) {
                      setState(() {
                        _selectedProviderId = value;
                      });
                    }
                  },
                ),
              const SizedBox(height: 16),
              // Preview
              if (widget.prompt['prompt'] != null)
                Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text(
                      'Prompt Preview:',
                      style: TextStyle(fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(height: 8),
                    Container(
                      padding: const EdgeInsets.all(12),
                      decoration: BoxDecoration(
                        color: Colors.grey.withOpacity(0.1),
                        borderRadius: BorderRadius.circular(8),
                      ),
                      child: Text(
                        widget.prompt['prompt'].toString().length > 200
                            ? '${widget.prompt['prompt'].toString().substring(0, 200)}...'
                            : widget.prompt['prompt'].toString(),
                        style: const TextStyle(
                          fontSize: 12,
                          fontStyle: FontStyle.italic,
                        ),
                      ),
                    ),
                  ],
                ),
            ],
          ),
        ),
      ),
      actions: [
        TextButton(
          onPressed: _isExecuting ? null : () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton.icon(
          onPressed: (activeProviders.isEmpty || _isExecuting) ? null : _executePrompt,
          icon: _isExecuting
              ? const SizedBox(
                  width: 16,
                  height: 16,
                  child: CircularProgressIndicator(strokeWidth: 2),
                )
              : const Icon(Icons.play_arrow),
          label: Text(_isExecuting ? 'Executing...' : 'Execute'),
        ),
      ],
    );
  }

  void _executePrompt() async {
    final symbol = _symbolController.text.trim().toUpperCase();

    if (symbol.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Please enter a symbol')),
      );
      return;
    }

    if (_selectedProviderId == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Please select an AI provider')),
      );
      return;
    }

    setState(() {
      _isExecuting = true;
    });

    try {
      await widget.provider.executePrompt(
        promptId: widget.prompt['id'],
        symbol: symbol,
        providerId: _selectedProviderId!,
      );

      if (mounted) {
        Navigator.of(context).pop();
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Prompt executed successfully')),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Error: $e')),
        );
      }
    } finally {
      if (mounted) {
        setState(() {
          _isExecuting = false;
        });
      }
    }
  }

  @override
  void dispose() {
    _symbolController.dispose();
    super.dispose();
  }
}
