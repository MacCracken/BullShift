use crate::error::BullShiftError;
use crate::integration::{TradeEvent, TradeEventType};
use chrono::{DateTime, Utc};
use reqwest::Client;
use ring::hmac;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

/// A cryptographically signed audit entry.
///
/// Each entry includes an HMAC-SHA256 signature over its contents and a
/// reference to the previous entry's hash, forming a tamper-evident chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: Uuid,
    pub sequence: u64,
    pub event_type: AuditEventType,
    pub actor: String,
    pub action: String,
    pub resource: String,
    pub details: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub previous_hash: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    OrderSubmitted,
    OrderFilled,
    OrderCancelled,
    OrderRejected,
    PositionOpened,
    PositionClosed,
    ConfigurationChanged,
    CredentialAccessed,
    ProviderAdded,
    ProviderRemoved,
    UserLogin,
    UserLogout,
    PermissionChanged,
    SystemEvent,
}

impl std::fmt::Display for AuditEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OrderSubmitted => write!(f, "order.submitted"),
            Self::OrderFilled => write!(f, "order.filled"),
            Self::OrderCancelled => write!(f, "order.cancelled"),
            Self::OrderRejected => write!(f, "order.rejected"),
            Self::PositionOpened => write!(f, "position.opened"),
            Self::PositionClosed => write!(f, "position.closed"),
            Self::ConfigurationChanged => write!(f, "config.changed"),
            Self::CredentialAccessed => write!(f, "credential.accessed"),
            Self::ProviderAdded => write!(f, "provider.added"),
            Self::ProviderRemoved => write!(f, "provider.removed"),
            Self::UserLogin => write!(f, "user.login"),
            Self::UserLogout => write!(f, "user.logout"),
            Self::PermissionChanged => write!(f, "permission.changed"),
            Self::SystemEvent => write!(f, "system.event"),
        }
    }
}

/// Configuration for the audit trail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    pub max_entries: usize,
    pub emit_to_secureyeoman: bool,
    pub secureyeoman_url: String,
    pub secureyeoman_api_key: Option<String>,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            max_entries: 10_000,
            emit_to_secureyeoman: false,
            secureyeoman_url: "http://localhost:18789".to_string(),
            secureyeoman_api_key: None,
        }
    }
}

/// Manages a tamper-evident cryptographic audit trail.
///
/// Every trade event and system action is signed with HMAC-SHA256 and chained
/// to the previous entry. The chain can be forwarded to SecureYeoman's audit
/// system for compliance and long-term storage.
pub struct AuditTrail {
    signing_key: hmac::Key,
    entries: VecDeque<AuditEntry>,
    sequence: u64,
    last_hash: String,
    config: AuditConfig,
    client: Client,
}

impl AuditTrail {
    /// Create a new audit trail with the given signing key bytes (32 bytes recommended).
    pub fn new(key_bytes: &[u8], config: AuditConfig) -> Self {
        let signing_key = hmac::Key::new(hmac::HMAC_SHA256, key_bytes);
        Self {
            signing_key,
            entries: VecDeque::with_capacity(config.max_entries.min(10_000)),
            sequence: 0,
            last_hash: "genesis".to_string(),
            config,
            client: Client::new(),
        }
    }

    /// Record a new audit entry, sign it, and optionally emit to SecureYeoman.
    pub async fn record(
        &mut self,
        event_type: AuditEventType,
        actor: &str,
        action: &str,
        resource: &str,
        details: serde_json::Value,
    ) -> Result<Uuid, BullShiftError> {
        self.sequence += 1;

        let entry_id = Uuid::new_v4();
        let timestamp = Utc::now();

        // Build the signing payload: sequence | event_type | actor | action | resource | timestamp | previous_hash
        let sign_payload = format!(
            "{}|{}|{}|{}|{}|{}|{}",
            self.sequence, event_type, actor, action, resource, timestamp, self.last_hash
        );
        let tag = hmac::sign(&self.signing_key, sign_payload.as_bytes());
        let signature = hex::encode(tag.as_ref());

        let entry = AuditEntry {
            id: entry_id,
            sequence: self.sequence,
            event_type,
            actor: actor.to_string(),
            action: action.to_string(),
            resource: resource.to_string(),
            details,
            timestamp,
            previous_hash: self.last_hash.clone(),
            signature: signature.clone(),
        };

        // Update chain
        self.last_hash = signature;

        // Store
        if self.entries.len() >= self.config.max_entries {
            self.entries.pop_front();
        }
        self.entries.push_back(entry.clone());

        // Emit to SecureYeoman if configured
        if self.config.emit_to_secureyeoman {
            self.emit_to_secureyeoman(&entry).await;
        }

        Ok(entry_id)
    }

    /// Record a trade event from the integration bridge.
    pub async fn record_trade_event(
        &mut self,
        event: &TradeEvent,
        actor: &str,
    ) -> Result<Uuid, BullShiftError> {
        let audit_type = match event.event_type {
            TradeEventType::OrderSubmitted => AuditEventType::OrderSubmitted,
            TradeEventType::OrderFilled => AuditEventType::OrderFilled,
            TradeEventType::OrderCancelled => AuditEventType::OrderCancelled,
            TradeEventType::OrderRejected => AuditEventType::OrderRejected,
            TradeEventType::PositionOpened => AuditEventType::PositionOpened,
            TradeEventType::PositionClosed => AuditEventType::PositionClosed,
            TradeEventType::OrderPartiallyFilled => AuditEventType::OrderFilled,
            TradeEventType::PositionUpdated => AuditEventType::PositionOpened,
            TradeEventType::StopLossTriggered => AuditEventType::OrderFilled,
            TradeEventType::TakeProfitTriggered => AuditEventType::OrderFilled,
        };

        let details = serde_json::to_value(event)
            .map_err(|e| BullShiftError::AiBridge(format!("Failed to serialize event: {}", e)))?;

        self.record(
            audit_type,
            actor,
            &event.event_type.to_string(),
            &event.symbol,
            details,
        )
        .await
    }

    /// Verify the integrity of the audit chain.
    /// Returns the number of valid entries (should equal total entries if intact).
    pub fn verify_chain(&self) -> Result<usize, BullShiftError> {
        let mut expected_prev = "genesis".to_string();
        let mut valid_count = 0;

        for entry in &self.entries {
            if entry.previous_hash != expected_prev {
                return Err(BullShiftError::Security(format!(
                    "Chain integrity violation at sequence {}. Expected prev_hash '{}', got '{}'",
                    entry.sequence, expected_prev, entry.previous_hash
                )));
            }

            // Re-compute signature
            let sign_payload = format!(
                "{}|{}|{}|{}|{}|{}|{}",
                entry.sequence,
                entry.event_type,
                entry.actor,
                entry.action,
                entry.resource,
                entry.timestamp,
                entry.previous_hash
            );

            let expected_tag = hmac::sign(&self.signing_key, sign_payload.as_bytes());
            let expected_sig = hex::encode(expected_tag.as_ref());

            if entry.signature != expected_sig {
                return Err(BullShiftError::Security(format!(
                    "Signature mismatch at sequence {}",
                    entry.sequence
                )));
            }

            expected_prev = entry.signature.clone();
            valid_count += 1;
        }

        Ok(valid_count)
    }

    /// Get recent audit entries.
    pub fn recent_entries(&self, limit: usize) -> Vec<&AuditEntry> {
        self.entries.iter().rev().take(limit).collect()
    }

    /// Get entries filtered by event type.
    pub fn entries_by_type(&self, event_type: &AuditEventType) -> Vec<&AuditEntry> {
        let type_str = event_type.to_string();
        self.entries
            .iter()
            .filter(|e| e.event_type.to_string() == type_str)
            .collect()
    }

    /// Get entries for a specific resource (e.g., symbol).
    pub fn entries_by_resource(&self, resource: &str) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.resource == resource)
            .collect()
    }

    /// Total number of entries in the chain.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Current sequence number.
    pub fn current_sequence(&self) -> u64 {
        self.sequence
    }

    async fn emit_to_secureyeoman(&self, entry: &AuditEntry) {
        let url = format!(
            "{}/api/v1/integrations/bullshift/audit",
            self.config.secureyeoman_url
        );

        let mut req = self.client.post(&url).json(entry);
        if let Some(ref key) = self.config.secureyeoman_api_key {
            req = req.header("x-api-key", key);
        }

        match req.send().await {
            Ok(resp) if resp.status().is_success() => {
                log::debug!("Audit entry {} emitted to SecureYeoman", entry.id);
            }
            Ok(resp) => {
                log::warn!(
                    "SecureYeoman audit emission returned {}",
                    resp.status()
                );
            }
            Err(e) => {
                log::warn!("Failed to emit audit entry to SecureYeoman: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_trail() -> AuditTrail {
        AuditTrail::new(b"test-signing-key-for-bullshift!!", AuditConfig::default())
    }

    #[tokio::test]
    async fn test_record_and_verify() {
        let mut trail = test_trail();

        trail
            .record(
                AuditEventType::OrderSubmitted,
                "user:alice",
                "submit_order",
                "AAPL",
                serde_json::json!({"quantity": 100, "side": "BUY"}),
            )
            .await
            .unwrap();

        trail
            .record(
                AuditEventType::OrderFilled,
                "system",
                "fill_order",
                "AAPL",
                serde_json::json!({"fill_price": 150.50}),
            )
            .await
            .unwrap();

        assert_eq!(trail.len(), 2);
        assert_eq!(trail.current_sequence(), 2);
        assert_eq!(trail.verify_chain().unwrap(), 2);
    }

    #[tokio::test]
    async fn test_chain_integrity_detection() {
        let mut trail = test_trail();

        trail
            .record(
                AuditEventType::OrderSubmitted,
                "user:alice",
                "submit_order",
                "AAPL",
                serde_json::json!({}),
            )
            .await
            .unwrap();

        trail
            .record(
                AuditEventType::OrderFilled,
                "system",
                "fill_order",
                "AAPL",
                serde_json::json!({}),
            )
            .await
            .unwrap();

        // Tamper with an entry
        if let Some(entry) = self::tamper_entry(&mut trail) {
            entry.actor = "TAMPERED".to_string();
        }

        assert!(trail.verify_chain().is_err());
    }

    fn tamper_entry(trail: &mut AuditTrail) -> Option<&mut AuditEntry> {
        trail.entries.back_mut()
    }

    #[tokio::test]
    async fn test_entries_by_type() {
        let mut trail = test_trail();

        trail
            .record(
                AuditEventType::OrderSubmitted,
                "user:alice",
                "submit",
                "AAPL",
                serde_json::json!({}),
            )
            .await
            .unwrap();

        trail
            .record(
                AuditEventType::ConfigurationChanged,
                "admin",
                "update_config",
                "system",
                serde_json::json!({}),
            )
            .await
            .unwrap();

        trail
            .record(
                AuditEventType::OrderSubmitted,
                "user:bob",
                "submit",
                "TSLA",
                serde_json::json!({}),
            )
            .await
            .unwrap();

        let order_entries = trail.entries_by_type(&AuditEventType::OrderSubmitted);
        assert_eq!(order_entries.len(), 2);

        let config_entries = trail.entries_by_type(&AuditEventType::ConfigurationChanged);
        assert_eq!(config_entries.len(), 1);
    }

    #[tokio::test]
    async fn test_entries_by_resource() {
        let mut trail = test_trail();

        trail
            .record(
                AuditEventType::OrderSubmitted,
                "user:alice",
                "submit",
                "AAPL",
                serde_json::json!({}),
            )
            .await
            .unwrap();

        trail
            .record(
                AuditEventType::OrderFilled,
                "system",
                "fill",
                "AAPL",
                serde_json::json!({}),
            )
            .await
            .unwrap();

        trail
            .record(
                AuditEventType::OrderSubmitted,
                "user:bob",
                "submit",
                "TSLA",
                serde_json::json!({}),
            )
            .await
            .unwrap();

        let aapl_entries = trail.entries_by_resource("AAPL");
        assert_eq!(aapl_entries.len(), 2);
    }

    #[test]
    fn test_audit_event_type_display() {
        assert_eq!(AuditEventType::OrderSubmitted.to_string(), "order.submitted");
        assert_eq!(
            AuditEventType::ConfigurationChanged.to_string(),
            "config.changed"
        );
        assert_eq!(
            AuditEventType::CredentialAccessed.to_string(),
            "credential.accessed"
        );
    }

    #[tokio::test]
    async fn test_max_entries_eviction() {
        let config = AuditConfig {
            max_entries: 3,
            ..Default::default()
        };
        let mut trail = AuditTrail::new(b"test-signing-key-for-bullshift!!", config);

        for i in 0..5 {
            trail
                .record(
                    AuditEventType::SystemEvent,
                    "system",
                    &format!("event_{}", i),
                    "test",
                    serde_json::json!({}),
                )
                .await
                .unwrap();
        }

        assert_eq!(trail.len(), 3);
        assert_eq!(trail.current_sequence(), 5);
    }
}
