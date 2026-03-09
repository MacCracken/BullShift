use tungstenite::{connect, Message};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::mpsc;
use std::thread;
use crate::error::BullShiftError;
use crate::security::SecurityManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTick {
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub timestamp: i64,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketBar {
    pub symbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub timestamp: i64,
    pub timeframe: String,
}

/// Credentials for API authentication
/// These are loaded from secure storage, not hardcoded
#[derive(Clone)]
pub struct ApiCredentials {
    pub api_key: String,
    pub api_secret: String,
}

impl ApiCredentials {
    /// Create credentials from secure storage
    pub fn from_secure_storage(key: String, secret: String) -> Self {
        Self {
            api_key: key,
            api_secret: secret,
        }
    }
    
    /// Validate that credentials are properly configured
    pub fn validate(&self) -> Result<(), BullShiftError> {
        if self.api_key.is_empty() {
            return Err(BullShiftError::Validation("API key is empty".to_string()));
        }
        if self.api_secret.is_empty() {
            return Err(BullShiftError::Validation("API secret is empty".to_string()));
        }
        if self.api_key.len() < 10 {
            return Err(BullShiftError::Validation("API key appears to be invalid (too short)".to_string()));
        }
        Ok(())
    }
}

pub trait MarketDataStream {
    fn connect(&mut self, symbols: Vec<String>) -> Result<(), BullShiftError>;
    fn subscribe_ticks(&mut self, symbols: Vec<String>) -> Result<(), BullShiftError>;
    fn subscribe_bars(&mut self, symbols: Vec<String>, timeframe: String) -> Result<(), BullShiftError>;
    fn get_tick_receiver(&self) -> mpsc::UnboundedReceiver<MarketTick>;
    fn get_bar_receiver(&self) -> mpsc::UnboundedReceiver<MarketBar>;
}

pub struct AlpacaStream {
    tick_sender: mpsc::UnboundedSender<MarketTick>,
    bar_sender: mpsc::UnboundedSender<MarketBar>,
    connected: bool,
    subscriptions: HashMap<String, String>,
    credentials: Option<ApiCredentials>,
    shutdown_tx: Option<std::sync::mpsc::Sender<()>>,
}

impl Default for AlpacaStream {
    fn default() -> Self {
        Self::new()
    }
}

impl AlpacaStream {
    pub fn new() -> Self {
        let (tick_sender, _) = mpsc::unbounded_channel();
        let (bar_sender, _) = mpsc::unbounded_channel();
        
        Self {
            tick_sender,
            bar_sender,
            connected: false,
            subscriptions: HashMap::new(),
            credentials: None,
            shutdown_tx: None,
        }
    }
    
    /// Set credentials for authentication
    pub fn set_credentials(&mut self, credentials: ApiCredentials) {
        self.credentials = Some(credentials);
    }
    
    /// Load credentials from secure storage using the security manager
    /// This retrieves encrypted credentials for the "alpaca" broker
    pub fn load_credentials(&mut self) -> Result<(), BullShiftError> {
        // Initialize security manager
        let security_manager = SecurityManager::new()?;

        // Attempt to load credentials for Alpaca broker
        let (api_key, api_secret) = security_manager.get_credentials("alpaca")?;
        let credentials = ApiCredentials::from_secure_storage(api_key, api_secret);

        // Validate credentials before storing
        credentials.validate()?;

        self.credentials = Some(credentials);
        log::info!("Successfully loaded Alpaca credentials from secure storage");
        Ok(())
    }
    
    /// Store credentials securely using the security manager
    pub fn store_credentials_securely(&self, api_key: String, api_secret: String) -> Result<(), BullShiftError> {
        let mut security_manager = SecurityManager::new()?;
        security_manager.store_credentials("alpaca".to_string(), api_key, api_secret)?;
        log::info!("Successfully stored Alpaca credentials in secure storage");
        Ok(())
    }

    pub fn process_message(&mut self, msg: Message) {
        match msg {
            Message::Text(text) => {
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                    self.handle_alpaca_message(data);
                }
            }
            Message::Binary(data) => {
                // Handle binary data if needed
                log::debug!("Received binary data: {} bytes", data.len());
            }
            Message::Close(_) => {
                self.connected = false;
                log::warn!("WebSocket connection closed");
            }
            Message::Ping(_) => {
                // Handle ping
            }
            Message::Pong(_) => {
                // Handle pong
            }
            _ => {
                log::debug!("Unhandled WebSocket message type");
            }
        }
    }

    pub fn handle_alpaca_message(&mut self, data: serde_json::Value) {
        if let Some(msg_type) = data.get("T").and_then(|v| v.as_str()) {
            match msg_type {
                "t" => {
                    // Trade message
                    if let Some(tick) = self.parse_trade_message(&data) {
                        let _ = self.tick_sender.send(tick);
                    }
                }
                "b" => {
                    // Bar message
                    if let Some(bar) = self.parse_bar_message(&data) {
                        let _ = self.bar_sender.send(bar);
                    }
                }
                _ => {
                    log::debug!("Unknown message type: {}", msg_type);
                }
            }
        }
    }

    pub fn parse_trade_message(&self, data: &serde_json::Value) -> Option<MarketTick> {
        Some(MarketTick {
            symbol: data.get("S")?.as_str()?.to_string(),
            price: data.get("p")?.as_f64()?,
            volume: data.get("v")?.as_f64()?,
            timestamp: data.get("t")?.as_i64()?,
            bid: data.get("bs")?.as_f64(),
            ask: data.get("as")?.as_f64(),
        })
    }

    pub fn parse_bar_message(&self, data: &serde_json::Value) -> Option<MarketBar> {
        Some(MarketBar {
            symbol: data.get("S")?.as_str()?.to_string(),
            open: data.get("o")?.as_f64()?,
            high: data.get("h")?.as_f64()?,
            low: data.get("l")?.as_f64()?,
            close: data.get("c")?.as_f64()?,
            volume: data.get("v")?.as_f64()?,
            timestamp: data.get("t")?.as_i64()?,
            timeframe: "1m".to_string(), // Default timeframe
        })
    }
}

impl Drop for AlpacaStream {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

impl MarketDataStream for AlpacaStream {
    fn connect(&mut self, symbols: Vec<String>) -> Result<(), BullShiftError> {
        // Validate credentials are set
        let credentials = self.credentials.as_ref()
            .ok_or_else(|| BullShiftError::Configuration("No credentials configured. Call set_credentials() first.".to_string()))?;

        // Validate credentials
        credentials.validate()?;

        let url = "wss://stream.data.alpaca.markets/v2/iex";

        match connect(url) {
            Ok((mut ws_stream, _)) => {
                self.connected = true;
                
                // Send authentication with securely loaded credentials
                // NOTE: The credentials are sent over WSS (WebSocket Secure)
                // which provides TLS encryption. The plaintext here is 
                // encrypted in transit by the TLS layer.
                let auth_msg = serde_json::json!({
                    "action": "auth",
                    "key": credentials.api_key,
                    "secret": credentials.api_secret
                });
                
                if let Err(e) = ws_stream.send(Message::Text(auth_msg.to_string())) {
                    return Err(BullShiftError::DataStream(format!("Failed to send auth: {}", e)));
                }
                
                log::info!("WebSocket authentication sent (credentials transmitted over TLS)");
                
                // Subscribe to symbols
                for symbol in symbols {
                    let sub_msg = serde_json::json!({
                        "action": "subscribe",
                        "trades": [symbol],
                        "quotes": [symbol],
                        "bars": [symbol]
                    });
                    
                    if let Err(e) = ws_stream.send(Message::Text(sub_msg.to_string())) {
                        return Err(BullShiftError::DataStream(format!("Failed to subscribe to {}: {}", symbol, e)));
                    }
                    
                    self.subscriptions.insert(symbol, "active".to_string());
                }
                
                // Start message processing loop with shutdown signal
                let _tick_sender = self.tick_sender.clone();
                let _bar_sender = self.bar_sender.clone();
                let (shutdown_tx, shutdown_rx) = std::sync::mpsc::channel::<()>();
                self.shutdown_tx = Some(shutdown_tx);

                thread::spawn(move || {
                    // Set read timeout so we can check shutdown signal
                    // Note: set_read_timeout not available on MaybeTlsStream;
                    // the read loop uses try_recv for shutdown instead.
                    loop {
                        // Check for shutdown signal
                        if shutdown_rx.try_recv().is_ok() {
                            log::info!("WebSocket thread shutting down");
                            let _ = ws_stream.close(None);
                            break;
                        }
                        match ws_stream.read() {
                            Ok(msg) => {
                                log::debug!("Received message: {:?}", msg);
                            }
                            Err(tungstenite::Error::Io(ref e))
                                if e.kind() == std::io::ErrorKind::WouldBlock
                                    || e.kind() == std::io::ErrorKind::TimedOut =>
                            {
                                // Timeout — loop back to check shutdown
                                continue;
                            }
                            Err(e) => {
                                log::error!("WebSocket error: {}", e);
                                break;
                            }
                        }
                    }
                });
                
                Ok(())
            }
            Err(e) => Err(BullShiftError::DataStream(format!("Failed to connect: {}", e)))
        }
    }

    fn subscribe_ticks(&mut self, symbols: Vec<String>) -> Result<(), BullShiftError> {
        for symbol in symbols {
            self.subscriptions.insert(symbol, "ticks".to_string());
        }
        Ok(())
    }

    fn subscribe_bars(&mut self, symbols: Vec<String>, timeframe: String) -> Result<(), BullShiftError> {
        for symbol in symbols {
            self.subscriptions.insert(symbol, timeframe.clone());
        }
        Ok(())
    }

    fn get_tick_receiver(&self) -> mpsc::UnboundedReceiver<MarketTick> {
        let (_, receiver) = mpsc::unbounded_channel();
        receiver
    }

    fn get_bar_receiver(&self) -> mpsc::UnboundedReceiver<MarketBar> {
        let (_, receiver) = mpsc::unbounded_channel();
        receiver
    }
}

pub struct MarketDataManager {
    streams: HashMap<String, Box<dyn MarketDataStream + Send + Sync>>,
    tick_cache: HashMap<String, Vec<MarketTick>>,
    bar_cache: HashMap<String, Vec<MarketBar>>,
}

impl Default for MarketDataManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketDataManager {
    pub fn new() -> Self {
        Self {
            streams: HashMap::new(),
            tick_cache: HashMap::new(),
            bar_cache: HashMap::new(),
        }
    }

    pub fn add_stream(&mut self, name: String, stream: Box<dyn MarketDataStream + Send + Sync>) {
        self.streams.insert(name, stream);
    }

    pub fn get_latest_ticks(&self, symbol: &str) -> Option<&Vec<MarketTick>> {
        self.tick_cache.get(symbol)
    }

    pub fn get_latest_bars(&self, symbol: &str) -> Option<&Vec<MarketBar>> {
        self.bar_cache.get(symbol)
    }

    pub fn start_data_collection(&mut self) -> Result<(), BullShiftError> {
        // Start collecting data from all streams
        for name in self.streams.keys() {
            log::info!("Starting data collection for stream: {}", name);
            // Implementation would start async tasks to receive data
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_credentials_validation() {
        let valid_creds = ApiCredentials::from_secure_storage(
            "PK_VALID_API_KEY_123".to_string(),
            "valid_secret_key_here".to_string()
        );
        assert!(valid_creds.validate().is_ok());

        let empty_key = ApiCredentials::from_secure_storage(
            "".to_string(),
            "valid_secret".to_string()
        );
        assert!(empty_key.validate().is_err());

        let short_key = ApiCredentials::from_secure_storage(
            "short".to_string(),
            "valid_secret".to_string()
        );
        assert!(short_key.validate().is_err());
    }

    #[test]
    fn test_alpaca_stream_credentials() {
        let mut stream = AlpacaStream::new();

        // Should fail without credentials
        assert!(stream.load_credentials().is_err());

        // Set credentials
        let creds = ApiCredentials::from_secure_storage(
            "PK_TEST_API_KEY_123".to_string(),
            "test_secret_key".to_string()
        );
        stream.set_credentials(creds);

        // Should now have credentials
        assert!(stream.credentials.is_some());
    }

    #[test]
    fn test_alpaca_stream_new_defaults() {
        let stream = AlpacaStream::new();
        assert!(!stream.connected);
        assert!(stream.subscriptions.is_empty());
        assert!(stream.credentials.is_none());
        assert!(stream.shutdown_tx.is_none());
    }

    #[test]
    fn test_parse_trade_message_valid() {
        let stream = AlpacaStream::new();
        let data = serde_json::json!({
            "T": "t",
            "S": "AAPL",
            "p": 150.25,
            "v": 100.0,
            "t": 1678886400,
            "bs": 150.20,
            "as": 150.30
        });
        let tick = stream.parse_trade_message(&data);
        assert!(tick.is_some());
        let tick = tick.unwrap();
        assert_eq!(tick.symbol, "AAPL");
        assert_eq!(tick.price, 150.25);
        assert_eq!(tick.volume, 100.0);
        assert_eq!(tick.timestamp, 1678886400);
        assert_eq!(tick.bid, Some(150.20));
        assert_eq!(tick.ask, Some(150.30));
    }

    #[test]
    fn test_parse_trade_message_missing_symbol() {
        let stream = AlpacaStream::new();
        let data = serde_json::json!({
            "T": "t",
            "p": 150.25,
            "v": 100.0,
            "t": 1678886400
        });
        assert!(stream.parse_trade_message(&data).is_none());
    }

    #[test]
    fn test_parse_trade_message_missing_price() {
        let stream = AlpacaStream::new();
        let data = serde_json::json!({
            "T": "t",
            "S": "AAPL",
            "v": 100.0,
            "t": 1678886400
        });
        assert!(stream.parse_trade_message(&data).is_none());
    }

    #[test]
    fn test_parse_bar_message_valid() {
        let stream = AlpacaStream::new();
        let data = serde_json::json!({
            "T": "b",
            "S": "GOOG",
            "o": 100.0,
            "h": 105.0,
            "l": 99.0,
            "c": 103.0,
            "v": 50000.0,
            "t": 1678886400
        });
        let bar = stream.parse_bar_message(&data);
        assert!(bar.is_some());
        let bar = bar.unwrap();
        assert_eq!(bar.symbol, "GOOG");
        assert_eq!(bar.open, 100.0);
        assert_eq!(bar.high, 105.0);
        assert_eq!(bar.low, 99.0);
        assert_eq!(bar.close, 103.0);
        assert_eq!(bar.volume, 50000.0);
        assert_eq!(bar.timeframe, "1m");
    }

    #[test]
    fn test_parse_bar_message_missing_field() {
        let stream = AlpacaStream::new();
        let data = serde_json::json!({
            "T": "b",
            "S": "GOOG",
            "o": 100.0
            // Missing h, l, c, v, t
        });
        assert!(stream.parse_bar_message(&data).is_none());
    }

    #[test]
    fn test_handle_alpaca_message_trade() {
        let mut stream = AlpacaStream::new();
        let data = serde_json::json!({
            "T": "t",
            "S": "AAPL",
            "p": 150.0,
            "v": 100.0,
            "t": 1678886400,
            "bs": 149.0,
            "as": 151.0
        });
        // Should not panic even though receiver is dropped
        stream.handle_alpaca_message(data);
    }

    #[test]
    fn test_handle_alpaca_message_bar() {
        let mut stream = AlpacaStream::new();
        let data = serde_json::json!({
            "T": "b",
            "S": "TSLA",
            "o": 200.0,
            "h": 210.0,
            "l": 195.0,
            "c": 205.0,
            "v": 75000.0,
            "t": 1678886400
        });
        stream.handle_alpaca_message(data);
    }

    #[test]
    fn test_handle_alpaca_message_unknown_type() {
        let mut stream = AlpacaStream::new();
        let data = serde_json::json!({
            "T": "unknown",
            "data": "test"
        });
        stream.handle_alpaca_message(data);
    }

    #[test]
    fn test_handle_alpaca_message_no_type() {
        let mut stream = AlpacaStream::new();
        let data = serde_json::json!({ "data": "test" });
        stream.handle_alpaca_message(data);
    }

    #[test]
    fn test_subscribe_ticks() {
        let mut stream = AlpacaStream::new();
        stream.subscribe_ticks(vec!["AAPL".to_string(), "GOOG".to_string()]).unwrap();
        assert_eq!(stream.subscriptions.len(), 2);
        assert_eq!(stream.subscriptions.get("AAPL"), Some(&"ticks".to_string()));
    }

    #[test]
    fn test_subscribe_bars() {
        let mut stream = AlpacaStream::new();
        stream.subscribe_bars(vec!["AAPL".to_string()], "5m".to_string()).unwrap();
        assert_eq!(stream.subscriptions.get("AAPL"), Some(&"5m".to_string()));
    }

    #[test]
    fn test_market_data_manager_new() {
        let manager = MarketDataManager::new();
        assert!(manager.get_latest_ticks("AAPL").is_none());
        assert!(manager.get_latest_bars("AAPL").is_none());
    }

    #[test]
    fn test_market_data_manager_start_empty() {
        let mut manager = MarketDataManager::new();
        assert!(manager.start_data_collection().is_ok());
    }

    #[test]
    fn test_process_message_close() {
        let mut stream = AlpacaStream::new();
        stream.connected = true;
        stream.process_message(Message::Close(None));
        assert!(!stream.connected);
    }

    #[test]
    fn test_process_message_ping_pong() {
        let mut stream = AlpacaStream::new();
        stream.process_message(Message::Ping(vec![]));
        stream.process_message(Message::Pong(vec![]));
        // Should not panic
    }

    #[test]
    fn test_process_message_binary() {
        let mut stream = AlpacaStream::new();
        stream.process_message(Message::Binary(vec![1, 2, 3]));
        // Should not panic
    }

    #[test]
    fn test_process_message_text_valid_trade() {
        let mut stream = AlpacaStream::new();
        let json = serde_json::json!({
            "T": "t", "S": "AAPL", "p": 150.0, "v": 100.0, "t": 1678886400, "bs": 149.0, "as": 151.0
        });
        stream.process_message(Message::Text(json.to_string()));
    }

    #[test]
    fn test_process_message_text_invalid_json() {
        let mut stream = AlpacaStream::new();
        stream.process_message(Message::Text("not json".to_string()));
        // Should not panic — invalid JSON is silently ignored
    }

    #[test]
    fn test_credentials_empty_secret() {
        let creds = ApiCredentials::from_secure_storage(
            "PK_VALID_KEY_1234567890".to_string(),
            "".to_string(),
        );
        assert!(creds.validate().is_err());
    }

    #[test]
    fn test_connect_without_credentials() {
        let mut stream = AlpacaStream::new();
        let result = stream.connect(vec!["AAPL".to_string()]);
        assert!(result.is_err());
    }
}
