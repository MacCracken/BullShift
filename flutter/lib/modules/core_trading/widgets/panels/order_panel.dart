import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../../trading_provider.dart';
import '../../../watchlist/watchlist_provider.dart';

class OrderPanel extends StatelessWidget {
  final TradingProvider tradingProvider;
  final WatchlistProvider watchlistProvider;

  const OrderPanel({
    super.key,
    required this.tradingProvider,
    required this.watchlistProvider,
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
          const Text(
            'Quick Order',
            style: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 16),
          TextField(
            decoration: InputDecoration(
              labelText: 'Symbol',
              hintText: 'e.g. AAPL',
              border: const OutlineInputBorder(),
              suffixIcon:
                  tradingProvider.currentSymbol.isNotEmpty &&
                      watchlistProvider.isInWatchlist(
                        tradingProvider.currentSymbol,
                      )
                  ? const Icon(Icons.star, color: Colors.yellow)
                  : null,
            ),
            onChanged: (value) => tradingProvider.setSymbol(value),
          ),
          const SizedBox(height: 12),
          TextField(
            decoration: const InputDecoration(
              labelText: 'Quantity',
              hintText: '0.00',
              border: OutlineInputBorder(),
            ),
            keyboardType: TextInputType.number,
            onChanged: (value) =>
                tradingProvider.setQuantity(double.tryParse(value) ?? 0.0),
          ),
          const SizedBox(height: 12),
          Row(
            children: [
              Expanded(
                child: ElevatedButton(
                  onPressed: () => tradingProvider.submitMarketOrder('BUY'),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Colors.green,
                  ),
                  child: const Text('BUY'),
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: ElevatedButton(
                  onPressed: () => tradingProvider.submitMarketOrder('SELL'),
                  style: ElevatedButton.styleFrom(backgroundColor: Colors.red),
                  child: const Text('SELL'),
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          const Text(
            'Order Type',
            style: TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 8),
          RadioListTile<String>(
            title: const Text('Market', style: TextStyle(color: Colors.white)),
            value: 'MARKET',
            groupValue: tradingProvider.orderType,
            onChanged: (value) => tradingProvider.setOrderType(value!),
          ),
          RadioListTile<String>(
            title: const Text('Limit', style: TextStyle(color: Colors.white)),
            value: 'LIMIT',
            groupValue: tradingProvider.orderType,
            onChanged: (value) => tradingProvider.setOrderType(value!),
          ),
          if (tradingProvider.orderType == 'LIMIT') ...[
            const SizedBox(height: 8),
            TextField(
              decoration: const InputDecoration(
                labelText: 'Limit Price',
                hintText: '0.00',
                border: OutlineInputBorder(),
              ),
              keyboardType: TextInputType.number,
              onChanged: (value) =>
                  tradingProvider.setPrice(double.tryParse(value)),
            ),
          ],
        ],
      ),
    );
  }
}
