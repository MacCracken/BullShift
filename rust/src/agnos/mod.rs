//! AGNOS agent runtime integration.
//!
//! When `AGNOS_AGENT_REGISTRY_URL` is set, the BullShift API server registers
//! itself with the AGNOS daimon agent registry on startup, sends heartbeats
//! every 30 seconds, and deregisters on shutdown.

use reqwest::Client;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::watch;

/// Agent registration payload sent to AGNOS daimon.
#[derive(Debug, Serialize)]
struct AgentRegistration {
    agent_id: String,
    name: String,
    version: String,
    endpoint: String,
    capabilities: Vec<String>,
}

/// Heartbeat payload sent to AGNOS daimon.
#[derive(Debug, Serialize)]
struct AgentHeartbeat {
    agent_id: String,
    status: String,
}

/// Manages the lifecycle of BullShift's registration with the AGNOS agent runtime.
/// Handles registration, periodic heartbeats, and graceful deregistration.
pub struct AgnosAgentRegistration {
    registry_url: String,
    agent_id: String,
    bullshift_port: u16,
    client: Client,
    shutdown_tx: watch::Sender<bool>,
    shutdown_rx: watch::Receiver<bool>,
}

impl AgnosAgentRegistration {
    /// Create a new AGNOS agent registration from env vars.
    /// Returns `None` if `AGNOS_AGENT_REGISTRY_URL` is not set.
    pub fn from_env(bullshift_port: u16) -> Option<Self> {
        let registry_url = std::env::var("AGNOS_AGENT_REGISTRY_URL").ok()?;
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        log::info!("AGNOS agent registry enabled: {}", registry_url);

        Some(Self {
            registry_url,
            agent_id: "bullshift".to_string(),
            bullshift_port,
            client: Client::new(),
            shutdown_tx,
            shutdown_rx,
        })
    }

    /// Register with the AGNOS daimon and start the heartbeat loop.
    /// Returns a handle that, when dropped, triggers deregistration.
    pub async fn start(self) -> AgnosRegistrationHandle {
        let registration = Arc::new(AgnosRegistrationInner {
            registry_url: self.registry_url,
            agent_id: self.agent_id,
            client: self.client,
        });

        // Register
        let reg_payload = AgentRegistration {
            agent_id: registration.agent_id.clone(),
            name: "BullShift Trading Platform".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            endpoint: format!("http://localhost:{}", self.bullshift_port),
            capabilities: vec![
                "trading".to_string(),
                "market-data".to_string(),
                "algo-strategies".to_string(),
                "sentiment".to_string(),
                "alerts".to_string(),
                "ai-providers".to_string(),
            ],
        };

        let url = format!("{}/v1/agents/register", registration.registry_url);
        match registration
            .client
            .post(&url)
            .header("x-agent-id", &registration.agent_id)
            .json(&reg_payload)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                log::info!(
                    "Registered with AGNOS daimon at {}",
                    registration.registry_url
                );
            }
            Ok(resp) => {
                log::warn!("AGNOS agent registration returned {}", resp.status());
            }
            Err(e) => {
                log::warn!("Failed to register with AGNOS daimon: {}", e);
            }
        }

        // Start heartbeat loop
        let heartbeat_reg = Arc::clone(&registration);
        let mut shutdown_rx = self.shutdown_rx;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        heartbeat_reg.send_heartbeat().await;
                    }
                    _ = shutdown_rx.changed() => {
                        log::debug!("AGNOS heartbeat loop shutting down");
                        break;
                    }
                }
            }
        });

        AgnosRegistrationHandle {
            inner: registration,
            shutdown_tx: self.shutdown_tx,
        }
    }
}

struct AgnosRegistrationInner {
    registry_url: String,
    agent_id: String,
    client: Client,
}

impl AgnosRegistrationInner {
    async fn send_heartbeat(&self) {
        let url = format!(
            "{}/v1/agents/{}/heartbeat",
            self.registry_url, self.agent_id
        );
        let payload = AgentHeartbeat {
            agent_id: self.agent_id.clone(),
            status: "healthy".to_string(),
        };

        match self
            .client
            .post(&url)
            .header("x-agent-id", &self.agent_id)
            .json(&payload)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                log::trace!("AGNOS heartbeat sent");
            }
            Ok(resp) => {
                log::warn!("AGNOS heartbeat returned {}", resp.status());
            }
            Err(e) => {
                log::warn!("AGNOS heartbeat failed: {}", e);
            }
        }
    }

    async fn deregister(&self) {
        let url = format!(
            "{}/v1/agents/{}/deregister",
            self.registry_url, self.agent_id
        );

        match self
            .client
            .post(&url)
            .header("x-agent-id", &self.agent_id)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                log::info!("Deregistered from AGNOS daimon");
            }
            Ok(resp) => {
                log::warn!("AGNOS deregistration returned {}", resp.status());
            }
            Err(e) => {
                log::warn!("Failed to deregister from AGNOS daimon: {}", e);
            }
        }
    }
}

/// Handle that keeps the AGNOS registration alive. When dropped, it signals
/// the heartbeat loop to stop and deregisters from the daimon.
pub struct AgnosRegistrationHandle {
    inner: Arc<AgnosRegistrationInner>,
    shutdown_tx: watch::Sender<bool>,
}

impl AgnosRegistrationHandle {
    /// Gracefully shut down: stop heartbeats and deregister from AGNOS.
    pub async fn shutdown(self) {
        let _ = self.shutdown_tx.send(true);
        self.inner.deregister().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_env_returns_none_without_var() {
        // AGNOS_AGENT_REGISTRY_URL is not set in test env
        std::env::remove_var("AGNOS_AGENT_REGISTRY_URL");
        assert!(AgnosAgentRegistration::from_env(8787).is_none());
    }

    #[test]
    fn test_from_env_returns_some_with_var() {
        std::env::set_var("AGNOS_AGENT_REGISTRY_URL", "http://localhost:8090");
        let reg = AgnosAgentRegistration::from_env(8787);
        assert!(reg.is_some());
        let reg = reg.unwrap();
        assert_eq!(reg.registry_url, "http://localhost:8090");
        assert_eq!(reg.agent_id, "bullshift");
        assert_eq!(reg.bullshift_port, 8787);
        std::env::remove_var("AGNOS_AGENT_REGISTRY_URL");
    }

    #[test]
    fn test_agent_registration_serialization() {
        let payload = AgentRegistration {
            agent_id: "bullshift".to_string(),
            name: "BullShift Trading Platform".to_string(),
            version: "2026.3.9".to_string(),
            endpoint: "http://localhost:8787".to_string(),
            capabilities: vec!["trading".to_string(), "market-data".to_string()],
        };

        let json = serde_json::to_value(&payload).unwrap();
        assert_eq!(json["agent_id"], "bullshift");
        assert_eq!(json["name"], "BullShift Trading Platform");
        assert_eq!(json["capabilities"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_heartbeat_serialization() {
        let payload = AgentHeartbeat {
            agent_id: "bullshift".to_string(),
            status: "healthy".to_string(),
        };

        let json = serde_json::to_value(&payload).unwrap();
        assert_eq!(json["agent_id"], "bullshift");
        assert_eq!(json["status"], "healthy");
    }
}
