import 'package:flutter/material.dart';
import '../../bearly_managed_provider.dart';
import '../../../../services/safe_cast.dart';

class GenerateStrategyDialog extends StatefulWidget {
  final BearlyManagedProvider provider;

  const GenerateStrategyDialog({
    super.key,
    required this.provider,
  });

  @override
  State<GenerateStrategyDialog> createState() => _GenerateStrategyDialogState();
}

class _GenerateStrategyDialogState extends State<GenerateStrategyDialog> {
  final _nameController = TextEditingController();
  final _symbolsController = TextEditingController();
  String _selectedType = 'Momentum';
  String _selectedTimeframe = '1D';
  String _selectedRiskLevel = 'Medium';
  String? _selectedProviderId;

  final List<String> _strategyTypes = [
    'Momentum',
    'Mean Reversion',
    'Trend Following',
    'Breakout',
    'Scalping',
    'Swing Trading',
  ];

  final List<String> _timeframes = [
    '1m',
    '5m',
    '15m',
    '1h',
    '4h',
    '1D',
    '1W',
  ];

  final List<String> _riskLevels = [
    'Low',
    'Medium',
    'High',
    'Very High',
  ];

  @override
  void initState() {
    super.initState();
    // Select first active provider as default
    final activeProviders = widget.provider.aiProviders
        .where((p) => p['isActive'] == true)
        .toList();
    if (activeProviders.isNotEmpty) {
      _selectedProviderId = activeProviders.first.safeString('id');
    }
  }

  @override
  Widget build(BuildContext context) {
    final activeProviders = widget.provider.aiProviders
        .where((p) => p['isActive'] == true)
        .toList();

    return AlertDialog(
      title: const Text('Generate AI Strategy'),
      content: SizedBox(
        width: 450,
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              TextField(
                controller: _nameController,
                decoration: const InputDecoration(
                  labelText: 'Strategy Name',
                  hintText: 'e.g., AAPL Momentum Strategy',
                  border: OutlineInputBorder(),
                ),
              ),
              const SizedBox(height: 16),
              DropdownButtonFormField<String>(
                value: _selectedType,
                decoration: const InputDecoration(
                  labelText: 'Strategy Type',
                  border: OutlineInputBorder(),
                ),
                items: _strategyTypes.map((type) {
                  return DropdownMenuItem(
                    value: type,
                    child: Text(type),
                  );
                }).toList(),
                onChanged: (value) {
                  if (value != null) {
                    setState(() {
                      _selectedType = value;
                    });
                  }
                },
              ),
              const SizedBox(height: 16),
              TextField(
                controller: _symbolsController,
                decoration: const InputDecoration(
                  labelText: 'Symbols',
                  hintText: 'AAPL, MSFT, GOOGL (comma-separated)',
                  border: OutlineInputBorder(),
                ),
              ),
              const SizedBox(height: 16),
              Row(
                children: [
                  Expanded(
                    child: DropdownButtonFormField<String>(
                      value: _selectedTimeframe,
                      decoration: const InputDecoration(
                        labelText: 'Timeframe',
                        border: OutlineInputBorder(),
                      ),
                      items: _timeframes.map((tf) {
                        return DropdownMenuItem(
                          value: tf,
                          child: Text(tf),
                        );
                      }).toList(),
                      onChanged: (value) {
                        if (value != null) {
                          setState(() {
                            _selectedTimeframe = value;
                          });
                        }
                      },
                    ),
                  ),
                  const SizedBox(width: 16),
                  Expanded(
                    child: DropdownButtonFormField<String>(
                      value: _selectedRiskLevel,
                      decoration: const InputDecoration(
                        labelText: 'Risk Level',
                        border: OutlineInputBorder(),
                      ),
                      items: _riskLevels.map((risk) {
                        return DropdownMenuItem(
                          value: risk,
                          child: Text(risk),
                        );
                      }).toList(),
                      onChanged: (value) {
                        if (value != null) {
                          setState(() {
                            _selectedRiskLevel = value;
                          });
                        }
                      },
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 16),
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
                      value: provider.safeString('id'),
                      child: Text(provider.safeString('name')),
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
          onPressed: activeProviders.isEmpty ? null : _generateStrategy,
          child: const Text('Generate Strategy'),
        ),
      ],
    );
  }

  void _generateStrategy() {
    final name = _nameController.text.trim();
    final symbolsText = _symbolsController.text.trim();

    if (name.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Please enter a strategy name')),
      );
      return;
    }

    if (symbolsText.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Please enter at least one symbol')),
      );
      return;
    }

    final symbols = symbolsText
        .split(',')
        .map((s) => s.trim().toUpperCase())
        .where((s) => s.isNotEmpty)
        .toList();

    if (_selectedProviderId == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Please select an AI provider')),
      );
      return;
    }

    widget.provider.generateStrategy(
      name: name,
      type: _selectedType,
      symbols: symbols,
      timeframe: _selectedTimeframe,
      riskLevel: _selectedRiskLevel,
      providerId: _selectedProviderId!,
    );

    Navigator.of(context).pop();
  }

  @override
  void dispose() {
    _nameController.dispose();
    _symbolsController.dispose();
    super.dispose();
  }
}
