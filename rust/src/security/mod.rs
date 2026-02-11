use ring::aead::{AES_256_GCM, LessSafeKey, Nonce, UnboundKey};
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
    encryption_key: UnboundKey,
    rng: SystemRandom,
    credential_store: HashMap<String, SecureCredentials>,
}

impl SecurityManager {
    pub fn new() -> Result<Self, String> {
        let key_material = Self::derive_key_from_system()?;
        let encryption_key = UnboundKey::new(&AES_256_GCM, &key_material)
            .map_err(|e| format!("Failed to create encryption key: {}", e))?;
        
        Ok(Self {
            encryption_key,
            rng: SystemRandom::new(),
            credential_store: HashMap::new(),
        })
    }

    fn derive_key_from_system() -> Result<Vec<u8>, String> {
        // Use platform-specific secure key derivation
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let output = Command::new("security")
                .args(&["find-generic-password", "-wa", "BullShift_Encryption_Key"])
                .output()
                .map_err(|e| format!("Failed to access keychain: {}", e))?;
            
            if output.status.success() {
                let key_data = String::from_utf8(output.stdout)
                    .map_err(|e| format!("Invalid key data: {}", e))?;
                return Ok(key_data.as_bytes()[0..32].to_vec());
            } else {
                // Generate and store new key
                let rng = SystemRandom::new();
                let mut key_bytes = [0u8; 32];
                rng.fill(&mut key_bytes)
                    .map_err(|e| format!("Failed to generate key: {}", e))?;
                
                let key_hex = hex::encode(&key_bytes);
                Command::new("security")
                    .args(&["add-generic-password", "-a", "BullShift", "-s", "BullShift_Encryption_Key", "-w", &key_hex])
                    .output()
                    .map_err(|e| format!("Failed to store key: {}", e))?;
                
                return Ok(key_bytes.to_vec());
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            let output = Command::new("secret-tool")
                .args(&["lookup", "name", "BullShift_Encryption_Key"])
                .output()
                .map_err(|e| format!("Failed to access libsecret: {}", e))?;
            
            if output.status.success() {
                let key_data = String::from_utf8(output.stdout)
                    .map_err(|e| format!("Invalid key data: {}", e))?;
                return Ok(key_data.as_bytes()[0..32].to_vec());
            } else {
                // Generate and store new key
                let rng = SystemRandom::new();
                let mut key_bytes = [0u8; 32];
                rng.fill(&mut key_bytes)
                    .map_err(|e| format!("Failed to generate key: {}", e))?;
                
                let key_hex = hex::encode(&key_bytes);
                Command::new("secret-tool")
                    .args(&["store", "--label=BullShift Encryption Key", "name", "BullShift_Encryption_Key", "password", &key_hex])
                    .output()
                    .map_err(|e| format!("Failed to store key: {}", e))?;
                
                return Ok(key_bytes.to_vec());
            }
        }
        
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            use std::fs;
            use std::path::PathBuf;
            
            let home_dir = dirs::home_dir().ok_or("Cannot find home directory")?;
            let key_file = home_dir.join(".bullshift").join(".encryption_key");
            
            if key_file.exists() {
                let key_data = fs::read(&key_file)
                    .map_err(|e| format!("Failed to read key file: {}", e))?;
                
                if key_data.len() >= 32 {
                    return Ok(key_data[0..32].to_vec());
                }
            }
            
            // Generate and store new key
            let rng = SystemRandom::new();
            let mut key_bytes = [0u8; 32];
            rng.fill(&mut key_bytes)
                .map_err(|e| format!("Failed to generate key: {}", e))?;
            
            let key_dir = key_file.parent().unwrap();
            if !key_dir.exists() {
                fs::create_dir_all(key_dir)
                    .map_err(|e| format!("Failed to create key directory: {}", e))?;
            }
            
            fs::write(&key_file, &key_bytes)
                .map_err(|e| format!("Failed to write key file: {}", e))?;
            
            // Set restrictive permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&key_file)
                    .map_err(|e| format!("Failed to get file metadata: {}", e))?
                    .permissions();
                perms.set_mode(0o600); // Read/write for owner only
                fs::set_permissions(&key_file, perms)
                    .map_err(|e| format!("Failed to set file permissions: {}", e))?;
            }
            
            return Ok(key_bytes.to_vec());
        }
    }

    pub fn store_credentials(&mut self, broker: String, api_key: String, api_secret: String) -> Result<(), String> {
        let credential_data = format!("{}:{}", api_key, api_secret);
        let credential_bytes = credential_data.as_bytes();
        
        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        self.rng.fill(&mut nonce_bytes)
            .map_err(|e| format!("Failed to generate nonce: {}", e))?;
        
        // Encrypt credentials
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let sealing_key = LessSafeKey::new(self.encryption_key.clone());
        
        let mut encrypted_data = credential_bytes.to_vec();
        encrypted_data.resize(encrypted_data.len() + AES_256_GCM.tag_len(), 0);
        
        sealing_key.seal_in_place_append_tag(nonce, &[], &mut encrypted_data)
            .map_err(|e| format!("Encryption failed: {}", e))?;
        
        let secure_credentials = SecureCredentials {
            api_key: "*".repeat(api_key.len()), // Store masked version
            api_secret: "*".repeat(api_secret.len()), // Store masked version
            broker,
            encrypted_data,
            nonce: nonce_bytes.to_vec(),
        };
        
        self.credential_store.insert(broker.clone(), secure_credentials);
        log::info!("Securely stored credentials for broker: {}", broker);
        Ok(())
    }

    pub fn get_credentials(&self, broker: &str) -> Result<(String, String), String> {
        let credentials = self.credential_store.get(broker)
            .ok_or_else(|| format!("No credentials found for broker: {}", broker))?;
        
        // Decrypt credentials
        let nonce = Nonce::assume_unique_for_key(credentials.nonce.clone().try_into()
            .map_err(|_| "Invalid nonce length".to_string())?);
        
        let sealing_key = LessSafeKey::new(self.encryption_key.clone());
        let mut decrypted_data = credentials.encrypted_data.clone();
        
        let decrypted_bytes = sealing_key.open_in_place(nonce, &[], &mut decrypted_data)
            .map_err(|e| format!("Decryption failed: {}", e))?;
        
        let credential_string = String::from_utf8(decrypted_bytes.to_vec())
            .map_err(|e| format!("Failed to parse credentials: {}", e))?;
        
        let mut parts = credential_string.splitn(2, ':');
        let api_key = parts.next().ok_or("Invalid credential format")?.to_string();
        let api_secret = parts.next().ok_or("Invalid credential format")?.to_string();
        
        Ok((api_key, api_secret))
    }

    pub fn list_brokers(&self) -> Vec<String> {
        self.credential_store.keys().cloned().collect()
    }

    pub fn remove_credentials(&mut self, broker: &str) -> Result<(), String> {
        if self.credential_store.remove(broker).is_some() {
            log::info!("Removed credentials for broker: {}", broker);
            Ok(())
        } else {
            Err(format!("No credentials found for broker: {}", broker))
        }
    }

    pub fn validate_credentials(&self, broker: &str) -> Result<bool, String> {
        match self.get_credentials(broker) {
            Ok((api_key, api_secret)) => {
                // Basic validation - in production would validate with broker
                let is_valid = !api_key.is_empty() && !api_secret.is_empty() && api_key.len() > 10;
                Ok(is_valid)
            }
            Err(_) => Ok(false),
        }
    }

    pub fn encrypt_sensitive_data(&self, data: &str) -> Result<String, String> {
        let data_bytes = data.as_bytes();
        
        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        self.rng.fill(&mut nonce_bytes)
            .map_err(|e| format!("Failed to generate nonce: {}", e))?;
        
        // Encrypt data
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let sealing_key = LessSafeKey::new(self.encryption_key.clone());
        
        let mut encrypted_data = data_bytes.to_vec();
        encrypted_data.resize(encrypted_data.len() + AES_256_GCM.tag_len(), 0);
        
        sealing_key.seal_in_place_append_tag(nonce, &[], &mut encrypted_data)
            .map_err(|e| format!("Encryption failed: {}", e))?;
        
        // Combine nonce and encrypted data for storage
        let mut combined = nonce_bytes.to_vec();
        combined.extend_from_slice(&encrypted_data);
        
        // Encode as hex for safe storage
        Ok(hex::encode(combined))
    }

    pub fn decrypt_sensitive_data(&self, encrypted_data: &str) -> Result<String, String> {
        let combined = hex::decode(encrypted_data)
            .map_err(|e| format!("Failed to decode encrypted data: {}", e))?;
        
        if combined.len() < 12 {
            return Err("Invalid encrypted data format".to_string());
        }
        
        // Extract nonce and encrypted data
        let nonce_bytes = &combined[..12];
        let encrypted_bytes = &combined[12..];
        
        // Decrypt data
        let nonce = Nonce::assume_unique_for_key(nonce_bytes.try_into()
            .map_err(|_| "Invalid nonce length".to_string())?);
        
        let sealing_key = LessSafeKey::new(self.encryption_key.clone());
        let mut decrypted_data = encrypted_bytes.to_vec();
        
        let decrypted_bytes = sealing_key.open_in_place(nonce, &[], &mut decrypted_data)
            .map_err(|e| format!("Decryption failed: {}", e))?;
        
        String::from_utf8(decrypted_bytes.to_vec())
            .map_err(|e| format!("Failed to parse decrypted data: {}", e))
    }
}

// Platform-specific secure storage integration
#[cfg(target_os = "macos")]
pub mod macos_keychain {
    use super::*;
    
    pub fn store_in_keychain(service: &str, account: &str, password: &str) -> Result<(), String> {
        // Integration with macOS Keychain
        // Would use security framework
        log::info!("Storing in macOS Keychain: {}@{}", account, service);
        Ok(())
    }
    
    pub fn get_from_keychain(service: &str, account: &str) -> Result<String, String> {
        // Integration with macOS Keychain
        log::info!("Retrieving from macOS Keychain: {}@{}", account, service);
        Err("Keychain integration not implemented".to_string())
    }
}

#[cfg(target_os = "linux")]
pub mod linux_libsecret {
    use super::*;
    
    pub fn store_in_libsecret(service: &str, account: &str, password: &str) -> Result<(), String> {
        // Integration with libsecret
        log::info!("Storing in libsecret: {}@{}", account, service);
        Ok(())
    }
    
    pub fn get_from_libsecret(service: &str, account: &str) -> Result<String, String> {
        // Integration with libsecret
        log::info!("Retrieving from libsecret: {}@{}", account, service);
        Err("Libsecret integration not implemented".to_string())
    }
}

// Fallback for other platforms
pub mod fallback_storage {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    
    fn get_config_path() -> Result<PathBuf, String> {
        let home_dir = dirs::home_dir().ok_or("Cannot find home directory")?;
        let config_dir = home_dir.join(".bullshift");
        
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }
        
        Ok(config_dir.join("secure_credentials.json"))
    }
    
    pub fn store_encrypted(broker: &str, encrypted_data: &[u8]) -> Result<(), String> {
        let config_path = get_config_path()?;
        let mut stored_data: HashMap<String, Vec<u8>> = if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| format!("Failed to read config file: {}", e))?;
            serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse config file: {}", e))?
        } else {
            HashMap::new()
        };
        
        stored_data.insert(broker.to_string(), encrypted_data.to_vec());
        
        let json_content = serde_json::to_string_pretty(&stored_data)
            .map_err(|e| format!("Failed to serialize credentials: {}", e))?;
        
        fs::write(&config_path, json_content)
            .map_err(|e| format!("Failed to write config file: {}", e))?;
        
        // Set restrictive permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&config_path)
                .map_err(|e| format!("Failed to get file metadata: {}", e))?
                .permissions();
            perms.set_mode(0o600); // Read/write for owner only
            fs::set_permissions(&config_path, perms)
                .map_err(|e| format!("Failed to set file permissions: {}", e))?;
        }
        
        Ok(())
    }
}