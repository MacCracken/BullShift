import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../services/rust_trading_engine.dart';
import 'trading_provider.dart';

class CoreTradingView extends StatelessWidget {
  const CoreTradingView({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer<TradingProvider>(
      builder: (context, tradingProvider, child) {
        return Row(
          children: [
            // Order Panel
            Expanded(
              flex: 1,
              child: Column(
                children: [
                  Expanded(
                    flex: 2,
                    child: OrderPanel(tradingProvider: tradingProvider),
                  ),
                  const SizedBox(height: 8),
                  Expanded(
                    flex: 1,
                    child: NotesPanel(symbol: tradingProvider.currentSymbol.isEmpty ? 'GENERAL' : tradingProvider.currentSymbol),
                  ),
                ],
              ),
            ),
            // Chart Area
            Expanded(
              flex: 2,
              child: ChartArea(tradingProvider: tradingProvider),
            ),
            // Positions Panel
            Expanded(
              flex: 1,
              child: PositionsPanel(tradingProvider: tradingProvider),
            ),
          ],
        );
      },
    );
  }
}

class OrderPanel extends StatelessWidget {
  final TradingProvider tradingProvider;
  
  const OrderPanel({
    super.key,
    required this.tradingProvider,
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
            decoration: const InputDecoration(
              labelText: 'Symbol',
              hintText: 'e.g. AAPL',
              border: OutlineInputBorder(),
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
            onChanged: (value) => tradingProvider.setQuantity(double.tryParse(value) ?? 0.0),
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
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Colors.red,
                  ),
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
              onChanged: (value) => tradingProvider.setPrice(double.tryParse(value)),
            ),
          ],
        ],
      ),
    );
  }
}

class ChartArea extends StatelessWidget {
  final TradingProvider tradingProvider;
  
  const ChartArea({
    super.key,
    required this.tradingProvider,
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
        children: [
          Row(
            children: [
              Text(
                tradingProvider.currentSymbol.isEmpty 
                    ? 'Select a symbol' 
                    : '${tradingProvider.currentSymbol} Chart',
                style: const TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const Spacer(),
              IconButton(
                icon: const Icon(Icons.refresh, color: Colors.white),
                onPressed: () => tradingProvider.refreshData(),
              ),
            ],
          ),
          const SizedBox(height: 16),
          Expanded(
            child: Container(
              decoration: BoxDecoration(
                color: const Color(0xFF1E1E1E),
                borderRadius: BorderRadius.circular(4),
              ),
              child: const Center(
                child: Text(
                  'Chart View\n(Real-time data will appear here)',
                  textAlign: TextAlign.center,
                  style: TextStyle(
                    color: Colors.grey,
                    fontSize: 16,
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class PositionsPanel extends StatelessWidget {
  final TradingProvider tradingProvider;
  
  const PositionsPanel({
    super.key,
    required this.tradingProvider,
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
            'Positions',
            style: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 16),
          Expanded(
            child: ListView.builder(
              itemCount: tradingProvider.positions.length,
              itemBuilder: (context, index) {
                final position = tradingProvider.positions[index];
                return PositionCard(position: position);
              },
            ),
          ),
        ],
      ),
    );
  }
}

class PositionCard extends StatelessWidget {
  final Map<String, dynamic> position;
  
  const PositionCard({
    super.key,
    required this.position,
  });

  @override
  Widget build(BuildContext context) {
    final pnl = position['unrealizedPnl'] as double;
    final pnlColor = pnl >= 0 ? Colors.green : Colors.red;
    
    return Card(
      color: const Color(0xFF37474F),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Text(
                  position['symbol'] as String,
                  style: const TextStyle(
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                    fontSize: 16,
                  ),
                ),
                const Spacer(),
                Text(
                  '${position['quantity']}',
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Text(
                  '\$${position['currentPrice'].toStringAsFixed(2)}',
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                  ),
                ),
                const Spacer(),
                Text(
                  '\$${pnl.toStringAsFixed(2)}',
                  style: TextStyle(
                    color: pnlColor,
                    fontWeight: FontWeight.bold,
                    fontSize: 14,
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