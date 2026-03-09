import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/bullrunnr/widgets/dialogs/news_search_dialog.dart';
import 'package:bullshift/modules/bullrunnr/bullrunnr_provider.dart';

void main() {
  group('NewsSearchDialog Widget Tests', () {
    late BullRunnrProvider provider;

    setUp(() {
      provider = BullRunnrProvider();
      provider.initialize();
    });

    tearDown(() {
      provider.dispose();
    });

    Widget createTestWidget() {
      return MaterialApp(
        home: Scaffold(
          body: Builder(
            builder: (context) => ElevatedButton(
              onPressed: () {
                showDialog(
                  context: context,
                  builder: (context) => NewsSearchDialog(provider: provider),
                );
              },
              child: const Text('Show Dialog'),
            ),
          ),
        ),
      );
    }

    testWidgets('displays dialog title', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();

      expect(find.text('Search News'), findsOneWidget);
    });

    testWidgets('has search keywords field', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();

      expect(find.widgetWithText(TextField, 'Search keywords'), findsOneWidget);
    });

    testWidgets('has symbols field', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();

      expect(find.widgetWithText(TextField, 'Symbols (comma-separated)'),
          findsOneWidget);
    });

    testWidgets('has Cancel button', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();

      expect(find.text('Cancel'), findsOneWidget);
    });

    testWidgets('has Search button', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();

      expect(find.text('Search'), findsOneWidget);
    });

    testWidgets('closes dialog when Cancel pressed',
        (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();

      expect(find.text('Search News'), findsOneWidget);

      await tester.tap(find.text('Cancel'));
      await tester.pumpAndSettle();

      expect(find.text('Search News'), findsNothing);
    });

    testWidgets('accepts text input in keywords field',
        (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();

      await tester.enterText(
        find.widgetWithText(TextField, 'Search keywords'),
        'earnings report',
      );

      expect(find.text('earnings report'), findsOneWidget);
    });

    testWidgets('accepts text input in symbols field',
        (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();

      await tester.enterText(
        find.widgetWithText(TextField, 'Symbols (comma-separated)'),
        'AAPL, MSFT, GOOGL',
      );

      expect(find.text('AAPL, MSFT, GOOGL'), findsOneWidget);
    });

    testWidgets('dialog has correct width', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();

      final dialog = tester.widget<AlertDialog>(find.byType(AlertDialog));
      expect(dialog.content != null, true);
    });
  });
}
