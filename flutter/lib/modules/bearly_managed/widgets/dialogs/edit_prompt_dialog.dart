import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';

class EditPromptDialog extends StatefulWidget {
  final BearlyManagedProvider provider;
  final Map<String, dynamic> prompt;

  const EditPromptDialog({
    super.key,
    required this.provider,
    required this.prompt,
  });

  @override
  State<EditPromptDialog> createState() => _EditPromptDialogState();
}

class _EditPromptDialogState extends State<EditPromptDialog> {
  late final TextEditingController _nameController;
  late final TextEditingController _promptController;
  late final TextEditingController _descriptionController;
  late String _selectedCategory;

  final List<String> _categories = [
    'Analysis',
    'Strategy',
    'Risk Management',
    'Market Sentiment',
    'Trade Ideas',
    'Custom',
  ];

  @override
  void initState() {
    super.initState();
    _nameController = TextEditingController(text: widget.prompt['name']);
    _promptController = TextEditingController(text: widget.prompt['prompt']);
    _descriptionController = TextEditingController(
      text: widget.prompt['description'] ?? '',
    );
    _selectedCategory = widget.prompt['category'] ?? 'Custom';
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text('Edit: ${widget.prompt['name']}'),
      content: SizedBox(
        width: 500,
        height: 450,
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              TextField(
                controller: _nameController,
                decoration: const InputDecoration(
                  labelText: 'Prompt Name',
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
              const SizedBox(height: 16),
              // Metadata
              if (widget.prompt['createdAt'] != null)
                Text(
                  'Created: ${_formatDateTime(widget.prompt['createdAt'])}',
                  style: const TextStyle(
                    color: Colors.grey,
                    fontSize: 12,
                  ),
                ),
              if (widget.prompt['lastUsed'] != null)
                Text(
                  'Last Used: ${_formatDateTime(widget.prompt['lastUsed'])}',
                  style: const TextStyle(
                    color: Colors.grey,
                    fontSize: 12,
                  ),
                ),
            ],
          ),
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => _deletePrompt(),
          style: TextButton.styleFrom(foregroundColor: Colors.red),
          child: const Text('Delete'),
        ),
        const Spacer(),
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: _savePrompt,
          child: const Text('Save Changes'),
        ),
      ],
    );
  }

  String _formatDateTime(dynamic dateTime) {
    if (dateTime is DateTime) {
      return '${dateTime.year}-${dateTime.month.toString().padLeft(2, '0')}-${dateTime.day.toString().padLeft(2, '0')}';
    }
    return 'Unknown';
  }

  void _savePrompt() {
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

    widget.provider.updatePrompt(
      promptId: widget.prompt['id'],
      name: name,
      category: _selectedCategory,
      prompt: prompt,
      description: description.isEmpty ? null : description,
    );

    Navigator.of(context).pop();
  }

  void _deletePrompt() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Prompt'),
        content: Text('Are you sure you want to delete "${widget.prompt['name']}"?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              widget.provider.deletePrompt(widget.prompt['id']);
              Navigator.of(context).pop(); // Close confirm dialog
              Navigator.of(context).pop(); // Close edit dialog
            },
            style: TextButton.styleFrom(foregroundColor: Colors.red),
            child: const Text('Delete'),
          ),
        ],
      ),
    );
  }

  @override
  void dispose() {
    _nameController.dispose();
    _promptController.dispose();
    _descriptionController.dispose();
    super.dispose();
  }
}
