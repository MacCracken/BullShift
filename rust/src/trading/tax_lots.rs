use crate::error::BullShiftError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Cost basis accounting method for tax lot disposition.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CostBasisMethod {
    /// First-in, first-out.
    #[default]
    Fifo,
    /// Last-in, first-out.
    Lifo,
    /// Highest cost lots sold first (minimizes gains).
    HighCost,
    /// Lowest cost lots sold first (maximizes gains).
    LowCost,
    /// Specific lot identification (manual selection).
    SpecificId,
}

impl std::fmt::Display for CostBasisMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CostBasisMethod::Fifo => write!(f, "FIFO"),
            CostBasisMethod::Lifo => write!(f, "LIFO"),
            CostBasisMethod::HighCost => write!(f, "Highest Cost"),
            CostBasisMethod::LowCost => write!(f, "Lowest Cost"),
            CostBasisMethod::SpecificId => write!(f, "Specific ID"),
        }
    }
}

/// A tax lot representing a specific purchase of shares.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxLot {
    pub id: Uuid,
    pub symbol: String,
    pub purchase_date: DateTime<Utc>,
    pub quantity: f64,
    pub remaining_quantity: f64,
    pub cost_per_share: f64,
    pub commission: f64,
    pub order_id: String,
}

impl TaxLot {
    pub fn new(
        symbol: &str,
        quantity: f64,
        cost_per_share: f64,
        commission: f64,
        order_id: &str,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            symbol: symbol.to_string(),
            purchase_date: Utc::now(),
            quantity,
            remaining_quantity: quantity,
            cost_per_share,
            commission,
            order_id: order_id.to_string(),
        }
    }

    pub fn with_date(mut self, date: DateTime<Utc>) -> Self {
        self.purchase_date = date;
        self
    }

    /// Total cost basis for remaining shares in this lot.
    pub fn cost_basis(&self) -> f64 {
        if self.quantity == 0.0 {
            return 0.0;
        }
        self.remaining_quantity * self.cost_per_share
            + (self.commission * self.remaining_quantity / self.quantity)
    }

    /// Whether this lot qualifies for long-term capital gains (held > 1 year).
    pub fn is_long_term(&self) -> bool {
        let held_days = (Utc::now() - self.purchase_date).num_days();
        held_days > 365
    }

    /// Whether this lot is fully disposed.
    pub fn is_closed(&self) -> bool {
        self.remaining_quantity <= 0.0
    }
}

/// A realized gain/loss from disposing shares from a tax lot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealizedGainLoss {
    pub id: Uuid,
    pub symbol: String,
    pub quantity: f64,
    pub purchase_date: DateTime<Utc>,
    pub sale_date: DateTime<Utc>,
    pub cost_per_share: f64,
    pub sale_price: f64,
    pub commission_buy: f64,
    pub commission_sell: f64,
    pub gain_loss: f64,
    pub is_long_term: bool,
    pub lot_id: Uuid,
    pub order_id: String,
}

/// Summary of tax lots for a single symbol.
#[derive(Debug, Clone, Serialize)]
pub struct SymbolTaxSummary {
    pub symbol: String,
    pub total_shares: f64,
    pub total_cost_basis: f64,
    pub average_cost: f64,
    pub open_lots: usize,
    pub unrealized_gain_loss: f64,
}

/// Annual tax report summary.
#[derive(Debug, Clone, Serialize)]
pub struct TaxReport {
    pub tax_year: i32,
    pub short_term_gains: f64,
    pub short_term_losses: f64,
    pub long_term_gains: f64,
    pub long_term_losses: f64,
    pub total_gains: f64,
    pub total_losses: f64,
    pub net_gain_loss: f64,
    pub wash_sale_disallowed: f64,
    pub transactions: Vec<RealizedGainLoss>,
}

/// Tracks tax lots and realized gains/losses across all positions.
pub struct TaxLotTracker {
    lots: HashMap<String, Vec<TaxLot>>,
    realized: Vec<RealizedGainLoss>,
    method: CostBasisMethod,
}

impl Default for TaxLotTracker {
    fn default() -> Self {
        Self::new(CostBasisMethod::Fifo)
    }
}

impl TaxLotTracker {
    pub fn new(method: CostBasisMethod) -> Self {
        Self {
            lots: HashMap::new(),
            realized: Vec::new(),
            method,
        }
    }

    pub fn set_method(&mut self, method: CostBasisMethod) {
        self.method = method;
    }

    pub fn method(&self) -> CostBasisMethod {
        self.method
    }

    /// Record a purchase (creates a new tax lot).
    /// Returns an error if quantity or price are not positive finite values.
    pub fn record_buy(
        &mut self,
        symbol: &str,
        quantity: f64,
        price: f64,
        commission: f64,
        order_id: &str,
    ) -> Result<Uuid, BullShiftError> {
        if quantity <= 0.0 || !quantity.is_finite() {
            return Err(BullShiftError::Validation(
                "Quantity must be a positive finite number".to_string(),
            ));
        }
        if price < 0.0 || !price.is_finite() {
            return Err(BullShiftError::Validation(
                "Price must be a non-negative finite number".to_string(),
            ));
        }
        if commission < 0.0 || !commission.is_finite() {
            return Err(BullShiftError::Validation(
                "Commission must be a non-negative finite number".to_string(),
            ));
        }
        let lot = TaxLot::new(symbol, quantity, price, commission, order_id);
        let lot_id = lot.id;
        self.lots.entry(symbol.to_string()).or_default().push(lot);
        Ok(lot_id)
    }

    /// Record a purchase with a specific date (for imported trades).
    pub fn record_buy_with_date(
        &mut self,
        symbol: &str,
        quantity: f64,
        price: f64,
        commission: f64,
        order_id: &str,
        date: DateTime<Utc>,
    ) -> Result<Uuid, BullShiftError> {
        if quantity <= 0.0 || !quantity.is_finite() {
            return Err(BullShiftError::Validation(
                "Quantity must be a positive finite number".to_string(),
            ));
        }
        if price < 0.0 || !price.is_finite() {
            return Err(BullShiftError::Validation(
                "Price must be a non-negative finite number".to_string(),
            ));
        }
        let lot = TaxLot::new(symbol, quantity, price, commission, order_id).with_date(date);
        let lot_id = lot.id;
        self.lots.entry(symbol.to_string()).or_default().push(lot);
        Ok(lot_id)
    }

    /// Record a sale (disposes shares from tax lots using the configured method).
    pub fn record_sell(
        &mut self,
        symbol: &str,
        quantity: f64,
        sale_price: f64,
        commission: f64,
        order_id: &str,
    ) -> Result<Vec<RealizedGainLoss>, BullShiftError> {
        let lots = self
            .lots
            .get_mut(symbol)
            .ok_or_else(|| BullShiftError::Trading(format!("No tax lots found for {}", symbol)))?;

        let total_available: f64 = lots.iter().map(|l| l.remaining_quantity).sum();
        if total_available < quantity {
            return Err(BullShiftError::Trading(format!(
                "Insufficient shares for {}: have {}, selling {}",
                symbol, total_available, quantity
            )));
        }

        // Sort lots according to cost basis method
        self.sort_lots(symbol);

        let lots = self.lots.get_mut(symbol).ok_or_else(|| {
            BullShiftError::Trading(format!("Tax lots for {} disappeared after sort", symbol))
        })?;
        let mut remaining_to_sell = quantity;
        let commission_per_share = if quantity > 0.0 {
            commission / quantity
        } else {
            0.0
        };
        let mut gains = Vec::new();

        for lot in lots.iter_mut() {
            if remaining_to_sell <= 0.0 {
                break;
            }
            if lot.remaining_quantity <= 0.0 {
                continue;
            }

            let sell_from_lot = remaining_to_sell.min(lot.remaining_quantity);
            let buy_commission_portion = if lot.quantity > 0.0 {
                lot.commission * sell_from_lot / lot.quantity
            } else {
                0.0
            };
            let sell_commission_portion = commission_per_share * sell_from_lot;

            let cost = sell_from_lot * lot.cost_per_share + buy_commission_portion;
            let proceeds = sell_from_lot * sale_price - sell_commission_portion;
            let gain_loss = proceeds - cost;

            let held_days = (Utc::now() - lot.purchase_date).num_days();

            let realized = RealizedGainLoss {
                id: Uuid::new_v4(),
                symbol: symbol.to_string(),
                quantity: sell_from_lot,
                purchase_date: lot.purchase_date,
                sale_date: Utc::now(),
                cost_per_share: lot.cost_per_share,
                sale_price,
                commission_buy: buy_commission_portion,
                commission_sell: sell_commission_portion,
                gain_loss,
                is_long_term: held_days > 365,
                lot_id: lot.id,
                order_id: order_id.to_string(),
            };

            lot.remaining_quantity -= sell_from_lot;
            remaining_to_sell -= sell_from_lot;

            self.realized.push(realized.clone());
            gains.push(realized);
        }

        Ok(gains)
    }

    /// Record a sale from a specific tax lot (SpecificId method).
    pub fn record_sell_specific_lot(
        &mut self,
        lot_id: &Uuid,
        quantity: f64,
        sale_price: f64,
        commission: f64,
        order_id: &str,
    ) -> Result<RealizedGainLoss, BullShiftError> {
        let (symbol, lot) = self
            .lots
            .iter_mut()
            .flat_map(|(sym, lots)| lots.iter_mut().map(move |lot| (sym.clone(), lot)))
            .find(|(_, lot)| &lot.id == lot_id)
            .ok_or_else(|| BullShiftError::Trading("Tax lot not found".to_string()))?;

        if lot.remaining_quantity < quantity {
            return Err(BullShiftError::Trading(format!(
                "Lot has {} shares remaining, cannot sell {}",
                lot.remaining_quantity, quantity
            )));
        }

        let buy_commission_portion = if lot.quantity > 0.0 {
            lot.commission * quantity / lot.quantity
        } else {
            0.0
        };
        let sell_commission_portion = commission;
        let cost = quantity * lot.cost_per_share + buy_commission_portion;
        let proceeds = quantity * sale_price - sell_commission_portion;
        let held_days = (Utc::now() - lot.purchase_date).num_days();

        let realized = RealizedGainLoss {
            id: Uuid::new_v4(),
            symbol,
            quantity,
            purchase_date: lot.purchase_date,
            sale_date: Utc::now(),
            cost_per_share: lot.cost_per_share,
            sale_price,
            commission_buy: buy_commission_portion,
            commission_sell: sell_commission_portion,
            gain_loss: proceeds - cost,
            is_long_term: held_days > 365,
            lot_id: *lot_id,
            order_id: order_id.to_string(),
        };

        lot.remaining_quantity -= quantity;
        self.realized.push(realized.clone());
        Ok(realized)
    }

    /// Get all open (non-fully-disposed) tax lots for a symbol.
    pub fn open_lots(&self, symbol: &str) -> Vec<&TaxLot> {
        self.lots
            .get(symbol)
            .map(|lots| lots.iter().filter(|l| !l.is_closed()).collect())
            .unwrap_or_default()
    }

    /// Get all open tax lots across all symbols.
    pub fn all_open_lots(&self) -> Vec<&TaxLot> {
        self.lots
            .values()
            .flat_map(|lots| lots.iter().filter(|l| !l.is_closed()))
            .collect()
    }

    /// Get all realized gains/losses.
    pub fn realized_gains(&self) -> &[RealizedGainLoss] {
        &self.realized
    }

    /// Get realized gains/losses for a specific symbol.
    pub fn realized_for_symbol(&self, symbol: &str) -> Vec<&RealizedGainLoss> {
        self.realized
            .iter()
            .filter(|r| r.symbol == symbol)
            .collect()
    }

    /// Compute a tax summary for a single symbol (open lots).
    pub fn symbol_summary(&self, symbol: &str, current_price: f64) -> SymbolTaxSummary {
        let lots = self.open_lots(symbol);
        let total_shares: f64 = lots.iter().map(|l| l.remaining_quantity).sum();
        let total_cost_basis: f64 = lots.iter().map(|l| l.cost_basis()).sum();
        let average_cost = if total_shares > 0.0 {
            total_cost_basis / total_shares
        } else {
            0.0
        };
        let market_value = total_shares * current_price;
        let unrealized = market_value - total_cost_basis;

        SymbolTaxSummary {
            symbol: symbol.to_string(),
            total_shares,
            total_cost_basis,
            average_cost,
            open_lots: lots.len(),
            unrealized_gain_loss: unrealized,
        }
    }

    /// Generate a tax report for a given year.
    pub fn tax_report(&self, year: i32) -> TaxReport {
        let transactions: Vec<RealizedGainLoss> = self
            .realized
            .iter()
            .filter(|r| r.sale_date.date_naive().year() == year)
            .cloned()
            .collect();

        let mut short_term_gains = 0.0;
        let mut short_term_losses = 0.0;
        let mut long_term_gains = 0.0;
        let mut long_term_losses = 0.0;

        for t in &transactions {
            if t.is_long_term {
                if t.gain_loss >= 0.0 {
                    long_term_gains += t.gain_loss;
                } else {
                    long_term_losses += t.gain_loss.abs();
                }
            } else if t.gain_loss >= 0.0 {
                short_term_gains += t.gain_loss;
            } else {
                short_term_losses += t.gain_loss.abs();
            }
        }

        let total_gains = short_term_gains + long_term_gains;
        let total_losses = short_term_losses + long_term_losses;

        // Basic wash sale detection: if a loss is realized and the same symbol
        // was purchased within 30 days before or after the sale.
        let wash_sale_disallowed = self.detect_wash_sales(&transactions);

        TaxReport {
            tax_year: year,
            short_term_gains,
            short_term_losses,
            long_term_gains,
            long_term_losses,
            total_gains,
            total_losses,
            net_gain_loss: total_gains - total_losses,
            wash_sale_disallowed,
            transactions,
        }
    }

    fn detect_wash_sales(&self, transactions: &[RealizedGainLoss]) -> f64 {
        let mut disallowed = 0.0;

        for t in transactions {
            if t.gain_loss >= 0.0 {
                continue; // Only losses can be wash sales
            }

            let sale_date = t.sale_date;
            let wash_window_start = sale_date - chrono::Duration::days(30);
            let wash_window_end = sale_date + chrono::Duration::days(30);

            // Check if any purchase of the same symbol occurred within the window
            if let Some(lots) = self.lots.get(&t.symbol) {
                for lot in lots {
                    if lot.id == t.lot_id {
                        continue; // Don't match against the lot being sold
                    }
                    if lot.purchase_date >= wash_window_start
                        && lot.purchase_date <= wash_window_end
                    {
                        disallowed += t.gain_loss.abs();
                        break;
                    }
                }
            }
        }

        disallowed
    }

    fn sort_lots(&mut self, symbol: &str) {
        if let Some(lots) = self.lots.get_mut(symbol) {
            match self.method {
                CostBasisMethod::Fifo => {
                    lots.sort_by(|a, b| a.purchase_date.cmp(&b.purchase_date));
                }
                CostBasisMethod::Lifo => {
                    lots.sort_by(|a, b| b.purchase_date.cmp(&a.purchase_date));
                }
                CostBasisMethod::HighCost => {
                    lots.sort_by(|a, b| {
                        b.cost_per_share
                            .partial_cmp(&a.cost_per_share)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                }
                CostBasisMethod::LowCost => {
                    lots.sort_by(|a, b| {
                        a.cost_per_share
                            .partial_cmp(&b.cost_per_share)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                }
                CostBasisMethod::SpecificId => {
                    // No sorting needed — use record_sell_specific_lot instead
                }
            }
        }
    }
}

// Needed for tax_report year filtering
use chrono::Datelike;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_buy_creates_lot() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Fifo);
        let lot_id = tracker
            .record_buy("AAPL", 10.0, 150.0, 5.0, "order-1")
            .unwrap();
        assert!(!lot_id.is_nil());

        let lots = tracker.open_lots("AAPL");
        assert_eq!(lots.len(), 1);
        assert_eq!(lots[0].quantity, 10.0);
        assert_eq!(lots[0].remaining_quantity, 10.0);
        assert_eq!(lots[0].cost_per_share, 150.0);
    }

    #[test]
    fn test_record_sell_fifo() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Fifo);
        tracker
            .record_buy("AAPL", 10.0, 100.0, 0.0, "buy-1")
            .unwrap();
        tracker
            .record_buy("AAPL", 10.0, 200.0, 0.0, "buy-2")
            .unwrap();

        let gains = tracker
            .record_sell("AAPL", 10.0, 150.0, 0.0, "sell-1")
            .unwrap();
        assert_eq!(gains.len(), 1);
        assert!((gains[0].cost_per_share - 100.0).abs() < f64::EPSILON); // FIFO: sells cheapest first
        assert!((gains[0].gain_loss - 500.0).abs() < f64::EPSILON); // (150-100)*10 = 500
    }

    #[test]
    fn test_record_sell_lifo() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Lifo);
        tracker
            .record_buy("AAPL", 10.0, 100.0, 0.0, "buy-1")
            .unwrap();
        tracker
            .record_buy("AAPL", 10.0, 200.0, 0.0, "buy-2")
            .unwrap();

        let gains = tracker
            .record_sell("AAPL", 10.0, 150.0, 0.0, "sell-1")
            .unwrap();
        assert_eq!(gains.len(), 1);
        assert!((gains[0].cost_per_share - 200.0).abs() < f64::EPSILON); // LIFO: sells newest first
        assert!((gains[0].gain_loss - -500.0).abs() < f64::EPSILON); // (150-200)*10 = -500
    }

    #[test]
    fn test_record_sell_high_cost() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::HighCost);
        tracker
            .record_buy("AAPL", 10.0, 100.0, 0.0, "buy-1")
            .unwrap();
        tracker
            .record_buy("AAPL", 10.0, 200.0, 0.0, "buy-2")
            .unwrap();
        tracker
            .record_buy("AAPL", 10.0, 150.0, 0.0, "buy-3")
            .unwrap();

        let gains = tracker
            .record_sell("AAPL", 10.0, 175.0, 0.0, "sell-1")
            .unwrap();
        assert_eq!(gains.len(), 1);
        assert!((gains[0].cost_per_share - 200.0).abs() < f64::EPSILON); // Highest cost first
    }

    #[test]
    fn test_record_sell_low_cost() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::LowCost);
        tracker
            .record_buy("AAPL", 10.0, 200.0, 0.0, "buy-1")
            .unwrap();
        tracker
            .record_buy("AAPL", 10.0, 100.0, 0.0, "buy-2")
            .unwrap();
        tracker
            .record_buy("AAPL", 10.0, 150.0, 0.0, "buy-3")
            .unwrap();

        let gains = tracker
            .record_sell("AAPL", 10.0, 175.0, 0.0, "sell-1")
            .unwrap();
        assert!((gains[0].cost_per_share - 100.0).abs() < f64::EPSILON); // Lowest cost first
    }

    #[test]
    fn test_partial_lot_sell() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Fifo);
        tracker
            .record_buy("AAPL", 100.0, 150.0, 10.0, "buy-1")
            .unwrap();

        let gains = tracker
            .record_sell("AAPL", 30.0, 180.0, 3.0, "sell-1")
            .unwrap();
        assert_eq!(gains.len(), 1);
        assert!((gains[0].quantity - 30.0).abs() < f64::EPSILON);

        let lots = tracker.open_lots("AAPL");
        assert_eq!(lots.len(), 1);
        assert!((lots[0].remaining_quantity - 70.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_multi_lot_sell() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Fifo);
        tracker
            .record_buy("AAPL", 5.0, 100.0, 0.0, "buy-1")
            .unwrap();
        tracker
            .record_buy("AAPL", 5.0, 150.0, 0.0, "buy-2")
            .unwrap();

        let gains = tracker
            .record_sell("AAPL", 8.0, 200.0, 0.0, "sell-1")
            .unwrap();
        assert_eq!(gains.len(), 2); // Spans two lots
        assert!((gains[0].quantity - 5.0).abs() < f64::EPSILON); // All of lot 1
        assert!((gains[1].quantity - 3.0).abs() < f64::EPSILON); // Part of lot 2
    }

    #[test]
    fn test_sell_insufficient_shares() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Fifo);
        tracker
            .record_buy("AAPL", 10.0, 150.0, 0.0, "buy-1")
            .unwrap();

        let result = tracker.record_sell("AAPL", 20.0, 200.0, 0.0, "sell-1");
        assert!(result.is_err());
    }

    #[test]
    fn test_sell_no_lots() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Fifo);
        let result = tracker.record_sell("AAPL", 10.0, 200.0, 0.0, "sell-1");
        assert!(result.is_err());
    }

    #[test]
    fn test_sell_specific_lot() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::SpecificId);
        let lot1 = tracker
            .record_buy("AAPL", 10.0, 100.0, 0.0, "buy-1")
            .unwrap();
        let _lot2 = tracker
            .record_buy("AAPL", 10.0, 200.0, 0.0, "buy-2")
            .unwrap();

        let gain = tracker
            .record_sell_specific_lot(&lot1, 5.0, 150.0, 0.0, "sell-1")
            .unwrap();
        assert!((gain.cost_per_share - 100.0).abs() < f64::EPSILON);
        assert!((gain.quantity - 5.0).abs() < f64::EPSILON);
        assert!((gain.gain_loss - 250.0).abs() < f64::EPSILON); // (150-100)*5
    }

    #[test]
    fn test_symbol_summary() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Fifo);
        tracker
            .record_buy("AAPL", 10.0, 100.0, 0.0, "buy-1")
            .unwrap();
        tracker
            .record_buy("AAPL", 10.0, 200.0, 0.0, "buy-2")
            .unwrap();

        let summary = tracker.symbol_summary("AAPL", 160.0);
        assert_eq!(summary.total_shares, 20.0);
        assert!((summary.total_cost_basis - 3000.0).abs() < f64::EPSILON); // 10*100 + 10*200
        assert!((summary.average_cost - 150.0).abs() < f64::EPSILON); // 3000/20
        assert_eq!(summary.open_lots, 2);
        // unrealized = 20*160 - 3000 = 3200 - 3000 = 200
        assert!((summary.unrealized_gain_loss - 200.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_tax_report() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Fifo);
        tracker
            .record_buy("AAPL", 10.0, 100.0, 0.0, "buy-1")
            .unwrap();
        tracker
            .record_sell("AAPL", 10.0, 150.0, 0.0, "sell-1")
            .unwrap();

        let report = tracker.tax_report(Utc::now().year());
        assert_eq!(report.transactions.len(), 1);
        assert!((report.short_term_gains - 500.0).abs() < f64::EPSILON);
        assert!((report.short_term_losses).abs() < f64::EPSILON);
        assert!((report.net_gain_loss - 500.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_tax_report_with_losses() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Fifo);
        tracker
            .record_buy("AAPL", 10.0, 200.0, 0.0, "buy-1")
            .unwrap();
        tracker
            .record_sell("AAPL", 10.0, 150.0, 0.0, "sell-1")
            .unwrap();

        let report = tracker.tax_report(Utc::now().year());
        assert!((report.short_term_losses - 500.0).abs() < f64::EPSILON);
        assert!((report.net_gain_loss - -500.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_commission_in_cost_basis() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Fifo);
        tracker
            .record_buy("AAPL", 10.0, 100.0, 10.0, "buy-1")
            .unwrap(); // $10 commission

        let gains = tracker
            .record_sell("AAPL", 10.0, 100.0, 5.0, "sell-1")
            .unwrap();
        // Cost = 10*100 + 10 = 1010, Proceeds = 10*100 - 5 = 995
        // Loss = 995 - 1010 = -15
        assert!((gains[0].gain_loss - -15.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_all_open_lots_multi_symbol() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Fifo);
        tracker
            .record_buy("AAPL", 10.0, 150.0, 0.0, "buy-1")
            .unwrap();
        tracker
            .record_buy("TSLA", 5.0, 200.0, 0.0, "buy-2")
            .unwrap();
        tracker
            .record_buy("GOOG", 3.0, 100.0, 0.0, "buy-3")
            .unwrap();

        let all_lots = tracker.all_open_lots();
        assert_eq!(all_lots.len(), 3);
    }

    #[test]
    fn test_closed_lot_excluded() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Fifo);
        tracker
            .record_buy("AAPL", 10.0, 100.0, 0.0, "buy-1")
            .unwrap();
        tracker
            .record_sell("AAPL", 10.0, 150.0, 0.0, "sell-1")
            .unwrap();

        let lots = tracker.open_lots("AAPL");
        assert_eq!(lots.len(), 0);

        let all_lots = tracker.all_open_lots();
        assert_eq!(all_lots.len(), 0);
    }

    #[test]
    fn test_realized_for_symbol() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Fifo);
        tracker
            .record_buy("AAPL", 10.0, 100.0, 0.0, "buy-1")
            .unwrap();
        tracker
            .record_buy("TSLA", 5.0, 200.0, 0.0, "buy-2")
            .unwrap();
        tracker
            .record_sell("AAPL", 10.0, 150.0, 0.0, "sell-1")
            .unwrap();
        tracker
            .record_sell("TSLA", 5.0, 250.0, 0.0, "sell-2")
            .unwrap();

        let aapl_realized = tracker.realized_for_symbol("AAPL");
        assert_eq!(aapl_realized.len(), 1);
        assert_eq!(aapl_realized[0].symbol, "AAPL");
    }

    #[test]
    fn test_cost_basis_method_display() {
        assert_eq!(format!("{}", CostBasisMethod::Fifo), "FIFO");
        assert_eq!(format!("{}", CostBasisMethod::Lifo), "LIFO");
        assert_eq!(format!("{}", CostBasisMethod::HighCost), "Highest Cost");
        assert_eq!(format!("{}", CostBasisMethod::LowCost), "Lowest Cost");
        assert_eq!(format!("{}", CostBasisMethod::SpecificId), "Specific ID");
    }

    #[test]
    fn test_default_method() {
        let tracker = TaxLotTracker::default();
        assert_eq!(tracker.method(), CostBasisMethod::Fifo);
    }

    #[test]
    fn test_set_method() {
        let mut tracker = TaxLotTracker::default();
        tracker.set_method(CostBasisMethod::HighCost);
        assert_eq!(tracker.method(), CostBasisMethod::HighCost);
    }

    #[test]
    fn test_lot_is_long_term() {
        let mut lot = TaxLot::new("AAPL", 10.0, 150.0, 0.0, "order-1");
        assert!(!lot.is_long_term()); // Just created

        lot.purchase_date = Utc::now() - chrono::Duration::days(400);
        assert!(lot.is_long_term());
    }

    #[test]
    fn test_wash_sale_detection() {
        let mut tracker = TaxLotTracker::new(CostBasisMethod::Fifo);

        // Buy, sell at loss, buy again within 30 days
        tracker
            .record_buy("AAPL", 10.0, 200.0, 0.0, "buy-1")
            .unwrap();
        tracker
            .record_sell("AAPL", 10.0, 150.0, 0.0, "sell-1")
            .unwrap();
        tracker
            .record_buy("AAPL", 10.0, 155.0, 0.0, "buy-2")
            .unwrap(); // Within 30 days

        let report = tracker.tax_report(Utc::now().year());
        assert!(report.wash_sale_disallowed > 0.0);
    }
}
