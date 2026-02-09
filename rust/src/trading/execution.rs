use crate::trading::{Order, OrderStatus, OrderType};
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ExecutionEngine {
    order_tx: mpsc::UnboundedSender<Order>,
    positions: Arc<RwLock<std::collections::HashMap<String, crate::trading::Position>>>,
}

impl ExecutionEngine {
    pub fn new() -> Self {
        let (order_tx, mut order_rx) = mpsc::unbounded_channel::<Order>();
        let positions = Arc::new(RwLock::new(std::collections::HashMap::new()));
        
        let positions_clone = positions.clone();
        tokio::spawn(async move {
            while let Some(mut order) = order_rx.recv().await {
                order.status = OrderStatus::Submitted;
                
                // Simulate order execution
                match order.order_type {
                    OrderType::Market => {
                        order.status = OrderStatus::Filled;
                        log::info!("Market order executed: {:?}", order);
                    }
                    OrderType::Limit => {
                        // In real implementation, would wait for price match
                        order.status = OrderStatus::Submitted;
                        log::info!("Limit order submitted: {:?}", order);
                    }
                    _ => {}
                }
                
                // Update positions
                let mut positions = positions_clone.write().await;
                // Position update logic here
            }
        });
        
        Self {
            order_tx,
            positions,
        }
    }
    
    pub async fn submit_order(&self, order: Order) -> Result<(), String> {
        self.order_tx.send(order).map_err(|e| e.to_string())
    }
    
    pub async fn get_positions(&self) -> std::collections::HashMap<String, crate::trading::Position> {
        self.positions.read().await.clone()
    }
}