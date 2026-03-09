import 'package:flutter/material.dart';
import '../../paper_hands_provider.dart';
import '../dialogs/paper_positions_dialog.dart';

class PaperTradingControls extends StatelessWidget {
  final PaperHandsProvider provider;

  const PaperTradingControls({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF37474F),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'Trading Controls',
            style: TextStyle(
              color: Colors.white,
              fontSize: 16,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 16),
          // Quick Order Entry
          Row(
            children: [
              Expanded(
                child: TextField(
                  decoration: const InputDecoration(
                    labelText: 'Symbol',
                    hintText: 'e.g. AAPL',
                    border: OutlineInputBorder(),
                    labelStyle: TextStyle(color: Colors.white),
                  ),
                  style: const TextStyle(color: Colors.white),
                  onChanged: (value) => provider.setSymbol(value),
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: TextField(
                  decoration: const InputDecoration(
                    labelText: 'Quantity',
                    hintText: '0',
                    border: OutlineInputBorder(),
                    labelStyle: TextStyle(color: Colors.white),
                  ),
                  style: const TextStyle(color: Colors.white),
                  keyboardType: TextInputType.number,
                  onChanged: (value) =>
                      provider.setQuantity(double.tryParse(value) ?? 0.0),
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
          // Order Type Selection
          Row(
            children: [
              Expanded(
                child: DropdownButtonFormField<String>(
                  decoration: const InputDecoration(
                    labelText: 'Order Type',
                    border: OutlineInputBorder(),
                    labelStyle: TextStyle(color: Colors.white),
                  ),
                  value: provider.selectedOrderType,
                  items: ['Market', 'Limit', 'Stop', 'StopLimit'].map((type) {
                    return DropdownMenuItem(
                      value: type,
                      child: Text(type,
                          style: const TextStyle(color: Colors.white)),
                    );
                  }).toList(),
                  onChanged: (value) {
                    if (value != null) {
                      provider.setOrderType(value);
                    }
                  },
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: DropdownButtonFormField<String>(
                  decoration: const InputDecoration(
                    labelText: 'Side',
                    border: OutlineInputBorder(),
                    labelStyle: TextStyle(color: Colors.white),
                  ),
                  value: provider.selectedSide,
                  items: ['Buy', 'Sell'].map((side) {
                    return DropdownMenuItem(
                      value: side,
                      child: Text(side,
                          style: const TextStyle(color: Colors.white)),
                    );
                  }).toList(),
                  onChanged: (value) {
                    if (value != null) {
                      provider.setSide(value);
                    }
                  },
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
          // Price for limit orders
          if (provider.selectedOrderType == 'Limit' ||
              provider.selectedOrderType == 'StopLimit')
            TextField(
              decoration: const InputDecoration(
                labelText: 'Price',
                hintText: '0.00',
                border: OutlineInputBorder(),
                labelStyle: TextStyle(color: Colors.white),
              ),
              style: const TextStyle(color: Colors.white),
              keyboardType: TextInputType.number,
              onChanged: (value) => provider.setPrice(double.tryParse(value)),
            ),
          const SizedBox(height: 16),
          // Action Buttons
          Row(
            children: [
              Expanded(
                child: ElevatedButton(
                  onPressed: provider.canPlaceOrder
                      ? () => _placePaperOrder(context)
                      : null,
                  style: ElevatedButton.styleFrom(
                    backgroundColor: provider.selectedSide == 'Buy'
                        ? Colors.green
                        : Colors.red,
                    padding: const EdgeInsets.symmetric(vertical: 12),
                  ),
                  child: Text(
                    provider.selectedSide == 'Buy' ? 'Paper Buy' : 'Paper Sell',
                  ),
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: ElevatedButton(
                  onPressed: () => _showPositionsDialog(context),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Colors.blue,
                    padding: const EdgeInsets.symmetric(vertical: 12),
                  ),
                  child: const Text('Positions'),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  void _placePaperOrder(BuildContext context) {
    provider.placePaperOrder();

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('Paper ${provider.selectedSide} order placed'),
        backgroundColor: Colors.green,
      ),
    );
  }

  void _showPositionsDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => PaperPositionsDialog(provider: provider),
    );
  }
}
