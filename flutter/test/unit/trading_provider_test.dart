import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/core_trading/trading_provider.dart';
import 'package:bullshift/services/rust_trading_engine.dart';
import 'package:mockito/mockito.dart';
import 'package:mockito/annotations.dart';

@GenerateMocks([RustTradingEngine])
import 'trading_provider_test.mocks.dart';

void main() {
  group('TradingProvider Tests', () {
    late TradingProvider provider;
    late MockRustTradingEngine mockEngine;

    setUp(() {
      mockEngine = MockRustTradingEngine();
      provider = TradingProvider(mockEngine);
    });

    tearDown(() {
      provider.dispose();
    });

    group('Symbol Management', () {
      test('sets symbol to uppercase', () {
        provider.setSymbol('aapl');
        expect(provider.currentSymbol, 'AAPL');
      });

      test('clears symbol correctly', () {
        provider.setSymbol('AAPL');
        expect(provider.currentSymbol, 'AAPL');

        provider.setSymbol('');
        expect(provider.currentSymbol, '');
      });
    });

    group('Quantity Management', () {
      test('sets quantity correctly', () {
        provider.setQuantity(100.0);
        expect(provider.currentQuantity, 100.0);
      });

      test('sets quantity to zero', () {
        provider.setQuantity(0.0);
        expect(provider.currentQuantity, 0.0);
      });
    });

    group('Order Type Management', () {
      test('default order type is MARKET', () {
        expect(provider.orderType, 'MARKET');
      });

      test('sets order type correctly', () {
        provider.setOrderType('LIMIT');
        expect(provider.orderType, 'LIMIT');
      });
    });

    group('Price Management', () {
      test('initial limit price is null', () {
        expect(provider.limitPrice, null);
      });

      test('sets limit price correctly', () {
        provider.setPrice(150.50);
        expect(provider.limitPrice, 150.50);
      });

      test('clears limit price', () {
        provider.setPrice(150.50);
        provider.setPrice(null);
        expect(provider.limitPrice, null);
      });
    });

    group('Notes Management', () {
      test('adds note for symbol', () {
        provider.addNote(
          symbol: 'AAPL',
          note: 'Test note',
          tags: ['#test'],
        );

        final notes = provider.getNotesForSymbol('AAPL');
        expect(notes.length, 1);
        expect(notes[0]['note'], 'Test note');
        expect(notes[0]['tags'], ['#test']);
      });

      test('gets empty list for symbol with no notes', () {
        final notes = provider.getNotesForSymbol('TSLA');
        expect(notes, isEmpty);
      });

      test('deletes note by id', () {
        provider.addNote(
          symbol: 'AAPL',
          note: 'Note to delete',
        );

        final notes = provider.getNotesForSymbol('AAPL');
        final noteId = notes[0]['id'] as String;

        provider.deleteNote(noteId);

        final updatedNotes = provider.getNotesForSymbol('AAPL');
        expect(updatedNotes, isEmpty);
      });

      test('pins note correctly', () {
        provider.addNote(
          symbol: 'AAPL',
          note: 'Note to pin',
        );

        final notes = provider.getNotesForSymbol('AAPL');
        final noteId = notes[0]['id'] as String;

        expect(notes[0]['isPinned'], false);

        provider.pinNote(noteId);

        final pinnedNotes = provider.getNotesForSymbol('AAPL');
        expect(pinnedNotes[0]['isPinned'], true);
      });

      test('gets all notes sorted by timestamp', () {
        provider.addNote(symbol: 'AAPL', note: 'First');
        provider.addNote(symbol: 'TSLA', note: 'Second');

        final allNotes = provider.getAllNotes();
        expect(allNotes.length, 2);
      });

      test('clears notes for symbol', () {
        provider.addNote(symbol: 'AAPL', note: 'Note 1');
        provider.addNote(symbol: 'AAPL', note: 'Note 2');

        provider.clearNotesForSymbol('AAPL');

        expect(provider.getNotesForSymbol('AAPL'), isEmpty);
      });

      test('gets all unique tags', () {
        provider.addNote(symbol: 'AAPL', note: 'Note 1', tags: ['#bullish']);
        provider.addNote(symbol: 'TSLA', note: 'Note 2', tags: ['#bearish']);

        final tags = provider.getAllTags();
        expect(tags.length, 2);
        expect(tags, contains('#bullish'));
        expect(tags, contains('#bearish'));
      });

      test('searches notes by content', () {
        provider.addNote(symbol: 'AAPL', note: 'Great earnings report');
        provider.addNote(symbol: 'TSLA', note: 'Missed delivery targets');

        final results = provider.searchNotes('earnings');
        expect(results.length, 1);
        expect(results[0]['note'], contains('earnings'));
      });
    });
  });
}
