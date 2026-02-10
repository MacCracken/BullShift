import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:bullshift/modules/bearly_managed/bearly_managed_provider.dart';
import 'package:bullshift/modules/bearly_managed/widgets/cards/ai_provider_card.dart';

void main() {
  group('AIProviderCard Widget Tests', () {
    late BearlyManagedProvider provider;
    
    setUp(() {
      provider = BearlyManagedProvider();
      provider.initialize();
    });
    
    tearDown(() {
      provider.dispose();
    });
    
    Widget createTestWidget(Map<String, dynamic> aiProvider) {
      return MaterialApp(
        home: Scaffold(
          body: AIProviderCard(
            provider: aiProvider,
            onConfigure: () {},
            onTest: () {},
            onToggle: () {},
            onDelete: () {},
          ),
        ),
      );
    }
    
    testWidgets('displays provider name and type', (WidgetTester tester) async {
      final aiProvider = {
        'name': 'OpenAI GPT-4',
        'type': 'OpenAI',
        'modelName': 'gpt-4',
        'isConfigured': true,
        'isActive': true,
        'lastUsed': '2h ago',
      };
      
      await tester.pumpWidget(createTestWidget(aiProvider));
      
      expect(find.text('OpenAI GPT-4'), findsOneWidget);
      expect(find.text('OPENAI'), findsOneWidget);
      expect(find.text('Model: gpt-4'), findsOneWidget);
    });
    
    testWidgets('shows configured status when configured', (WidgetTester tester) async {
      final aiProvider = {
        'name': 'Test Provider',
        'type': 'OpenAI',
        'modelName': 'gpt-4',
        'isConfigured': true,
        'isActive': false,
        'lastUsed': null,
      };
      
      await tester.pumpWidget(createTestWidget(aiProvider));
      
      expect(find.text('Configured'), findsOneWidget);
      expect(find.byIcon(Icons.check_circle), findsOneWidget);
    });
    
    testWidgets('shows not configured status when not configured', (WidgetTester tester) async {
      final aiProvider = {
        'name': 'Test Provider',
        'type': 'OpenAI',
        'modelName': 'gpt-4',
        'isConfigured': false,
        'isActive': false,
        'lastUsed': null,
      };
      
      await tester.pumpWidget(createTestWidget(aiProvider));
      
      expect(find.text('Not Configured'), findsOneWidget);
      expect(find.byIcon(Icons.error), findsOneWidget);
    });
    
    testWidgets('shows configure button when not configured', (WidgetTester tester) async {
      final aiProvider = {
        'name': 'Test Provider',
        'type': 'OpenAI',
        'modelName': 'gpt-4',
        'isConfigured': false,
        'isActive': false,
        'lastUsed': null,
      };
      
      await tester.pumpWidget(createTestWidget(aiProvider));
      
      expect(find.text('Configure'), findsOneWidget);
      expect(find.text('Test'), findsNothing);
      expect(find.text('Edit'), findsNothing);
    });
    
    testWidgets('shows test and edit buttons when configured', (WidgetTester tester) async {
      final aiProvider = {
        'name': 'Test Provider',
        'type': 'OpenAI',
        'modelName': 'gpt-4',
        'isConfigured': true,
        'isActive': false,
        'lastUsed': null,
      };
      
      await tester.pumpWidget(createTestWidget(aiProvider));
      
      expect(find.text('Configure'), findsNothing);
      expect(find.text('Test'), findsOneWidget);
      expect(find.text('Edit'), findsOneWidget);
    });
    
    testWidgets('displays last used time when available', (WidgetTester tester) async {
      final aiProvider = {
        'name': 'Test Provider',
        'type': 'OpenAI',
        'modelName': 'gpt-4',
        'isConfigured': true,
        'isActive': true,
        'lastUsed': '2h ago',
      };
      
      await tester.pumpWidget(createTestWidget(aiProvider));
      
      expect(find.text('Last used: 2h ago'), findsOneWidget);
    });
    
    testWidgets('has toggle switch', (WidgetTester tester) async {
      final aiProvider = {
        'name': 'Test Provider',
        'type': 'OpenAI',
        'modelName': 'gpt-4',
        'isConfigured': true,
        'isActive': true,
        'lastUsed': null,
      };
      
      await tester.pumpWidget(createTestWidget(aiProvider));
      
      expect(find.byType(Switch), findsOneWidget);
    });
  });
}
