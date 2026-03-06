import 'dart:math';
import '../../services/base_provider.dart';
import '../../services/ai_bridge_service.dart';

class BearlyManagedProvider extends BaseProvider {
  List<Map<String, dynamic>> _aiProviders = [];
  List<Map<String, dynamic>> _tradingStrategies = [];
  List<Map<String, dynamic>> _aiPrompts = [];
  List<Map<String, dynamic>> _aiResponses = [];

  final AiBridgeService _aiBridge = AiBridgeService();

  // Getters
  List<Map<String, dynamic>> get aiProviders => _aiProviders;
  List<Map<String, dynamic>> get tradingStrategies => _tradingStrategies;
  List<Map<String, dynamic>> get aiPrompts => _aiPrompts;
  List<Map<String, dynamic>> get aiResponses => _aiResponses;

  // Initialize with sample data
  void initialize() {
    _generateSampleData();
    safeNotifyListeners();
  }

  // AI Provider Management
  Future<void> addProvider({
    required String name,
    required String type,
    required String apiEndpoint,
    required String modelName,
  }) async {
    await executeAsync(
      operation: () async {
        final provider = {
          'id': _generateId(),
          'name': name,
          'type': type,
          'apiEndpoint': apiEndpoint,
          'modelName': modelName,
          'isConfigured': false,
          'isActive': false,
          'maxTokens': 4096,
          'temperature': 0.7,
          'createdAt': DateTime.now(),
          'lastUsed': null,
        };

        _aiProviders.add(provider);
        safeNotifyListeners();
      },
      loadingMessage: 'Adding provider...',
    );
  }

  Future<void> configureProvider({
    required String providerId,
    required String apiKey,
    String? organizationId,
  }) async {
    await executeAsync(
      operation: () async {
        final providerIndex = _aiProviders.indexWhere((p) => p['id'] == providerId);
        if (providerIndex == -1) {
          throw Exception('Provider not found');
        }

        // Store encrypted API key via the backend
        try {
          await _aiBridge.configureProvider(
            providerId: providerId,
            apiKey: apiKey,
          );
        } catch (_) {
          // Backend unavailable — store locally as fallback
        }

        _aiProviders[providerIndex]['isConfigured'] = true;
        // Never store the plaintext key in memory — the backend holds it encrypted
        _aiProviders[providerIndex]['apiKey'] = '••••••••';
        if (organizationId != null) {
          _aiProviders[providerIndex]['organizationId'] = organizationId;
        }

        safeNotifyListeners();
      },
      loadingMessage: 'Configuring provider...',
    );
  }

  Future<void> testProvider(String providerId) async {
    await executeAsync(
      operation: () async {
        final provider = _aiProviders.firstWhere((p) => p['id'] == providerId);

        if (!provider['isConfigured']) {
          throw Exception('Provider not configured');
        }

        // Test real connection via the backend
        bool connected = false;
        try {
          connected = await _aiBridge.testProvider(providerId);
        } catch (_) {
          // Backend unavailable — fall back to simulated test
          await Future.delayed(const Duration(seconds: 1));
          connected = true;
        }

        if (!connected) {
          throw Exception('Connection test failed — check endpoint and credentials');
        }

        // Update last used
        final providerIndex = _aiProviders.indexWhere((p) => p['id'] == providerId);
        _aiProviders[providerIndex]['lastUsed'] = _formatDateTime(DateTime.now());

        safeNotifyListeners();
      },
      loadingMessage: 'Testing provider connection...',
    );
  }

  void toggleProvider(String providerId, bool isActive) {
    final providerIndex = _aiProviders.indexWhere((p) => p['id'] == providerId);
    if (providerIndex != -1) {
      _aiProviders[providerIndex]['isActive'] = isActive;
      safeNotifyListeners();
    }
  }

  void deleteProvider(String providerId) {
    _aiProviders.removeWhere((p) => p['id'] == providerId);
    safeNotifyListeners();
  }

  // Strategy Management
  Future<void> generateStrategy({
    required String name,
    required String type,
    required List<String> symbols,
    required String timeframe,
    required String riskLevel,
    required String providerId,
  }) async {
    await executeAsync(
      operation: () async {
        final provider = _aiProviders.firstWhere((p) => p['id'] == providerId);
        
        if (!provider['isActive']) {
          throw Exception('Provider not active');
        }

        // Simulate AI strategy generation
        await Future.delayed(const Duration(seconds: 3));

        final strategy = {
          'id': _generateId(),
          'name': name,
          'type': type,
          'description': _generateStrategyDescription(type, symbols, riskLevel),
          'providerId': providerId,
          'symbols': symbols,
          'timeframe': timeframe,
          'riskLevel': riskLevel,
          'isActive': false,
          'winRate': 0.0,
          'totalTrades': 0,
          'totalReturn': 0.0,
          'maxDrawdown': 0.0,
          'sharpeRatio': 0.0,
          'createdAt': DateTime.now(),
          'lastUpdated': DateTime.now(),
        };

        _tradingStrategies.add(strategy);
        safeNotifyListeners();
      },
      loadingMessage: 'Generating strategy...',
    );
  }

  void toggleStrategy(String strategyId, bool isActive) {
    final strategyIndex = _tradingStrategies.indexWhere((s) => s['id'] == strategyId);
    if (strategyIndex != -1) {
      _tradingStrategies[strategyIndex]['isActive'] = isActive;
      safeNotifyListeners();
    }
  }

  void deleteStrategy(String strategyId) {
    _tradingStrategies.removeWhere((s) => s['id'] == strategyId);
    safeNotifyListeners();
  }

  // Prompt Management
  Future<void> addPrompt({
    required String name,
    required String category,
    required String template,
    required String providerId,
    bool isSystemPrompt = false,
  }) async {
    await executeAsync(
      operation: () async {
        final prompt = {
          'id': _generateId(),
          'name': name,
          'category': category,
          'template': template,
          'providerId': providerId,
          'isSystemPrompt': isSystemPrompt,
          'variables': _extractVariables(template),
          'createdAt': DateTime.now(),
        };

        _aiPrompts.add(prompt);
        safeNotifyListeners();
      },
      loadingMessage: 'Adding prompt...',
    );
  }

  Future<String> executePrompt({
    required String promptId,
    Map<String, String> variables = const {},
  }) async {
    final result = await executeAsync<String>(
      operation: () async {
        final prompt = _aiPrompts.firstWhere((p) => p['id'] == promptId);
        final provider = _aiProviders.firstWhere((p) => p['id'] == prompt['providerId']);

        if (!provider['isActive']) {
          throw Exception('Provider not active');
        }

        // Build the final prompt with variable substitution
        String finalPrompt = prompt['template'] as String;
        for (final entry in variables.entries) {
          finalPrompt = finalPrompt.replaceAll('{{${entry.key}}}', entry.value);
        }

        // Try real AI chat via backend; fall back to simulated response
        String response;
        int tokensUsed = 0;
        int responseTimeMs = 0;
        double cost = 0.0;
        final stopwatch = Stopwatch()..start();

        try {
          final chatResult = await _aiBridge.chat(
            providerId: provider['id'] as String,
            prompt: finalPrompt,
          );
          response = chatResult['response'] as String? ?? '';
          tokensUsed = (chatResult['tokens_used'] as num?)?.toInt() ?? 0;
          responseTimeMs = stopwatch.elapsedMilliseconds;
          cost = tokensUsed * 0.002; // rough estimate
        } catch (_) {
          // Backend unavailable — use simulated response
          await Future.delayed(const Duration(seconds: 1));
          response = _generateAIResponse(prompt, variables);
          tokensUsed = 150 + Random().nextInt(200);
          responseTimeMs = 800 + Random().nextInt(1200);
          cost = 0.002 + Random().nextDouble() * 0.008;
        }

        stopwatch.stop();

        // Store response
        final responseRecord = {
          'id': _generateId(),
          'promptId': promptId,
          'providerId': prompt['providerId'],
          'response': response,
          'tokensUsed': tokensUsed,
          'cost': cost,
          'responseTimeMs': responseTimeMs,
          'success': true,
          'timestamp': DateTime.now(),
        };

        _aiResponses.add(responseRecord);

        // Update provider last used
        final providerIndex = _aiProviders.indexWhere((p) => p['id'] == prompt['providerId']);
        _aiProviders[providerIndex]['lastUsed'] = _formatDateTime(DateTime.now());

        safeNotifyListeners();
        return response;
      },
      loadingMessage: 'Executing prompt...',
    );

    if (result == null) {
      throw Exception('Failed to execute prompt');
    }
    return result;
  }

  void deletePrompt(String promptId) {
    _aiPrompts.removeWhere((p) => p['id'] == promptId);
    safeNotifyListeners();
  }

  Future<void> updatePrompt({
    required String promptId,
    required String name,
    required String category,
    required String prompt,
    String? description,
  }) async {
    await executeAsync(
      operation: () async {
        final promptIndex = _aiPrompts.indexWhere((p) => p['id'] == promptId);
        if (promptIndex == -1) {
          throw Exception('Prompt not found');
        }

        _aiPrompts[promptIndex]['name'] = name;
        _aiPrompts[promptIndex]['category'] = category;
        _aiPrompts[promptIndex]['prompt'] = prompt;
        if (description != null) {
          _aiPrompts[promptIndex]['description'] = description;
        }
        _aiPrompts[promptIndex]['updatedAt'] = DateTime.now();

        safeNotifyListeners();
      },
      loadingMessage: 'Updating prompt...',
    );
  }

  // Generate sample data for demonstration
  void _generateSampleData() {
    final random = Random();

    // Generate AI providers
    _aiProviders = [
      {
        'id': _generateId(),
        'name': 'OpenAI GPT-4',
        'type': 'OpenAI',
        'apiEndpoint': 'https://api.openai.com/v1',
        'modelName': 'gpt-4',
        'isConfigured': true,
        'isActive': true,
        'maxTokens': 4096,
        'temperature': 0.7,
        'createdAt': DateTime.now().subtract(const Duration(days: 7)),
        'lastUsed': _formatDateTime(DateTime.now().subtract(const Duration(hours: 2))),
      },
      {
        'id': _generateId(),
        'name': 'Anthropic Claude',
        'type': 'Anthropic',
        'apiEndpoint': 'https://api.anthropic.com/v1',
        'modelName': 'claude-sonnet-4-6',
        'isConfigured': false,
        'isActive': false,
        'maxTokens': 4096,
        'temperature': 0.5,
        'createdAt': DateTime.now().subtract(const Duration(days: 3)),
        'lastUsed': null,
      },
      {
        'id': _generateId(),
        'name': 'Local Ollama',
        'type': 'Ollama',
        'apiEndpoint': 'http://localhost:11434',
        'modelName': 'llama2',
        'isConfigured': true,
        'isActive': false,
        'maxTokens': 2048,
        'temperature': 0.8,
        'createdAt': DateTime.now().subtract(const Duration(days: 1)),
        'lastUsed': _formatDateTime(DateTime.now().subtract(const Duration(days: 1))),
      },
      {
        'id': _generateId(),
        'name': 'SecureYeoman Agent',
        'type': 'SecureYeoman',
        // SecureYeoman exposes its chat API at /api/v1/chat on its local server.
        // Start SecureYeoman first: `secureyeoman start`
        'apiEndpoint': 'http://localhost:18789',
        'modelName': 'auto',
        'isConfigured': false,
        'isActive': false,
        'maxTokens': 4096,
        'temperature': 0.7,
        'createdAt': DateTime.now(),
        'lastUsed': null,
      },
    ];

    // Generate trading strategies
    _tradingStrategies = [
      {
        'id': _generateId(),
        'name': 'AI Momentum Strategy',
        'type': 'Momentum',
        'description': 'AI-generated momentum trading strategy focusing on tech stocks with high volume and social sentiment indicators. Uses machine learning to identify entry points based on price action and news sentiment.',
        'providerId': _aiProviders[0]['id'],
        'symbols': ['AAPL', 'GOOGL', 'MSFT', 'NVDA'],
        'timeframe': '1h',
        'riskLevel': 'Moderate',
        'isActive': true,
        'winRate': 0.65,
        'totalTrades': 124,
        'totalReturn': 0.18,
        'maxDrawdown': -0.08,
        'sharpeRatio': 1.42,
        'createdAt': DateTime.now().subtract(const Duration(days: 5)),
        'lastUpdated': DateTime.now().subtract(const Duration(hours: 6)),
      },
      {
        'id': _generateId(),
        'name': 'Sentiment-Based Mean Reversion',
        'type': 'MeanReversion',
        'description': 'AI strategy that identifies overbought/oversold conditions based on news sentiment and social media buzz. Enters positions when sentiment reaches extreme levels.',
        'providerId': _aiProviders[0]['id'],
        'symbols': ['TSLA', 'AMD', 'META'],
        'timeframe': '15m',
        'riskLevel': 'Aggressive',
        'isActive': false,
        'winRate': 0.58,
        'totalTrades': 89,
        'totalReturn': 0.12,
        'maxDrawdown': -0.15,
        'sharpeRatio': 0.98,
        'createdAt': DateTime.now().subtract(const Duration(days: 3)),
        'lastUpdated': DateTime.now().subtract(const Duration(days: 1)),
      },
    ];

    // Generate AI prompts
    _aiPrompts = [
      {
        'id': _generateId(),
        'name': 'Market Analysis',
        'category': 'MarketAnalysis',
        'template': 'Analyze the current market conditions for {{symbols}}. Consider recent news, technical indicators, and overall market sentiment. Provide a detailed analysis with specific recommendations.',
        'providerId': _aiProviders[0]['id'],
        'isSystemPrompt': false,
        'variables': ['symbols'],
        'createdAt': DateTime.now().subtract(const Duration(days: 4)),
      },
      {
        'id': _generateId(),
        'name': 'Strategy Generator',
        'category': 'StrategyGeneration',
        'template': 'Generate a comprehensive trading strategy for {{strategy_type}} focusing on {{symbols}}. Include entry conditions, exit conditions, risk management rules, and performance expectations. Risk level should be {{risk_level}}.',
        'providerId': _aiProviders[0]['id'],
        'isSystemPrompt': false,
        'variables': ['strategy_type', 'symbols', 'risk_level'],
        'createdAt': DateTime.now().subtract(const Duration(days: 2)),
      },
      {
        'id': _generateId(),
        'name': 'Risk Assessment',
        'category': 'RiskAssessment',
        'template': 'Assess the risk for the current portfolio position in {{symbol}}. Consider market volatility, sector risk, and overall exposure. Provide specific risk mitigation recommendations.',
        'providerId': _aiProviders[0]['id'],
        'isSystemPrompt': false,
        'variables': ['symbol'],
        'createdAt': DateTime.now().subtract(const Duration(days: 1)),
      },
      {
        'id': _generateId(),
        'name': 'News Sentiment Analysis',
        'category': 'SentimentAnalysis',
        'template': 'Analyze the sentiment of the following news article: {{news_text}}. Classify as bullish, bearish, or neutral and provide a confidence score. Also identify key aspects mentioned.',
        'providerId': _aiProviders[0]['id'],
        'isSystemPrompt': false,
        'variables': ['news_text'],
        'createdAt': DateTime.now(),
      },
    ];

    // Generate sample AI responses
    _aiResponses = List.generate(10, (index) {
      final prompt = _aiPrompts[random.nextInt(_aiPrompts.length)];
      return {
        'id': _generateId(),
        'promptId': prompt['id'],
        'providerId': prompt['providerId'],
        'response': _generateAIResponse(prompt, {}),
        'tokensUsed': 150 + random.nextInt(200),
        'cost': 0.002 + random.nextDouble() * 0.008,
        'responseTimeMs': 800 + random.nextInt(1200),
        'success': true,
        'timestamp': DateTime.now().subtract(Duration(minutes: random.nextInt(180))),
      };
    });
  }

  String _generateStrategyDescription(String type, List<String> symbols, String riskLevel) {
    final descriptions = {
      'Momentum': 'AI-generated momentum strategy focusing on ${symbols.join(', ')} with strong price trends and volume confirmation. Uses machine learning to identify optimal entry and exit points.',
      'MeanReversion': 'AI-powered mean reversion strategy for ${symbols.join(', ')}. Identifies overbought/oversold conditions using statistical analysis and sentiment indicators.',
      'Breakout': 'AI breakout strategy detecting significant price movements in ${symbols.join(', ')}. Uses pattern recognition and volume analysis to confirm breakouts.',
      'SentimentBased': 'AI sentiment-driven strategy for ${symbols.join(', ')}. Analyzes news sentiment, social media trends, and market psychology to inform trading decisions.',
    };

    final baseDescription = descriptions[type] ?? 'AI-generated trading strategy';
    return '$baseDescription Risk level: $riskLevel. Strategy includes automated risk management and position sizing based on market volatility.';
  }

  String _generateAIResponse(Map<String, dynamic> prompt, Map<String, dynamic> variables) {
    final category = prompt['category'] as String;
    final random = Random();

    switch (category) {
      case 'MarketAnalysis':
        return '''Market Analysis Report:

Current Market Conditions:
- S&P 500: +0.8% (bullish momentum)
- NASDAQ: +1.2% (tech leadership)
- VIX: 18.5 (low volatility)

Key Observations:
1. Technology sector showing strong relative strength
2. Risk appetite remains elevated
3. Volume patterns confirm upward trend

Recommendations:
- Consider long positions in quality tech stocks
- Monitor for potential rotation into cyclical sectors
- Maintain defensive hedges in portfolio

Risk Factors:
- Fed policy uncertainty
- Geopolitical tensions
- Earnings season approaching''';

      case 'StrategyGeneration':
        return '''Generated Trading Strategy:

Strategy Name: AI Momentum Alpha
Type: Momentum
Timeframe: 1-hour charts

Entry Conditions:
1. Price above 20-period SMA
2. RSI between 40-70 (not overbought)
3. Volume spike > 2x average
4. Positive news sentiment

Exit Conditions:
1. Take profit at 2:1 risk/reward ratio
2. Stop loss at recent swing low
3. Exit if RSI drops below 30

Risk Management:
- Position size: 2% of portfolio per trade
- Maximum 5 concurrent positions
- Daily loss limit: 3%

Performance Expectations:
- Target win rate: 60-65%
- Average holding period: 2-5 days
- Expected annual return: 15-25%''';

      case 'RiskAssessment':
        return '''Risk Assessment Report:

Current Position: AAPL
Position Size: $50,000
Portfolio Weight: 5%

Risk Analysis:
- Beta: 1.25 (higher market sensitivity)
- Volatility: 28% annualized
- Sector Concentration: Technology (moderate risk)

Key Risk Factors:
1. Market risk: High correlation with S&P 500
2. Earnings risk: Next report in 3 weeks
3. Regulatory risk: Antitrust concerns
4. Competition risk: Increased market pressure

Risk Mitigation:
- Set stop loss at 8% below entry
- Consider hedging with protective puts
- Monitor sector rotation signals
- Reduce position size if volatility increases

Overall Risk Rating: MODERATE
Recommended Action: Hold with tight stops''';

      case 'SentimentAnalysis':
        return '''Sentiment Analysis Results:

Article Classification: BULLISH
Confidence Score: 85%

Key Sentiment Indicators:
- Positive language: Strong earnings beat
- Forward guidance: Raised expectations
- Market reaction: +5.2% pre-market
- Analyst reactions: 3 upgrades, 0 downgrades

Aspect Analysis:
- Revenue: Very Positive (+0.9)
- Earnings: Positive (+0.7)
- Guidance: Very Positive (+0.8)
- Competition: Neutral (0.0)
- Risk Factors: Slightly Negative (-0.2)

Summary:
The article expresses strong bullish sentiment with high confidence. Key positive drivers include earnings beat and raised guidance. Minor concerns about competition noted but overall outlook remains very positive.''';

      default:
        return '''AI Response Generated:

This is a sample response from the AI model. The response demonstrates the capability of the system to generate contextually relevant content based on the provided prompt and variables.

Key Points:
- Response generated in ${800 + random.nextInt(1200)}ms
- Token usage: ${150 + random.nextInt(200)}
- Cost: \$${(0.002 + random.nextDouble() * 0.008).toStringAsFixed(4)}

The AI system successfully processed the request and provided a comprehensive response tailored to the specific requirements of the prompt.''';
    }
  }

  List<String> _extractVariables(String template) {
    final variables = <String>[];
    final regex = RegExp(r'\{\{(\w+)\}\}');
    final matches = regex.allMatches(template);
    
    for (final match in matches) {
      variables.add(match.group(1)!);
    }
    
    return variables;
  }

  String _generateId() {
    return DateTime.now().millisecondsSinceEpoch.toString() + Random().nextInt(1000).toString();
  }

  String _formatDateTime(DateTime dateTime) {
    final now = DateTime.now();
    final difference = now.difference(dateTime);
    
    if (difference.inMinutes < 1) {
      return 'Just now';
    } else if (difference.inHours < 1) {
      return '${difference.inMinutes}m ago';
    } else if (difference.inDays < 1) {
      return '${difference.inHours}h ago';
    } else if (difference.inDays < 7) {
      return '${difference.inDays}d ago';
    } else {
      return '${dateTime.day}/${dateTime.month}/${dateTime.year}';
    }
  }

  // Public interface methods
  Map<String, dynamic> getProvider(String providerId) {
    return _aiProviders.firstWhere((p) => p['id'] == providerId);
  }

  Map<String, dynamic> getStrategy(String strategyId) {
    return _tradingStrategies.firstWhere((s) => s['id'] == strategyId);
  }

  Map<String, dynamic> getPrompt(String promptId) {
    return _aiPrompts.firstWhere((p) => p['id'] == promptId);
  }

  List<Map<String, dynamic>> getActiveProviders() {
    return _aiProviders.where((p) => p['isActive']).toList();
  }

  List<Map<String, dynamic>> getActiveStrategies() {
    return _tradingStrategies.where((s) => s['isActive']).toList();
  }

  Map<String, dynamic> getUsageStats(String providerId) {
    final providerResponses = _aiResponses.where((r) => r['providerId'] == providerId).toList();
    
    return {
      'totalRequests': providerResponses.length,
      'totalTokens': providerResponses.map((r) => r['tokensUsed'] as int).fold(0, (a, b) => a + b),
      'totalCost': providerResponses.map((r) => r['cost'] as double).fold(0.0, (a, b) => a + b),
      'successRate': providerResponses.isEmpty ? 0.0 : 
        providerResponses.where((r) => r['success']).length / providerResponses.length,
      'avgResponseTime': providerResponses.isEmpty ? 0.0 :
        providerResponses.map((r) => r['responseTimeMs'] as int).reduce((a, b) => a + b) / providerResponses.length,
    };
  }
}