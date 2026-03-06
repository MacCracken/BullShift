import 'package:flutter/material.dart';
import 'dart:math';
import 'chart_enums.dart';

/// Manages comparison symbol data generation and provides the dialog
/// for adding new comparison symbols.
class ComparisonChartHelper {
  static List<String> getDefaultComparisonSymbols() {
    return ['SPY', 'QQQ'];
  }

  static Map<String, List<PriceData>> generateComparisonData(
    List<String> comparisonSymbols,
    List<PriceData> chartData,
  ) {
    final comparisonData = <String, List<PriceData>>{};
    for (final symbol in comparisonSymbols) {
      final random = Random(symbol.hashCode);
      final data = <PriceData>[];

      double currentPrice = 100.0 + random.nextDouble() * 100;

      for (int i = 0; i < chartData.length; i++) {
        final change = (random.nextDouble() - 0.5) * 2.0;
        currentPrice += change;
        currentPrice = currentPrice.clamp(50.0, 200.0);

        data.add(
          PriceData(
            timestamp: chartData[i].timestamp,
            open: currentPrice - random.nextDouble() * 0.5,
            high: currentPrice + random.nextDouble() * 0.5,
            low: currentPrice - random.nextDouble() * 0.5,
            close: currentPrice,
            volume: 1000000 + random.nextInt(2000000),
          ),
        );
      }
      comparisonData[symbol] = data;
    }
    return comparisonData;
  }

  static void showAddComparisonDialog({
    required BuildContext context,
    required List<String> currentSymbols,
    required ValueChanged<String> onSymbolAdded,
  }) {
    showDialog(
      context: context,
      builder: (dialogContext) {
        String newSymbol = '';
        return AlertDialog(
          backgroundColor: const Color(0xFF263238),
          title: const Text(
            'Add Symbol to Compare',
            style: TextStyle(color: Colors.white),
          ),
          content: TextField(
            autofocus: true,
            style: const TextStyle(color: Colors.white),
            decoration: const InputDecoration(
              hintText: 'Enter symbol (e.g., SPY)',
              hintStyle: TextStyle(color: Colors.grey),
            ),
            onChanged: (value) => newSymbol = value.toUpperCase(),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(dialogContext),
              child: const Text('Cancel'),
            ),
            TextButton(
              onPressed: () {
                if (newSymbol.isNotEmpty &&
                    !currentSymbols.contains(newSymbol)) {
                  onSymbolAdded(newSymbol);
                }
                Navigator.pop(dialogContext);
              },
              child: const Text('Add'),
            ),
          ],
        );
      },
    );
  }
}
