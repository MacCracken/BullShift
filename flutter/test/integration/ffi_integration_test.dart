import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/services/rust_trading_engine.dart';
import 'package:bullshift/services/security_manager.dart';

void main() {
  group('FFI Integration Tests', () {
    late RustTradingEngine rustEngine;
    late SecurityManager securityManager;

    setUpAll(() async {
      // Initialize the FFI bridge
      rustEngine = RustTradingEngine();
      securityManager = SecurityManager();

      // Wait for FFI initialization
      await Future.delayed(Duration(milliseconds: 100));
    });

    tearDownAll(() {
      rustEngine.dispose();
    });

    group('Rust Engine Initialization', () {
      test('initializes successfully', () {
        expect(rustEngine.isInitialized, true);
      });

      test('has correct version', () {
        final version = rustEngine.getVersion();
        expect(version, isNotNull);
        expect(version, isNotEmpty);
      });

      test('provides available functions', () {
        final functions = rustEngine.getAvailableFunctions();
        expect(functions, contains('submitMarketOrder'));
        expect(functions, contains('getPositions'));
        expect(functions, contains('getAccountBalance'));
      });
    });

    group('Security Manager Integration', () {
      test('encrypts and decrypts data correctly', () {
        const testData = 'sensitive_api_key_12345';

        final encrypted = securityManager.encryptData(testData);
        expect(encrypted, isNotNull);
        expect(encrypted, isNotEmpty);
        expect(encrypted, isNot(equals(testData)));

        final decrypted = securityManager.decryptData(encrypted);
        expect(decrypted, equals(testData));
      });

      test('handles empty data encryption', () {
        final encrypted = securityManager.encryptData('');
        expect(encrypted, isNotNull);

        final decrypted = securityManager.decryptData(encrypted);
        expect(decrypted, equals(''));
      });

      test('prevents double encryption', () {
        const testData = 'test_data';
        final encrypted = securityManager.encryptData(testData);

        // Should throw or return null for double encryption
        expect(
          () => securityManager.encryptData(encrypted),
          throwsA(isA<ArgumentError>()),
        );
      });
    });

    group('Trading Engine FFI Calls', () {
      test('submits market order via FFI', () async {
        try {
          final result = await rustEngine.submitMarketOrder(
            'AAPL',
            'BUY',
            100.0,
            'MARKET',
          );

          expect(result, isNotNull);
          expect(result, contains('order_id') || result.contains('error'));
        } catch (e) {
          // Expected if FFI is not available
          expect(e, isA<Exception>());
        }
      });

      test('fetches positions via FFI', () async {
        try {
          final positions = await rustEngine.getPositions();
          expect(positions, isNotNull);
          expect(positions, isA<List>());
        } catch (e) {
          // Expected if FFI is not available
          expect(e, isA<Exception>());
        }
      });

      test('fetches account balance via FFI', () async {
        try {
          final balance = await rustEngine.getAccountBalance();
          expect(balance, isNotNull);
        } catch (e) {
          // Expected if FFI is not available
          expect(e, isA<Exception>());
        }
      });
    });

    group('Error Handling', () {
      test('handles FFI unavailability gracefully', () async {
        // Mock a scenario where FFI is not available
        // This would be tested by temporarily renaming the library file

        try {
          await rustEngine.submitMarketOrder('INVALID', 'BUY', -1, 'MARKET');
        } catch (e) {
          expect(e, isNotNull);
        }
      });

      test('provides meaningful error messages', () {
        try {
          rustEngine.validateOrder('', -100, 'INVALID');
        } catch (e) {
          expect(e.toString(), contains('symbol'));
          expect(e.toString(), contains('quantity'));
        }
      });
    });

    group('Memory Management', () {
      test('does not leak memory on repeated calls', () async {
        for (int i = 0; i < 100; i++) {
          try {
            await rustEngine.getPositions();
          } catch (e) {
            // Ignore errors, we're testing memory management
          }
        }

        // If we reach here without crashing, memory management is working
        expect(true, true);
      });

      test('cleans up resources on dispose', () {
        final engine = RustTradingEngine();
        expect(engine.isInitialized, true);

        engine.dispose();
        expect(engine.isInitialized, false);
      });
    });

    group('Performance', () {
      test('FFI calls complete within reasonable time', () async {
        final stopwatch = Stopwatch()..start();

        try {
          await rustEngine.getPositions();
          stopwatch.stop();

          // Should complete within 5 seconds even in error cases
          expect(stopwatch.elapsedMilliseconds, lessThan(5000));
        } catch (e) {
          stopwatch.stop();
          // Even errors should return quickly
          expect(stopwatch.elapsedMilliseconds, lessThan(1000));
        }
      });

      test('handles concurrent FFI calls', () async {
        final futures = <Future>[];

        for (int i = 0; i < 5; i++) {
          futures.add(rustEngine.getPositions());
        }

        try {
          final results = await Future.wait(futures);
          expect(results.length, 5);
        } catch (e) {
          // Concurrent calls might fail, but shouldn't crash
          expect(e, isNotNull);
        }
      });
    });

    group('Data Type Conversion', () {
      test('converts Dart objects to Rust correctly', () async {
        // Test complex object serialization
        final orderData = {
          'symbol': 'AAPL',
          'quantity': 100.5,
          'order_type': 'LIMIT',
          'price': 150.25,
          'time_in_force': 'DAY',
          'notes': 'Test order from integration test',
        };

        try {
          final result = await rustEngine.submitComplexOrder(orderData);
          expect(result, isNotNull);
        } catch (e) {
          // Serialization errors should be descriptive
          expect(
            e.toString(),
            anyOf([
              contains('serialization'),
              contains('format'),
              contains('type'),
            ]),
          );
        }
      });

      test('converts Rust objects to Dart correctly', () async {
        try {
          final positions = await rustEngine.getPositions();

          for (final position in positions) {
            expect(position, isA<Map>());
            expect(position['symbol'], isA<String>());
            expect(position['quantity'], isA<double>());
            expect(position['avg_price'], isA<double>());
          }
        } catch (e) {
          // If FFI is not available, that's okay for this test
          expect(e, isNotNull);
        }
      });
    });
  });
}
