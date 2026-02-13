use crate::logging::{ErrorDetails, Logger, LogLevel, StructuredLogger};
use crate::trading::{Order, OrderStatus, OrderType};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

pub struct ExecutionEngine {
    order_tx: mpsc::UnboundedSender<Order>,
    positions: Arc<RwLock<std::collections::HashMap<String, crate::trading::Position>>>,
    logger: StructuredLogger,
}

impl ExecutionEngine {
    pub fn new() -> Self {
        let (order_tx, mut order_rx) = mpsc::unbounded_channel::<Order>();
        let positions = Arc::new(RwLock::new(std::collections::HashMap::new()));
        let logger = StructuredLogger::new("execution_engine".to_string(), LogLevel::Info);
        
        let positions_clone = positions.clone();
        let logger_clone = logger.clone();
        
        tokio::spawn(async move {
            while let Some(mut order) = order_rx.recv().await {
                order.status = OrderStatus::Submitted;
                
                match order.order_type {
                    OrderType::Market => {
                        order.status = OrderStatus::Filled;
                        logger_clone.log(
                            LogLevel::Info,
                            "execution",
                            &format!("Market order executed: {:?}", order),
                        );
                    }
                    OrderType::Limit => {
                        order.status = OrderStatus::Submitted;
                        logger_clone.log(
                            LogLevel::Info,
                            "execution",
                            &format!("Limit order submitted: {:?}", order),
                        );
                    }
                    _ => {}
                }
                
                let mut positions = positions_clone.write().await;
            }
        });
        
        Self {
            order_tx,
            positions,
            logger,
        }
    }
}