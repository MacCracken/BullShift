//! Real-time WebSocket streaming API for BullShift.
//!
//! Provides a pub/sub streaming infrastructure for real-time market data,
//! trade notifications, order updates, position changes, and system alerts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

/// Subscribable channels for the streaming API.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "symbol")]
pub enum StreamChannel {
    /// Price updates for a specific symbol.
    Prices(String),
    /// All trade execution events.
    Trades,
    /// Order status changes.
    Orders,
    /// Position updates.
    Positions,
    /// System alerts.
    Alerts,
}

/// Messages broadcast to subscribers.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum StreamMessage {
    PriceUpdate {
        symbol: String,
        price: f64,
        volume: f64,
        bid: f64,
        ask: f64,
        timestamp: DateTime<Utc>,
    },
    TradeExecuted {
        trade_id: String,
        symbol: String,
        side: String,
        quantity: f64,
        price: f64,
        timestamp: DateTime<Utc>,
    },
    OrderUpdate {
        order_id: String,
        symbol: String,
        status: String,
        filled_qty: f64,
        remaining_qty: f64,
        timestamp: DateTime<Utc>,
    },
    PositionUpdate {
        symbol: String,
        quantity: f64,
        avg_entry_price: f64,
        market_value: f64,
        unrealized_pnl: f64,
        timestamp: DateTime<Utc>,
    },
    Alert {
        alert_id: String,
        severity: String,
        message: String,
        timestamp: DateTime<Utc>,
    },
    Subscribed {
        channel: StreamChannel,
    },
    Unsubscribed {
        channel: StreamChannel,
    },
    Error {
        code: u32,
        message: String,
    },
    Heartbeat {
        server_time: DateTime<Utc>,
    },
}

/// Commands sent by clients to the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum ClientCommand {
    #[serde(rename = "subscribe")]
    Subscribe { channel: StreamChannel },
    #[serde(rename = "unsubscribe")]
    Unsubscribe { channel: StreamChannel },
    #[serde(rename = "ping")]
    Ping,
}

/// Tracks a connected client session.
pub struct ClientSession {
    pub id: Uuid,
    pub subscriptions: HashSet<StreamChannel>,
    pub connected_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
    pub messages_sent: u64,
}

impl Default for ClientSession {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientSession {
    /// Create a new client session with a unique ID and no subscriptions.
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            subscriptions: HashSet::new(),
            connected_at: now,
            last_heartbeat: now,
            messages_sent: 0,
        }
    }

    /// Subscribe to a channel. Returns `true` if the channel was newly added,
    /// `false` if the client was already subscribed.
    pub fn subscribe(&mut self, channel: StreamChannel) -> bool {
        self.subscriptions.insert(channel)
    }

    /// Unsubscribe from a channel. Returns `true` if the channel was removed,
    /// `false` if the client was not subscribed.
    pub fn unsubscribe(&mut self, channel: &StreamChannel) -> bool {
        self.subscriptions.remove(channel)
    }

    /// Check whether the client is subscribed to a given channel.
    pub fn is_subscribed(&self, channel: &StreamChannel) -> bool {
        self.subscriptions.contains(channel)
    }

    /// Determine whether a message matches the client's current subscriptions.
    ///
    /// - `Prices(symbol)` matches `PriceUpdate` with the same symbol.
    /// - `Trades` matches `TradeExecuted`.
    /// - `Orders` matches `OrderUpdate`.
    /// - `Positions` matches `PositionUpdate`.
    /// - `Alerts` matches `Alert`.
    ///
    /// Control messages (`Subscribed`, `Unsubscribed`, `Error`, `Heartbeat`)
    /// are always delivered regardless of subscriptions.
    pub fn should_receive(&self, message: &StreamMessage) -> bool {
        match message {
            StreamMessage::PriceUpdate { symbol, .. } => self
                .subscriptions
                .contains(&StreamChannel::Prices(symbol.clone())),
            StreamMessage::TradeExecuted { .. } => {
                self.subscriptions.contains(&StreamChannel::Trades)
            }
            StreamMessage::OrderUpdate { .. } => {
                self.subscriptions.contains(&StreamChannel::Orders)
            }
            StreamMessage::PositionUpdate { .. } => {
                self.subscriptions.contains(&StreamChannel::Positions)
            }
            StreamMessage::Alert { .. } => self.subscriptions.contains(&StreamChannel::Alerts),
            // Control messages are always delivered.
            StreamMessage::Subscribed { .. }
            | StreamMessage::Unsubscribed { .. }
            | StreamMessage::Error { .. }
            | StreamMessage::Heartbeat { .. } => true,
        }
    }

    /// Record that a message was sent to this client.
    pub fn record_message_sent(&mut self) {
        self.messages_sent += 1;
        self.last_heartbeat = Utc::now();
    }
}

/// Server statistics snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingStats {
    pub connected_clients: usize,
    pub total_subscriptions: usize,
    pub uptime_seconds: i64,
    pub timestamp: DateTime<Utc>,
}

/// The streaming server that manages connections and broadcasts messages.
pub struct StreamingServer {
    sender: broadcast::Sender<StreamMessage>,
    clients: Arc<RwLock<HashMap<Uuid, ClientSession>>>,
    capacity: usize,
    started_at: DateTime<Utc>,
}

impl StreamingServer {
    /// Create a new streaming server with the given broadcast channel capacity.
    /// A typical default is 1024.
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self {
            sender,
            clients: Arc::new(RwLock::new(HashMap::new())),
            capacity,
            started_at: Utc::now(),
        }
    }

    /// Get a broadcast receiver for consuming messages.
    pub fn subscribe(&self) -> broadcast::Receiver<StreamMessage> {
        self.sender.subscribe()
    }

    /// Publish a message to all broadcast subscribers.
    /// Returns the number of active receivers that will receive the message.
    pub fn publish(&self, message: StreamMessage) -> Result<usize, String> {
        self.sender
            .send(message)
            .map_err(|e| format!("Failed to publish message: {}", e))
    }

    /// Register a new client session and return the assigned client ID.
    pub async fn register_client(&self) -> Uuid {
        let session = ClientSession::new();
        let id = session.id;
        self.clients.write().await.insert(id, session);
        id
    }

    /// Remove a client session. Returns `true` if the client existed.
    pub async fn disconnect_client(&self, client_id: &Uuid) -> bool {
        self.clients.write().await.remove(client_id).is_some()
    }

    /// Handle a client command (subscribe / unsubscribe / ping).
    /// Returns the response message to send back to the client, or `None` if
    /// the client ID is not found.
    pub async fn handle_command(
        &self,
        client_id: &Uuid,
        command: ClientCommand,
    ) -> Option<StreamMessage> {
        let mut clients = self.clients.write().await;
        let session = clients.get_mut(client_id)?;

        match command {
            ClientCommand::Subscribe { channel } => {
                session.subscribe(channel.clone());
                Some(StreamMessage::Subscribed { channel })
            }
            ClientCommand::Unsubscribe { channel } => {
                session.unsubscribe(&channel);
                Some(StreamMessage::Unsubscribed { channel })
            }
            ClientCommand::Ping => Some(StreamMessage::Heartbeat {
                server_time: Utc::now(),
            }),
        }
    }

    /// Get the number of currently connected clients.
    pub async fn client_count(&self) -> usize {
        self.clients.read().await.len()
    }

    /// Get a client's active subscriptions, or `None` if the client is not
    /// registered.
    pub async fn client_subscriptions(&self, client_id: &Uuid) -> Option<Vec<StreamChannel>> {
        self.clients
            .read()
            .await
            .get(client_id)
            .map(|s| s.subscriptions.iter().cloned().collect())
    }

    // -- Convenience publishers --------------------------------------------------

    /// Publish a price update for the given symbol.
    pub fn publish_price_update(
        &self,
        symbol: &str,
        price: f64,
        volume: f64,
        bid: f64,
        ask: f64,
    ) -> Result<usize, String> {
        self.publish(StreamMessage::PriceUpdate {
            symbol: symbol.to_string(),
            price,
            volume,
            bid,
            ask,
            timestamp: Utc::now(),
        })
    }

    /// Publish a trade execution event.
    pub fn publish_trade(
        &self,
        trade_id: &str,
        symbol: &str,
        side: &str,
        quantity: f64,
        price: f64,
    ) -> Result<usize, String> {
        self.publish(StreamMessage::TradeExecuted {
            trade_id: trade_id.to_string(),
            symbol: symbol.to_string(),
            side: side.to_string(),
            quantity,
            price,
            timestamp: Utc::now(),
        })
    }

    /// Publish an order status update.
    pub fn publish_order_update(
        &self,
        order_id: &str,
        symbol: &str,
        status: &str,
        filled_qty: f64,
        remaining_qty: f64,
    ) -> Result<usize, String> {
        self.publish(StreamMessage::OrderUpdate {
            order_id: order_id.to_string(),
            symbol: symbol.to_string(),
            status: status.to_string(),
            filled_qty,
            remaining_qty,
            timestamp: Utc::now(),
        })
    }

    /// Publish a system alert.
    pub fn publish_alert(&self, severity: &str, message: &str) -> Result<usize, String> {
        self.publish(StreamMessage::Alert {
            alert_id: Uuid::new_v4().to_string(),
            severity: severity.to_string(),
            message: message.to_string(),
            timestamp: Utc::now(),
        })
    }

    /// Get a snapshot of server statistics.
    pub async fn stats(&self) -> StreamingStats {
        let clients = self.clients.read().await;
        let total_subscriptions: usize = clients.values().map(|s| s.subscriptions.len()).sum();
        let now = Utc::now();
        StreamingStats {
            connected_clients: clients.len(),
            total_subscriptions,
            uptime_seconds: (now - self.started_at).num_seconds(),
            timestamp: now,
        }
    }

    /// Return the broadcast channel capacity this server was created with.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Default for StreamingServer {
    fn default() -> Self {
        Self::new(1024)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_streaming_server_new() {
        let server = StreamingServer::new(1024);
        assert_eq!(server.capacity(), 1024);
        assert_eq!(server.client_count().await, 0);
    }

    #[tokio::test]
    async fn test_client_registration() {
        let server = StreamingServer::new(1024);
        let id = server.register_client().await;
        assert_eq!(server.client_count().await, 1);

        // The returned ID should be retrievable.
        let subs = server.client_subscriptions(&id).await;
        assert!(subs.is_some());
        assert!(subs.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_client_disconnect() {
        let server = StreamingServer::new(1024);
        let id = server.register_client().await;
        assert_eq!(server.client_count().await, 1);

        let removed = server.disconnect_client(&id).await;
        assert!(removed);
        assert_eq!(server.client_count().await, 0);

        // Disconnecting the same client again should return false.
        let removed_again = server.disconnect_client(&id).await;
        assert!(!removed_again);
    }

    #[tokio::test]
    async fn test_subscribe_channel() {
        let server = StreamingServer::new(1024);
        let id = server.register_client().await;

        let response = server
            .handle_command(
                &id,
                ClientCommand::Subscribe {
                    channel: StreamChannel::Trades,
                },
            )
            .await;

        assert!(matches!(
            response,
            Some(StreamMessage::Subscribed {
                channel: StreamChannel::Trades
            })
        ));

        let subs = server.client_subscriptions(&id).await.unwrap();
        assert_eq!(subs.len(), 1);
        assert!(subs.contains(&StreamChannel::Trades));
    }

    #[tokio::test]
    async fn test_unsubscribe_channel() {
        let server = StreamingServer::new(1024);
        let id = server.register_client().await;

        server
            .handle_command(
                &id,
                ClientCommand::Subscribe {
                    channel: StreamChannel::Orders,
                },
            )
            .await;

        let response = server
            .handle_command(
                &id,
                ClientCommand::Unsubscribe {
                    channel: StreamChannel::Orders,
                },
            )
            .await;

        assert!(matches!(
            response,
            Some(StreamMessage::Unsubscribed {
                channel: StreamChannel::Orders
            })
        ));

        let subs = server.client_subscriptions(&id).await.unwrap();
        assert!(subs.is_empty());
    }

    #[tokio::test]
    async fn test_duplicate_subscribe() {
        let mut session = ClientSession::new();
        let first = session.subscribe(StreamChannel::Trades);
        assert!(first);
        let second = session.subscribe(StreamChannel::Trades);
        assert!(!second);
    }

    #[tokio::test]
    async fn test_should_receive_price_update() {
        let mut session = ClientSession::new();
        session.subscribe(StreamChannel::Prices("AAPL".to_string()));

        let matching = StreamMessage::PriceUpdate {
            symbol: "AAPL".to_string(),
            price: 150.0,
            volume: 1000.0,
            bid: 149.9,
            ask: 150.1,
            timestamp: Utc::now(),
        };
        assert!(session.should_receive(&matching));

        // Different symbol should not match.
        let non_matching = StreamMessage::PriceUpdate {
            symbol: "GOOG".to_string(),
            price: 2800.0,
            volume: 500.0,
            bid: 2799.0,
            ask: 2801.0,
            timestamp: Utc::now(),
        };
        assert!(!session.should_receive(&non_matching));
    }

    #[tokio::test]
    async fn test_should_receive_trade() {
        let mut session = ClientSession::new();
        session.subscribe(StreamChannel::Trades);

        let trade_msg = StreamMessage::TradeExecuted {
            trade_id: "t1".to_string(),
            symbol: "AAPL".to_string(),
            side: "BUY".to_string(),
            quantity: 100.0,
            price: 150.0,
            timestamp: Utc::now(),
        };
        assert!(session.should_receive(&trade_msg));

        // A session without the Trades channel should not receive it.
        let other_session = ClientSession::new();
        assert!(!other_session.should_receive(&trade_msg));
    }

    #[tokio::test]
    async fn test_publish_message() {
        let server = StreamingServer::new(1024);
        let mut rx = server.subscribe();

        let msg = StreamMessage::Heartbeat {
            server_time: Utc::now(),
        };
        let count = server.publish(msg.clone()).unwrap();
        assert_eq!(count, 1);

        let received = rx.recv().await.unwrap();
        assert!(matches!(received, StreamMessage::Heartbeat { .. }));
    }

    #[tokio::test]
    async fn test_handle_subscribe_command() {
        let server = StreamingServer::new(1024);
        let id = server.register_client().await;

        let response = server
            .handle_command(
                &id,
                ClientCommand::Subscribe {
                    channel: StreamChannel::Positions,
                },
            )
            .await;

        assert!(matches!(
            response,
            Some(StreamMessage::Subscribed {
                channel: StreamChannel::Positions
            })
        ));
    }

    #[tokio::test]
    async fn test_handle_ping_command() {
        let server = StreamingServer::new(1024);
        let id = server.register_client().await;

        let response = server.handle_command(&id, ClientCommand::Ping).await;

        assert!(matches!(response, Some(StreamMessage::Heartbeat { .. })));
    }

    #[tokio::test]
    async fn test_streaming_stats() {
        let server = StreamingServer::new(1024);
        let id = server.register_client().await;

        server
            .handle_command(
                &id,
                ClientCommand::Subscribe {
                    channel: StreamChannel::Trades,
                },
            )
            .await;
        server
            .handle_command(
                &id,
                ClientCommand::Subscribe {
                    channel: StreamChannel::Alerts,
                },
            )
            .await;

        let stats = server.stats().await;
        assert_eq!(stats.connected_clients, 1);
        assert_eq!(stats.total_subscriptions, 2);
        assert!(stats.uptime_seconds >= 0);
    }

    #[tokio::test]
    async fn test_publish_price_convenience() {
        let server = StreamingServer::new(1024);
        let mut rx = server.subscribe();

        let count = server
            .publish_price_update("AAPL", 150.0, 1000.0, 149.9, 150.1)
            .unwrap();
        assert_eq!(count, 1);

        let received = rx.recv().await.unwrap();
        match received {
            StreamMessage::PriceUpdate {
                symbol,
                price,
                volume,
                bid,
                ask,
                ..
            } => {
                assert_eq!(symbol, "AAPL");
                assert!((price - 150.0).abs() < f64::EPSILON);
                assert!((volume - 1000.0).abs() < f64::EPSILON);
                assert!((bid - 149.9).abs() < f64::EPSILON);
                assert!((ask - 150.1).abs() < f64::EPSILON);
            }
            other => panic!("Expected PriceUpdate, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_publish_trade_convenience() {
        let server = StreamingServer::new(1024);
        let mut rx = server.subscribe();

        let count = server
            .publish_trade("t-123", "TSLA", "SELL", 50.0, 250.0)
            .unwrap();
        assert_eq!(count, 1);

        let received = rx.recv().await.unwrap();
        match received {
            StreamMessage::TradeExecuted {
                trade_id,
                symbol,
                side,
                quantity,
                price,
                ..
            } => {
                assert_eq!(trade_id, "t-123");
                assert_eq!(symbol, "TSLA");
                assert_eq!(side, "SELL");
                assert!((quantity - 50.0).abs() < f64::EPSILON);
                assert!((price - 250.0).abs() < f64::EPSILON);
            }
            other => panic!("Expected TradeExecuted, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_client_session_new() {
        let session = ClientSession::new();
        assert!(session.subscriptions.is_empty());
        assert_eq!(session.messages_sent, 0);
        assert!(session.connected_at <= Utc::now());
    }
}
