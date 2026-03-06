pub mod api;
pub mod brokers;
pub mod portfolio;
pub mod execution;
pub mod trade_history;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: uuid::Uuid,
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: f64,
    pub order_type: OrderType,
    pub price: Option<f64>,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub status: OrderStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Submitted,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

impl Order {
    pub fn new_market_buy(symbol: String, quantity: f64) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            symbol,
            side: OrderSide::Buy,
            quantity,
            order_type: OrderType::Market,
            price: None,
            stop_loss: None,
            take_profit: None,
            status: OrderStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn new_limit_sell(symbol: String, quantity: f64, price: f64) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            symbol,
            side: OrderSide::Sell,
            quantity,
            order_type: OrderType::Limit,
            price: Some(price),
            stop_loss: None,
            take_profit: None,
            status: OrderStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }
}