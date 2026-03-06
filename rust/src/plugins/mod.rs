use std::collections::HashMap;
use std::fmt;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::error::BullShiftError;

// ---------------------------------------------------------------------------
// PluginType
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum PluginType {
    DataSource,
    Indicator,
    Strategy,
    Notification,
    Integration,
    Custom(String),
}

impl fmt::Display for PluginType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginType::DataSource => write!(f, "DataSource"),
            PluginType::Indicator => write!(f, "Indicator"),
            PluginType::Strategy => write!(f, "Strategy"),
            PluginType::Notification => write!(f, "Notification"),
            PluginType::Integration => write!(f, "Integration"),
            PluginType::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

// ---------------------------------------------------------------------------
// PluginEvent
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum PluginEvent {
    TradeExecuted {
        symbol: String,
        side: String,
        quantity: f64,
        price: f64,
    },
    PriceUpdate {
        symbol: String,
        price: f64,
        volume: f64,
    },
    OrderFilled {
        order_id: String,
        symbol: String,
        fill_price: f64,
    },
    AlertTriggered {
        alert_id: String,
        message: String,
    },
    TimerTick {
        interval_seconds: u64,
    },
    Custom {
        name: String,
        data: String,
    },
}

impl fmt::Display for PluginEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginEvent::TradeExecuted { .. } => write!(f, "TradeExecuted"),
            PluginEvent::PriceUpdate { .. } => write!(f, "PriceUpdate"),
            PluginEvent::OrderFilled { .. } => write!(f, "OrderFilled"),
            PluginEvent::AlertTriggered { .. } => write!(f, "AlertTriggered"),
            PluginEvent::TimerTick { .. } => write!(f, "TimerTick"),
            PluginEvent::Custom { .. } => write!(f, "Custom"),
        }
    }
}

// ---------------------------------------------------------------------------
// PluginAction
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum PluginAction {
    SubmitOrder {
        symbol: String,
        side: String,
        quantity: f64,
        order_type: String,
        price: Option<f64>,
    },
    CancelOrder {
        order_id: String,
    },
    SendNotification {
        channel: String,
        message: String,
    },
    EmitEvent {
        event: PluginEvent,
    },
    Log {
        level: String,
        message: String,
    },
    NoAction,
}

// ---------------------------------------------------------------------------
// PluginState
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum PluginState {
    Registered,
    Active,
    Paused,
    Error(String),
    Stopped,
}

impl fmt::Display for PluginState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginState::Registered => write!(f, "Registered"),
            PluginState::Active => write!(f, "Active"),
            PluginState::Paused => write!(f, "Paused"),
            PluginState::Error(msg) => write!(f, "Error({})", msg),
            PluginState::Stopped => write!(f, "Stopped"),
        }
    }
}

// ---------------------------------------------------------------------------
// PluginMetadata
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub plugin_type: PluginType,
    pub state: PluginState,
    pub registered_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Plugin trait
// ---------------------------------------------------------------------------

pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    fn plugin_type(&self) -> PluginType;
    fn on_init(&mut self) -> Result<(), BullShiftError>;
    fn on_shutdown(&mut self) -> Result<(), BullShiftError>;
    fn on_event(&mut self, event: &PluginEvent) -> Result<Option<PluginAction>, BullShiftError>;
}

// ---------------------------------------------------------------------------
// PluginRegistry
// ---------------------------------------------------------------------------

struct PluginEntry {
    plugin: Box<dyn Plugin + Send>,
    metadata: PluginMetadata,
}

pub struct PluginRegistry {
    plugins: HashMap<Uuid, PluginEntry>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        PluginRegistry {
            plugins: HashMap::new(),
        }
    }

    pub fn register(&mut self, mut plugin: Box<dyn Plugin + Send>) -> Uuid {
        let now = Utc::now();
        let metadata = PluginMetadata {
            name: plugin.name().to_string(),
            version: plugin.version().to_string(),
            description: plugin.description().to_string(),
            plugin_type: plugin.plugin_type(),
            state: PluginState::Registered,
            registered_at: now,
            last_active: now,
        };

        let id = Uuid::new_v4();

        let state = match plugin.on_init() {
            Ok(()) => PluginState::Active,
            Err(e) => PluginState::Error(e.to_string()),
        };

        let mut entry = PluginEntry { plugin, metadata };
        entry.metadata.state = state;
        self.plugins.insert(id, entry);
        id
    }

    pub fn unregister(&mut self, id: &Uuid) -> bool {
        if let Some(mut entry) = self.plugins.remove(id) {
            let _ = entry.plugin.on_shutdown();
            true
        } else {
            false
        }
    }

    pub fn pause(&mut self, id: &Uuid) -> bool {
        if let Some(entry) = self.plugins.get_mut(id) {
            if entry.metadata.state == PluginState::Active {
                entry.metadata.state = PluginState::Paused;
                return true;
            }
        }
        false
    }

    pub fn resume(&mut self, id: &Uuid) -> bool {
        if let Some(entry) = self.plugins.get_mut(id) {
            if entry.metadata.state == PluginState::Paused {
                entry.metadata.state = PluginState::Active;
                entry.metadata.last_active = Utc::now();
                return true;
            }
        }
        false
    }

    pub fn get_metadata(&self, id: &Uuid) -> Option<&PluginMetadata> {
        self.plugins.get(id).map(|e| &e.metadata)
    }

    pub fn list_plugins(&self) -> Vec<(Uuid, &PluginMetadata)> {
        self.plugins.iter().map(|(id, e)| (*id, &e.metadata)).collect()
    }

    pub fn list_by_type(&self, plugin_type: &PluginType) -> Vec<(Uuid, &PluginMetadata)> {
        self.plugins
            .iter()
            .filter(|(_, e)| &e.metadata.plugin_type == plugin_type)
            .map(|(id, e)| (*id, &e.metadata))
            .collect()
    }

    pub fn dispatch_event(&mut self, event: &PluginEvent) -> Vec<PluginAction> {
        let mut actions = Vec::new();
        for entry in self.plugins.values_mut() {
            if entry.metadata.state != PluginState::Active {
                continue;
            }
            match entry.plugin.on_event(event) {
                Ok(Some(action)) => {
                    entry.metadata.last_active = Utc::now();
                    actions.push(action);
                }
                Ok(None) => {
                    entry.metadata.last_active = Utc::now();
                }
                Err(e) => {
                    entry.metadata.state = PluginState::Error(e.to_string());
                }
            }
        }
        actions
    }

    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin {
        events_received: Vec<String>,
    }

    impl TestPlugin {
        fn new() -> Self {
            TestPlugin {
                events_received: Vec::new(),
            }
        }
    }

    impl Plugin for TestPlugin {
        fn name(&self) -> &str {
            "TestPlugin"
        }

        fn version(&self) -> &str {
            "1.0.0"
        }

        fn description(&self) -> &str {
            "A test plugin for unit testing"
        }

        fn plugin_type(&self) -> PluginType {
            PluginType::Strategy
        }

        fn on_init(&mut self) -> Result<(), BullShiftError> {
            Ok(())
        }

        fn on_shutdown(&mut self) -> Result<(), BullShiftError> {
            Ok(())
        }

        fn on_event(
            &mut self,
            event: &PluginEvent,
        ) -> Result<Option<PluginAction>, BullShiftError> {
            self.events_received.push(format!("{}", event));
            Ok(Some(PluginAction::NoAction))
        }
    }

    /// Helper to create a second plugin type for type-filtering tests.
    struct NotificationTestPlugin;

    impl Plugin for NotificationTestPlugin {
        fn name(&self) -> &str {
            "NotificationTestPlugin"
        }
        fn version(&self) -> &str {
            "0.1.0"
        }
        fn description(&self) -> &str {
            "Notification test"
        }
        fn plugin_type(&self) -> PluginType {
            PluginType::Notification
        }
        fn on_init(&mut self) -> Result<(), BullShiftError> {
            Ok(())
        }
        fn on_shutdown(&mut self) -> Result<(), BullShiftError> {
            Ok(())
        }
        fn on_event(
            &mut self,
            _event: &PluginEvent,
        ) -> Result<Option<PluginAction>, BullShiftError> {
            Ok(Some(PluginAction::NoAction))
        }
    }

    #[test]
    fn test_plugin_type_display() {
        assert_eq!(PluginType::DataSource.to_string(), "DataSource");
        assert_eq!(PluginType::Indicator.to_string(), "Indicator");
        assert_eq!(PluginType::Strategy.to_string(), "Strategy");
        assert_eq!(PluginType::Notification.to_string(), "Notification");
        assert_eq!(PluginType::Integration.to_string(), "Integration");
        assert_eq!(
            PluginType::Custom("MyType".to_string()).to_string(),
            "Custom(MyType)"
        );
    }

    #[test]
    fn test_plugin_state_display() {
        assert_eq!(PluginState::Registered.to_string(), "Registered");
        assert_eq!(PluginState::Active.to_string(), "Active");
        assert_eq!(PluginState::Paused.to_string(), "Paused");
        assert_eq!(
            PluginState::Error("broken".to_string()).to_string(),
            "Error(broken)"
        );
        assert_eq!(PluginState::Stopped.to_string(), "Stopped");
    }

    #[test]
    fn test_plugin_event_display() {
        let event = PluginEvent::TradeExecuted {
            symbol: "AAPL".to_string(),
            side: "BUY".to_string(),
            quantity: 10.0,
            price: 150.0,
        };
        assert_eq!(event.to_string(), "TradeExecuted");

        let event = PluginEvent::PriceUpdate {
            symbol: "AAPL".to_string(),
            price: 151.0,
            volume: 1000.0,
        };
        assert_eq!(event.to_string(), "PriceUpdate");

        let event = PluginEvent::OrderFilled {
            order_id: "123".to_string(),
            symbol: "AAPL".to_string(),
            fill_price: 150.5,
        };
        assert_eq!(event.to_string(), "OrderFilled");

        let event = PluginEvent::AlertTriggered {
            alert_id: "a1".to_string(),
            message: "test".to_string(),
        };
        assert_eq!(event.to_string(), "AlertTriggered");

        let event = PluginEvent::TimerTick {
            interval_seconds: 60,
        };
        assert_eq!(event.to_string(), "TimerTick");

        let event = PluginEvent::Custom {
            name: "x".to_string(),
            data: "y".to_string(),
        };
        assert_eq!(event.to_string(), "Custom");
    }

    #[test]
    fn test_register_plugin() {
        let mut registry = PluginRegistry::new();
        let id = registry.register(Box::new(TestPlugin::new()));

        let meta = registry.get_metadata(&id).unwrap();
        assert_eq!(meta.name, "TestPlugin");
        assert_eq!(meta.version, "1.0.0");
        assert_eq!(meta.state, PluginState::Active);
        assert_eq!(meta.plugin_type, PluginType::Strategy);
    }

    #[test]
    fn test_unregister_plugin() {
        let mut registry = PluginRegistry::new();
        let id = registry.register(Box::new(TestPlugin::new()));
        assert_eq!(registry.plugin_count(), 1);

        let removed = registry.unregister(&id);
        assert!(removed);
        assert_eq!(registry.plugin_count(), 0);

        // Removing again should return false.
        let removed_again = registry.unregister(&id);
        assert!(!removed_again);
    }

    #[test]
    fn test_pause_resume() {
        let mut registry = PluginRegistry::new();
        let id = registry.register(Box::new(TestPlugin::new()));

        assert!(registry.pause(&id));
        assert_eq!(
            registry.get_metadata(&id).unwrap().state,
            PluginState::Paused
        );

        // Pausing an already paused plugin returns false.
        assert!(!registry.pause(&id));

        assert!(registry.resume(&id));
        assert_eq!(
            registry.get_metadata(&id).unwrap().state,
            PluginState::Active
        );

        // Resuming an already active plugin returns false.
        assert!(!registry.resume(&id));
    }

    #[test]
    fn test_dispatch_event_to_active() {
        let mut registry = PluginRegistry::new();
        let _id = registry.register(Box::new(TestPlugin::new()));

        let event = PluginEvent::PriceUpdate {
            symbol: "AAPL".to_string(),
            price: 155.0,
            volume: 5000.0,
        };

        let actions = registry.dispatch_event(&event);
        assert_eq!(actions.len(), 1);
        assert!(matches!(actions[0], PluginAction::NoAction));
    }

    #[test]
    fn test_paused_plugin_skipped() {
        let mut registry = PluginRegistry::new();
        let id = registry.register(Box::new(TestPlugin::new()));
        registry.pause(&id);

        let event = PluginEvent::TimerTick {
            interval_seconds: 10,
        };

        let actions = registry.dispatch_event(&event);
        assert!(actions.is_empty());
    }

    #[test]
    fn test_list_plugins() {
        let mut registry = PluginRegistry::new();
        let id1 = registry.register(Box::new(TestPlugin::new()));
        let id2 = registry.register(Box::new(NotificationTestPlugin));

        let list = registry.list_plugins();
        assert_eq!(list.len(), 2);

        let ids: Vec<Uuid> = list.iter().map(|(id, _)| *id).collect();
        assert!(ids.contains(&id1));
        assert!(ids.contains(&id2));
    }

    #[test]
    fn test_list_by_type() {
        let mut registry = PluginRegistry::new();
        let _id1 = registry.register(Box::new(TestPlugin::new()));
        let _id2 = registry.register(Box::new(NotificationTestPlugin));

        let strategies = registry.list_by_type(&PluginType::Strategy);
        assert_eq!(strategies.len(), 1);
        assert_eq!(strategies[0].1.name, "TestPlugin");

        let notifications = registry.list_by_type(&PluginType::Notification);
        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].1.name, "NotificationTestPlugin");

        let indicators = registry.list_by_type(&PluginType::Indicator);
        assert!(indicators.is_empty());
    }

    #[test]
    fn test_plugin_count() {
        let mut registry = PluginRegistry::new();
        assert_eq!(registry.plugin_count(), 0);

        let id1 = registry.register(Box::new(TestPlugin::new()));
        assert_eq!(registry.plugin_count(), 1);

        let _id2 = registry.register(Box::new(NotificationTestPlugin));
        assert_eq!(registry.plugin_count(), 2);

        registry.unregister(&id1);
        assert_eq!(registry.plugin_count(), 1);
    }

    #[test]
    fn test_plugin_action_variants() {
        let submit = PluginAction::SubmitOrder {
            symbol: "AAPL".to_string(),
            side: "BUY".to_string(),
            quantity: 100.0,
            order_type: "LIMIT".to_string(),
            price: Some(150.0),
        };
        assert!(matches!(submit, PluginAction::SubmitOrder { .. }));

        let cancel = PluginAction::CancelOrder {
            order_id: "ord-123".to_string(),
        };
        assert!(matches!(cancel, PluginAction::CancelOrder { .. }));

        let notify = PluginAction::SendNotification {
            channel: "slack".to_string(),
            message: "hello".to_string(),
        };
        assert!(matches!(notify, PluginAction::SendNotification { .. }));

        let emit = PluginAction::EmitEvent {
            event: PluginEvent::TimerTick {
                interval_seconds: 5,
            },
        };
        assert!(matches!(emit, PluginAction::EmitEvent { .. }));

        let log = PluginAction::Log {
            level: "INFO".to_string(),
            message: "test".to_string(),
        };
        assert!(matches!(log, PluginAction::Log { .. }));

        let no_action = PluginAction::NoAction;
        assert!(matches!(no_action, PluginAction::NoAction));
    }
}
