use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingCredentials {
    pub api_key: String,
    pub api_secret: String,
    pub sandbox: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiOrderRequest {
    pub symbol: String,
    pub side: String,
    pub quantity: f64,
    pub order_type: String,
    pub price: Option<f64>,
    pub time_in_force: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiOrderResponse {
    pub order_id: String,
    pub symbol: String,
    pub side: String,
    pub quantity: f64,
    pub order_type: String,
    pub price: Option<f64>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPosition {
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub unrealized_pnl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAccount {
    pub balance: f64,
    pub available: f64,
    pub margin_used: f64,
}

pub trait TradingApi {
    async fn submit_order(&self, order: ApiOrderRequest) -> Result<ApiOrderResponse, String>;
    async fn get_positions(&self) -> Result<Vec<ApiPosition>, String>;
    async fn get_account(&self) -> Result<ApiAccount, String>;
    async fn cancel_order(&self, order_id: String) -> Result<bool, String>;
}

pub struct AlpacaApi {
    client: Client,
    credentials: TradingCredentials,
    base_url: String,
}

impl AlpacaApi {
    pub fn new(credentials: TradingCredentials) -> Self {
        let base_url = if credentials.sandbox {
            "https://paper-api.alpaca.markets".to_string()
        } else {
            "https://api.alpaca.markets".to_string()
        };

        Self {
            client: Client::new(),
            credentials,
            base_url,
        }
    }

    fn get_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("APCA-API-KEY-ID".to_string(), self.credentials.api_key.clone());
        headers.insert("APCA-API-SECRET-KEY".to_string(), self.credentials.api_secret.clone());
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers
    }
}

impl TradingApi for AlpacaApi {
    async fn submit_order(&self, order: ApiOrderRequest) -> Result<ApiOrderResponse, String> {
        let url = format!("{}/v2/orders", self.base_url);
        let headers = self.get_headers();

        let response = self.client
            .post(&url)
            .header("APCA-API-KEY-ID", &headers["APCA-API-KEY-ID"])
            .header("APCA-API-SECRET-KEY", &headers["APCA-API-SECRET-KEY"])
            .header("Content-Type", "application/json")
            .json(&order)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.status().is_success() {
            response
                .json::<ApiOrderResponse>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Order submission failed: {}", response.status()))
        }
    }

    async fn get_positions(&self) -> Result<Vec<ApiPosition>, String> {
        let url = format!("{}/v2/positions", self.base_url);
        let headers = self.get_headers();

        let response = self.client
            .get(&url)
            .header("APCA-API-KEY-ID", &headers["APCA-API-KEY-ID"])
            .header("APCA-API-SECRET-KEY", &headers["APCA-API-SECRET-KEY"])
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.status().is_success() {
            response
                .json::<Vec<ApiPosition>>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Failed to get positions: {}", response.status()))
        }
    }

    async fn get_account(&self) -> Result<ApiAccount, String> {
        let url = format!("{}/v2/account", self.base_url);
        let headers = self.get_headers();

        let response = self.client
            .get(&url)
            .header("APCA-API-KEY-ID", &headers["APCA-API-KEY-ID"])
            .header("APCA-API-SECRET-KEY", &headers["APCA-API-SECRET-KEY"])
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.status().is_success() {
            response
                .json::<ApiAccount>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Failed to get account: {}", response.status()))
        }
    }

    async fn cancel_order(&self, order_id: String) -> Result<bool, String> {
        let url = format!("{}/v2/orders/{}", self.base_url, order_id);
        let headers = self.get_headers();

        let response = self.client
            .delete(&url)
            .header("APCA-API-KEY-ID", &headers["APCA-API-KEY-ID"])
            .header("APCA-API-SECRET-KEY", &headers["APCA-API-SECRET-KEY"])
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        Ok(response.status().is_success())
    }
}

pub struct TradingApiManager {
    apis: HashMap<String, Box<dyn TradingApi + Send + Sync>>,
    default_api: String,
}

impl TradingApiManager {
    pub fn new() -> Self {
        Self {
            apis: HashMap::new(),
            default_api: "alpaca".to_string(),
        }
    }

    pub fn add_api(&mut self, name: String, api: Box<dyn TradingApi + Send + Sync>) {
        self.apis.insert(name, api);
    }

    pub fn set_default(&mut self, name: String) {
        if self.apis.contains_key(&name) {
            self.default_api = name;
        }
    }

    pub async fn submit_order(&self, order: ApiOrderRequest) -> Result<ApiOrderResponse, String> {
        if let Some(api) = self.apis.get(&self.default_api) {
            api.submit_order(order).await
        } else {
            Err("No trading API configured".to_string())
        }
    }

    pub async fn get_positions(&self) -> Result<Vec<ApiPosition>, String> {
        if let Some(api) = self.apis.get(&self.default_api) {
            api.get_positions().await
        } else {
            Err("No trading API configured".to_string())
        }
    }

    pub async fn get_account(&self) -> Result<ApiAccount, String> {
        if let Some(api) = self.apis.get(&self.default_api) {
            api.get_account().await
        } else {
            Err("No trading API configured".to_string())
        }
    }
}