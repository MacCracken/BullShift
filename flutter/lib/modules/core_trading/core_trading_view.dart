import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../services/rust_trading_engine.dart';
import '../watchlist/watchlist_provider.dart';
import '../core_trading/trading_provider.dart';
import 'widgets/widgets.dart';

class CoreTradingView extends StatelessWidget {
  const CoreTradingView({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer2<TradingProvider, WatchlistProvider>(
      builder: (context, tradingProvider, watchlistProvider, child) {
        return Row(
          children: [
            Expanded(
              flex: 1,
              child: Column(
                children: [
                  Expanded(
                    flex: 1,
                    child: OrderPanel(
                      tradingProvider: tradingProvider,
                      watchlistProvider: watchlistProvider,
                    ),
                  ),
                  const SizedBox(height: 8),
                  Expanded(
                    flex: 2,
                    child: AdvancedChartingWidget(
                      symbol: tradingProvider.currentSymbol.isEmpty
                          ? 'AAPL'
                          : tradingProvider.currentSymbol,
                      timeframe: '1D',
                    ),
                  ),
                  const SizedBox(height: 8),
                  Expanded(
                    flex: 1,
                    child: NotesPanel(
                      symbol: tradingProvider.currentSymbol.isEmpty
                          ? 'GENERAL'
                          : tradingProvider.currentSymbol,
                    ),
                  ),
                ],
              ),
            ),
            Expanded(
              flex: 2,
              child: ChartArea(tradingProvider: tradingProvider),
            ),
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
