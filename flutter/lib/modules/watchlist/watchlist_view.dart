import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'watchlist_provider.dart';
import 'widgets/widgets.dart';

class WatchlistView extends StatelessWidget {
  const WatchlistView({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer<WatchlistProvider>(
      builder: (context, provider, child) {
        return Row(
          children: [
            Expanded(flex: 3, child: WatchlistPanel(provider: provider)),
            Expanded(flex: 1, child: SearchAndStatsPanel(provider: provider)),
          ],
        );
      },
    );
  }
}
