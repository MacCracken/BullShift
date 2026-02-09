import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../../services/rust_trading_engine.dart';
import '../core_trading/trading_provider.dart';
import 'bearly_managed_provider.dart';

class BearlyManagedView extends StatelessWidget {
  const BearlyManagedView({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer<BearlyManagedProvider>(
      builder: (context, provider, child) {
        return Row(
          children: [
            // AI Provider Setup
            Expanded(
              flex: 1,
              child: ProviderSetupPanel(provider: provider),
            ),
            // Strategy Generation
            Expanded(
              flex: 1,
              child: StrategyGenerationPanel(provider: provider),
            ),
            // AI Prompt Management
            Expanded(
              flex: 1,
              child: PromptManagementPanel(provider: provider),
            ),
          ],
        );
      },
    );
  }
}

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

class AIProviderCard extends StatelessWidget {
  final Map<String, dynamic> provider;
  final VoidCallback onConfigure;
  final VoidCallback onTest;
  final VoidCallback onToggle;
  final VoidCallback onDelete;

  const AIProviderCard({
    super.key,
    required this.provider,
    required this.onConfigure,
    required this.onTest,
    required this.onToggle,
    required this.onDelete,
  });

  @override
  Widget build(BuildContext context) {
    final name = provider['name'] as String;
    final providerType = provider['type'] as String;
    final modelName = provider['modelName'] as String;
    final isConfigured = provider['isConfigured'] as bool;
    final isActive = provider['isActive'] as bool;
    final lastUsed = provider['lastUsed'] as String?;
    
    Color getProviderColor() {
      switch (providerType.toLowerCase()) {
        case 'openai': return Colors.green;
        case 'anthropic': return Colors.purple;
        case 'ollama': return Colors.orange;
        case 'local': return Colors.blue;
        default: return Colors.grey;
      }
    }

    return Card(
      color: const Color(0xFF37474F),
      margin: const EdgeInsets.only(bottom: 12),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: getProviderColor(),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Text(
                    providerType.toUpperCase(),
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                      fontSize: 10,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    name,
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                      fontSize: 16,
                    ),
                  ),
                ),
                Switch(
                  value: isActive,
                  onChanged: (_) => onToggle(),
                  activeColor: getProviderColor(),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Text(
              'Model: $modelName',
              style: const TextStyle(
                color: Colors.grey,
                fontSize: 12,
              ),
            ),
            const SizedBox(height: 4),
            Row(
              children: [
                Icon(
                  isConfigured ? Icons.check_circle : Icons.error,
                  color: isConfigured ? Colors.green : Colors.red,
                  size: 16,
                ),
                const SizedBox(width: 4),
                Text(
                  isConfigured ? 'Configured' : 'Not Configured',
                  style: TextStyle(
                    color: isConfigured ? Colors.green : Colors.red,
                    fontSize: 12,
                  ),
                ),
                if (lastUsed != null) ...[
                  const Spacer(),
                  Text(
                    'Last used: $lastUsed',
                    style: const TextStyle(
                      color: Colors.grey,
                      fontSize: 10,
                    ),
                  ),
                ],
              ],
            ),
            const SizedBox(height: 12),
            Row(
              children: [
                if (!isConfigured)
                  Expanded(
                    child: ElevatedButton(
                      onPressed: onConfigure,
                      style: ElevatedButton.styleFrom(
                        backgroundColor: Colors.blue,
                        padding: const EdgeInsets.symmetric(vertical: 8),
                      ),
                      child: const Text('Configure'),
                    ),
                  ),
                if (isConfigured) ...[
                  Expanded(
                    child: ElevatedButton(
                      onPressed: onTest,
                      style: ElevatedButton.styleFrom(
                        backgroundColor: Colors.green,
                        padding: const EdgeInsets.symmetric(vertical: 8),
                      ),
                      child: const Text('Test'),
                    ),
                  ),
                  const SizedBox(width: 8),
                  Expanded(
                    child: ElevatedButton(
                      onPressed: onConfigure,
                      style: ElevatedButton.styleFrom(
                        backgroundColor: Colors.orange,
                        padding: const EdgeInsets.symmetric(vertical: 8),
                      ),
                      child: const Text('Edit'),
                    ),
                  ),
                ],
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class StrategyGenerationPanel extends StatelessWidget {
  final BearlyManagedProvider provider;
  
  const StrategyGenerationPanel({
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
              const Icon(Icons.auto_graph, color: Colors.white, size: 20),
              const SizedBox(width: 8),
              const Text(
                '📈 AI Strategy Generation',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const Spacer(),
              IconButton(
                icon: const Icon(Icons.add, color: Colors.white),
                onPressed: () => _showGenerateStrategyDialog(context),
                tooltip: 'Generate New Strategy',
              ),
            ],
          ),
          const SizedBox(height: 16),
          // Strategy list
          Expanded(
            child: provider.tradingStrategies.isEmpty
                ? const Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(
                          Icons.auto_graph_outlined,
                          color: Colors.grey,
                          size: 48,
                        ),
                        SizedBox(height: 16),
                        Text(
                          'No AI strategies generated',
                          style: TextStyle(color: Colors.grey),
                        ),
                        SizedBox(height: 8),
                        Text(
                          'Use AI to generate trading strategies',
                          style: TextStyle(
                            color: Colors.grey,
                            fontSize: 12,
                          ),
                        ),
                      ],
                    ),
                  )
                : ListView.builder(
                    itemCount: provider.tradingStrategies.length,
                    itemBuilder: (context, index) {
                      final strategy = provider.tradingStrategies[index];
                      return StrategyCard(
                        strategy: strategy,
                        onView: () => _viewStrategy(context, strategy),
                        onActivate: () => _activateStrategy(context, strategy),
                        onDelete: () => _deleteStrategy(context, strategy),
                      );
                    },
                  ),
          ),
        ],
      ),
    );
  }

  void _showGenerateStrategyDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => GenerateStrategyDialog(provider: provider),
    );
  }

  void _viewStrategy(BuildContext context, Map<String, dynamic> strategy) {
    showDialog(
      context: context,
      builder: (context) => StrategyDetailsDialog(strategy: strategy),
    );
  }

  void _activateStrategy(BuildContext context, Map<String, dynamic> strategy) {
    provider.toggleStrategy(strategy['id'], !strategy['isActive']);
  }

  void _deleteStrategy(BuildContext context, Map<String, dynamic> strategy) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Strategy'),
        content: Text('Are you sure you want to delete ${strategy['name']}?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              provider.deleteStrategy(strategy['id']);
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

class StrategyCard extends StatelessWidget {
  final Map<String, dynamic> strategy;
  final VoidCallback onView;
  final VoidCallback onActivate;
  final VoidCallback onDelete;

  const StrategyCard({
    super.key,
    required this.strategy,
    required this.onView,
    required this.onActivate,
    required this.onDelete,
  });

  @override
  Widget build(BuildContext context) {
    final name = strategy['name'] as String;
    final description = strategy['description'] as String;
    final strategyType = strategy['type'] as String;
    final riskLevel = strategy['riskLevel'] as String;
    final isActive = strategy['isActive'] as bool;
    final winRate = (strategy['winRate'] as double?) ?? 0.0;
    
    Color getStrategyColor() {
      switch (strategyType.toLowerCase()) {
        case 'momentum': return Colors.blue;
        case 'meanreversion': return Colors.green;
        case 'breakout': return Colors.orange;
        case 'sentiment': return Colors.purple;
        default: return Colors.grey;
      }
    }

    Color getRiskColor() {
      switch (riskLevel.toLowerCase()) {
        case 'conservative': return Colors.green;
        case 'moderate': return Colors.yellow;
        case 'aggressive': return Colors.orange;
        case 'veryaggressive': return Colors.red;
        default: return Colors.grey;
      }
    }

    return Card(
      color: const Color(0xFF37474F),
      margin: const EdgeInsets.only(bottom: 12),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: getStrategyColor(),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Text(
                    strategyType.toUpperCase(),
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                      fontSize: 10,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    name,
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                      fontSize: 16,
                    ),
                  ),
                ),
                Switch(
                  value: isActive,
                  onChanged: (_) => onActivate(),
                  activeColor: getStrategyColor(),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Text(
              description,
              style: const TextStyle(
                color: Colors.grey,
                fontSize: 12,
              ),
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                  decoration: BoxDecoration(
                    color: getRiskColor().withOpacity(0.3),
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: getRiskColor().withOpacity(0.5)),
                  ),
                  child: Text(
                    riskLevel,
                    style: TextStyle(
                      color: getRiskColor(),
                      fontSize: 10,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                if (winRate > 0) ...[
                  Text(
                    'Win Rate: ${(winRate * 100).toInt()}%',
                    style: TextStyle(
                      color: winRate > 0.5 ? Colors.green : Colors.red,
                      fontSize: 12,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ],
                const Spacer(),
                IconButton(
                  icon: const Icon(Icons.info_outline, color: Colors.white, size: 16),
                  onPressed: onView,
                  tooltip: 'View Details',
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

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

class PromptCard extends StatelessWidget {
  final Map<String, dynamic> prompt;
  final VoidCallback onExecute;
  final VoidCallback onEdit;
  final VoidCallback onDelete;

  const PromptCard({
    super.key,
    required this.prompt,
    required this.onExecute,
    required this.onEdit,
    required this.onDelete,
  });

  @override
  Widget build(BuildContext context) {
    final name = prompt['name'] as String;
    final category = prompt['category'] as String;
    final template = prompt['template'] as String;
    final isSystemPrompt = prompt['isSystemPrompt'] as bool;
    
    Color getCategoryColor() {
      switch (category.toLowerCase()) {
        case 'marketanalysis': return Colors.blue;
        case 'strategygeneration': return Colors.green;
        case 'riskassessment': return Colors.red;
        case 'sentimentanalysis': return Colors.purple;
        case 'technicalanalysis': return Colors.orange;
        default: return Colors.grey;
      }
    }

    return Card(
      color: const Color(0xFF37474F),
      margin: const EdgeInsets.only(bottom: 12),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: getCategoryColor(),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Text(
                    category.replaceAll(' ', ''),
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                      fontSize: 10,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                if (isSystemPrompt)
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                    decoration: BoxDecoration(
                      color: Colors.orange.withOpacity(0.3),
                      borderRadius: BorderRadius.circular(8),
                      border: Border.all(color: Colors.orange.withOpacity(0.5)),
                    ),
                    child: const Text(
                      'SYSTEM',
                      style: TextStyle(
                        color: Colors.orange,
                        fontSize: 10,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                const Spacer(),
                PopupMenuButton<String>(
                  icon: const Icon(Icons.more_vert, color: Colors.white, size: 16),
                  itemBuilder: (context) => [
                    const PopupMenuItem(
                      value: 'execute',
                      child: Row(
                        children: [
                          Icon(Icons.play_arrow, size: 16),
                          SizedBox(width: 8),
                          Text('Execute'),
                        ],
                      ),
                    ),
                    const PopupMenuItem(
                      value: 'edit',
                      child: Row(
                        children: [
                          Icon(Icons.edit, size: 16),
                          SizedBox(width: 8),
                          Text('Edit'),
                        ],
                      ),
                    ),
                    const PopupMenuItem(
                      value: 'delete',
                      child: Row(
                        children: [
                          Icon(Icons.delete, size: 16, color: Colors.red),
                          SizedBox(width: 8),
                          Text('Delete', style: TextStyle(color: Colors.red)),
                        ],
                      ),
                    ),
                  ],
                  onSelected: (value) {
                    switch (value) {
                      case 'execute':
                        onExecute();
                        break;
                      case 'edit':
                        onEdit();
                        break;
                      case 'delete':
                        onDelete();
                        break;
                    }
                  },
                ),
              ],
            ),
            const SizedBox(height: 8),
            Text(
              name,
              style: const TextStyle(
                color: Colors.white,
                fontWeight: FontWeight.bold,
                fontSize: 16,
              ),
            ),
            const SizedBox(height: 4),
            Text(
              template,
              style: const TextStyle(
                color: Colors.grey,
                fontSize: 12,
              ),
              maxLines: 3,
              overflow: TextOverflow.ellipsis,
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Expanded(
                  child: ElevatedButton(
                    onPressed: onExecute,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: getCategoryColor(),
                      padding: const EdgeInsets.symmetric(vertical: 8),
                    ),
                    child: const Text('Execute'),
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

// Dialog classes would go here...
class AddProviderDialog extends StatelessWidget {
  final BearlyManagedProvider provider;
  
  const AddProviderDialog({super.key, required this.provider});

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Add AI Provider'),
      content: const Text('Add provider dialog implementation...'),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () {
            // TODO: Implement add provider logic
            Navigator.of(context).pop();
          },
          child: const Text('Add'),
        ),
      ],
    );
  }
}

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
    return AlertDialog(
      title: Text('Configure ${aiProvider['name']}'),
      content: const Text('Configure provider dialog implementation...'),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () {
            // TODO: Implement configure logic
            Navigator.of(context).pop();
          },
          child: const Text('Save'),
        ),
      ],
    );
  }
}

class GenerateStrategyDialog extends StatelessWidget {
  final BearlyManagedProvider provider;
  
  const GenerateStrategyDialog({super.key, required this.provider});

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Generate AI Strategy'),
      content: const Text('Generate strategy dialog implementation...'),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () {
            // TODO: Implement generate strategy logic
            Navigator.of(context).pop();
          },
          child: const Text('Generate'),
        ),
      ],
    );
  }
}

class StrategyDetailsDialog extends StatelessWidget {
  final Map<String, dynamic> strategy;
  
  const StrategyDetailsDialog({super.key, required this.strategy});

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(strategy['name']),
      content: SizedBox(
        width: 400,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Type: ${strategy['type']}'),
            const SizedBox(height: 8),
            Text('Risk Level: ${strategy['riskLevel']}'),
            const SizedBox(height: 8),
            Text('Description: ${strategy['description']}'),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Close'),
        ),
      ],
    );
  }
}

class AddPromptDialog extends StatelessWidget {
  final BearlyManagedProvider provider;
  
  const AddPromptDialog({super.key, required this.provider});

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Add AI Prompt'),
      content: const Text('Add prompt dialog implementation...'),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () {
            // TODO: Implement add prompt logic
            Navigator.of(context).pop();
          },
          child: const Text('Add'),
        ),
      ],
    );
  }
}

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
    return AlertDialog(
      title: Text('Execute ${prompt['name']}'),
      content: const Text('Execute prompt dialog implementation...'),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () {
            // TODO: Implement execute prompt logic
            Navigator.of(context).pop();
          },
          child: const Text('Execute'),
        ),
      ],
    );
  }
}

class EditPromptDialog extends StatelessWidget {
  final BearlyManagedProvider provider;
  final Map<String, dynamic> prompt;
  
  const EditPromptDialog({
    super.key,
    required this.provider,
    required this.prompt,
  });

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text('Edit ${prompt['name']}'),
      content: const Text('Edit prompt dialog implementation...'),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () {
            // TODO: Implement edit prompt logic
            Navigator.of(context).pop();
          },
          child: const Text('Save'),
        ),
      ],
    );
  }
}