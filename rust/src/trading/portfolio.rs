use crate::trading::{Order, OrderStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub orders: Vec<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub positions: HashMap<String, Position>,
    pub cash_balance: f64,
    pub total_value: f64,
    pub available_margin: f64,
}

impl Portfolio {
    pub fn new(initial_cash: f64) -> Self {
        Self {
            positions: HashMap::new(),
            cash_balance: initial_cash,
            total_value: initial_cash,
            available_margin: initial_cash,
        }
    }
    
    pub fn update_position(&mut self, symbol: &str, current_price: f64) {
        if let Some(position) = self.positions.get_mut(symbol) {
            position.current_price = current_price;
            position.unrealized_pnl = (current_price - position.entry_price) * position.quantity;
            self.calculate_total_value();
        }
    }
    
    pub fn add_position(&mut self, position: Position) {
        self.positions.insert(position.symbol.clone(), position);
        self.calculate_total_value();
    }
    
    fn calculate_total_value(&mut self) {
        self.total_value = self.cash_balance + 
            self.positions.values()
                .map(|p| p.quantity * p.current_price + p.unrealized_pnl)
                .sum();
        
        self.available_margin = self.total_value - 
            self.positions.values()
                .map(|p| p.quantity * p.current_price * 0.5) // 50% margin requirement
                .sum::<f64>();
    }
}