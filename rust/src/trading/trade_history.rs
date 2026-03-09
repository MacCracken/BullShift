use crate::database::Database;
use crate::error::BullShiftError;
use crate::trading::{Order, OrderSide};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub order_id: String,
    pub symbol: String,
    pub side: String,
    pub quantity: f64,
    pub price: f64,
    pub commission: f64,
    pub executed_at: String,
}

impl Trade {
    pub fn from_order(order: &Order, executed_price: f64, commission: f64) -> Self {
        let side = match order.side {
            OrderSide::Buy => "BUY",
            OrderSide::Sell => "SELL",
        };

        Self {
            order_id: order.id.to_string(),
            symbol: order.symbol.clone(),
            side: side.to_string(),
            quantity: order.quantity,
            price: executed_price,
            commission,
            executed_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

pub struct TradeHistory {
    db: Arc<Database>,
}

impl TradeHistory {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn record_trade(&self, trade: &Trade) -> Result<i64, BullShiftError> {
        Ok(self.db.save_trade(
            &trade.order_id,
            &trade.symbol,
            &trade.side,
            trade.quantity,
            trade.price,
            trade.commission,
        )?)
    }

    pub fn get_trades(
        &self,
        symbol: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<Trade>, BullShiftError> {
        let db_trades = self.db.get_trades(symbol, limit)?;

        Ok(db_trades
            .into_iter()
            .map(|t| Trade {
                order_id: t.order_id,
                symbol: t.symbol,
                side: t.side,
                quantity: t.quantity,
                price: t.price,
                commission: t.commission,
                executed_at: t.executed_at,
            })
            .collect())
    }

    pub fn get_trades_by_date_range(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<Trade>, BullShiftError> {
        let db_trades = self.db.get_trades_by_date_range(start_date, end_date)?;

        Ok(db_trades
            .into_iter()
            .map(|t| Trade {
                order_id: t.order_id,
                symbol: t.symbol,
                side: t.side,
                quantity: t.quantity,
                price: t.price,
                commission: t.commission,
                executed_at: t.executed_at,
            })
            .collect())
    }

    pub fn record_order_fill(
        &self,
        order: &Order,
        executed_price: f64,
    ) -> Result<i64, BullShiftError> {
        let trade = Trade::from_order(order, executed_price, 0.0);
        self.record_trade(&trade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trading::Order;

    #[test]
    fn test_trade_from_buy_order() {
        let order = Order::new_market_buy("AAPL".to_string(), 10.0);
        let trade = Trade::from_order(&order, 150.0, 1.50);
        assert_eq!(trade.symbol, "AAPL");
        assert_eq!(trade.side, "BUY");
        assert_eq!(trade.quantity, 10.0);
        assert_eq!(trade.price, 150.0);
        assert_eq!(trade.commission, 1.50);
        assert_eq!(trade.order_id, order.id.to_string());
        assert!(!trade.executed_at.is_empty());
    }

    #[test]
    fn test_trade_from_sell_order() {
        let order = Order::new_limit_sell("TSLA".to_string(), 5.0, 250.0);
        let trade = Trade::from_order(&order, 251.0, 0.0);
        assert_eq!(trade.symbol, "TSLA");
        assert_eq!(trade.side, "SELL");
        assert_eq!(trade.quantity, 5.0);
        assert_eq!(trade.price, 251.0);
    }

    #[test]
    fn test_trade_history_record_and_get() {
        let dir = std::env::temp_dir().join(format!("bullshift_test_{}", uuid::Uuid::new_v4()));
        let db = std::sync::Arc::new(Database::new(dir).unwrap());
        let history = TradeHistory::new(db);

        let order = Order::new_market_buy("GOOG".to_string(), 3.0);
        let trade = Trade::from_order(&order, 175.0, 0.50);
        let id = history.record_trade(&trade).unwrap();
        assert!(id > 0);

        let trades = history.get_trades(Some("GOOG"), None).unwrap();
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].symbol, "GOOG");
        assert_eq!(trades[0].quantity, 3.0);
    }

    #[test]
    fn test_trade_history_get_trades_no_filter() {
        let dir = std::env::temp_dir().join(format!("bullshift_test_{}", uuid::Uuid::new_v4()));
        let db = std::sync::Arc::new(Database::new(dir).unwrap());
        let history = TradeHistory::new(db);

        let o1 = Order::new_market_buy("AAPL".to_string(), 1.0);
        let o2 = Order::new_market_buy("MSFT".to_string(), 2.0);
        history.record_trade(&Trade::from_order(&o1, 100.0, 0.0)).unwrap();
        history.record_trade(&Trade::from_order(&o2, 200.0, 0.0)).unwrap();

        let all = history.get_trades(None, None).unwrap();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_trade_history_get_trades_with_limit() {
        let dir = std::env::temp_dir().join(format!("bullshift_test_{}", uuid::Uuid::new_v4()));
        let db = std::sync::Arc::new(Database::new(dir).unwrap());
        let history = TradeHistory::new(db);

        for i in 0..5 {
            let o = Order::new_market_buy("AAPL".to_string(), (i + 1) as f64);
            history.record_trade(&Trade::from_order(&o, 100.0, 0.0)).unwrap();
        }

        let limited = history.get_trades(None, Some(3)).unwrap();
        assert_eq!(limited.len(), 3);
    }

    #[test]
    fn test_trade_history_record_order_fill() {
        let dir = std::env::temp_dir().join(format!("bullshift_test_{}", uuid::Uuid::new_v4()));
        let db = std::sync::Arc::new(Database::new(dir).unwrap());
        let history = TradeHistory::new(db);

        let order = Order::new_market_buy("AMZN".to_string(), 2.0);
        let id = history.record_order_fill(&order, 185.0).unwrap();
        assert!(id > 0);

        let trades = history.get_trades(Some("AMZN"), None).unwrap();
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].price, 185.0);
        assert_eq!(trades[0].commission, 0.0);
    }

    #[test]
    fn test_trade_history_filter_by_symbol() {
        let dir = std::env::temp_dir().join(format!("bullshift_test_{}", uuid::Uuid::new_v4()));
        let db = std::sync::Arc::new(Database::new(dir).unwrap());
        let history = TradeHistory::new(db);

        let o1 = Order::new_market_buy("AAPL".to_string(), 1.0);
        let o2 = Order::new_market_buy("MSFT".to_string(), 1.0);
        let o3 = Order::new_market_buy("AAPL".to_string(), 1.0);
        history.record_trade(&Trade::from_order(&o1, 100.0, 0.0)).unwrap();
        history.record_trade(&Trade::from_order(&o2, 100.0, 0.0)).unwrap();
        history.record_trade(&Trade::from_order(&o3, 100.0, 0.0)).unwrap();

        let aapl = history.get_trades(Some("AAPL"), None).unwrap();
        assert_eq!(aapl.len(), 2);

        let msft = history.get_trades(Some("MSFT"), None).unwrap();
        assert_eq!(msft.len(), 1);
    }
}
