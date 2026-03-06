use crate::trading::portfolio::Position;
use crate::trading::{Order, OrderStatus, OrderType};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ExecutionEngine {
    #[allow(dead_code)]
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
