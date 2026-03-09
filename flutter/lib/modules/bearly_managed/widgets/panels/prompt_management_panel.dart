import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';
import '../cards/prompt_card.dart';
import '../dialogs/add_prompt_dialog.dart';
import '../dialogs/edit_prompt_dialog.dart';
import '../dialogs/execute_prompt_dialog.dart';

class PromptManagementPanel extends StatelessWidget {
  final BearlyManagedProvider provider;

  const PromptManagementPanel({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.all(8),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF263238),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              const Icon(Icons.psychology, color: Colors.white, size: 20),
              const SizedBox(width: 8),
              const Text(
                '🧠 AI Prompt Management',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const Spacer(),
              IconButton(
                icon: const Icon(Icons.add, color: Colors.white),
                onPressed: () => _showAddPromptDialog(context),
                tooltip: 'Add Prompt',
              ),
            ],
          ),
          const SizedBox(height: 16),
          // Prompt list
          Expanded(
            child: provider.aiPrompts.isEmpty
                ? const Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(
                          Icons.psychology_outlined,
                          color: Colors.grey,
                          size: 48,
                        ),
                        SizedBox(height: 16),
                        Text(
                          'No AI prompts configured',
                          style: TextStyle(color: Colors.grey),
                        ),
                        SizedBox(height: 8),
                        Text(
                          'Create prompts for AI interactions',
                          style: TextStyle(
                            color: Colors.grey,
                            fontSize: 12,
                          ),
                        ),
                      ],
                    ),
                  )
                : ListView.builder(
                    itemCount: provider.aiPrompts.length,
                    itemBuilder: (context, index) {
                      final prompt = provider.aiPrompts[index];
                      return PromptCard(
                        prompt: prompt,
                        onExecute: () => _executePrompt(context, prompt),
                        onEdit: () => _editPrompt(context, prompt),
                        onDelete: () => _deletePrompt(context, prompt),
                      );
                    },
                  ),
          ),
        ],
      ),
    );
  }

  void _showAddPromptDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AddPromptDialog(provider: provider),
    );
  }

  void _executePrompt(BuildContext context, Map<String, dynamic> prompt) {
    showDialog(
      context: context,
      builder: (context) => ExecutePromptDialog(
        provider: provider,
        prompt: prompt,
      ),
    );
  }

  void _editPrompt(BuildContext context, Map<String, dynamic> prompt) {
    showDialog(
      context: context,
      builder: (context) => EditPromptDialog(
        provider: provider,
        prompt: prompt,
      ),
    );
  }

  void _deletePrompt(BuildContext context, Map<String, dynamic> prompt) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Prompt'),
        content: Text('Are you sure you want to delete ${prompt['name']}?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              provider.deletePrompt(prompt['id']);
              Navigator.of(context).pop();
            },
            style: TextButton.styleFrom(foregroundColor: Colors.red),
            child: const Text('Delete'),
          ),
        ],
      ),
    );
  }
}
