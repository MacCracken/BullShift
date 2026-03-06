use crate::error::BullShiftError;
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
    async fn submit_order(&self, order: ApiOrderRequest) -> Result<ApiOrderResponse, BullShiftError>;
    async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError>;
    async fn get_account(&self) -> Result<ApiAccount, BullShiftError>;
    async fn cancel_order(&self, order_id: String) -> Result<bool, BullShiftError>;
}

pub struct AlpacaApi {
    client: Client,
    base_url: String,
    api_key: String,
    api_secret: String,
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
            api_key: credentials.api_key,
            api_secret: credentials.api_secret,
            base_url,
        }
    }
}

impl TradingApi for AlpacaApi {
    async fn submit_order(&self, order: ApiOrderRequest) -> Result<ApiOrderResponse, BullShiftError> {
        let url = format!("{}/v2/orders", self.base_url);

        let response = self.client
            .post(&url)
            .header("APCA-API-KEY-ID", &self.api_key)
            .header("APCA-API-SECRET-KEY", &self.api_secret)
            .header("Content-Type", "application/json")
            .json(&order)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<ApiOrderResponse>().await?)
        } else {
            Err(BullShiftError::Api(format!("Order submission failed: {}", response.status())))
        }
    }

    async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError> {
        let url = format!("{}/v2/positions", self.base_url);

        let response = self.client
            .get(&url)
            .header("APCA-API-KEY-ID", &self.api_key)
            .header("APCA-API-SECRET-KEY", &self.api_secret)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<Vec<ApiPosition>>().await?)
        } else {
            Err(BullShiftError::Api(format!("Failed to get positions: {}", response.status())))
        }
    }

    async fn get_account(&self) -> Result<ApiAccount, BullShiftError> {
        let url = format!("{}/v2/account", self.base_url);

        let response = self.client
            .get(&url)
            .header("APCA-API-KEY-ID", &self.api_key)
            .header("APCA-API-SECRET-KEY", &self.api_secret)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<ApiAccount>().await?)
        } else {
            Err(BullShiftError::Api(format!("Failed to get account: {}", response.status())))
        }
    }

    async fn cancel_order(&self, order_id: String) -> Result<bool, BullShiftError> {
        let url = format!("{}/v2/orders/{}", self.base_url, order_id);

        let response = self.client
            .delete(&url)
            .header("APCA-API-KEY-ID", &self.api_key)
            .header("APCA-API-SECRET-KEY", &self.api_secret)
            .send()
            .await?;

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

    pub async fn submit_order(&self, order: ApiOrderRequest) -> Result<ApiOrderResponse, BullShiftError> {
        if let Some(api) = self.apis.get(&self.default_api) {
            api.submit_order(order).await
        } else {
            Err(BullShiftError::Configuration("No trading API configured".to_string()))
        }
    }

    pub async fn get_positions(&self) -> Result<Vec<ApiPosition>, BullShiftError> {
        if let Some(api) = self.apis.get(&self.default_api) {
            api.get_positions().await
        } else {
            Err(BullShiftError::Configuration("No trading API configured".to_string()))
        }
    }

    pub async fn get_account(&self) -> Result<ApiAccount, BullShiftError> {
        if let Some(api) = self.apis.get(&self.default_api) {
            api.get_account().await
        } else {
            Err(BullShiftError::Configuration("No trading API configured".to_string()))
        }
    }
}