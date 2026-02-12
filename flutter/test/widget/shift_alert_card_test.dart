import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/watchlist/watchlist_provider.dart';
import 'package:bullshift/modules/trendsetter/widgets/cards/shift_alert_card.dart';

class MockWatchlistProvider extends WatchlistProvider {
  final Set<String> _watchlist = {};

  @override
  bool isInWatchlist(String symbol) => _watchlist.contains(symbol);

  @override
  Future<bool> addToWatchlist(String symbol) async {
    _watchlist.add(symbol);
    return true;
  }

  @override
  Future<void> removeFromWatchlist(String symbol) async {
    _watchlist.remove(symbol);
  }
}

void main() {
  group('ShiftAlertCard Tests', () {
    late MockWatchlistProvider mockWatchlistProvider;

    setUp(() {
      mockWatchlistProvider = MockWatchlistProvider();
    });

    Widget createWidgetUnderTest(Map<String, dynamic> alert) {
      return MaterialApp(
        home: Scaffold(
          body: ShiftAlertCard(
            alert: alert,
            watchlistProvider: mockWatchlistProvider,
          ),
        ),
      );
    }

    testWidgets('displays symbol correctly', (tester) async {
      final alert = {
        'symbol': 'AAPL',
        'message': 'Volume spike detected',
        'type': 'VolumeSpike',
        'confidence': 0.85,
      };

      await tester.pumpWidget(createWidgetUnderTest(alert));

      expect(find.text('AAPL'), findsOneWidget);
    });

    testWidgets('displays alert message', (tester) async {
      final alert = {
        'symbol': 'AAPL',
        'message': 'Volume spike detected',
        'type': 'VolumeSpike',
        'confidence': 0.85,
      };

      await tester.pumpWidget(createWidgetUnderTest(alert));

      expect(find.text('Volume spike detected'), findsOneWidget);
    });

    testWidgets('displays confidence percentage', (tester) async {
      final alert = {
        'symbol': 'AAPL',
        'message': 'Volume spike detected',
        'type': 'VolumeSpike',
        'confidence': 0.85,
      };

      await tester.pumpWidget(createWidgetUnderTest(alert));

      expect(find.text('85%'), findsOneWidget);
    });

    testWidgets('shows Watch button when symbol not in watchlist', (
      tester,
    ) async {
      final alert = {
        'symbol': 'AAPL',
        'message': 'Volume spike detected',
        'type': 'VolumeSpike',
        'confidence': 0.85,
      };

      await tester.pumpWidget(createWidgetUnderTest(alert));

      expect(find.text('Watch'), findsOneWidget);
    });

    testWidgets('does not show Watch button when symbol in watchlist', (
      tester,
    ) async {
      mockWatchlistProvider.addToWatchlist('AAPL');

      final alert = {
        'symbol': 'AAPL',
        'message': 'Volume spike detected',
        'type': 'VolumeSpike',
        'confidence': 0.85,
      };

      await tester.pumpWidget(createWidgetUnderTest(alert));

      expect(find.text('Watch'), findsNothing);
    });

    testWidgets('shows star icon when symbol in watchlist', (tester) async {
      mockWatchlistProvider.addToWatchlist('AAPL');

      final alert = {
        'symbol': 'AAPL',
        'message': 'Volume spike detected',
        'type': 'VolumeSpike',
        'confidence': 0.85,
      };

      await tester.pumpWidget(createWidgetUnderTest(alert));

      expect(find.byIcon(Icons.star), findsOneWidget);
    });
  });
}
