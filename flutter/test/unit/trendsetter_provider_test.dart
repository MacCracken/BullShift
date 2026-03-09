import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/trendsetter/trendsetter_provider.dart';

void main() {
  group('TrendSetterProvider Tests', () {
    late TrendSetterProvider provider;

    setUp(() {
      provider = TrendSetterProvider();
    });

    tearDown(() {
      provider.dispose();
    });

    group('Initialization', () {
      test('initializes with default values', () {
        expect(provider.momentumAssets, isEmpty);
        expect(provider.watchlistAssets, isEmpty);
        expect(provider.heatMapData, isEmpty);
        expect(provider.shiftAlerts, isEmpty);
        expect(provider.selectedTimeframe, '1D');
        expect(provider.minMomentumScore, 0.0);
        expect(provider.showVolumeSpikes, true);
        expect(provider.showSocialSentiment, true);
      });

      test('initialize populates data', () {
        provider.initialize();

        expect(provider.momentumAssets, isNotEmpty);
        expect(provider.watchlistAssets, isNotEmpty);
        expect(provider.heatMapData, isNotEmpty);
      });
    });

    group('Momentum Assets', () {
      test('refreshMomentumData updates assets', () {
        provider.initialize();

        provider.refreshMomentumData();

        // Assets should be refreshed
        expect(provider.momentumAssets, isNotEmpty);
      });

      test('momentum assets have required fields', () {
        provider.initialize();

        final asset = provider.momentumAssets.first;
        expect(asset.containsKey('symbol'), true);
        expect(asset.containsKey('name'), true);
        expect(asset.containsKey('momentumScore'), true);
        expect(asset.containsKey('price'), true);
        expect(asset.containsKey('volume'), true);
        expect(asset.containsKey('volumeSpike'), true);
      });

      test('momentum scores are within valid range', () {
        provider.initialize();

        for (final asset in provider.momentumAssets) {
          final score = asset['momentumScore'] as double;
          expect(score >= 0.0 && score <= 100.0, true);
        }
      });
    });

    group('Filters', () {
      test('setTimeframe updates selected timeframe', () {
        provider.setTimeframe('4h');

        expect(provider.selectedTimeframe, '4h');
      });

      test('setMinMomentumScore updates threshold', () {
        provider.setMinMomentumScore(70.0);

        expect(provider.minMomentumScore, 70.0);
      });

      test('toggleVolumeSpikes updates setting', () {
        provider.initialize();

        final initialValue = provider.showVolumeSpikes;
        provider.toggleVolumeSpikes();

        expect(provider.showVolumeSpikes, !initialValue);
      });

      test('toggleSocialSentiment updates setting', () {
        provider.initialize();

        final initialValue = provider.showSocialSentiment;
        provider.toggleSocialSentiment();

        expect(provider.showSocialSentiment, !initialValue);
      });
    });

    group('Watchlist', () {
      test('addToWatchlist adds asset', () {
        provider.initialize();
        final asset = provider.momentumAssets.first;
        final symbol = asset['symbol'];

        provider.addToWatchlist(symbol);

        expect(provider.watchlistAssets.contains(symbol), true);
      });

      test('removeFromWatchlist removes asset', () {
        provider.initialize();
        final asset = provider.momentumAssets.first;
        final symbol = asset['symbol'];

        provider.addToWatchlist(symbol);
        expect(provider.watchlistAssets.contains(symbol), true);

        provider.removeFromWatchlist(symbol);
        expect(provider.watchlistAssets.contains(symbol), false);
      });

      test('isInWatchlist returns correct status', () {
        provider.initialize();
        final asset = provider.momentumAssets.first;
        final symbol = asset['symbol'];

        expect(provider.isInWatchlist(symbol), false);

        provider.addToWatchlist(symbol);
        expect(provider.isInWatchlist(symbol), true);
      });
    });

    group('Heat Map', () {
      test('refreshHeatMap updates data', () {
        provider.initialize();

        provider.refreshHeatMap();

        expect(provider.heatMapData, isNotEmpty);
      });

      test('heat map data has valid structure', () {
        provider.initialize();

        final heatData = provider.heatMapData.first;
        expect(heatData.containsKey('sector'), true);
        expect(heatData.containsKey('symbol'), true);
        expect(heatData.containsKey('performance'), true);
        expect(heatData.containsKey('value'), true);
      });
    });

    group('Shift Alerts', () {
      test('getLatestAlerts returns recent alerts', () {
        provider.initialize();

        final alerts = provider.getLatestAlerts(5);

        expect(alerts, isNotNull);
        expect(alerts.length <= 5, true);
      });

      test('dismissAlert removes alert', () {
        provider.initialize();

        if (provider.shiftAlerts.isNotEmpty) {
          final alertId = provider.shiftAlerts.first['id'];
          final initialCount = provider.shiftAlerts.length;

          provider.dismissAlert(alertId);

          expect(provider.shiftAlerts.length, initialCount - 1);
        }
      });

      test('dismissAllAlerts clears all alerts', () {
        provider.initialize();

        provider.dismissAllAlerts();

        expect(provider.shiftAlerts, isEmpty);
      });
    });

    group('Sorting', () {
      test('sortByMomentum sorts assets by score', () {
        provider.initialize();

        provider.sortByMomentum();

        // Assets should be sorted by momentum score
        final assets = provider.momentumAssets;
        for (int i = 1; i < assets.length; i++) {
          final prevScore = assets[i - 1]['momentumScore'] as double;
          final currScore = assets[i]['momentumScore'] as double;
          expect(prevScore >= currScore, true);
        }
      });

      test('sortByVolume sorts assets by volume', () {
        provider.initialize();

        provider.sortByVolume();

        // Assets should be sorted by volume
        final assets = provider.momentumAssets;
        for (int i = 1; i < assets.length; i++) {
          final prevVol = assets[i - 1]['volume'] as int;
          final currVol = assets[i]['volume'] as int;
          expect(prevVol >= currVol, true);
        }
      });

      test('sortByPrice sorts assets by price', () {
        provider.initialize();

        provider.sortByPrice();

        // Assets should be sorted by price
        final assets = provider.momentumAssets;
        for (int i = 1; i < assets.length; i++) {
          final prevPrice = assets[i - 1]['price'] as double;
          final currPrice = assets[i]['price'] as double;
          expect(prevPrice >= currPrice, true);
        }
      });
    });

    group('Asset Details', () {
      test('getAssetDetails returns asset info', () {
        provider.initialize();
        final symbol = provider.momentumAssets.first['symbol'];

        final details = provider.getAssetDetails(symbol);

        expect(details, isNotNull);
        expect(details!['symbol'], symbol);
      });

      test('getAssetDetails returns null for unknown symbol', () {
        final details = provider.getAssetDetails('UNKNOWN');

        expect(details, isNull);
      });
    });
  });
}
