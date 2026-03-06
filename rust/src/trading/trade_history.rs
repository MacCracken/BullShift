use crate::database::{Database, Trade as DbTrade};
use crate::error::BullShiftError;
use crate::trading::{Order, OrderSide, OrderStatus};
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

    pub fn record_order_fill(&self, order: &Order, executed_price: f64) -> Result<i64, BullShiftError> {
        let trade = Trade::from_order(order, executed_price, 0.0);
        self.record_trade(&trade)
    }
}
