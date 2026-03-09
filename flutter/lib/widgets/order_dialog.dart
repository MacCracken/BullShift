import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../modules/core_trading/trading_provider.dart';

class OrderDialog extends StatefulWidget {
  const OrderDialog({super.key});

  @override
  State<OrderDialog> createState() => _OrderDialogState();
}

class _OrderDialogState extends State<OrderDialog> {
  final _symbolController = TextEditingController();
  final _quantityController = TextEditingController();
  final _priceController = TextEditingController();
  String _orderType = 'MARKET';
  String _orderSide = 'BUY';

  @override
  Widget build(BuildContext context) {
    return Consumer<TradingProvider>(
      builder: (context, tradingProvider, child) {
        return AlertDialog(
          title: const Text('Submit Order'),
          content: SizedBox(
            width: 400,
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                TextField(
                  controller: _symbolController,
                  decoration: const InputDecoration(
                    labelText: 'Symbol',
                    hintText: 'e.g. AAPL',
                    border: OutlineInputBorder(),
                  ),
                ),
                const SizedBox(height: 12),
                TextField(
                  controller: _quantityController,
                  decoration: const InputDecoration(
                    labelText: 'Quantity',
                    hintText: '0.00',
                    border: OutlineInputBorder(),
                  ),
                  keyboardType: TextInputType.number,
                ),
                const SizedBox(height: 12),
                Row(
                  children: [
                    const Text('Side: '),
                    Expanded(
                      child: DropdownButton<String>(
                        value: _orderSide,
                        items: const [
                          DropdownMenuItem(value: 'BUY', child: Text('BUY')),
                          DropdownMenuItem(value: 'SELL', child: Text('SELL')),
                        ],
                        onChanged: (value) =>
                            setState(() => _orderSide = value!),
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 12),
                Row(
                  children: [
                    const Text('Type: '),
                    Expanded(
                      child: DropdownButton<String>(
                        value: _orderType,
                        items: const [
                          DropdownMenuItem(
                              value: 'MARKET', child: Text('Market')),
                          DropdownMenuItem(
                              value: 'LIMIT', child: Text('Limit')),
                          DropdownMenuItem(value: 'STOP', child: Text('Stop')),
                        ],
                        onChanged: (value) =>
                            setState(() => _orderType = value!),
                      ),
                    ),
                  ],
                ),
                if (_orderType != 'MARKET') ...[
                  const SizedBox(height: 12),
                  TextField(
                    controller: _priceController,
                    decoration: InputDecoration(
                      labelText: '${_orderType} Price',
                      hintText: '0.00',
                      border: const OutlineInputBorder(),
                    ),
                    keyboardType: TextInputType.number,
                  ),
                ],
                if (tradingProvider.errorMessage != null) ...[
                  const SizedBox(height: 12),
                  Text(
                    tradingProvider.errorMessage!,
                    style: const TextStyle(color: Colors.red),
                  ),
                ],
              ],
            ),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.of(context).pop(),
              child: const Text('Cancel'),
            ),
            ElevatedButton(
              onPressed: tradingProvider.isLoading ? null : _submitOrder,
              child: tradingProvider.isLoading
                  ? const SizedBox(
                      width: 16,
                      height: 16,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    )
                  : Text(_orderSide),
            ),
          ],
        );
      },
    );
  }

  void _submitOrder() {
    final tradingProvider = context.read<TradingProvider>();

    tradingProvider.setSymbol(_symbolController.text);
    tradingProvider
        .setQuantity(double.tryParse(_quantityController.text) ?? 0.0);

    if (_orderType != 'MARKET') {
      tradingProvider.setPrice(double.tryParse(_priceController.text));
    }

    if (_orderType == 'MARKET') {
      tradingProvider.submitMarketOrder(_orderSide);
    } else {
      tradingProvider.submitLimitOrder(_orderSide);
    }

    if (!tradingProvider.isLoading && tradingProvider.errorMessage == null) {
      Navigator.of(context).pop();
    }
  }

  @override
  void dispose() {
    _symbolController.dispose();
    _quantityController.dispose();
    _priceController.dispose();
    super.dispose();
  }
}
