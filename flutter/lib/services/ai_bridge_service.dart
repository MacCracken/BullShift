import 'package:dio/dio.dart';

/// HTTP client for the BullShift AI provider API endpoints.
///
/// Connects to the api_server binary at the configured base URL
/// (default: http://localhost:8787).
class AiBridgeService {
  final Dio _dio;
  final String baseUrl;

  AiBridgeService({this.baseUrl = 'http://localhost:8787'})
      : _dio = Dio(BaseOptions(
          baseUrl: baseUrl,
          connectTimeout: const Duration(seconds: 5),
          receiveTimeout: const Duration(seconds: 30),
        ));

  /// List all registered AI providers.
  Future<List<Map<String, dynamic>>> listProviders() async {
    final response = await _dio.get('/v1/ai/providers');
    final providers = response.data['providers'] as List;
    return providers.cast<Map<String, dynamic>>();
  }

  /// Add a new AI provider. Returns the new provider ID.
  Future<String> addProvider({
    required String name,
    required String providerType,
    required String apiEndpoint,
    required String modelName,
    String apiKey = '',
    int maxTokens = 4096,
    double temperature = 0.7,
  }) async {
    final response = await _dio.post('/v1/ai/providers', data: {
      'name': name,
      'provider_type': providerType,
      'api_endpoint': apiEndpoint,
      'model_name': modelName,
      'api_key': apiKey,
      'max_tokens': maxTokens,
      'temperature': temperature,
    });
    return response.data['id'] as String;
  }

  /// Store an encrypted API key for a provider.
  Future<bool> configureProvider({
    required String providerId,
    required String apiKey,
  }) async {
    final response = await _dio.post(
      '/v1/ai/providers/$providerId/configure',
      data: {'api_key': apiKey},
    );
    return response.data['configured'] == true;
  }

  /// Test connectivity to a provider's endpoint.
  Future<bool> testProvider(String providerId) async {
    final response = await _dio.post('/v1/ai/providers/$providerId/test');
    return response.data['connected'] == true;
  }

  /// Send a chat prompt to a provider and get the response.
  Future<Map<String, dynamic>> chat({
    required String providerId,
    required String prompt,
  }) async {
    final response = await _dio.post('/v1/ai/chat', data: {
      'provider_id': providerId,
      'prompt': prompt,
    });
    return response.data as Map<String, dynamic>;
  }
}
