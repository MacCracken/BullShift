import 'package:flutter/material.dart';
import '../../trading_provider.dart';
import '../cards/position_card.dart';

class PositionsPanel extends StatelessWidget {
  final TradingProvider tradingProvider;

  const PositionsPanel({super.key, required this.tradingProvider});

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
