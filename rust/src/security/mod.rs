use crate::error::BullShiftError;
use crate::logging::{LogLevel, Logger, StructuredLogger};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureCredentials {
    pub api_key: String,
    pub api_secret: String,
    pub broker: String,
    pub encrypted_data: Vec<u8>,
    pub nonce: Vec<u8>,
}

pub struct SecurityManager {
    key_bytes: Vec<u8>,
    rng: SystemRandom,
    credential_store: HashMap<String, SecureCredentials>,
    logger: StructuredLogger,
    nonce_counter: std::sync::atomic::AtomicU64,
}

impl SecurityManager {
    pub fn new() -> Result<Self, BullShiftError> {
        let key_material = Self::derive_key_from_system()?;

        Ok(Self {
            key_bytes: key_material,
            rng: SystemRandom::new(),
            credential_store: HashMap::new(),
            logger: StructuredLogger::new("security_manager".to_string(), LogLevel::Info),
            nonce_counter: std::sync::atomic::AtomicU64::new(0),
        })
    }

    fn derive_key_from_system() -> Result<Vec<u8>, BullShiftError> {
        // Use platform-specific secure key derivation
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let output = Command::new("security")
                .args(["find-generic-password", "-wa", "BullShift_Encryption_Key"])
                .output()
                .map_err(|e| {
                    BullShiftError::Keychain(format!("Failed to access keychain: {}", e))
                })?;

            if output.status.success() {
                let key_data = String::from_utf8(output.stdout)
                    .map_err(|e| BullShiftError::Security(format!("Invalid key data: {}", e)))?;
                Ok(key_data.as_bytes()[0..32].to_vec())
            } else {
                // Generate and store new key
                let rng = SystemRandom::new();
                let mut key_bytes = [0u8; 32];
                rng.fill(&mut key_bytes).map_err(|e| {
                    BullShiftError::Encryption(format!("Failed to generate key: {}", e))
                })?;

                let key_hex = hex::encode(key_bytes);
                Command::new("security")
                    .args([
                        "add-generic-password",
                        "-a",
                        "BullShift",
                        "-s",
                        "BullShift_Encryption_Key",
                        "-w",
                        &key_hex,
                    ])
                    .output()
                    .map_err(|e| BullShiftError::Keychain(format!("Failed to store key: {}", e)))?;

                Ok(key_bytes.to_vec())
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;

            // Try libsecret first, fall back to file-based storage
            if let Ok(output) = Command::new("secret-tool")
                .args(["lookup", "name", "BullShift_Encryption_Key"])
                .output()
            {
                if output.status.success() {
                    if let Ok(key_data) = String::from_utf8(output.stdout) {
                        if key_data.len() >= 32 {
                            return Ok(key_data.as_bytes()[0..32].to_vec());
                        }
                    }
                }

                // secret-tool exists but no key stored yet — generate and store
                let rng = SystemRandom::new();
                let mut key_bytes = [0u8; 32];
                rng.fill(&mut key_bytes).map_err(|e| {
                    BullShiftError::Encryption(format!("Failed to generate key: {}", e))
                })?;

                let key_hex = hex::encode(key_bytes);
                match Command::new("secret-tool")
                    .args([
                        "store",
                        "--label=BullShift Encryption Key",
                        "name",
                        "BullShift_Encryption_Key",
                        "password",
                        &key_hex,
                    ])
                    .output()
                {
                    Ok(output) if !output.status.success() => {
                        log::warn!("secret-tool store failed (exit {}), key not persisted", output.status);
                    }
                    Err(e) => {
                        log::warn!("secret-tool store failed: {}, key not persisted", e);
                    }
                    _ => {}
                }

                return Ok(key_bytes.to_vec());
            }

            // secret-tool not available — fall back to file-based key storage
            Self::derive_key_from_file()
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            Self::derive_key_from_file()
        }
    }

    fn derive_key_from_file() -> Result<Vec<u8>, BullShiftError> {
        use std::fs;

        let home_dir = dirs::home_dir()
            .ok_or_else(|| BullShiftError::Security("Cannot find home directory".to_string()))?;
        let key_file = home_dir.join(".bullshift").join(".encryption_key");

        if key_file.exists() {
            let key_data = fs::read(&key_file)
                .map_err(|e| BullShiftError::Security(format!("Failed to read key file: {}", e)))?;

            if key_data.len() >= 32 {
                return Ok(key_data[0..32].to_vec());
            }
        }

        // Generate and store new key
        let rng = SystemRandom::new();
        let mut key_bytes = [0u8; 32];
        rng.fill(&mut key_bytes)
            .map_err(|e| BullShiftError::Encryption(format!("Failed to generate key: {}", e)))?;

        let key_dir = key_file.parent().unwrap();
        if !key_dir.exists() {
            fs::create_dir_all(key_dir).map_err(|e| {
                BullShiftError::Security(format!("Failed to create key directory: {}", e))
            })?;
        }

        fs::write(&key_file, key_bytes)
            .map_err(|e| BullShiftError::Security(format!("Failed to write key file: {}", e)))?;

        // Set restrictive permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&key_file)
                .map_err(|e| {
                    BullShiftError::Security(format!("Failed to get file metadata: {}", e))
                })?
                .permissions();
            perms.set_mode(0o600); // Read/write for owner only
            fs::set_permissions(&key_file, perms).map_err(|e| {
                BullShiftError::Security(format!("Failed to set file permissions: {}", e))
            })?;
        }

        Ok(key_bytes.to_vec())
    }

    pub fn store_credentials(
        &mut self,
        broker: String,
        api_key: String,
        api_secret: String,
    ) -> Result<(), BullShiftError> {
        let credential_data = format!("{}:{}", api_key, api_secret);
        let credential_bytes = credential_data.as_bytes();

        let mut nonce_bytes = [0u8; 12];
        self.rng
            .fill(&mut nonce_bytes)
            .map_err(|e| BullShiftError::Encryption(format!("Failed to generate nonce: {}", e)))?;

        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.key_bytes)
            .map_err(|e| BullShiftError::Encryption(format!("Failed to create key: {}", e)))?;
        let sealing_key = LessSafeKey::new(unbound_key);

        let mut encrypted_data = credential_bytes.to_vec();

        sealing_key
            .seal_in_place_append_tag(nonce, Aad::empty(), &mut encrypted_data)
            .map_err(|e| BullShiftError::Encryption(format!("Encryption failed: {}", e)))?;

        let secure_credentials = SecureCredentials {
            api_key: "*".repeat(api_key.len()),
            api_secret: "*".repeat(api_secret.len()),
            broker: broker.clone(),
            encrypted_data,
            nonce: nonce_bytes.to_vec(),
        };

        self.credential_store
            .insert(broker.clone(), secure_credentials);

        self.logger.log(
            LogLevel::Info,
            "security",
            &format!("Securely stored credentials for broker: {}", broker),
        );
        Ok(())
    }

    pub fn get_credentials(&self, broker: &str) -> Result<(String, String), BullShiftError> {
        let credentials = self
            .credential_store
            .get(broker)
            .ok_or_else(|| {
                BullShiftError::Security(format!("No credentials found for broker: {}", broker))
            })?
            .clone();

        // Decrypt credentials
        let nonce = Nonce::assume_unique_for_key(
            credentials
                .nonce
                .clone()
                .try_into()
                .map_err(|_| BullShiftError::Encryption("Invalid nonce length".to_string()))?,
        );

        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.key_bytes)
            .map_err(|e| BullShiftError::Encryption(format!("Failed to create key: {}", e)))?;
        let sealing_key = LessSafeKey::new(unbound_key);
        let mut decrypted_data = credentials.encrypted_data.clone();

        let decrypted_bytes = sealing_key
            .open_in_place(nonce, Aad::empty(), &mut decrypted_data)
            .map_err(|e| BullShiftError::Encryption(format!("Decryption failed: {}", e)))?;

        let credential_string = String::from_utf8(decrypted_bytes.to_vec())
            .map_err(|e| BullShiftError::Security(format!("Failed to parse credentials: {}", e)))?;

        let mut parts = credential_string.splitn(2, ':');
        let api_key = parts
            .next()
            .ok_or_else(|| BullShiftError::Security("Invalid credential format".to_string()))?
            .to_string();
        let api_secret = parts
            .next()
            .ok_or_else(|| BullShiftError::Security("Invalid credential format".to_string()))?
            .to_string();

        Ok((api_key, api_secret))
    }

    pub fn list_brokers(&self) -> Vec<String> {
        self.credential_store.keys().cloned().collect()
    }

    pub fn remove_credentials(&mut self, broker: &str) -> Result<(), BullShiftError> {
        if self.credential_store.remove(broker).is_some() {
            log::info!("Removed credentials for broker: {}", broker);
            Ok(())
        } else {
            Err(BullShiftError::Security(format!(
                "No credentials found for broker: {}",
                broker
            )))
        }
    }

    pub fn validate_credentials(&self, broker: &str) -> Result<bool, BullShiftError> {
        match self.get_credentials(broker) {
            Ok((api_key, api_secret)) => {
                // Basic validation - in production would validate with broker
                let is_valid = !api_key.is_empty() && !api_secret.is_empty() && api_key.len() > 10;
                Ok(is_valid)
            }
            Err(_) => Ok(false),
        }
    }

    /// Store an encrypted API key for an AI provider, keyed by provider name.
    pub fn store_api_key(
        &mut self,
        provider_name: &str,
        api_key: &str,
    ) -> Result<(), BullShiftError> {
        let encrypted = self.encrypt_sensitive_data(api_key)?;
        let key = format!("ai_provider:{}", provider_name);
        // Store as a credential with the encrypted key in the api_key field
        let cred = SecureCredentials {
            api_key: "*".repeat(api_key.len().min(32)),
            api_secret: String::new(),
            broker: key.clone(),
            encrypted_data: encrypted.into_bytes(),
            nonce: Vec::new(), // nonce is embedded in the encrypted hex string
        };
        self.credential_store.insert(key, cred);
        self.logger.log(
            LogLevel::Info,
            "security",
            &format!(
                "Stored encrypted API key for AI provider: {}",
                provider_name
            ),
        );
        Ok(())
    }

    /// Retrieve and decrypt an AI provider API key by provider name.
    pub fn get_api_key(&self, provider_name: &str) -> Result<String, BullShiftError> {
        let key = format!("ai_provider:{}", provider_name);
        let cred = self.credential_store.get(&key).ok_or_else(|| {
            BullShiftError::Security(format!("No API key found for provider: {}", provider_name))
        })?;
        let encrypted_hex = String::from_utf8(cred.encrypted_data.clone())
            .map_err(|e| BullShiftError::Security(format!("Invalid stored key data: {}", e)))?;
        self.decrypt_sensitive_data(&encrypted_hex)
    }

    /// Check if an API key is stored for a given AI provider.
    pub fn has_api_key(&self, provider_name: &str) -> bool {
        let key = format!("ai_provider:{}", provider_name);
        self.credential_store.contains_key(&key)
    }

    /// Remove a stored AI provider API key.
    pub fn remove_api_key(&mut self, provider_name: &str) -> Result<(), BullShiftError> {
        let key = format!("ai_provider:{}", provider_name);
        if self.credential_store.remove(&key).is_some() {
            log::info!("Removed API key for AI provider: {}", provider_name);
            Ok(())
        } else {
            Err(BullShiftError::Security(format!(
                "No API key found for provider: {}",
                provider_name
            )))
        }
    }

    pub fn encrypt_sensitive_data(&self, data: &str) -> Result<String, BullShiftError> {
        let data_bytes = data.as_bytes();

        // Generate nonce using counter (8 bytes) + random (4 bytes) for uniqueness
        let counter = self
            .nonce_counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[..8].copy_from_slice(&counter.to_le_bytes());
        self.rng
            .fill(&mut nonce_bytes[8..])
            .map_err(|e| BullShiftError::Encryption(format!("Failed to generate nonce: {}", e)))?;

        // Encrypt data
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.key_bytes)
            .map_err(|e| BullShiftError::Encryption(format!("Failed to create key: {}", e)))?;
        let sealing_key = LessSafeKey::new(unbound_key);

        let mut in_out = data_bytes.to_vec();

        sealing_key
            .seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
            .map_err(|e| BullShiftError::Encryption(format!("Encryption failed: {}", e)))?;

        // Combine nonce and encrypted data (ciphertext + tag) for storage
        let mut combined = nonce_bytes.to_vec();
        combined.extend_from_slice(&in_out);

        // Encode as hex for safe storage
        Ok(hex::encode(combined))
    }

    pub fn decrypt_sensitive_data(&self, encrypted_data: &str) -> Result<String, BullShiftError> {
        let combined = hex::decode(encrypted_data).map_err(|e| {
            BullShiftError::Encryption(format!("Failed to decode encrypted data: {}", e))
        })?;

        if combined.len() < 12 {
            return Err(BullShiftError::Encryption(
                "Invalid encrypted data format".to_string(),
            ));
        }

        // Extract nonce and encrypted data
        let nonce_bytes = &combined[..12];
        let encrypted_bytes = &combined[12..];

        // Decrypt data
        let nonce = Nonce::assume_unique_for_key(
            nonce_bytes
                .try_into()
                .map_err(|_| BullShiftError::Encryption("Invalid nonce length".to_string()))?,
        );

        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.key_bytes)
            .map_err(|e| BullShiftError::Encryption(format!("Failed to create key: {}", e)))?;
        let sealing_key = LessSafeKey::new(unbound_key);
        let mut decrypted_data = encrypted_bytes.to_vec();

        let decrypted_bytes = sealing_key
            .open_in_place(nonce, Aad::empty(), &mut decrypted_data)
            .map_err(|e| BullShiftError::Encryption(format!("Decryption failed: {}", e)))?;

        String::from_utf8(decrypted_bytes.to_vec()).map_err(|e| {
            BullShiftError::Encryption(format!("Failed to parse decrypted data: {}", e))
        })
    }
}

// Platform-specific secure storage integration
#[cfg(target_os = "macos")]
pub mod macos_keychain {
    use super::*;

    pub fn store_in_keychain(
        service: &str,
        account: &str,
        password: &str,
    ) -> Result<(), BullShiftError> {
        // Integration with macOS Keychain
        // Would use security framework
        log::info!("Storing in macOS Keychain: {}@{}", account, service);
        Ok(())
    }

    pub fn get_from_keychain(service: &str, account: &str) -> Result<String, BullShiftError> {
        // Integration with macOS Keychain
        log::info!("Retrieving from macOS Keychain: {}@{}", account, service);
        Err(BullShiftError::Keychain(
            "Keychain integration not implemented".to_string(),
        ))
    }
}

#[cfg(target_os = "linux")]
pub mod linux_libsecret {
    use super::*;

    pub fn store_in_libsecret(
        service: &str,
        account: &str,
        _password: &str,
    ) -> Result<(), BullShiftError> {
        // Integration with libsecret
        log::info!("Storing in libsecret: {}@{}", account, service);
        Ok(())
    }

    pub fn get_from_libsecret(service: &str, account: &str) -> Result<String, BullShiftError> {
        // Integration with libsecret
        log::info!("Retrieving from libsecret: {}@{}", account, service);
        Err(BullShiftError::Keychain(
            "Libsecret integration not implemented".to_string(),
        ))
    }
}

// Fallback for other platforms
pub mod fallback_storage {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn get_config_path() -> Result<PathBuf, BullShiftError> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| BullShiftError::Security("Cannot find home directory".to_string()))?;
        let config_dir = home_dir.join(".bullshift");

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).map_err(|e| {
                BullShiftError::Security(format!("Failed to create config directory: {}", e))
            })?;
        }

        Ok(config_dir.join("secure_credentials.json"))
    }

    pub fn store_encrypted(broker: &str, encrypted_data: &[u8]) -> Result<(), BullShiftError> {
        let config_path = get_config_path()?;
        let mut stored_data: HashMap<String, Vec<u8>> = if config_path.exists() {
            let content = fs::read_to_string(&config_path).map_err(|e| {
                BullShiftError::Security(format!("Failed to read config file: {}", e))
            })?;
            serde_json::from_str(&content).map_err(|e| {
                BullShiftError::Security(format!("Failed to parse config file: {}", e))
            })?
        } else {
            HashMap::new()
        };

        stored_data.insert(broker.to_string(), encrypted_data.to_vec());

        let json_content = serde_json::to_string_pretty(&stored_data).map_err(|e| {
            BullShiftError::Security(format!("Failed to serialize credentials: {}", e))
        })?;

        fs::write(&config_path, json_content)
            .map_err(|e| BullShiftError::Security(format!("Failed to write config file: {}", e)))?;

        // Set restrictive permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&config_path)
                .map_err(|e| {
                    BullShiftError::Security(format!("Failed to get file metadata: {}", e))
                })?
                .permissions();
            perms.set_mode(0o600); // Read/write for owner only
            fs::set_permissions(&config_path, perms).map_err(|e| {
                BullShiftError::Security(format!("Failed to set file permissions: {}", e))
            })?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    impl SecurityManager {
        fn new_for_test() -> Self {
            Self {
                key_bytes: vec![0x42u8; 32],
                rng: SystemRandom::new(),
                credential_store: HashMap::new(),
                logger: StructuredLogger::new("test_security".to_string(), LogLevel::Info),
                nonce_counter: std::sync::atomic::AtomicU64::new(0),
            }
        }
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let mgr = SecurityManager::new_for_test();
        let plaintext = "hello world secret data";
        let encrypted = mgr.encrypt_sensitive_data(plaintext).unwrap();
        let decrypted = mgr.decrypt_sensitive_data(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_different_nonces() {
        let mgr = SecurityManager::new_for_test();
        let plaintext = "same input twice";
        let enc1 = mgr.encrypt_sensitive_data(plaintext).unwrap();
        let enc2 = mgr.encrypt_sensitive_data(plaintext).unwrap();
        assert_ne!(
            enc1, enc2,
            "Two encryptions of the same plaintext should produce different ciphertexts"
        );
    }

    #[test]
    fn test_decrypt_invalid_data() {
        let mgr = SecurityManager::new_for_test();
        // Valid hex but garbage ciphertext (long enough to have a 12-byte nonce prefix)
        let garbage_hex = hex::encode(vec![0xFFu8; 64]);
        let result = mgr.decrypt_sensitive_data(&garbage_hex);
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_short_data() {
        let mgr = SecurityManager::new_for_test();
        // Hex that decodes to less than 12 bytes
        let short_hex = hex::encode(vec![0xAAu8; 8]);
        let result = mgr.decrypt_sensitive_data(&short_hex);
        assert!(result.is_err());
    }

    #[test]
    fn test_store_and_get_credentials() {
        let mut mgr = SecurityManager::new_for_test();
        mgr.store_credentials(
            "test_broker".to_string(),
            "my_api_key_12345".to_string(),
            "my_api_secret_67890".to_string(),
        )
        .unwrap();

        let (key, secret) = mgr.get_credentials("test_broker").unwrap();
        assert_eq!(key, "my_api_key_12345");
        assert_eq!(secret, "my_api_secret_67890");
    }

    #[test]
    fn test_get_credentials_not_found() {
        let mgr = SecurityManager::new_for_test();
        let result = mgr.get_credentials("nonexistent_broker");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_brokers() {
        let mut mgr = SecurityManager::new_for_test();
        mgr.store_credentials(
            "broker_a".to_string(),
            "key_aaaaaaaaaaa".to_string(),
            "secret_a".to_string(),
        )
        .unwrap();
        mgr.store_credentials(
            "broker_b".to_string(),
            "key_bbbbbbbbbbb".to_string(),
            "secret_b".to_string(),
        )
        .unwrap();

        let mut brokers = mgr.list_brokers();
        brokers.sort();
        assert_eq!(brokers, vec!["broker_a", "broker_b"]);
    }

    #[test]
    fn test_remove_credentials() {
        let mut mgr = SecurityManager::new_for_test();
        mgr.store_credentials(
            "removable".to_string(),
            "key_removable11".to_string(),
            "secret_removable".to_string(),
        )
        .unwrap();
        assert!(mgr.get_credentials("removable").is_ok());

        mgr.remove_credentials("removable").unwrap();
        assert!(mgr.get_credentials("removable").is_err());
    }

    #[test]
    fn test_remove_credentials_not_found() {
        let mut mgr = SecurityManager::new_for_test();
        let result = mgr.remove_credentials("ghost_broker");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_credentials_valid() {
        let mut mgr = SecurityManager::new_for_test();
        mgr.store_credentials(
            "valid_broker".to_string(),
            "long_enough_key".to_string(),
            "some_secret_value".to_string(),
        )
        .unwrap();
        let valid = mgr.validate_credentials("valid_broker").unwrap();
        assert!(
            valid,
            "Credentials with api_key length > 10 should validate as true"
        );
    }

    #[test]
    fn test_validate_credentials_not_found() {
        let mgr = SecurityManager::new_for_test();
        let result = mgr.validate_credentials("missing_broker").unwrap();
        assert!(
            !result,
            "Validating non-existent broker should return Ok(false)"
        );
    }

    #[test]
    fn test_store_and_get_api_key() {
        let mut mgr = SecurityManager::new_for_test();
        let original_key = "sk-test-1234567890abcdef";
        mgr.store_api_key("openai", original_key).unwrap();
        let retrieved = mgr.get_api_key("openai").unwrap();
        assert_eq!(retrieved, original_key);
    }

    #[test]
    fn test_has_api_key() {
        let mut mgr = SecurityManager::new_for_test();
        assert!(
            !mgr.has_api_key("anthropic"),
            "Should not have key before storing"
        );
        mgr.store_api_key("anthropic", "sk-ant-test-key-value")
            .unwrap();
        assert!(
            mgr.has_api_key("anthropic"),
            "Should have key after storing"
        );
    }

    #[test]
    fn test_remove_api_key() {
        let mut mgr = SecurityManager::new_for_test();
        mgr.store_api_key("deepmind", "dm-key-abcdef12345").unwrap();
        assert!(mgr.has_api_key("deepmind"));
        mgr.remove_api_key("deepmind").unwrap();
        assert!(
            !mgr.has_api_key("deepmind"),
            "Key should be gone after removal"
        );
    }
}
