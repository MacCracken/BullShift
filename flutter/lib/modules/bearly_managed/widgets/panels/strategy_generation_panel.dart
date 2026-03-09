import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';
import '../cards/strategy_card.dart';
import '../dialogs/generate_strategy_dialog.dart';
import '../dialogs/strategy_details_dialog.dart';

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
