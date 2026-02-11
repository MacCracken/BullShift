import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/paper_hands/widgets/dialogs/create_portfolio_dialog.dart';
import 'package:bullshift/modules/paper_hands/paper_hands_provider.dart';

void main() {
  group('CreatePortfolioDialog Widget Tests', () {
    late PaperHandsProvider provider;
    
    setUp(() {
      provider = PaperHandsProvider();
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
                  builder: (context) => CreatePortfolioDialog(provider: provider),
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
      
      expect(find.text('Create Paper Portfolio'), findsOneWidget);
    });
    
    testWidgets('has portfolio name field', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();
      
      expect(find.widgetWithText(TextField, 'Portfolio Name'), findsOneWidget);
    });
    
    testWidgets('has initial balance field', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();
      
      expect(find.widgetWithText(TextField, 'Initial Balance'), findsOneWidget);
    });
    
    testWidgets('initial balance has dollar prefix', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();
      
      final textField = tester.widget<TextField>(
        find.widgetWithText(TextField, 'Initial Balance'),
      );
      expect(textField.decoration?.prefixText, '\$');
    });
    
    testWidgets('has Cancel button', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();
      
      expect(find.text('Cancel'), findsOneWidget);
    });
    
    testWidgets('has Create button', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();
      
      expect(find.text('Create'), findsOneWidget);
    });
    
    testWidgets('closes dialog when Cancel pressed', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();
      
      expect(find.text('Create Paper Portfolio'), findsOneWidget);
      
      await tester.tap(find.text('Cancel'));
      await tester.pumpAndSettle();
      
      expect(find.text('Create Paper Portfolio'), findsNothing);
    });
    
    testWidgets('accepts text input in name field', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();
      
      await tester.enterText(
        find.widgetWithText(TextField, 'Portfolio Name'),
        'My Test Portfolio',
      );
      
      expect(find.text('My Test Portfolio'), findsOneWidget);
    });
    
    testWidgets('accepts numeric input in balance field', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();
      
      await tester.enterText(
        find.widgetWithText(TextField, 'Initial Balance'),
        '50000',
      );
      
      expect(find.text('50000'), findsOneWidget);
    });
    
    testWidgets('balance field uses number keyboard', (WidgetTester tester) async {
      await tester.pumpWidget(createTestWidget());
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();
      
      final textField = tester.widget<TextField>(
        find.widgetWithText(TextField, 'Initial Balance'),
      );
      expect(textField.keyboardType, TextInputType.number);
    });
  });
}
