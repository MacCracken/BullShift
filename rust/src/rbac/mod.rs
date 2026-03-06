use crate::error::BullShiftError;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Roles that determine what a user or agent can do within BullShift.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    /// Full access: trading, configuration, user management
    Admin,
    /// Can submit/cancel orders, view positions and account
    Trader,
    /// Read-only: view positions, market data, sentiment — no order submission
    Analyst,
    /// Minimal: can only view public market data
    ReadOnly,
    /// AI agent with limited trading permissions
    Agent,
    /// Custom role with explicit permission set
    Custom(String),
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Admin => write!(f, "admin"),
            Self::Trader => write!(f, "trader"),
            Self::Analyst => write!(f, "analyst"),
            Self::ReadOnly => write!(f, "read_only"),
            Self::Agent => write!(f, "agent"),
            Self::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

/// Fine-grained permissions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    SubmitOrders,
    CancelOrders,
    ViewPositions,
    ViewAccount,
    ViewMarketData,
    ManageProviders,
    ManageCredentials,
    ManageUsers,
    ManageRoles,
    ViewAuditLog,
    ConfigureSystem,
    ViewSentiment,
    ExecuteAiPrompts,
    ManageStrategies,
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SubmitOrders => write!(f, "orders.submit"),
            Self::CancelOrders => write!(f, "orders.cancel"),
            Self::ViewPositions => write!(f, "positions.view"),
            Self::ViewAccount => write!(f, "account.view"),
            Self::ViewMarketData => write!(f, "market_data.view"),
            Self::ManageProviders => write!(f, "providers.manage"),
            Self::ManageCredentials => write!(f, "credentials.manage"),
            Self::ManageUsers => write!(f, "users.manage"),
            Self::ManageRoles => write!(f, "roles.manage"),
            Self::ViewAuditLog => write!(f, "audit.view"),
            Self::ConfigureSystem => write!(f, "system.configure"),
            Self::ViewSentiment => write!(f, "sentiment.view"),
            Self::ExecuteAiPrompts => write!(f, "ai.execute"),
            Self::ManageStrategies => write!(f, "strategies.manage"),
        }
    }
}

/// A BullShift user or service identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub roles: HashSet<Role>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    /// If synced from SecureYeoman, the remote user ID
    pub secureyeoman_id: Option<String>,
}

/// Manages users, roles, and permissions.
///
/// Integrates with SecureYeoman's RBAC system — when connected, role changes
/// are synced bidirectionally. When disconnected, local RBAC is authoritative.
pub struct RbacManager {
    users: HashMap<Uuid, User>,
    /// Custom role → permission set
    custom_roles: HashMap<String, HashSet<Permission>>,
    /// API key → user ID mapping for token-based auth
    api_keys: HashMap<String, Uuid>,
    secureyeoman_url: Option<String>,
    secureyeoman_api_key: Option<String>,
    client: Client,
}

impl Default for RbacManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RbacManager {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            custom_roles: HashMap::new(),
            api_keys: HashMap::new(),
            secureyeoman_url: None,
            secureyeoman_api_key: None,
            client: Client::new(),
        }
    }

    /// Configure SecureYeoman RBAC sync.
    pub fn configure_secureyeoman(&mut self, url: String, api_key: Option<String>) {
        self.secureyeoman_url = Some(url);
        self.secureyeoman_api_key = api_key;
    }

    /// Create a new user with the given roles.
    pub fn create_user(&mut self, username: &str, roles: HashSet<Role>) -> Uuid {
        let id = Uuid::new_v4();
        let user = User {
            id,
            username: username.to_string(),
            roles,
            is_active: true,
            created_at: Utc::now(),
            last_login: None,
            secureyeoman_id: None,
        };
        self.users.insert(id, user);
        id
    }

    /// Look up a user by ID.
    pub fn get_user(&self, user_id: &Uuid) -> Option<&User> {
        self.users.get(user_id)
    }

    /// Look up a user by username.
    pub fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    /// List all users.
    pub fn list_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    /// Assign a role to a user.
    pub fn assign_role(&mut self, user_id: &Uuid, role: Role) -> Result<(), BullShiftError> {
        let user = self
            .users
            .get_mut(user_id)
            .ok_or_else(|| BullShiftError::Security(format!("User {} not found", user_id)))?;
        user.roles.insert(role);
        Ok(())
    }

    /// Remove a role from a user.
    pub fn revoke_role(&mut self, user_id: &Uuid, role: &Role) -> Result<(), BullShiftError> {
        let user = self
            .users
            .get_mut(user_id)
            .ok_or_else(|| BullShiftError::Security(format!("User {} not found", user_id)))?;
        user.roles.remove(role);
        Ok(())
    }

    /// Deactivate a user (they retain their roles but can't authenticate).
    pub fn deactivate_user(&mut self, user_id: &Uuid) -> Result<(), BullShiftError> {
        let user = self
            .users
            .get_mut(user_id)
            .ok_or_else(|| BullShiftError::Security(format!("User {} not found", user_id)))?;
        user.is_active = false;
        Ok(())
    }

    /// Register an API key for a user (for token-based auth).
    pub fn register_api_key(&mut self, user_id: &Uuid, api_key: String) -> Result<(), BullShiftError> {
        if !self.users.contains_key(user_id) {
            return Err(BullShiftError::Security(format!("User {} not found", user_id)));
        }
        self.api_keys.insert(api_key, *user_id);
        Ok(())
    }

    /// Resolve a user from an API key.
    pub fn user_from_api_key(&self, api_key: &str) -> Option<&User> {
        self.api_keys
            .get(api_key)
            .and_then(|uid| self.users.get(uid))
    }

    /// Define a custom role with an explicit permission set.
    pub fn define_custom_role(&mut self, name: &str, permissions: HashSet<Permission>) {
        self.custom_roles.insert(name.to_string(), permissions);
    }

    /// Get the effective permissions for a role.
    pub fn permissions_for_role(&self, role: &Role) -> HashSet<Permission> {
        match role {
            Role::Admin => HashSet::from([
                Permission::SubmitOrders,
                Permission::CancelOrders,
                Permission::ViewPositions,
                Permission::ViewAccount,
                Permission::ViewMarketData,
                Permission::ManageProviders,
                Permission::ManageCredentials,
                Permission::ManageUsers,
                Permission::ManageRoles,
                Permission::ViewAuditLog,
                Permission::ConfigureSystem,
                Permission::ViewSentiment,
                Permission::ExecuteAiPrompts,
                Permission::ManageStrategies,
            ]),
            Role::Trader => HashSet::from([
                Permission::SubmitOrders,
                Permission::CancelOrders,
                Permission::ViewPositions,
                Permission::ViewAccount,
                Permission::ViewMarketData,
                Permission::ViewSentiment,
                Permission::ExecuteAiPrompts,
            ]),
            Role::Analyst => HashSet::from([
                Permission::ViewPositions,
                Permission::ViewAccount,
                Permission::ViewMarketData,
                Permission::ViewSentiment,
                Permission::ViewAuditLog,
                Permission::ExecuteAiPrompts,
            ]),
            Role::ReadOnly => HashSet::from([
                Permission::ViewMarketData,
                Permission::ViewSentiment,
            ]),
            Role::Agent => HashSet::from([
                Permission::SubmitOrders,
                Permission::ViewPositions,
                Permission::ViewMarketData,
                Permission::ViewSentiment,
                Permission::ExecuteAiPrompts,
            ]),
            Role::Custom(name) => self
                .custom_roles
                .get(name)
                .cloned()
                .unwrap_or_default(),
        }
    }

    /// Get all effective permissions for a user (union of all their roles).
    pub fn user_permissions(&self, user_id: &Uuid) -> HashSet<Permission> {
        self.users
            .get(user_id)
            .map(|user| {
                user.roles
                    .iter()
                    .flat_map(|role| self.permissions_for_role(role))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Check if a user has a specific permission.
    pub fn has_permission(&self, user_id: &Uuid, permission: &Permission) -> bool {
        let user = match self.users.get(user_id) {
            Some(u) if u.is_active => u,
            _ => return false,
        };

        user.roles
            .iter()
            .any(|role| self.permissions_for_role(role).contains(permission))
    }

    /// Check permission and return an error if denied.
    pub fn require_permission(
        &self,
        user_id: &Uuid,
        permission: &Permission,
    ) -> Result<(), BullShiftError> {
        if self.has_permission(user_id, permission) {
            Ok(())
        } else {
            Err(BullShiftError::Security(format!(
                "Permission denied: {} requires '{}'",
                user_id, permission
            )))
        }
    }

    /// Sync roles from SecureYeoman for a specific user.
    pub async fn sync_user_from_secureyeoman(
        &mut self,
        secureyeoman_user_id: &str,
    ) -> Result<(), BullShiftError> {
        let base_url = self
            .secureyeoman_url
            .as_ref()
            .ok_or_else(|| {
                BullShiftError::Configuration("SecureYeoman URL not configured".to_string())
            })?
            .clone();

        let url = format!(
            "{}/api/v1/rbac/users/{}/roles",
            base_url, secureyeoman_user_id
        );

        let mut req = self.client.get(&url);
        if let Some(ref key) = self.secureyeoman_api_key {
            req = req.header("x-api-key", key);
        }

        let resp = req
            .send()
            .await
            .map_err(|e| BullShiftError::Network(format!("RBAC sync failed: {}", e)))?;

        if !resp.status().is_success() {
            return Err(BullShiftError::Api(format!(
                "SecureYeoman RBAC sync returned {}",
                resp.status()
            )));
        }

        let remote_roles: Vec<String> = resp.json().await.map_err(|e| {
            BullShiftError::Api(format!("Failed to parse RBAC response: {}", e))
        })?;

        // Find or create the local user linked to this SecureYeoman ID
        let user_id = self
            .users
            .values()
            .find(|u| u.secureyeoman_id.as_deref() == Some(secureyeoman_user_id))
            .map(|u| u.id);

        let uid = match user_id {
            Some(id) => id,
            None => {
                // Create a new local user linked to SecureYeoman
                let id = Uuid::new_v4();
                let mut user = User {
                    id,
                    username: format!("sy:{}", secureyeoman_user_id),
                    roles: HashSet::new(),
                    is_active: true,
                    created_at: Utc::now(),
                    last_login: None,
                    secureyeoman_id: Some(secureyeoman_user_id.to_string()),
                };
                // Map remote roles
                for role_name in &remote_roles {
                    user.roles.insert(parse_role(role_name));
                }
                self.users.insert(id, user);
                return Ok(());
            }
        };

        // Update existing user's roles
        if let Some(user) = self.users.get_mut(&uid) {
            user.roles.clear();
            for role_name in &remote_roles {
                user.roles.insert(parse_role(role_name));
            }
        }

        Ok(())
    }
}

/// Map a role name string to a Role enum.
pub fn parse_role(name: &str) -> Role {
    match name.to_lowercase().as_str() {
        "admin" => Role::Admin,
        "trader" => Role::Trader,
        "analyst" => Role::Analyst,
        "read_only" | "readonly" | "viewer" => Role::ReadOnly,
        "agent" => Role::Agent,
        other => Role::Custom(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_and_check_permissions() {
        let mut mgr = RbacManager::new();
        let uid = mgr.create_user("alice", HashSet::from([Role::Trader]));

        assert!(mgr.has_permission(&uid, &Permission::SubmitOrders));
        assert!(mgr.has_permission(&uid, &Permission::ViewPositions));
        assert!(!mgr.has_permission(&uid, &Permission::ManageUsers));
        assert!(!mgr.has_permission(&uid, &Permission::ConfigureSystem));
    }

    #[test]
    fn test_admin_has_all_permissions() {
        let mut mgr = RbacManager::new();
        let uid = mgr.create_user("root", HashSet::from([Role::Admin]));

        assert!(mgr.has_permission(&uid, &Permission::SubmitOrders));
        assert!(mgr.has_permission(&uid, &Permission::ManageUsers));
        assert!(mgr.has_permission(&uid, &Permission::ConfigureSystem));
        assert!(mgr.has_permission(&uid, &Permission::ViewAuditLog));
    }

    #[test]
    fn test_readonly_cannot_trade() {
        let mut mgr = RbacManager::new();
        let uid = mgr.create_user("viewer", HashSet::from([Role::ReadOnly]));

        assert!(!mgr.has_permission(&uid, &Permission::SubmitOrders));
        assert!(!mgr.has_permission(&uid, &Permission::CancelOrders));
        assert!(!mgr.has_permission(&uid, &Permission::ViewPositions));
        assert!(mgr.has_permission(&uid, &Permission::ViewMarketData));
    }

    #[test]
    fn test_analyst_can_view_not_trade() {
        let mut mgr = RbacManager::new();
        let uid = mgr.create_user("bob", HashSet::from([Role::Analyst]));

        assert!(mgr.has_permission(&uid, &Permission::ViewPositions));
        assert!(mgr.has_permission(&uid, &Permission::ViewAuditLog));
        assert!(!mgr.has_permission(&uid, &Permission::SubmitOrders));
        assert!(!mgr.has_permission(&uid, &Permission::ManageUsers));
    }

    #[test]
    fn test_agent_limited_trading() {
        let mut mgr = RbacManager::new();
        let uid = mgr.create_user("ai-agent", HashSet::from([Role::Agent]));

        assert!(mgr.has_permission(&uid, &Permission::SubmitOrders));
        assert!(mgr.has_permission(&uid, &Permission::ViewPositions));
        assert!(!mgr.has_permission(&uid, &Permission::CancelOrders));
        assert!(!mgr.has_permission(&uid, &Permission::ManageProviders));
    }

    #[test]
    fn test_assign_and_revoke_roles() {
        let mut mgr = RbacManager::new();
        let uid = mgr.create_user("charlie", HashSet::from([Role::ReadOnly]));

        assert!(!mgr.has_permission(&uid, &Permission::SubmitOrders));

        mgr.assign_role(&uid, Role::Trader).unwrap();
        assert!(mgr.has_permission(&uid, &Permission::SubmitOrders));

        mgr.revoke_role(&uid, &Role::Trader).unwrap();
        assert!(!mgr.has_permission(&uid, &Permission::SubmitOrders));
    }

    #[test]
    fn test_deactivated_user_has_no_permissions() {
        let mut mgr = RbacManager::new();
        let uid = mgr.create_user("dave", HashSet::from([Role::Admin]));

        assert!(mgr.has_permission(&uid, &Permission::SubmitOrders));

        mgr.deactivate_user(&uid).unwrap();
        assert!(!mgr.has_permission(&uid, &Permission::SubmitOrders));
    }

    #[test]
    fn test_custom_role() {
        let mut mgr = RbacManager::new();

        mgr.define_custom_role(
            "data_viewer",
            HashSet::from([Permission::ViewMarketData, Permission::ViewSentiment]),
        );

        let uid = mgr.create_user(
            "intern",
            HashSet::from([Role::Custom("data_viewer".to_string())]),
        );

        assert!(mgr.has_permission(&uid, &Permission::ViewMarketData));
        assert!(mgr.has_permission(&uid, &Permission::ViewSentiment));
        assert!(!mgr.has_permission(&uid, &Permission::SubmitOrders));
    }

    #[test]
    fn test_api_key_lookup() {
        let mut mgr = RbacManager::new();
        let uid = mgr.create_user("api_user", HashSet::from([Role::Trader]));
        mgr.register_api_key(&uid, "sk-test-12345".to_string())
            .unwrap();

        let user = mgr.user_from_api_key("sk-test-12345").unwrap();
        assert_eq!(user.username, "api_user");
        assert!(user.roles.contains(&Role::Trader));

        assert!(mgr.user_from_api_key("bad-key").is_none());
    }

    #[test]
    fn test_require_permission_error() {
        let mut mgr = RbacManager::new();
        let uid = mgr.create_user("viewer", HashSet::from([Role::ReadOnly]));

        assert!(mgr
            .require_permission(&uid, &Permission::ViewMarketData)
            .is_ok());

        let err = mgr
            .require_permission(&uid, &Permission::SubmitOrders)
            .unwrap_err();
        assert!(format!("{}", err).contains("Permission denied"));
    }

    #[test]
    fn test_multi_role_union() {
        let mut mgr = RbacManager::new();
        let uid = mgr.create_user(
            "multi",
            HashSet::from([Role::ReadOnly, Role::Analyst]),
        );

        // Has permissions from both roles
        assert!(mgr.has_permission(&uid, &Permission::ViewMarketData)); // ReadOnly
        assert!(mgr.has_permission(&uid, &Permission::ViewAuditLog)); // Analyst
        assert!(!mgr.has_permission(&uid, &Permission::SubmitOrders)); // Neither
    }

    #[test]
    fn test_user_permissions_set() {
        let mut mgr = RbacManager::new();
        let uid = mgr.create_user("trader", HashSet::from([Role::Trader]));

        let perms = mgr.user_permissions(&uid);
        assert!(perms.contains(&Permission::SubmitOrders));
        assert!(perms.contains(&Permission::ViewPositions));
        assert!(!perms.contains(&Permission::ManageUsers));
    }

    #[test]
    fn test_parse_role() {
        assert_eq!(parse_role("admin"), Role::Admin);
        assert_eq!(parse_role("TRADER"), Role::Trader);
        assert_eq!(parse_role("readonly"), Role::ReadOnly);
        assert_eq!(parse_role("viewer"), Role::ReadOnly);
        assert_eq!(parse_role("agent"), Role::Agent);
        assert_eq!(
            parse_role("custom_role"),
            Role::Custom("custom_role".to_string())
        );
    }

    #[test]
    fn test_nonexistent_user() {
        let mgr = RbacManager::new();
        let fake_id = Uuid::new_v4();
        assert!(!mgr.has_permission(&fake_id, &Permission::SubmitOrders));
        assert!(mgr.user_permissions(&fake_id).is_empty());
    }

    #[test]
    fn test_get_user_by_name() {
        let mut mgr = RbacManager::new();
        mgr.create_user("alice", HashSet::from([Role::Trader]));

        assert!(mgr.get_user_by_name("alice").is_some());
        assert!(mgr.get_user_by_name("bob").is_none());
    }

    #[test]
    fn test_role_display() {
        assert_eq!(Role::Admin.to_string(), "admin");
        assert_eq!(Role::Trader.to_string(), "trader");
        assert_eq!(
            Role::Custom("manager".to_string()).to_string(),
            "custom:manager"
        );
    }
}
