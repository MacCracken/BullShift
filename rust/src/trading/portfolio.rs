use crate::database::Database;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    #[serde(skip)]
    pub orders: Vec<uuid::Uuid>,
}

pub struct Portfolio {
    pub id: Option<i64>,
    pub positions: HashMap<String, Position>,
    pub cash_balance: f64,
    pub total_value: f64,
    pub available_margin: f64,
    pub db: Option<Arc<Database>>,
}

impl Portfolio {
    pub fn new(initial_cash: f64) -> Self {
        Self {
            id: None,
            positions: HashMap::new(),
            cash_balance: initial_cash,
            total_value: initial_cash,
            available_margin: initial_cash,
            db: None,
        }
    }

    pub fn with_database(mut self, db: Arc<Database>) -> Self {
        self.db = Some(db);
        self
    }

    pub fn load(&mut self) -> Result<(), String> {
        let db = self.db.as_ref().ok_or("Database not initialized")?;

        if let Some((id, cash, total, margin)) = db
            .get_portfolio()
            .map_err(|e| format!("Failed to load portfolio: {}", e))?
        {
            self.id = Some(id);
            self.cash_balance = cash;
            self.total_value = total;
            self.available_margin = margin;

            let positions = db
                .get_positions(id)
                .map_err(|e| format!("Failed to load positions: {}", e))?;

            for pos in positions {
                self.positions.insert(
                    pos.symbol.clone(),
                    Position {
                        symbol: pos.symbol,
                        quantity: pos.quantity,
                        entry_price: pos.entry_price,
                        current_price: pos.current_price,
                        unrealized_pnl: pos.unrealized_pnl,
                        realized_pnl: pos.realized_pnl,
                        orders: Vec::new(),
                    },
                );
            }
        }

        Ok(())
    }

    pub fn save(&mut self) -> Result<(), String> {
        let db = self.db.as_ref().ok_or("Database not initialized")?;

        if let Some(id) = self.id {
            db.update_portfolio(
                id,
                self.cash_balance,
                self.total_value,
                self.available_margin,
            )
            .map_err(|e| format!("Failed to update portfolio: {}", e))?;
        } else {
            let id = db
                .save_portfolio(self.cash_balance, self.total_value, self.available_margin)
                .map_err(|e| format!("Failed to save portfolio: {}", e))?;
            self.id = Some(id);
        }

        for (symbol, position) in &self.positions {
            if let Some(db_position) = db
                .get_positions(self.id.unwrap())
                .map_err(|e| format!("Failed to check position: {}", e))?
                .into_iter()
                .find(|p| &p.symbol == symbol)
            {
                db.update_position(
                    db_position.id,
                    position.quantity,
                    position.entry_price,
                    position.current_price,
                    position.unrealized_pnl,
                    position.realized_pnl,
                )
                .map_err(|e| format!("Failed to update position: {}", e))?;
            } else {
                db.save_position(
                    self.id.unwrap(),
                    &position.symbol,
                    position.quantity,
                    position.entry_price,
                    position.current_price,
                    position.unrealized_pnl,
                    position.realized_pnl,
                )
                .map_err(|e| format!("Failed to save position: {}", e))?;
            }
        }

        Ok(())
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
        self.total_value = self.cash_balance
            + self
                .positions
                .values()
                .map(|p| p.quantity * p.current_price + p.unrealized_pnl)
                .sum::<f64>();

        self.available_margin = self.total_value
            - self
                .positions
                .values()
                .map(|p| p.quantity * p.current_price * 0.5)
                .sum::<f64>();
    }
}
