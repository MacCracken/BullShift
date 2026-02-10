import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';
import '../cards/ai_provider_card.dart';
import '../dialogs/add_provider_dialog.dart';
import '../dialogs/configure_provider_dialog.dart';

class ProviderSetupPanel extends StatelessWidget {
  final BearlyManagedProvider provider;
  
  const ProviderSetupPanel({
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
              const Icon(Icons.smart_toy, color: Colors.white, size: 20),
              const SizedBox(width: 8),
              const Text(
                '🤖 AI Provider Setup',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const Spacer(),
              IconButton(
                icon: const Icon(Icons.add, color: Colors.white),
                onPressed: () => _showAddProviderDialog(context),
                tooltip: 'Add AI Provider',
              ),
            ],
          ),
          const SizedBox(height: 16),
          // Provider list
          Expanded(
            child: provider.aiProviders.isEmpty
                ? const Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(
                          Icons.smart_toy_outlined,
                          color: Colors.grey,
                          size: 48,
                        ),
                        SizedBox(height: 16),
                        Text(
                          'No AI providers configured',
                          style: TextStyle(color: Colors.grey),
                        ),
                        SizedBox(height: 8),
                        Text(
                          'Click + to add your first AI provider',
                          style: TextStyle(
                            color: Colors.grey,
                            fontSize: 12,
                          ),
                        ),
                      ],
                    ),
                  )
                : ListView.builder(
                    itemCount: provider.aiProviders.length,
                    itemBuilder: (context, index) {
                      final aiProvider = provider.aiProviders[index];
                      return AIProviderCard(
                        provider: aiProvider,
                        onConfigure: () => _showConfigureDialog(context, aiProvider),
                        onTest: () => _testProvider(context, aiProvider),
                        onToggle: () => _toggleProvider(context, aiProvider),
                        onDelete: () => _deleteProvider(context, aiProvider),
                      );
                    },
                  ),
          ),
        ],
      ),
    );
  }

  void _showAddProviderDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AddProviderDialog(provider: provider),
    );
  }

  void _showConfigureDialog(BuildContext context, Map<String, dynamic> aiProvider) {
    showDialog(
      context: context,
      builder: (context) => ConfigureProviderDialog(
        provider: provider,
        aiProvider: aiProvider,
      ),
    );
  }

  void _testProvider(BuildContext context, Map<String, dynamic> aiProvider) {
    provider.testProvider(aiProvider['id']);
  }

  void _toggleProvider(BuildContext context, Map<String, dynamic> aiProvider) {
    provider.toggleProvider(aiProvider['id'], !aiProvider['isActive']);
  }

  void _deleteProvider(BuildContext context, Map<String, dynamic> aiProvider) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete AI Provider'),
        content: Text('Are you sure you want to delete ${aiProvider['name']}?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              provider.deleteProvider(aiProvider['id']);
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
