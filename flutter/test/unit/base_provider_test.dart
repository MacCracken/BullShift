import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/services/base_provider.dart';

// Test implementation of BaseProvider
class TestProvider extends BaseProvider {
  String _data = '';

  String get data => _data;

  void setData(String value) {
    _data = value;
    safeNotifyListeners();
  }

  Future<String> fetchData() async {
    return await executeAsync(
          operation: () async {
            await Future.delayed(const Duration(milliseconds: 100));
            _data = 'fetched';
            return _data;
          },
        ) ??
        '';
  }

  Future<void> fetchWithError() async {
    await executeAsync(
      operation: () async {
        await Future.delayed(const Duration(milliseconds: 100));
        throw Exception('Test error');
      },
    );
  }
}

void main() {
  group('BaseProvider Tests', () {
    late TestProvider provider;

    setUp(() {
      provider = TestProvider();
    });

    tearDown(() {
      provider.dispose();
    });

    group('Loading State', () {
      test('initial loading state is false', () {
        expect(provider.isLoading, false);
      });

      test('setLoading updates loading state', () {
        provider.setLoading(true);
        expect(provider.isLoading, true);

        provider.setLoading(false);
        expect(provider.isLoading, false);
      });

      test('setLoading with same value does not notify', () {
        var notifyCount = 0;
        provider.addListener(() => notifyCount++);

        provider.setLoading(false); // Same as initial
        expect(notifyCount, 0);

        provider.setLoading(true);
        expect(notifyCount, 1);

        provider.setLoading(true); // Same value
        expect(notifyCount, 1);
      });
    });

    group('Error State', () {
      test('initial error state is null', () {
        expect(provider.errorMessage, null);
        expect(provider.hasError, false);
      });

      test('setError updates error state', () {
        provider.setError('Test error');
        expect(provider.errorMessage, 'Test error');
        expect(provider.hasError, true);
      });

      test('clearError clears error state', () {
        provider.setError('Test error');
        expect(provider.hasError, true);

        provider.clearError();
        expect(provider.errorMessage, null);
        expect(provider.hasError, false);
      });

      test('setError with same value does not notify', () {
        var notifyCount = 0;
        provider.addListener(() => notifyCount++);

        provider.setError('Error 1');
        expect(notifyCount, 1);

        provider.setError('Error 1'); // Same value
        expect(notifyCount, 1);

        provider.setError('Error 2'); // Different value
        expect(notifyCount, 2);
      });
    });

    group('executeAsync', () {
      test('sets loading true during operation', () async {
        var wasLoadingDuringOperation = false;

        provider.addListener(() {
          if (provider.isLoading) {
            wasLoadingDuringOperation = true;
          }
        });

        await provider.fetchData();

        expect(wasLoadingDuringOperation, true);
        expect(provider.isLoading, false);
      });

      test('returns result on success', () async {
        final result = await provider.fetchData();
        expect(result, 'fetched');
        expect(provider.data, 'fetched');
      });

      test('sets error on failure', () async {
        await provider.fetchWithError();

        expect(provider.hasError, true);
        expect(provider.errorMessage, contains('Test error'));
        expect(provider.isLoading, false);
      });

      test('clears previous error before new operation', () async {
        await provider.fetchWithError();
        expect(provider.hasError, true);

        await provider.fetchData();
        expect(provider.hasError, false);
      });

      test('returns null on error', () async {
        final result = await provider.executeAsync<String>(
          operation: () async {
            throw Exception('Error');
          },
        );

        expect(result, null);
      });
    });

    group('safeNotifyListeners', () {
      test('notifies listeners when not disposed', () {
        var notifyCount = 0;
        provider.addListener(() => notifyCount++);

        provider.setData('test');
        expect(notifyCount, 1);
      });

      test('does not throw when disposed', () {
        provider.dispose();

        expect(() => provider.safeNotifyListeners(), returnsNormally);
      });
    });
  });
}
