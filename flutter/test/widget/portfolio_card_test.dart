import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/paper_hands/widgets/cards/portfolio_card.dart';

void main() {
  group('PortfolioCard Widget Tests', () {
    Widget createTestWidget({
      required Map<String, dynamic> portfolio,
      VoidCallback? onSelect,
      VoidCallback? onDelete,
      VoidCallback? onViewDetails,
    }) {
      return MaterialApp(
        home: Scaffold(
          body: PortfolioCard(
            portfolio: portfolio,
            onSelect: onSelect ?? () {},
            onDelete: onDelete ?? () {},
            onViewDetails: onViewDetails ?? () {},
          ),
        ),
      );
    }
    
    testWidgets('displays portfolio name', (WidgetTester tester) async {
      final portfolio = {
        'id': 'test-1',
        'name': 'Test Portfolio',
        'initialBalance': 10000.0,
        'currentBalance': 11500.0,
        'totalReturn': 1500.0,
        'totalReturnPercentage': 15.0,
        'winRate': 0.65,
        'isActive': false,
      };
      
      await tester.pumpWidget(createTestWidget(portfolio: portfolio));
      
      expect(find.text('Test Portfolio'), findsOneWidget);
    });
    
    testWidgets('displays current balance formatted', (WidgetTester tester) async {
      final portfolio = {
        'id': 'test-1',
        'name': 'Test',
        'initialBalance': 10000.0,
        'currentBalance': 11500.50,
        'totalReturn': 1500.50,
        'totalReturnPercentage': 15.0,
        'winRate': 0.65,
        'isActive': false,
      };
      
      await tester.pumpWidget(createTestWidget(portfolio: portfolio));
      
      expect(find.text('\$11500.50'), findsOneWidget);
    });
    
    testWidgets('displays return percentage', (WidgetTester tester) async {
      final portfolio = {
        'id': 'test-1',
        'name': 'Test',
        'initialBalance': 10000.0,
        'currentBalance': 11500.0,
        'totalReturn': 1500.0,
        'totalReturnPercentage': 15.0,
        'winRate': 0.65,
        'isActive': false,
      };
      
      await tester.pumpWidget(createTestWidget(portfolio: portfolio));
      
      expect(find.text('15.00%'), findsOneWidget);
    });
    
    testWidgets('shows green for positive returns', (WidgetTester tester) async {
      final portfolio = {
        'id': 'test-1',
        'name': 'Test',
        'initialBalance': 10000.0,
        'currentBalance': 11500.0,
        'totalReturn': 1500.0,
        'totalReturnPercentage': 15.0,
        'winRate': 0.65,
        'isActive': false,
      };
      
      await tester.pumpWidget(createTestWidget(portfolio: portfolio));
      
      final textWidget = tester.widget<Text>(find.text('15.00%'));
      expect(textWidget.style?.color, Colors.green);
    });
    
    testWidgets('shows red for negative returns', (WidgetTester tester) async {
      final portfolio = {
        'id': 'test-1',
        'name': 'Test',
        'initialBalance': 10000.0,
        'currentBalance': 8500.0,
        'totalReturn': -1500.0,
        'totalReturnPercentage': -15.0,
        'winRate': 0.35,
        'isActive': false,
      };
      
      await tester.pumpWidget(createTestWidget(portfolio: portfolio));
      
      final textWidget = tester.widget<Text>(find.text('-15.00%'));
      expect(textWidget.style?.color, Colors.red);
    });
    
    testWidgets('displays initial balance', (WidgetTester tester) async {
      final portfolio = {
        'id': 'test-1',
        'name': 'Test',
        'initialBalance': 10000.0,
        'currentBalance': 11500.0,
        'totalReturn': 1500.0,
        'totalReturnPercentage': 15.0,
        'winRate': 0.65,
        'isActive': false,
      };
      
      await tester.pumpWidget(createTestWidget(portfolio: portfolio));
      
      expect(find.text('Initial: \$10000.00'), findsOneWidget);
    });
    
    testWidgets('displays win rate when available', (WidgetTester tester) async {
      final portfolio = {
        'id': 'test-1',
        'name': 'Test',
        'initialBalance': 10000.0,
        'currentBalance': 11500.0,
        'totalReturn': 1500.0,
        'totalReturnPercentage': 15.0,
        'winRate': 0.65,
        'isActive': false,
      };
      
      await tester.pumpWidget(createTestWidget(portfolio: portfolio));
      
      expect(find.text('Win Rate: 65%'), findsOneWidget);
    });
    
    testWidgets('shows ACTIVE badge when portfolio is active', (WidgetTester tester) async {
      final portfolio = {
        'id': 'test-1',
        'name': 'Active Portfolio',
        'initialBalance': 10000.0,
        'currentBalance': 11500.0,
        'totalReturn': 1500.0,
        'totalReturnPercentage': 15.0,
        'winRate': 0.65,
        'isActive': true,
      };
      
      await tester.pumpWidget(createTestWidget(portfolio: portfolio));
      
      expect(find.text('ACTIVE'), findsOneWidget);
    });
    
    testWidgets('does not show ACTIVE badge when inactive', (WidgetTester tester) async {
      final portfolio = {
        'id': 'test-1',
        'name': 'Inactive Portfolio',
        'initialBalance': 10000.0,
        'currentBalance': 11500.0,
        'totalReturn': 1500.0,
        'totalReturnPercentage': 15.0,
        'winRate': 0.65,
        'isActive': false,
      };
      
      await tester.pumpWidget(createTestWidget(portfolio: portfolio));
      
      expect(find.text('ACTIVE'), findsNothing);
    });
    
    testWidgets('has Trade and Details buttons', (WidgetTester tester) async {
      final portfolio = {
        'id': 'test-1',
        'name': 'Test',
        'initialBalance': 10000.0,
        'currentBalance': 11500.0,
        'totalReturn': 1500.0,
        'totalReturnPercentage': 15.0,
        'winRate': 0.65,
        'isActive': false,
      };
      
      await tester.pumpWidget(createTestWidget(portfolio: portfolio));
      
      expect(find.text('Trade'), findsOneWidget);
      expect(find.text('Details'), findsOneWidget);
    });
    
    testWidgets('calls onSelect when Trade button pressed', (WidgetTester tester) async {
      var selectPressed = false;
      final portfolio = {
        'id': 'test-1',
        'name': 'Test',
        'initialBalance': 10000.0,
        'currentBalance': 11500.0,
        'totalReturn': 1500.0,
        'totalReturnPercentage': 15.0,
        'winRate': 0.65,
        'isActive': false,
      };
      
      await tester.pumpWidget(createTestWidget(
        portfolio: portfolio,
        onSelect: () => selectPressed = true,
      ));
      
      await tester.tap(find.text('Trade'));
      expect(selectPressed, true);
    });
    
    testWidgets('calls onViewDetails when Details button pressed', (WidgetTester tester) async {
      var detailsPressed = false;
      final portfolio = {
        'id': 'test-1',
        'name': 'Test',
        'initialBalance': 10000.0,
        'currentBalance': 11500.0,
        'totalReturn': 1500.0,
        'totalReturnPercentage': 15.0,
        'winRate': 0.65,
        'isActive': false,
      };
      
      await tester.pumpWidget(createTestWidget(
        portfolio: portfolio,
        onViewDetails: () => detailsPressed = true,
      ));
      
      await tester.tap(find.text('Details'));
      expect(detailsPressed, true);
    });
  });
}
