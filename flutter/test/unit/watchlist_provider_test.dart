import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/watchlist/watchlist_provider.dart';
import 'package:bullshift/services/rust_trading_engine.dart';
import 'package:mockito/mockito.dart';

class MockRustTradingEngine extends Mock implements RustTradingEngine {}

void main() {
  group('WatchlistProvider Tests', () {
    late WatchlistProvider provider;
    late MockRustTradingEngine mockEngine;

    setUp(() {
      mockEngine = MockRustTradingEngine();
      provider = WatchlistProvider(mockEngine);
    });

    tearDown(() {
      provider.dispose();
    });

    group('Initialization', () {
      test('initializes with empty watchlist', () {
        expect(provider.watchlist, isEmpty);
      });

      test('initializes with empty search results', () {
        expect(provider.searchResults, isEmpty);
      });

      test('default sort is by symbol', () {
        expect(provider.sortBy, 'symbol');
      });

      test('default sort order is ascending', () {
        expect(provider.sortAscending, true);
      });

      test('real-time updates enabled by default', () {
        expect(provider.realTimeUpdatesEnabled, true);
      });
    });

    group('Search', () {
      test('sets search query', () {
        provider.setSearchQuery('AAPL');
        expect(provider.searchQuery, 'AAPL');
      });

      test('clears search results', () {
        provider.setSearchQuery('AAPL');
        provider.clearSearchResults();

        expect(provider.searchQuery, '');
        expect(provider.searchResults, isEmpty);
      });
    });

    group('Sorting', () {
      test('toggles sort order for same field', () {
        provider.setSortBy('symbol'); // Initial
        expect(provider.sortAscending, true);

        provider.setSortBy('symbol'); // Toggle
        expect(provider.sortAscending, false);

        provider.setSortBy('symbol'); // Toggle back
        expect(provider.sortAscending, true);
      });

      test('resets to ascending for different field', () {
        provider.setSortBy('symbol');
        provider.setSortBy('symbol'); // Now descending
        expect(provider.sortAscending, false);

        provider.setSortBy('price'); // Different field
        expect(provider.sortAscending, true);
        expect(provider.sortBy, 'price');
      });

      test('supports all sort fields', () {
        final fields = ['symbol', 'price', 'change', 'volume'];

        for (final field in fields) {
          provider.setSortBy(field);
          expect(provider.sortBy, field);
        }
      });
    });

    group('Real-time Updates', () {
      test('can enable real-time updates', () {
        provider.setRealTimeUpdatesEnabled(true);
        expect(provider.realTimeUpdatesEnabled, true);
      });

      test('can disable real-time updates', () {
        provider.setRealTimeUpdatesEnabled(false);
        expect(provider.realTimeUpdatesEnabled, false);
      });
    });

    group('Watchlist Stats', () {
      test('returns empty stats for empty watchlist', () {
        final stats = provider.getWatchlistStats();

        expect(stats['totalSymbols'], 0);
        expect(stats['totalValue'], 0.0);
        expect(stats['dayChange'], 0.0);
      });
    });

    group('Symbol Check', () {
      test('returns false for symbol not in watchlist', () {
        expect(provider.isInWatchlist('AAPL'), false);
      });
    });

    group('Export/Import', () {
      test('exports empty watchlist', () {
        final exported = provider.exportWatchlist();

        expect(exported['watchlist'], isEmpty);
        expect(exported.containsKey('exportDate'), true);
        expect(exported.containsKey('version'), true);
      });
    });
  });
}
