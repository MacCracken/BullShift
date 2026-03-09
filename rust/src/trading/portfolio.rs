use crate::database::Database;
use crate::error::BullShiftError;
use serde::{Deserialize, Serialize};
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
    pub currency: Currency,
    #[serde(skip)]
    pub orders: Vec<uuid::Uuid>,
}

/// Supported currencies for portfolio positions and cash balances.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    #[default]
    USD,
    EUR,
    GBP,
    JPY,
    CAD,
    AUD,
    CHF,
    /// Crypto denominated (e.g., BTC, ETH). The symbol itself identifies the asset.
    USDT,
    USDC,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::USD => write!(f, "USD"),
            Currency::EUR => write!(f, "EUR"),
            Currency::GBP => write!(f, "GBP"),
            Currency::JPY => write!(f, "JPY"),
            Currency::CAD => write!(f, "CAD"),
            Currency::AUD => write!(f, "AUD"),
            Currency::CHF => write!(f, "CHF"),
            Currency::USDT => write!(f, "USDT"),
            Currency::USDC => write!(f, "USDC"),
        }
    }
}

impl Currency {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "USD" => Some(Currency::USD),
            "EUR" => Some(Currency::EUR),
            "GBP" => Some(Currency::GBP),
            "JPY" => Some(Currency::JPY),
            "CAD" => Some(Currency::CAD),
            "AUD" => Some(Currency::AUD),
            "CHF" => Some(Currency::CHF),
            "USDT" => Some(Currency::USDT),
            "USDC" => Some(Currency::USDC),
            _ => None,
        }
    }
}

/// Exchange rates relative to USD for portfolio value aggregation.
#[derive(Debug, Clone, Serialize)]
pub struct ExchangeRates {
    pub base: Currency,
    pub rates: HashMap<Currency, f64>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Default for ExchangeRates {
    fn default() -> Self {
        let mut rates = HashMap::new();
        rates.insert(Currency::USD, 1.0);
        rates.insert(Currency::EUR, 1.08);
        rates.insert(Currency::GBP, 1.27);
        rates.insert(Currency::JPY, 0.0067);
        rates.insert(Currency::CAD, 0.74);
        rates.insert(Currency::AUD, 0.65);
        rates.insert(Currency::CHF, 1.13);
        rates.insert(Currency::USDT, 1.0);
        rates.insert(Currency::USDC, 1.0);

        Self {
            base: Currency::USD,
            rates,
            updated_at: chrono::Utc::now(),
        }
    }
}

impl ExchangeRates {
    /// Convert an amount from one currency to another.
    /// Returns `None` if either currency is unknown or if `to_rate` is zero.
    pub fn convert(&self, amount: f64, from: Currency, to: Currency) -> Option<f64> {
        if from == to {
            return Some(amount);
        }
        let from_rate = self.rates.get(&from)?;
        let to_rate = self.rates.get(&to)?;
        if to_rate.abs() < f64::EPSILON {
            return None;
        }
        Some(amount * from_rate / to_rate)
    }

    /// Update a single exchange rate. Rate must be positive.
    pub fn set_rate(&mut self, currency: Currency, rate_to_usd: f64) {
        if rate_to_usd <= 0.0 || !rate_to_usd.is_finite() {
            return;
        }
        self.rates.insert(currency, rate_to_usd);
        self.updated_at = chrono::Utc::now();
    }
}

/// Per-currency cash balance summary.
#[derive(Debug, Clone, Serialize)]
pub struct CurrencyBalance {
    pub currency: Currency,
    pub amount: f64,
    pub usd_equivalent: f64,
}

pub struct Portfolio {
    pub id: Option<i64>,
    pub positions: HashMap<String, Position>,
    pub cash_balances: HashMap<Currency, f64>,
    pub base_currency: Currency,
    pub exchange_rates: ExchangeRates,
    pub total_value: f64,
    pub available_margin: f64,
    pub db: Option<Arc<Database>>,
}

impl Portfolio {
    pub fn new(initial_cash: f64) -> Self {
        Self::with_currency(initial_cash, Currency::USD)
    }

    pub fn with_currency(initial_cash: f64, currency: Currency) -> Self {
        let mut cash_balances = HashMap::new();
        cash_balances.insert(currency, initial_cash);
        let exchange_rates = ExchangeRates::default();

        let usd_value = exchange_rates
            .convert(initial_cash, currency, Currency::USD)
            .unwrap_or(initial_cash);

        Self {
            id: None,
            positions: HashMap::new(),
            cash_balances,
            base_currency: currency,
            exchange_rates,
            total_value: usd_value,
            available_margin: usd_value,
            db: None,
        }
    }

    /// Legacy accessor for backward compatibility — returns USD cash balance.
    pub fn cash_balance(&self) -> f64 {
        self.cash_in_currency(self.base_currency)
    }

    /// Get cash balance in a specific currency.
    pub fn cash_in_currency(&self, currency: Currency) -> f64 {
        *self.cash_balances.get(&currency).unwrap_or(&0.0)
    }

    /// Add cash in a specific currency. Amount must be positive.
    pub fn deposit(&mut self, amount: f64, currency: Currency) {
        if amount <= 0.0 || !amount.is_finite() {
            return;
        }
        *self.cash_balances.entry(currency).or_insert(0.0) += amount;
        self.calculate_total_value();
    }

    /// Withdraw cash in a specific currency. Returns error if insufficient funds or invalid amount.
    pub fn withdraw(&mut self, amount: f64, currency: Currency) -> Result<(), BullShiftError> {
        if amount <= 0.0 || !amount.is_finite() {
            return Err(BullShiftError::Portfolio(
                "Withdraw amount must be a positive finite number".to_string(),
            ));
        }
        let balance = self.cash_balances.get(&currency).copied().unwrap_or(0.0);
        if balance < amount {
            return Err(BullShiftError::Portfolio(format!(
                "Insufficient {} balance: have {}, need {}",
                currency, balance, amount
            )));
        }
        *self.cash_balances.entry(currency).or_insert(0.0) -= amount;
        self.calculate_total_value();
        Ok(())
    }

    /// Get all currency balances with USD equivalents.
    pub fn currency_balances(&self) -> Vec<CurrencyBalance> {
        self.cash_balances
            .iter()
            .map(|(&currency, &amount)| {
                let usd_equivalent = self
                    .exchange_rates
                    .convert(amount, currency, Currency::USD)
                    .unwrap_or(amount);
                CurrencyBalance {
                    currency,
                    amount,
                    usd_equivalent,
                }
            })
            .collect()
    }

    /// Total cash value across all currencies, converted to base currency.
    pub fn total_cash_value(&self) -> f64 {
        self.cash_balances
            .iter()
            .map(|(&currency, &amount)| {
                self.exchange_rates
                    .convert(amount, currency, self.base_currency)
                    .unwrap_or(amount)
            })
            .sum()
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
            self.cash_balances.insert(self.base_currency, cash);
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
                        currency: Currency::USD,
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

        let cash = self.cash_balance();

        if let Some(id) = self.id {
            db.update_portfolio(id, cash, self.total_value, self.available_margin)?;
        } else {
            let id = db.save_portfolio(cash, self.total_value, self.available_margin)?;
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
        if !current_price.is_finite() || current_price < 0.0 {
            return;
        }
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
        let cash_total = self.total_cash_value();

        let positions_value: f64 = self
            .positions
            .values()
            .map(|p| {
                let local_value = p.quantity * p.current_price;
                self.exchange_rates
                    .convert(local_value, p.currency, self.base_currency)
                    .unwrap_or(local_value)
            })
            .filter(|v| v.is_finite())
            .sum();

        self.total_value = if (cash_total + positions_value).is_finite() {
            cash_total + positions_value
        } else {
            cash_total
        };

        let margin_used: f64 = self
            .positions
            .values()
            .map(|p| {
                let local_margin = p.quantity * p.current_price * 0.5;
                self.exchange_rates
                    .convert(local_margin, p.currency, self.base_currency)
                    .unwrap_or(local_margin)
            })
            .filter(|v| v.is_finite())
            .sum();

        self.available_margin = self.total_value - margin_used;
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
            currency: Currency::USD,
            orders: Vec::new(),
        }
    }

    fn make_position_with_currency(
        symbol: &str,
        quantity: f64,
        entry_price: f64,
        currency: Currency,
    ) -> Position {
        Position {
            symbol: symbol.to_string(),
            quantity,
            entry_price,
            current_price: entry_price,
            unrealized_pnl: 0.0,
            realized_pnl: 0.0,
            currency,
            orders: Vec::new(),
        }
    }

    #[test]
    fn test_new_portfolio() {
        let portfolio = Portfolio::new(100_000.0);
        assert_eq!(portfolio.cash_balance(), 100_000.0);
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

    // Multi-currency tests

    #[test]
    fn test_portfolio_with_eur() {
        let portfolio = Portfolio::with_currency(10_000.0, Currency::EUR);
        assert_eq!(portfolio.base_currency, Currency::EUR);
        assert_eq!(portfolio.cash_in_currency(Currency::EUR), 10_000.0);
        assert_eq!(portfolio.cash_in_currency(Currency::USD), 0.0);
    }

    #[test]
    fn test_deposit_withdraw() {
        let mut portfolio = Portfolio::new(10_000.0);
        portfolio.deposit(5_000.0, Currency::EUR);

        assert_eq!(portfolio.cash_in_currency(Currency::USD), 10_000.0);
        assert_eq!(portfolio.cash_in_currency(Currency::EUR), 5_000.0);

        portfolio.withdraw(2_000.0, Currency::EUR).unwrap();
        assert_eq!(portfolio.cash_in_currency(Currency::EUR), 3_000.0);
    }

    #[test]
    fn test_withdraw_insufficient() {
        let mut portfolio = Portfolio::new(1_000.0);
        let result = portfolio.withdraw(5_000.0, Currency::USD);
        assert!(result.is_err());
    }

    #[test]
    fn test_currency_balances() {
        let mut portfolio = Portfolio::new(10_000.0);
        portfolio.deposit(5_000.0, Currency::EUR);
        portfolio.deposit(1_000_000.0, Currency::JPY);

        let balances = portfolio.currency_balances();
        assert_eq!(balances.len(), 3);

        for b in &balances {
            assert!(b.usd_equivalent > 0.0);
        }
    }

    #[test]
    fn test_exchange_rate_convert() {
        let rates = ExchangeRates::default();
        let result = rates.convert(100.0, Currency::USD, Currency::USD);
        assert_eq!(result, Some(100.0));

        let eur_to_usd = rates.convert(100.0, Currency::EUR, Currency::USD).unwrap();
        assert!(eur_to_usd > 100.0); // EUR is worth more than USD
    }

    #[test]
    fn test_exchange_rate_set_rate() {
        let mut rates = ExchangeRates::default();
        rates.set_rate(Currency::EUR, 1.10);
        let result = rates.convert(100.0, Currency::EUR, Currency::USD).unwrap();
        assert!((result - 110.0).abs() < 0.001);
    }

    #[test]
    fn test_multi_currency_positions() {
        let mut portfolio = Portfolio::new(100_000.0);
        let usd_pos = make_position("AAPL", 10.0, 150.0);
        let eur_pos = make_position_with_currency("SAP.DE", 5.0, 200.0, Currency::EUR);

        portfolio.add_position(usd_pos);
        portfolio.add_position(eur_pos);

        // Total value should include converted EUR position
        assert!(portfolio.total_value > 100_000.0);
        assert_eq!(portfolio.positions.len(), 2);
    }

    #[test]
    fn test_total_cash_multi_currency() {
        let mut portfolio = Portfolio::new(10_000.0);
        portfolio.deposit(10_000.0, Currency::EUR);

        let total_cash = portfolio.total_cash_value();
        // Should be USD 10k + EUR 10k converted to USD (>10k)
        assert!(total_cash > 10_000.0);
    }

    #[test]
    fn test_currency_from_str() {
        assert_eq!(Currency::parse("USD"), Some(Currency::USD));
        assert_eq!(Currency::parse("eur"), Some(Currency::EUR));
        assert_eq!(Currency::parse("GBP"), Some(Currency::GBP));
        assert_eq!(Currency::parse("jpy"), Some(Currency::JPY));
        assert_eq!(Currency::parse("USDT"), Some(Currency::USDT));
        assert_eq!(Currency::parse("USDC"), Some(Currency::USDC));
        assert_eq!(Currency::parse("XYZ"), None);
    }

    #[test]
    fn test_currency_display() {
        assert_eq!(format!("{}", Currency::USD), "USD");
        assert_eq!(format!("{}", Currency::EUR), "EUR");
        assert_eq!(format!("{}", Currency::JPY), "JPY");
    }

    #[test]
    fn test_default_currency() {
        let currency: Currency = Default::default();
        assert_eq!(currency, Currency::USD);
    }

    #[test]
    fn test_deposit_and_withdraw() {
        let mut portfolio = Portfolio::new(10_000.0);
        portfolio.deposit(5_000.0, Currency::USD);
        assert!((portfolio.cash_balance() - 15_000.0).abs() < f64::EPSILON);

        portfolio.withdraw(3_000.0, Currency::USD).unwrap();
        assert!((portfolio.cash_balance() - 12_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let mut portfolio = Portfolio::new(1_000.0);
        assert!(portfolio.withdraw(5_000.0, Currency::USD).is_err());
    }

    #[test]
    fn test_deposit_negative_ignored() {
        let mut portfolio = Portfolio::new(10_000.0);
        portfolio.deposit(-500.0, Currency::USD);
        assert!((portfolio.cash_balance() - 10_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_deposit_nan_ignored() {
        let mut portfolio = Portfolio::new(10_000.0);
        portfolio.deposit(f64::NAN, Currency::USD);
        assert!((portfolio.cash_balance() - 10_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_deposit_infinity_ignored() {
        let mut portfolio = Portfolio::new(10_000.0);
        portfolio.deposit(f64::INFINITY, Currency::USD);
        assert!((portfolio.cash_balance() - 10_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_add_position_and_value() {
        let mut portfolio = Portfolio::new(100_000.0);
        portfolio.add_position(Position {
            symbol: "AAPL".to_string(),
            quantity: 10.0,
            entry_price: 150.0,
            current_price: 160.0,
            unrealized_pnl: 100.0,
            realized_pnl: 0.0,
            currency: Currency::USD,
            orders: Vec::new(),
        });
        // total_value = cash + position value = 100000 + 10*160 = 101600
        assert!(portfolio.total_value > 100_000.0);
    }

    #[test]
    fn test_update_position_valid() {
        let mut portfolio = Portfolio::new(100_000.0);
        portfolio.add_position(Position {
            symbol: "AAPL".to_string(),
            quantity: 10.0,
            entry_price: 150.0,
            current_price: 150.0,
            unrealized_pnl: 0.0,
            realized_pnl: 0.0,
            currency: Currency::USD,
            orders: Vec::new(),
        });
        portfolio.update_position("AAPL", 170.0);
        let pos = portfolio.positions.get("AAPL").unwrap();
        assert_eq!(pos.current_price, 170.0);
        assert!((pos.unrealized_pnl - 200.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_update_position_negative_price_ignored() {
        let mut portfolio = Portfolio::new(100_000.0);
        portfolio.add_position(Position {
            symbol: "AAPL".to_string(),
            quantity: 10.0,
            entry_price: 150.0,
            current_price: 150.0,
            unrealized_pnl: 0.0,
            realized_pnl: 0.0,
            currency: Currency::USD,
            orders: Vec::new(),
        });
        portfolio.update_position("AAPL", -5.0);
        assert_eq!(portfolio.positions.get("AAPL").unwrap().current_price, 150.0);
    }

    #[test]
    fn test_update_position_nan_ignored() {
        let mut portfolio = Portfolio::new(100_000.0);
        portfolio.add_position(Position {
            symbol: "AAPL".to_string(),
            quantity: 10.0,
            entry_price: 150.0,
            current_price: 150.0,
            unrealized_pnl: 0.0,
            realized_pnl: 0.0,
            currency: Currency::USD,
            orders: Vec::new(),
        });
        portfolio.update_position("AAPL", f64::NAN);
        assert_eq!(portfolio.positions.get("AAPL").unwrap().current_price, 150.0);
    }

    #[test]
    fn test_update_position_nonexistent() {
        let mut portfolio = Portfolio::new(100_000.0);
        portfolio.update_position("NONEXISTENT", 100.0);
        // Should not panic, nothing happens
        assert!(portfolio.positions.is_empty());
    }

    #[test]
    fn test_exchange_rate_convert_same_currency() {
        let rates = ExchangeRates::default();
        let result = rates.convert(1000.0, Currency::USD, Currency::USD);
        assert_eq!(result, Some(1000.0));
    }

    #[test]
    fn test_exchange_rate_convert_cross() {
        let rates = ExchangeRates::default();
        let result = rates.convert(1000.0, Currency::EUR, Currency::USD);
        assert!(result.is_some());
        // EUR → USD: 1000 * (eur_rate / usd_rate) — value depends on defaults
        assert!(result.unwrap() > 0.0);
    }
}
