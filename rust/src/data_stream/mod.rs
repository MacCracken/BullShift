use tungstenite::{connect, Message};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::mpsc;
use std::thread;
use std::time::Duration;

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

pub trait MarketDataStream {
    fn connect(&mut self, symbols: Vec<String>) -> Result<(), String>;
    fn subscribe_ticks(&mut self, symbols: Vec<String>) -> Result<(), String>;
    fn subscribe_bars(&mut self, symbols: Vec<String>, timeframe: String) -> Result<(), String>;
    fn get_tick_receiver(&self) -> mpsc::UnboundedReceiver<MarketTick>;
    fn get_bar_receiver(&self) -> mpsc::UnboundedReceiver<MarketBar>;
}

pub struct AlpacaStream {
    tick_sender: mpsc::UnboundedSender<MarketTick>,
    bar_sender: mpsc::UnboundedSender<MarketBar>,
    connected: bool,
    subscriptions: HashMap<String, String>,
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
        }
    }

    fn process_message(&mut self, msg: Message) {
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
        }
    }

    fn handle_alpaca_message(&mut self, data: serde_json::Value) {
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

    fn parse_trade_message(&self, data: &serde_json::Value) -> Option<MarketTick> {
        Some(MarketTick {
            symbol: data.get("S")?.as_str()?.to_string(),
            price: data.get("p")?.as_f64()?,
            volume: data.get("v")?.as_f64()?,
            timestamp: data.get("t")?.as_i64()?,
            bid: data.get("bs")?.as_f64(),
            ask: data.get("as")?.as_f64(),
        })
    }

    fn parse_bar_message(&self, data: &serde_json::Value) -> Option<MarketBar> {
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

impl MarketDataStream for AlpacaStream {
    fn connect(&mut self, symbols: Vec<String>) -> Result<(), String> {
        let url = "wss://stream.data.alpaca.markets/v2/iex";
        
        match connect(url) {
            Ok((mut ws_stream, _)) => {
                self.connected = true;
                
                // Send authentication
                let auth_msg = serde_json::json!({
                    "action": "auth",
                    "key": "YOUR_API_KEY",
                    "secret": "YOUR_API_SECRET"
                });
                
                if let Err(e) = ws_stream.write_message(Message::Text(auth_msg.to_string())) {
                    return Err(format!("Failed to send auth: {}", e));
                }
                
                // Subscribe to symbols
                for symbol in symbols {
                    let sub_msg = serde_json::json!({
                        "action": "subscribe",
                        "trades": [symbol],
                        "quotes": [symbol],
                        "bars": [symbol]
                    });
                    
                    if let Err(e) = ws_stream.write_message(Message::Text(sub_msg.to_string())) {
                        return Err(format!("Failed to subscribe to {}: {}", symbol, e));
                    }
                    
                    self.subscriptions.insert(symbol, "active".to_string());
                }
                
                // Start message processing loop
                let tick_sender = self.tick_sender.clone();
                let bar_sender = self.bar_sender.clone();
                
                thread::spawn(move || {
                    loop {
                        match ws_stream.read_message() {
                            Ok(msg) => {
                                // Process message (simplified for example)
                                log::debug!("Received message: {:?}", msg);
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
            Err(e) => Err(format!("Failed to connect: {}", e))
        }
    }

    fn subscribe_ticks(&mut self, symbols: Vec<String>) -> Result<(), String> {
        for symbol in symbols {
            self.subscriptions.insert(symbol, "ticks".to_string());
        }
        Ok(())
    }

    fn subscribe_bars(&mut self, symbols: Vec<String>, timeframe: String) -> Result<(), String> {
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

    pub fn start_data_collection(&mut self) -> Result<(), String> {
        // Start collecting data from all streams
        for (name, stream) in &self.streams {
            log::info!("Starting data collection for stream: {}", name);
            // Implementation would start async tasks to receive data
        }
        Ok(())
    }
}