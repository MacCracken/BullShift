use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::error::BullShiftError;

// ---------------------------------------------------------------------------
// Push Notification Support
// ---------------------------------------------------------------------------

/// Mobile platform for push notifications.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "ios")]
    IOS,
    Android,
    Web,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::IOS => write!(f, "iOS"),
            Platform::Android => write!(f, "Android"),
            Platform::Web => write!(f, "Web"),
        }
    }
}

/// Notification priority level.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    High,
    Normal,
    Low,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::High => write!(f, "High"),
            Priority::Normal => write!(f, "Normal"),
            Priority::Low => write!(f, "Low"),
        }
    }
}

/// A registered device that can receive push notifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRegistration {
    pub id: Uuid,
    pub platform: Platform,
    pub push_token: String,
    pub user_id: String,
    pub topics: Vec<String>,
    pub registered_at: DateTime<Utc>,
}

/// A push notification to be sent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushNotification {
    pub title: String,
    pub body: String,
    pub topic: String,
    pub priority: Priority,
    pub data: HashMap<String, String>,
}

/// A platform-specific push payload ready for delivery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushPayload {
    pub device_id: Uuid,
    pub platform: Platform,
    pub payload_json: String,
}

/// Result of attempting to deliver a notification to a single device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryResult {
    pub device_id: Uuid,
    pub success: bool,
    pub error: Option<String>,
}

/// Manages push notification device registrations and delivery.
#[derive(Debug, Default)]
pub struct PushNotificationManager {
    devices: Vec<DeviceRegistration>,
}

impl PushNotificationManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    /// Register a device and return its assigned id.
    pub fn register_device(&mut self, device: DeviceRegistration) -> Uuid {
        let id = device.id;
        self.devices.push(device);
        id
    }

    /// Unregister a device by id. Returns `true` if found and removed.
    pub fn unregister_device(&mut self, device_id: &Uuid) -> bool {
        let before = self.devices.len();
        self.devices.retain(|d| d.id != *device_id);
        self.devices.len() < before
    }

    /// List all registered devices.
    pub fn list_devices(&self) -> &[DeviceRegistration] {
        &self.devices
    }

    /// Create platform-specific payloads for every registered device whose
    /// topics include the notification's topic.
    pub fn create_notification(&self, notification: &PushNotification) -> Vec<PushPayload> {
        self.devices
            .iter()
            .filter(|d| d.topics.contains(&notification.topic))
            .map(|d| {
                let payload_json = match d.platform {
                    Platform::IOS => serde_json::json!({
                        "aps": {
                            "alert": {
                                "title": notification.title,
                                "body": notification.body,
                            },
                            "sound": "default",
                            "priority": match notification.priority {
                                Priority::High => 10,
                                Priority::Normal => 5,
                                Priority::Low => 1,
                            },
                        },
                        "data": notification.data,
                    })
                    .to_string(),
                    Platform::Android => serde_json::json!({
                        "notification": {
                            "title": notification.title,
                            "body": notification.body,
                        },
                        "priority": match notification.priority {
                            Priority::High => "high",
                            Priority::Normal => "normal",
                            Priority::Low => "low",
                        },
                        "data": notification.data,
                    })
                    .to_string(),
                    Platform::Web => serde_json::json!({
                        "title": notification.title,
                        "body": notification.body,
                        "data": notification.data,
                    })
                    .to_string(),
                };

                PushPayload {
                    device_id: d.id,
                    platform: d.platform.clone(),
                    payload_json,
                }
            })
            .collect()
    }

    /// Build and "send" push notifications. In practice this builds payloads
    /// and returns a `DeliveryResult` for each targeted device.
    pub async fn send_notification(
        &self,
        notification: PushNotification,
    ) -> Result<Vec<DeliveryResult>, BullShiftError> {
        let payloads = self.create_notification(&notification);
        if payloads.is_empty() {
            return Err(BullShiftError::Validation(
                "No devices registered for topic".to_string(),
            ));
        }

        let results = payloads
            .iter()
            .map(|p| DeliveryResult {
                device_id: p.device_id,
                success: true,
                error: None,
            })
            .collect();

        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Offline Data Sync
// ---------------------------------------------------------------------------

/// The kind of change recorded for sync.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncOperation {
    Create,
    Update,
    Delete,
}

impl std::fmt::Display for SyncOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncOperation::Create => write!(f, "Create"),
            SyncOperation::Update => write!(f, "Update"),
            SyncOperation::Delete => write!(f, "Delete"),
        }
    }
}

/// A single local change queued for synchronisation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncChange {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: String,
    pub operation: SyncOperation,
    pub data: String,
    pub created_at: DateTime<Utc>,
    pub synced: bool,
}

/// A conflict between a local change and the server version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    pub id: Uuid,
    pub local_change: SyncChange,
    pub server_version: String,
    pub detected_at: DateTime<Utc>,
}

/// How to resolve a sync conflict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    KeepLocal,
    KeepServer,
    Merge(String),
}

impl std::fmt::Display for ConflictResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConflictResolution::KeepLocal => write!(f, "KeepLocal"),
            ConflictResolution::KeepServer => write!(f, "KeepServer"),
            ConflictResolution::Merge(s) => write!(f, "Merge({s})"),
        }
    }
}

/// Manages offline-first data synchronisation.
#[derive(Debug, Default)]
pub struct SyncManager {
    changes: Vec<SyncChange>,
    conflicts: Vec<SyncConflict>,
    last_sync: Option<DateTime<Utc>>,
}

impl SyncManager {
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
            conflicts: Vec::new(),
            last_sync: None,
        }
    }

    /// Queue a local change for later sync.
    pub fn queue_change(&mut self, change: SyncChange) {
        self.changes.push(change);
    }

    /// Return all pending (unsynced) changes.
    pub fn pending_changes(&self) -> &[SyncChange] {
        &self.changes
    }

    /// Number of pending changes.
    pub fn pending_count(&self) -> usize {
        self.changes.len()
    }

    /// Mark a single change as synced and remove it from the queue.
    /// Returns `true` if found.
    pub fn mark_synced(&mut self, change_id: &Uuid) -> bool {
        let before = self.changes.len();
        self.changes.retain(|c| c.id != *change_id);
        self.changes.len() < before
    }

    /// Clear the entire queue, marking everything as synced.
    pub fn mark_all_synced(&mut self) {
        self.changes.clear();
    }

    /// Return all unresolved conflicts.
    pub fn conflicts(&self) -> &[SyncConflict] {
        &self.conflicts
    }

    /// Resolve a conflict by id. Returns `true` if found and removed.
    pub fn resolve_conflict(
        &mut self,
        conflict_id: &Uuid,
        _resolution: ConflictResolution,
    ) -> bool {
        let before = self.conflicts.len();
        self.conflicts.retain(|c| c.id != *conflict_id);
        self.conflicts.len() < before
    }

    /// Timestamp of the most recent successful sync.
    pub fn last_sync(&self) -> Option<DateTime<Utc>> {
        self.last_sync
    }

    /// Record that a sync just completed successfully.
    pub fn record_sync(&mut self) {
        self.last_sync = Some(Utc::now());
    }
}

// ---------------------------------------------------------------------------
// Biometric Auth Support
// ---------------------------------------------------------------------------

/// Supported biometric authentication methods.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiometricType {
    FaceId,
    TouchId,
    Fingerprint,
    Pin,
}

impl std::fmt::Display for BiometricType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BiometricType::FaceId => write!(f, "FaceId"),
            BiometricType::TouchId => write!(f, "TouchId"),
            BiometricType::Fingerprint => write!(f, "Fingerprint"),
            BiometricType::Pin => write!(f, "Pin"),
        }
    }
}

/// A stored biometric registration for a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricRegistration {
    pub id: Uuid,
    pub user_id: String,
    pub biometric_type: BiometricType,
    pub challenge_hash: String,
    pub registered_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
}

/// Manages biometric authentication registrations and challenges.
#[derive(Debug, Default)]
pub struct BiometricManager {
    registrations: Vec<BiometricRegistration>,
}

impl BiometricManager {
    pub fn new() -> Self {
        Self {
            registrations: Vec::new(),
        }
    }

    /// Register a biometric method for a user. Returns the new registration.
    pub fn register_biometric(
        &mut self,
        user_id: &str,
        biometric_type: BiometricType,
    ) -> BiometricRegistration {
        let registration = BiometricRegistration {
            id: Uuid::new_v4(),
            user_id: user_id.to_string(),
            biometric_type,
            challenge_hash: Uuid::new_v4().to_string(),
            registered_at: Utc::now(),
            last_used: None,
        };
        self.registrations.push(registration.clone());
        registration
    }

    /// Verify a challenge response against the stored challenge hash.
    pub fn verify_challenge(&self, registration_id: &Uuid, challenge_response: &str) -> bool {
        self.registrations
            .iter()
            .any(|r| r.id == *registration_id && r.challenge_hash == challenge_response)
    }

    /// Revoke a biometric registration by id. Returns `true` if found.
    pub fn revoke(&mut self, registration_id: &Uuid) -> bool {
        let before = self.registrations.len();
        self.registrations.retain(|r| r.id != *registration_id);
        self.registrations.len() < before
    }

    /// List all registrations for a given user.
    pub fn list_registrations(&self, user_id: &str) -> Vec<&BiometricRegistration> {
        self.registrations
            .iter()
            .filter(|r| r.user_id == user_id)
            .collect()
    }

    /// Check whether a user has at least one biometric registration.
    pub fn is_registered(&self, user_id: &str) -> bool {
        self.registrations.iter().any(|r| r.user_id == user_id)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Display tests ------------------------------------------------------

    #[test]
    fn test_platform_display() {
        assert_eq!(Platform::IOS.to_string(), "iOS");
        assert_eq!(Platform::Android.to_string(), "Android");
        assert_eq!(Platform::Web.to_string(), "Web");
    }

    #[test]
    fn test_priority_display() {
        assert_eq!(Priority::High.to_string(), "High");
        assert_eq!(Priority::Normal.to_string(), "Normal");
        assert_eq!(Priority::Low.to_string(), "Low");
    }

    // -- Push notification tests --------------------------------------------

    fn make_device(platform: Platform, topics: Vec<&str>) -> DeviceRegistration {
        DeviceRegistration {
            id: Uuid::new_v4(),
            platform,
            push_token: "token-abc".to_string(),
            user_id: "user-1".to_string(),
            topics: topics.into_iter().map(String::from).collect(),
            registered_at: Utc::now(),
        }
    }

    fn make_notification(topic: &str) -> PushNotification {
        PushNotification {
            title: "Price Alert".to_string(),
            body: "AAPL hit $200".to_string(),
            topic: topic.to_string(),
            priority: Priority::High,
            data: HashMap::new(),
        }
    }

    #[test]
    fn test_register_device() {
        let mut mgr = PushNotificationManager::new();
        let device = make_device(Platform::IOS, vec!["alerts"]);
        let id = mgr.register_device(device);
        assert_eq!(mgr.list_devices().len(), 1);
        assert_eq!(mgr.list_devices()[0].id, id);
    }

    #[test]
    fn test_unregister_device() {
        let mut mgr = PushNotificationManager::new();
        let device = make_device(Platform::Android, vec!["alerts"]);
        let id = mgr.register_device(device);
        assert!(mgr.unregister_device(&id));
        assert!(mgr.list_devices().is_empty());
        assert!(!mgr.unregister_device(&id)); // already removed
    }

    #[test]
    fn test_create_notification_filters_by_topic() {
        let mut mgr = PushNotificationManager::new();
        mgr.register_device(make_device(Platform::IOS, vec!["alerts"]));
        mgr.register_device(make_device(Platform::Android, vec!["news"]));
        mgr.register_device(make_device(Platform::Web, vec!["alerts", "news"]));

        let payloads = mgr.create_notification(&make_notification("alerts"));
        assert_eq!(payloads.len(), 2);
        assert!(payloads
            .iter()
            .all(|p| p.platform == Platform::IOS || p.platform == Platform::Web));
    }

    #[test]
    fn test_push_payload_ios_format() {
        let mut mgr = PushNotificationManager::new();
        mgr.register_device(make_device(Platform::IOS, vec!["alerts"]));

        let payloads = mgr.create_notification(&make_notification("alerts"));
        assert_eq!(payloads.len(), 1);

        let json: serde_json::Value = serde_json::from_str(&payloads[0].payload_json).unwrap();
        assert!(json.get("aps").is_some());
        assert_eq!(json["aps"]["alert"]["title"], "Price Alert");
    }

    #[test]
    fn test_push_payload_android_format() {
        let mut mgr = PushNotificationManager::new();
        mgr.register_device(make_device(Platform::Android, vec!["alerts"]));

        let payloads = mgr.create_notification(&make_notification("alerts"));
        assert_eq!(payloads.len(), 1);

        let json: serde_json::Value = serde_json::from_str(&payloads[0].payload_json).unwrap();
        assert!(json.get("notification").is_some());
        assert_eq!(json["notification"]["title"], "Price Alert");
        assert_eq!(json["priority"], "high");
    }

    // -- Sync tests ---------------------------------------------------------

    fn make_sync_change() -> SyncChange {
        SyncChange {
            id: Uuid::new_v4(),
            entity_type: "Trade".to_string(),
            entity_id: "trade-1".to_string(),
            operation: SyncOperation::Create,
            data: r#"{"symbol":"AAPL"}"#.to_string(),
            created_at: Utc::now(),
            synced: false,
        }
    }

    #[test]
    fn test_sync_queue_change() {
        let mut mgr = SyncManager::new();
        mgr.queue_change(make_sync_change());
        mgr.queue_change(make_sync_change());
        assert_eq!(mgr.pending_count(), 2);
        assert_eq!(mgr.pending_changes().len(), 2);
    }

    #[test]
    fn test_sync_mark_synced() {
        let mut mgr = SyncManager::new();
        let change = make_sync_change();
        let id = change.id;
        mgr.queue_change(change);
        assert!(mgr.mark_synced(&id));
        assert_eq!(mgr.pending_count(), 0);
        assert!(!mgr.mark_synced(&id)); // already removed
    }

    #[test]
    fn test_sync_mark_all_synced() {
        let mut mgr = SyncManager::new();
        mgr.queue_change(make_sync_change());
        mgr.queue_change(make_sync_change());
        mgr.mark_all_synced();
        assert_eq!(mgr.pending_count(), 0);
    }

    #[test]
    fn test_sync_conflict_resolution() {
        let mut mgr = SyncManager::new();
        let conflict = SyncConflict {
            id: Uuid::new_v4(),
            local_change: make_sync_change(),
            server_version: r#"{"symbol":"GOOG"}"#.to_string(),
            detected_at: Utc::now(),
        };
        let cid = conflict.id;
        mgr.conflicts.push(conflict);

        assert_eq!(mgr.conflicts().len(), 1);
        assert!(mgr.resolve_conflict(&cid, ConflictResolution::KeepLocal));
        assert!(mgr.conflicts().is_empty());
    }

    // -- Biometric tests ----------------------------------------------------

    #[test]
    fn test_biometric_register_and_verify() {
        let mut mgr = BiometricManager::new();
        let reg = mgr.register_biometric("user-1", BiometricType::FaceId);
        assert!(mgr.is_registered("user-1"));
        assert!(mgr.verify_challenge(&reg.id, &reg.challenge_hash));
        assert!(!mgr.verify_challenge(&reg.id, "wrong-response"));
    }

    #[test]
    fn test_biometric_revoke() {
        let mut mgr = BiometricManager::new();
        let reg = mgr.register_biometric("user-1", BiometricType::TouchId);
        assert!(mgr.revoke(&reg.id));
        assert!(!mgr.is_registered("user-1"));
        assert!(!mgr.revoke(&reg.id)); // already revoked
    }

    #[test]
    fn test_biometric_list_by_user() {
        let mut mgr = BiometricManager::new();
        mgr.register_biometric("user-1", BiometricType::FaceId);
        mgr.register_biometric("user-1", BiometricType::Pin);
        mgr.register_biometric("user-2", BiometricType::Fingerprint);

        let user1 = mgr.list_registrations("user-1");
        assert_eq!(user1.len(), 2);

        let user2 = mgr.list_registrations("user-2");
        assert_eq!(user2.len(), 1);
        assert_eq!(user2[0].biometric_type, BiometricType::Fingerprint);
    }
}
