import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../services/rust_trading_engine.dart';
import '../watchlist/watchlist_provider.dart';
import '../core_trading/trading_provider.dart';
import '../market_data/market_data_provider.dart';
import '../../widgets/advanced_charting_widget.dart';
import 'widgets/widgets.dart';

class CoreTradingView extends StatelessWidget {
  const CoreTradingView({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer3<TradingProvider, WatchlistProvider, MarketDataProvider>(
      builder:
          (
            context,
            tradingProvider,
            watchlistProvider,
            marketDataProvider,
            child,
          ) {
            final symbol = tradingProvider.currentSymbol.isEmpty
                ? 'AAPL'
                : tradingProvider.currentSymbol;

            if (marketDataProvider.currentSymbol != symbol) {
              marketDataProvider.loadSymbolData(symbol);
            }

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
                          symbol: symbol,
                          timeframe: '1D',
                          priceData: marketDataProvider.priceHistory,
                          useRealtimeData: true,
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
                  child: AdvancedChartingWidget(
                    symbol: symbol,
                    timeframe: '1D',
                    priceData: marketDataProvider.priceHistory,
                    useRealtimeData: true,
                  ),
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
