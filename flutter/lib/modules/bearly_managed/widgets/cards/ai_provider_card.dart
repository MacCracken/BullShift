import 'package:flutter/material.dart';

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
