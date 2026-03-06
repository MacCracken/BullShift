use crate::database::Database;
use crate::error::BullShiftError;
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

    pub fn load(&mut self) -> Result<(), BullShiftError> {
        let db = self
            .db
            .as_ref()
            .ok_or_else(|| BullShiftError::Database("Database not initialized".to_string()))?;

        if let Some((id, cash, total, margin)) = db.get_portfolio()? {
            self.id = Some(id);
            self.cash_balance = cash;
            self.total_value = total;
            self.available_margin = margin;

            let positions = db.get_positions(id)?;

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

    pub fn save(&mut self) -> Result<(), BullShiftError> {
        let db = self
            .db
            .as_ref()
            .ok_or_else(|| BullShiftError::Database("Database not initialized".to_string()))?;

        if let Some(id) = self.id {
            db.update_portfolio(
                id,
                self.cash_balance,
                self.total_value,
                self.available_margin,
            )?;
        } else {
            let id =
                db.save_portfolio(self.cash_balance, self.total_value, self.available_margin)?;
            self.id = Some(id);
        }

        for (symbol, position) in &self.positions {
            if let Some(db_position) = db
                .get_positions(self.id.unwrap())?
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
                )?;
            } else {
                db.save_position(
                    self.id.unwrap(),
                    &position.symbol,
                    position.quantity,
                    position.entry_price,
                    position.current_price,
                    position.unrealized_pnl,
                    position.realized_pnl,
                )?;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_position(symbol: &str, quantity: f64, entry_price: f64) -> Position {
        Position {
            symbol: symbol.to_string(),
            quantity,
            entry_price,
            current_price: entry_price,
            unrealized_pnl: 0.0,
            realized_pnl: 0.0,
            orders: Vec::new(),
        }
    }

    #[test]
    fn test_new_portfolio() {
        let portfolio = Portfolio::new(100_000.0);
        assert_eq!(portfolio.cash_balance, 100_000.0);
        assert_eq!(portfolio.total_value, 100_000.0);
        assert_eq!(portfolio.available_margin, 100_000.0);
    }

    #[test]
    fn test_add_position() {
        let mut portfolio = Portfolio::new(100_000.0);
        let position = make_position("AAPL", 10.0, 150.0);
        portfolio.add_position(position);
        assert!(portfolio.positions.contains_key("AAPL"));
    }

    #[test]
    fn test_update_position_price() {
        let mut portfolio = Portfolio::new(100_000.0);
        let position = make_position("AAPL", 10.0, 100.0);
        portfolio.add_position(position);
        portfolio.update_position("AAPL", 110.0);

        let pos = portfolio.positions.get("AAPL").unwrap();
        assert_eq!(pos.current_price, 110.0);
        let expected_pnl = (110.0 - 100.0) * 10.0;
        assert!((pos.unrealized_pnl - expected_pnl).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_total_value() {
        let mut portfolio = Portfolio::new(100_000.0);
        let position = make_position("AAPL", 10.0, 150.0);
        portfolio.add_position(position);
        // total_value = cash + (quantity * current_price + unrealized_pnl)
        // = 100_000 + (10 * 150 + 0) = 101_500
        assert!((portfolio.total_value - 101_500.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_portfolio_no_db() {
        let mut portfolio = Portfolio::new(100_000.0);
        let result = portfolio.load();
        assert!(result.is_err());
    }
}
