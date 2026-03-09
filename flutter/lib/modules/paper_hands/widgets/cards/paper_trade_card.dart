import 'package:flutter/material.dart';

class PaperTradeCard extends StatelessWidget {
  final Map<String, dynamic> trade;

  const PaperTradeCard({
    super.key,
    required this.trade,
  });

  @override
  Widget build(BuildContext context) {
    final symbol = trade['symbol'] as String;
    final side = trade['side'] as String;
    final quantity = trade['quantity'] as double;
    final entryPrice = trade['entryPrice'] as double;
    final exitPrice = trade['exitPrice'] as double?;
    final pnl = trade['pnl'] as double?;
    final sideColor = side == 'Buy' ? Colors.green : Colors.red;
    final pnlColor = (pnl ?? 0.0) >= 0 ? Colors.green : Colors.red;

    return Card(
      color: const Color(0xFF37474F),
      margin: const EdgeInsets.only(bottom: 8),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Row(
          children: [
            Container(
              width: 4,
              height: 40,
              decoration: BoxDecoration(
                color: sideColor,
                borderRadius: BorderRadius.circular(2),
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Text(
                        symbol,
                        style: const TextStyle(
                          color: Colors.white,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const Spacer(),
                      Text(
                        side,
                        style: TextStyle(
                          color: sideColor,
                          fontWeight: FontWeight.bold,
                          fontSize: 12,
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 4),
                  Row(
                    children: [
                      Text(
                        '$quantity @ \$${entryPrice.toStringAsFixed(2)}',
                        style: const TextStyle(
                          color: Colors.grey,
                          fontSize: 12,
                        ),
                      ),
                      if (exitPrice != null) ...[
                        const SizedBox(width: 8),
                        Text(
                          '→ \$${exitPrice.toStringAsFixed(2)}',
                          style: const TextStyle(
                            color: Colors.grey,
                            fontSize: 12,
                          ),
                        ),
                      ],
                      const Spacer(),
                      if (pnl != null)
                        Text(
                          '\$${pnl.toStringAsFixed(2)}',
                          style: TextStyle(
                            color: pnlColor,
                            fontWeight: FontWeight.bold,
                            fontSize: 12,
                          ),
                        ),
                    ],
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
