import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';

class AddPromptDialog extends StatefulWidget {
  final BearlyManagedProvider provider;

  const AddPromptDialog({
    super.key,
    required this.provider,
  });

  @override
  State<AddPromptDialog> createState() => _AddPromptDialogState();
}

class _AddPromptDialogState extends State<AddPromptDialog> {
  final _nameController = TextEditingController();
  final _promptController = TextEditingController();
  final _descriptionController = TextEditingController();
  String _selectedCategory = 'Analysis';

  final List<String> _categories = [
    'Analysis',
    'Strategy',
    'Risk Management',
    'Market Sentiment',
    'Trade Ideas',
    'Custom',
  ];

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Add AI Prompt'),
      content: SizedBox(
        width: 500,
        height: 400,
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              TextField(
                controller: _nameController,
                decoration: const InputDecoration(
                  labelText: 'Prompt Name',
                  hintText: 'e.g., Analyze Market Trend',
                  border: OutlineInputBorder(),
                ),
              ),
              const SizedBox(height: 16),
              DropdownButtonFormField<String>(
                value: _selectedCategory,
                decoration: const InputDecoration(
                  labelText: 'Category',
                  border: OutlineInputBorder(),
                ),
                items: _categories.map((category) {
                  return DropdownMenuItem(
                    value: category,
                    child: Text(category),
                  );
                }).toList(),
                onChanged: (value) {
                  if (value != null) {
                    setState(() {
                      _selectedCategory = value;
                    });
                  }
                },
              ),
              const SizedBox(height: 16),
              TextField(
                controller: _descriptionController,
                decoration: const InputDecoration(
                  labelText: 'Description (Optional)',
                  hintText: 'Brief description of what this prompt does',
                  border: OutlineInputBorder(),
                ),
                maxLines: 2,
              ),
              const SizedBox(height: 16),
              TextField(
                controller: _promptController,
                decoration: const InputDecoration(
                  labelText: 'Prompt Template',
                  hintText: 'Enter your prompt template here. Use {{symbol}} for symbol placeholders.',
                  border: OutlineInputBorder(),
                  alignLabelWithHint: true,
                ),
                maxLines: 6,
                textAlignVertical: TextAlignVertical.top,
              ),
              const SizedBox(height: 8),
              const Text(
                'Tip: Use {{symbol}} as a placeholder for stock symbols',
                style: TextStyle(
                  color: Colors.grey,
                  fontSize: 12,
                  fontStyle: FontStyle.italic,
                ),
              ),
            ],
          ),
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: _addPrompt,
          child: const Text('Add Prompt'),
        ),
      ],
    );
  }

  void _addPrompt() {
    final name = _nameController.text.trim();
    final prompt = _promptController.text.trim();
    final description = _descriptionController.text.trim();

    if (name.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Please enter a prompt name')),
      );
      return;
    }

    if (prompt.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Please enter a prompt template')),
      );
      return;
    }

    widget.provider.addPrompt(
      name: name,
      category: _selectedCategory,
      prompt: prompt,
      description: description.isEmpty ? null : description,
    );

    Navigator.of(context).pop();
  }

  @override
  void dispose() {
    _nameController.dispose();
    _promptController.dispose();
    _descriptionController.dispose();
    super.dispose();
  }
}
