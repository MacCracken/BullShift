pub mod api;
pub mod brokers;
pub mod execution;
pub mod portfolio;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_market_buy() {
        let order = Order::new_market_buy("AAPL".to_string(), 10.0);
        assert_eq!(order.symbol, "AAPL");
        assert_eq!(order.quantity, 10.0);
        assert!(matches!(order.side, OrderSide::Buy));
        assert!(matches!(order.order_type, OrderType::Market));
        assert!(matches!(order.status, OrderStatus::Pending));
        assert!(order.price.is_none());
    }

    #[test]
    fn test_new_limit_sell() {
        let order = Order::new_limit_sell("TSLA".to_string(), 5.0, 250.0);
        assert_eq!(order.symbol, "TSLA");
        assert_eq!(order.quantity, 5.0);
        assert!(matches!(order.side, OrderSide::Sell));
        assert!(matches!(order.order_type, OrderType::Limit));
        assert!(matches!(order.status, OrderStatus::Pending));
        assert_eq!(order.price, Some(250.0));
    }

    #[test]
    fn test_order_id_unique() {
        let order1 = Order::new_market_buy("AAPL".to_string(), 1.0);
        let order2 = Order::new_market_buy("AAPL".to_string(), 1.0);
        assert_ne!(order1.id, order2.id);
    }

    #[test]
    fn test_order_timestamps() {
        let order = Order::new_market_buy("AAPL".to_string(), 1.0);
        assert_eq!(order.created_at, order.updated_at);
    }
}
