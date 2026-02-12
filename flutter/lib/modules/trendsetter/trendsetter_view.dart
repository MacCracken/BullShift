import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../watchlist/watchlist_provider.dart';
import 'trendsetter_provider.dart';
import 'widgets/widgets.dart';

class TrendSetterView extends StatelessWidget {
  const TrendSetterView({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer2<TrendSetterProvider, WatchlistProvider>(
      builder: (context, trendProvider, watchlistProvider, child) {
        return Row(
          children: [
            Expanded(
              flex: 2,
              child: MomentumScanner(
                provider: trendProvider,
                watchlistProvider: watchlistProvider,
              ),
            ),
            Expanded(
              flex: 1,
              child: HeatMapPanel(
                provider: trendProvider,
                watchlistProvider: watchlistProvider,
              ),
            ),
            Expanded(
              flex: 1,
              child: ShiftAlertsPanel(
                provider: trendProvider,
                watchlistProvider: watchlistProvider,
              ),
            ),
          ],
        );
      },
    );
  }
}
