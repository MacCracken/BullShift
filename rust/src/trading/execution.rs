use crate::trading::portfolio::Position;
use crate::trading::{Order, OrderStatus, OrderType};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ExecutionEngine {
    #[allow(dead_code)] // retained for live execution wiring
    positions: Arc<Mutex<HashMap<String, Position>>>,
}

impl Default for ExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionEngine {
    pub fn new() -> Self {
        let positions = Arc::new(Mutex::new(HashMap::new()));

        Self { positions }
    }

    pub fn submit_order(&self, order: &mut Order) {
        match order.order_type {
            OrderType::Market => {
                order.status = OrderStatus::Filled;
                log::info!("Market order executed: {:?}", order);
            }
            OrderType::Limit => {
                order.status = OrderStatus::Submitted;
                log::info!("Limit order submitted: {:?}", order);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trading::Order;

    #[test]
    fn test_execution_engine_default() {
        let _engine = ExecutionEngine::default();
    }

    #[test]
    fn test_submit_market_order_fills() {
        let engine = ExecutionEngine::new();
        let mut order = Order::new_market_buy("AAPL".to_string(), 10.0);
        engine.submit_order(&mut order);
        assert!(matches!(order.status, OrderStatus::Filled));
    }

    #[test]
    fn test_submit_limit_order_submitted() {
        let engine = ExecutionEngine::new();
        let mut order = Order::new_limit_sell("TSLA".to_string(), 5.0, 250.0);
        engine.submit_order(&mut order);
        assert!(matches!(order.status, OrderStatus::Submitted));
    }
}
