import 'dart:convert';
import 'dart:math';
import 'dart:typed_data';
import 'package:flutter/foundation.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:crypto/crypto.dart';
import 'package:encrypt/encrypt.dart' as encrypt;

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
      
      assert(() { debugPrint('Securely stored credentials for broker: $broker'); return true; }());
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
      assert(() { debugPrint('Removed credentials for broker: $broker'); return true; }());
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
      // Generate new master key using cryptographically secure random
      masterKey = _generateSecureKey();
      await _secureStorage.write(key: _masterKeyAlias, value: masterKey);
    }
    
    return masterKey;
  }

  // Generate cryptographically secure key using OS-level secure random
  static String _generateSecureKey() {
    final secureRandom = Random.secure();
    final bytes = Uint8List(32);
    
    for (int i = 0; i < 32; i++) {
      bytes[i] = secureRandom.nextInt(256);
    }
    
    return base64.encode(bytes);
  }

  // Encrypt data using AES-256-GCM
  static Future<String> _encryptData(String data, String masterKey) async {
    try {
      // Derive 32-byte key from master key using SHA-256
      final keyBytes = sha256.convert(utf8.encode(masterKey)).bytes;
      final key = encrypt.Key(Uint8List.fromList(keyBytes));
      
      // Generate cryptographically secure random IV (16 bytes for AES)
      final iv = encrypt.IV.fromSecureRandom(16);
      
      // Create encrypter with AES-256 in GCM mode
      final encrypter = encrypt.Encrypter(
        encrypt.AES(key, mode: encrypt.AESMode.gcm),
      );
      
      // Encrypt the data
      final encrypted = encrypter.encrypt(data, iv: iv);
      
      // Combine IV and encrypted data for storage
      // Format: base64(iv + encrypted)
      final combined = Uint8List(iv.bytes.length + encrypted.bytes.length);
      combined.setRange(0, iv.bytes.length, iv.bytes);
      combined.setRange(iv.bytes.length, combined.length, encrypted.bytes);
      
      return base64.encode(combined);
    } catch (e) {
      throw Exception('Encryption failed: $e');
    }
  }

  // Decrypt data using AES-256-GCM
  static Future<String> _decryptData(String encryptedData, String masterKey) async {
    try {
      // Derive 32-byte key from master key using SHA-256
      final keyBytes = sha256.convert(utf8.encode(masterKey)).bytes;
      final key = encrypt.Key(Uint8List.fromList(keyBytes));
      
      // Decode the combined data
      final combined = base64.decode(encryptedData);
      
      // Extract IV (first 16 bytes) and encrypted data
      final ivBytes = combined.sublist(0, 16);
      final encryptedBytes = combined.sublist(16);
      
      final iv = encrypt.IV(Uint8List.fromList(ivBytes));
      final encrypted = encrypt.Encrypted(Uint8List.fromList(encryptedBytes));
      
      // Create decrypter with AES-256 in GCM mode
      final encrypter = encrypt.Encrypter(
        encrypt.AES(key, mode: encrypt.AESMode.gcm),
      );
      
      // Decrypt the data
      final decrypted = encrypter.decrypt(encrypted, iv: iv);
      
      return decrypted;
    } catch (e) {
      throw Exception('Decryption failed: $e');
    }
  }

  // Clear all stored data (for testing/reset)
  static Future<void> clearAllData() async {
    try {
      await _secureStorage.deleteAll();
      assert(() { debugPrint('Cleared all secure storage data'); return true; }());
    } catch (e) {
      throw Exception('Failed to clear data: $e');
    }
  }
}
