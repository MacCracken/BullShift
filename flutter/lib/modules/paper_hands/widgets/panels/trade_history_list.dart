import 'package:flutter/material.dart';
import '../../paper_hands_provider.dart';
import '../cards/paper_trade_card.dart';

class TradeHistoryList extends StatelessWidget {
  final PaperHandsProvider provider;
  
  const TradeHistoryList({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'Recent Trades',
          style: TextStyle(
            color: Colors.white,
            fontSize: 14,
            fontWeight: FontWeight.bold,
          ),
        ),
        const SizedBox(height: 8),
        Expanded(
          child: ListView.builder(
            itemCount: provider.recentTrades.length,
            itemBuilder: (context, index) {
              final trade = provider.recentTrades[index];
              return PaperTradeCard(trade: trade);
            },
          ),
        ),
      ],
    );
  }
}
