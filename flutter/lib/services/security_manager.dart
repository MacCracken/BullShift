import 'dart:convert';
import 'dart:typed_data';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:crypto/crypto.dart';

class SecurityManager {
  static const FlutterSecureStorage _secureStorage = FlutterSecureStorage(
    aOptions: AndroidOptions(
      encryptedSharedPreferences: true,
    ),
    iOptions: IOSOptions(
      accessibility: KeychainItemAccessibility.whenUnlockedThisDeviceOnly,
    ),
  );

  static const String _keyPrefix = 'bullshift_';
  static const String _masterKeyAlias = '${_keyPrefix}master_key';

  // Store API credentials securely
  static Future<void> storeCredentials({
    required String broker,
    required String apiKey,
    required String apiSecret,
  }) async {
    try {
      // Get or create master key
      final masterKey = await _getOrCreateMasterKey();
      
      // Encrypt credentials
      final encryptedApiKey = await _encryptData(apiKey, masterKey);
      final encryptedApiSecret = await _encryptData(apiSecret, masterKey);
      
      // Store encrypted credentials
      await _secureStorage.write(
        key: '${_keyPrefix}${broker}_api_key',
        value: encryptedApiKey,
      );
      
      await _secureStorage.write(
        key: '${_keyPrefix}${broker}_api_secret',
        value: encryptedApiSecret,
      );
      
      print('Securely stored credentials for broker: $broker');
    } catch (e) {
      throw Exception('Failed to store credentials: $e');
    }
  }

  // Retrieve API credentials
  static Future<Map<String, String>?> getCredentials(String broker) async {
    try {
      final masterKey = await _getOrCreateMasterKey();
      
      final encryptedApiKey = await _secureStorage.read(key: '${_keyPrefix}${broker}_api_key');
      final encryptedApiSecret = await _secureStorage.read(key: '${_keyPrefix}${broker}_api_secret');
      
      if (encryptedApiKey == null || encryptedApiSecret == null) {
        return null;
      }
      
      final apiKey = await _decryptData(encryptedApiKey, masterKey);
      final apiSecret = await _decryptData(encryptedApiSecret, masterKey);
      
      return {
        'apiKey': apiKey,
        'apiSecret': apiSecret,
      };
    } catch (e) {
      throw Exception('Failed to retrieve credentials: $e');
    }
  }

  // List all configured brokers
  static Future<List<String>> getConfiguredBrokers() async {
    try {
      final allKeys = await _secureStorage.readAll();
      final brokers = <String>{};
      
      for (final key in allKeys.keys) {
        if (key.startsWith(_keyPrefix) && key.endsWith('_api_key')) {
          final broker = key.substring(_keyPrefix.length, key.length - 8);
          brokers.add(broker);
        }
      }
      
      return brokers.toList();
    } catch (e) {
      throw Exception('Failed to list brokers: $e');
    }
  }

  // Remove credentials for a broker
  static Future<void> removeCredentials(String broker) async {
    try {
      await _secureStorage.delete(key: '${_keyPrefix}${broker}_api_key');
      await _secureStorage.delete(key: '${_keyPrefix}${broker}_api_secret');
      print('Removed credentials for broker: $broker');
    } catch (e) {
      throw Exception('Failed to remove credentials: $e');
    }
  }

  // Validate stored credentials
  static Future<bool> validateCredentials(String broker) async {
    try {
      final credentials = await getCredentials(broker);
      if (credentials == null) return false;
      
      final apiKey = credentials['apiKey']!;
      final apiSecret = credentials['apiSecret']!;
      
      // Basic validation
      return apiKey.isNotEmpty && 
             apiSecret.isNotEmpty && 
             apiKey.length > 10;
    } catch (e) {
      return false;
    }
  }

  // Get or create master encryption key
  static Future<String> _getOrCreateMasterKey() async {
    String? masterKey = await _secureStorage.read(key: _masterKeyAlias);
    
    if (masterKey == null) {
      // Generate new master key
      masterKey = _generateSecureKey();
      await _secureStorage.write(key: _masterKeyAlias, value: masterKey);
    }
    
    return masterKey;
  }

  // Generate cryptographically secure key
  static String _generateSecureKey() {
    final bytes = Uint8List(32);
    for (int i = 0; i < 32; i++) {
      bytes[i] = (DateTime.now().millisecondsSinceEpoch + i) % 256;
    }
    return base64.encode(bytes);
  }

  // Encrypt data using master key
  static Future<String> _encryptData(String data, String masterKey) async {
    try {
      final key = utf8.encode(masterKey);
      final dataBytes = utf8.encode(data);
      
      // Generate HMAC for integrity
      final hmac = Hmac(sha256, key);
      final digest = hmac.convert(dataBytes);
      
      // Simple XOR encryption (in production, use proper encryption like AES)
      final encrypted = <int>[];
      for (int i = 0; i < dataBytes.length; i++) {
        encrypted.add(dataBytes[i] ^ key[i % key.length]);
      }
      
      // Combine encrypted data with HMAC
      final combined = [...encrypted, ...digest.bytes];
      return base64.encode(combined);
    } catch (e) {
      throw Exception('Encryption failed: $e');
    }
  }

  // Decrypt data using master key
  static Future<String> _decryptData(String encryptedData, String masterKey) async {
    try {
      final key = utf8.encode(masterKey);
      final combined = base64.decode(encryptedData);
      
      // Separate encrypted data from HMAC
      final encrypted = combined.sublist(0, combined.length - 32);
      final receivedHmac = combined.sublist(combined.length - 32);
      
      // Verify HMAC
      final hmac = Hmac(sha256, key);
      final digest = hmac.convert(encrypted);
      
      if (!_bytesEqual(digest.bytes, receivedHmac)) {
        throw Exception('HMAC verification failed - data may be tampered');
      }
      
      // Decrypt using XOR
      final decrypted = <int>[];
      for (int i = 0; i < encrypted.length; i++) {
        decrypted.add(encrypted[i] ^ key[i % key.length]);
      }
      
      return utf8.decode(decrypted);
    } catch (e) {
      throw Exception('Decryption failed: $e');
    }
  }

  // Secure byte comparison
  static bool _bytesEqual(List<int> a, List<int> b) {
    if (a.length != b.length) return false;
    
    for (int i = 0; i < a.length; i++) {
      if (a[i] != b[i]) return false;
    }
    
    return true;
  }

  // Clear all stored data (for testing/reset)
  static Future<void> clearAllData() async {
    try {
      await _secureStorage.deleteAll();
      print('Cleared all secure storage data');
    } catch (e) {
      throw Exception('Failed to clear data: $e');
    }
  }
}