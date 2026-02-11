import 'package:flutter_test/flutter_test.dart';
import 'package:bullshift/modules/bearly_managed/bearly_managed_provider.dart';

void main() {
  group('BearlyManagedProvider Tests', () {
    late BearlyManagedProvider provider;
    
    setUp(() {
      provider = BearlyManagedProvider();
    });
    
    tearDown(() {
      provider.dispose();
    });
    
    group('Initialization', () {
      test('initializes with empty lists', () {
        expect(provider.aiProviders, isEmpty);
        expect(provider.tradingStrategies, isEmpty);
        expect(provider.aiPrompts, isEmpty);
        expect(provider.aiResponses, isEmpty);
      });
      
      test('initialize populates sample data', () {
        provider.initialize();
        
        expect(provider.aiProviders, isNotEmpty);
        expect(provider.tradingStrategies, isNotEmpty);
        expect(provider.aiPrompts, isNotEmpty);
      });
    });
    
    group('AI Provider Management', () {
      test('addProvider adds a new provider', () async {
        await provider.addProvider(
          name: 'Test Provider',
          type: 'OpenAI',
          apiEndpoint: 'https://api.test.com',
          modelName: 'test-model',
        );
        
        expect(provider.aiProviders.length, 1);
        expect(provider.aiProviders.first['name'], 'Test Provider');
        expect(provider.aiProviders.first['type'], 'OpenAI');
        expect(provider.aiProviders.first['isConfigured'], false);
        expect(provider.aiProviders.first['isActive'], false);
      });
      
      test('configureProvider sets API key', () async {
        await provider.addProvider(
          name: 'Test Provider',
          type: 'OpenAI',
          apiEndpoint: 'https://api.test.com',
          modelName: 'test-model',
        );
        
        final providerId = provider.aiProviders.first['id'];
        
        await provider.configureProvider(
          providerId: providerId,
          apiKey: 'test-api-key-12345',
          organizationId: 'org-test',
        );
        
        expect(provider.aiProviders.first['isConfigured'], true);
        expect(provider.aiProviders.first['apiKey'], 'test-api-key-12345');
        expect(provider.aiProviders.first['organizationId'], 'org-test');
      });
      
      test('configureProvider throws for non-existent provider', () async {
        expect(
          () => provider.configureProvider(
            providerId: 'non-existent-id',
            apiKey: 'test-key',
          ),
          throwsException,
        );
      });
      
      test('toggleProvider activates/deactivates provider', () async {
        await provider.addProvider(
          name: 'Test Provider',
          type: 'OpenAI',
          apiEndpoint: 'https://api.test.com',
          modelName: 'test-model',
        );
        
        final providerId = provider.aiProviders.first['id'];
        
        // Initially inactive
        expect(provider.aiProviders.first['isActive'], false);
        
        // Activate
        provider.toggleProvider(providerId, true);
        expect(provider.aiProviders.first['isActive'], true);
        
        // Deactivate
        provider.toggleProvider(providerId, false);
        expect(provider.aiProviders.first['isActive'], false);
      });
      
      test('deleteProvider removes provider', () async {
        await provider.addProvider(
          name: 'Test Provider',
          type: 'OpenAI',
          apiEndpoint: 'https://api.test.com',
          modelName: 'test-model',
        );
        
        final providerId = provider.aiProviders.first['id'];
        
        provider.deleteProvider(providerId);
        
        expect(provider.aiProviders, isEmpty);
      });
    });
    
    group('Strategy Management', () {
      test('generateStrategy creates a new strategy', () async {
        // First add and configure an active provider
        await provider.addProvider(
          name: 'Test Provider',
          type: 'OpenAI',
          apiEndpoint: 'https://api.test.com',
          modelName: 'test-model',
        );
        
        final providerId = provider.aiProviders.first['id'];
        await provider.configureProvider(providerId: providerId, apiKey: 'test-key');
        provider.toggleProvider(providerId, true);
        
        await provider.generateStrategy(
          name: 'Test Strategy',
          type: 'Momentum',
          symbols: ['AAPL', 'MSFT'],
          timeframe: '1D',
          riskLevel: 'Medium',
          providerId: providerId,
        );
        
        expect(provider.tradingStrategies.length, 1);
        expect(provider.tradingStrategies.first['name'], 'Test Strategy');
        expect(provider.tradingStrategies.first['type'], 'Momentum');
        expect(provider.tradingStrategies.first['symbols'], ['AAPL', 'MSFT']);
      });
      
      test('generateStrategy throws if provider not active', () async {
        await provider.addProvider(
          name: 'Test Provider',
          type: 'OpenAI',
          apiEndpoint: 'https://api.test.com',
          modelName: 'test-model',
        );
        
        final providerId = provider.aiProviders.first['id'];
        // Don't activate the provider
        
        expect(
          () => provider.generateStrategy(
            name: 'Test Strategy',
            type: 'Momentum',
            symbols: ['AAPL'],
            timeframe: '1D',
            riskLevel: 'Medium',
            providerId: providerId,
          ),
          throwsException,
        );
      });
      
      test('activateStrategy activates strategy', () async {
        await provider.addProvider(
          name: 'Test Provider',
          type: 'OpenAI',
          apiEndpoint: 'https://api.test.com',
          modelName: 'test-model',
        );
        
        final providerId = provider.aiProviders.first['id'];
        await provider.configureProvider(providerId: providerId, apiKey: 'test-key');
        provider.toggleProvider(providerId, true);
        
        await provider.generateStrategy(
          name: 'Test Strategy',
          type: 'Momentum',
          symbols: ['AAPL'],
          timeframe: '1D',
          riskLevel: 'Medium',
          providerId: providerId,
        );
        
        final strategyId = provider.tradingStrategies.first['id'];
        
        // Initially inactive
        expect(provider.tradingStrategies.first['isActive'], false);
        
        provider.activateStrategy(strategyId);
        expect(provider.tradingStrategies.first['isActive'], true);
      });
      
      test('deleteStrategy removes strategy', () async {
        await provider.addProvider(
          name: 'Test Provider',
          type: 'OpenAI',
          apiEndpoint: 'https://api.test.com',
          modelName: 'test-model',
        );
        
        final providerId = provider.aiProviders.first['id'];
        await provider.configureProvider(providerId: providerId, apiKey: 'test-key');
        provider.toggleProvider(providerId, true);
        
        await provider.generateStrategy(
          name: 'Test Strategy',
          type: 'Momentum',
          symbols: ['AAPL'],
          timeframe: '1D',
          riskLevel: 'Medium',
          providerId: providerId,
        );
        
        final strategyId = provider.tradingStrategies.first['id'];
        
        provider.deleteStrategy(strategyId);
        
        expect(provider.tradingStrategies, isEmpty);
      });
    });
    
    group('Prompt Management', () {
      test('addPrompt adds a new prompt', () async {
        await provider.addPrompt(
          name: 'Test Prompt',
          category: 'Analysis',
          prompt: 'Analyze {{symbol}} for trading opportunities',
          description: 'A test prompt',
        );
        
        expect(provider.aiPrompts.length, 1);
        expect(provider.aiPrompts.first['name'], 'Test Prompt');
        expect(provider.aiPrompts.first['category'], 'Analysis');
        expect(provider.aiPrompts.first['prompt'], 'Analyze {{symbol}} for trading opportunities');
      });
      
      test('updatePrompt updates existing prompt', () async {
        await provider.addPrompt(
          name: 'Test Prompt',
          category: 'Analysis',
          prompt: 'Analyze {{symbol}}',
        );
        
        final promptId = provider.aiPrompts.first['id'];
        
        await provider.updatePrompt(
          promptId: promptId,
          name: 'Updated Prompt',
          category: 'Strategy',
          prompt: 'New prompt content',
        );
        
        expect(provider.aiPrompts.first['name'], 'Updated Prompt');
        expect(provider.aiPrompts.first['category'], 'Strategy');
        expect(provider.aiPrompts.first['prompt'], 'New prompt content');
      });
      
      test('deletePrompt removes prompt', () async {
        await provider.addPrompt(
          name: 'Test Prompt',
          category: 'Analysis',
          prompt: 'Analyze {{symbol}}',
        );
        
        final promptId = provider.aiPrompts.first['id'];
        
        provider.deletePrompt(promptId);
        
        expect(provider.aiPrompts, isEmpty);
      });
    });
    
    group('Execute Prompt', () {
      test('executePrompt returns response', () async {
        // Add an active provider
        await provider.addProvider(
          name: 'Test Provider',
          type: 'OpenAI',
          apiEndpoint: 'https://api.test.com',
          modelName: 'test-model',
        );
        
        final providerId = provider.aiProviders.first['id'];
        await provider.configureProvider(providerId: providerId, apiKey: 'test-key');
        provider.toggleProvider(providerId, true);
        
        // Add a prompt
        await provider.addPrompt(
          name: 'Test Prompt',
          category: 'Analysis',
          prompt: 'Analyze {{symbol}}',
        );
        
        final promptId = provider.aiPrompts.first['id'];
        
        final response = await provider.executePrompt(
          promptId: promptId,
          symbol: 'AAPL',
          providerId: providerId,
        );
        
        expect(response, isNotEmpty);
        expect(provider.aiResponses, isNotEmpty);
        expect(provider.aiResponses.first['symbol'], 'AAPL');
      });
    });
    
    group('Data Structure Validation', () {
      test('AI provider has all required fields', () async {
        await provider.addProvider(
          name: 'Test Provider',
          type: 'OpenAI',
          apiEndpoint: 'https://api.test.com',
          modelName: 'test-model',
        );
        
        final aiProvider = provider.aiProviders.first;
        expect(aiProvider.containsKey('id'), true);
        expect(aiProvider.containsKey('name'), true);
        expect(aiProvider.containsKey('type'), true);
        expect(aiProvider.containsKey('apiEndpoint'), true);
        expect(aiProvider.containsKey('modelName'), true);
        expect(aiProvider.containsKey('isConfigured'), true);
        expect(aiProvider.containsKey('isActive'), true);
        expect(aiProvider.containsKey('createdAt'), true);
      });
      
      test('Strategy has all required fields', () async {
        await provider.addProvider(
          name: 'Test Provider',
          type: 'OpenAI',
          apiEndpoint: 'https://api.test.com',
          modelName: 'test-model',
        );
        
        final providerId = provider.aiProviders.first['id'];
        await provider.configureProvider(providerId: providerId, apiKey: 'test-key');
        provider.toggleProvider(providerId, true);
        
        await provider.generateStrategy(
          name: 'Test Strategy',
          type: 'Momentum',
          symbols: ['AAPL'],
          timeframe: '1D',
          riskLevel: 'Medium',
          providerId: providerId,
        );
        
        final strategy = provider.tradingStrategies.first;
        expect(strategy.containsKey('id'), true);
        expect(strategy.containsKey('name'), true);
        expect(strategy.containsKey('type'), true);
        expect(strategy.containsKey('description'), true);
        expect(strategy.containsKey('symbols'), true);
        expect(strategy.containsKey('timeframe'), true);
        expect(strategy.containsKey('riskLevel'), true);
        expect(strategy.containsKey('isActive'), true);
      });
    });
  });
}
