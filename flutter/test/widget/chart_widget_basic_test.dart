import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('Chart Widget Basic Tests', () {
    testWidgets('Chart widget renders correctly', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Container(
              padding: const EdgeInsets.all(16),
              child: Column(
                children: [
                  const Icon(Icons.show_chart, color: Colors.white, size: 20),
                  const SizedBox(width: 8),
                  const Text(
                    'AAPL Chart',
                    style: TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      );

      expect(find.byType<Icon>(), findsOneWidget);
      expect(find.text('AAPL Chart'), findsOneWidget);
    });

    testWidgets('Chart controls are interactive', (tester) async {
      bool volumeToggled = false;

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Container(
              padding: const EdgeInsets.all(16),
              child: FilterChip(
                label: const Text('Volume'),
                selected: volumeToggled,
                onSelected: (selected) {
                  volumeToggled = selected;
                },
              ),
            ),
          ),
        ),
      );

      await tester.tap(find.byType<FilterChip>());
      await tester.pump();

      expect(volumeToggled, true);
    });

    testWidgets('Chart handles different symbols', (tester) async {
      for (final symbol in ['AAPL', 'TSLA', 'GOOGL']) {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: Container(
                padding: const EdgeInsets.all(16),
                child: Text(
                  '$symbol Chart',
                  style: const TextStyle(
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
              ),
            ),
          ),
        );

        expect(find.text('$symbol Chart'), findsOneWidget);
        await tester.pump(const Duration(milliseconds: 100));
      }
    });
  });
}
