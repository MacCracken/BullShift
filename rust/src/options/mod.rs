use crate::error::BullShiftError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Option type: Call or Put.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptionType {
    Call,
    Put,
}

impl std::fmt::Display for OptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Call => write!(f, "CALL"),
            Self::Put => write!(f, "PUT"),
        }
    }
}

/// Option exercise style.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExerciseStyle {
    American,
    European,
}

/// An options contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsContract {
    pub symbol: String,
    pub underlying: String,
    pub option_type: OptionType,
    pub strike: f64,
    pub expiration: DateTime<Utc>,
    pub exercise_style: ExerciseStyle,
    pub multiplier: f64,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub volume: u64,
    pub open_interest: u64,
    pub implied_volatility: f64,
    pub greeks: Greeks,
}

/// The Greeks — sensitivity measures for an option.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
}

/// A strike in an options chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainStrike {
    pub strike: f64,
    pub call: Option<OptionsContract>,
    pub put: Option<OptionsContract>,
}

/// An options chain for a given expiration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsChain {
    pub underlying: String,
    pub underlying_price: f64,
    pub expiration: DateTime<Utc>,
    pub strikes: Vec<ChainStrike>,
}

/// An options position (long or short).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsPosition {
    pub id: Uuid,
    pub contract: OptionsContract,
    pub quantity: i64,
    pub avg_cost: f64,
    pub current_value: f64,
    pub unrealized_pnl: f64,
    pub opened_at: DateTime<Utc>,
}

/// Common options strategies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionsStrategy {
    /// Long a single call or put.
    LongOption {
        contract: OptionsContract,
        quantity: u64,
    },
    /// Short a single call or put (covered or naked).
    ShortOption {
        contract: OptionsContract,
        quantity: u64,
        covered: bool,
    },
    /// Buy a call and put at the same strike (long straddle).
    Straddle {
        underlying: String,
        strike: f64,
        expiration: DateTime<Utc>,
        quantity: u64,
    },
    /// Buy a call and put at different strikes (long strangle).
    Strangle {
        underlying: String,
        call_strike: f64,
        put_strike: f64,
        expiration: DateTime<Utc>,
        quantity: u64,
    },
    /// Bull call spread: buy lower strike call, sell higher strike call.
    BullCallSpread {
        underlying: String,
        long_strike: f64,
        short_strike: f64,
        expiration: DateTime<Utc>,
        quantity: u64,
    },
    /// Bear put spread: buy higher strike put, sell lower strike put.
    BearPutSpread {
        underlying: String,
        long_strike: f64,
        short_strike: f64,
        expiration: DateTime<Utc>,
        quantity: u64,
    },
    /// Iron condor: OTM bull put spread + OTM bear call spread.
    IronCondor {
        underlying: String,
        put_long: f64,
        put_short: f64,
        call_short: f64,
        call_long: f64,
        expiration: DateTime<Utc>,
        quantity: u64,
    },
    /// Covered call: long stock + short call.
    CoveredCall {
        underlying: String,
        strike: f64,
        expiration: DateTime<Utc>,
        shares: u64,
    },
    /// Protective put: long stock + long put.
    ProtectivePut {
        underlying: String,
        strike: f64,
        expiration: DateTime<Utc>,
        shares: u64,
    },
}

impl std::fmt::Display for OptionsStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LongOption { contract, .. } => {
                write!(
                    f,
                    "Long {} {} {:.0}",
                    contract.underlying, contract.option_type, contract.strike
                )
            }
            Self::ShortOption {
                contract, covered, ..
            } => {
                let prefix = if *covered {
                    "Covered Short"
                } else {
                    "Naked Short"
                };
                write!(
                    f,
                    "{} {} {} {:.0}",
                    prefix, contract.underlying, contract.option_type, contract.strike
                )
            }
            Self::Straddle {
                underlying, strike, ..
            } => write!(f, "Straddle {} {:.0}", underlying, strike),
            Self::Strangle {
                underlying,
                call_strike,
                put_strike,
                ..
            } => {
                write!(
                    f,
                    "Strangle {} {:.0}/{:.0}",
                    underlying, put_strike, call_strike
                )
            }
            Self::BullCallSpread {
                underlying,
                long_strike,
                short_strike,
                ..
            } => {
                write!(
                    f,
                    "Bull Call {} {:.0}/{:.0}",
                    underlying, long_strike, short_strike
                )
            }
            Self::BearPutSpread {
                underlying,
                long_strike,
                short_strike,
                ..
            } => {
                write!(
                    f,
                    "Bear Put {} {:.0}/{:.0}",
                    underlying, long_strike, short_strike
                )
            }
            Self::IronCondor {
                underlying,
                put_short,
                call_short,
                ..
            } => {
                write!(
                    f,
                    "Iron Condor {} {:.0}/{:.0}",
                    underlying, put_short, call_short
                )
            }
            Self::CoveredCall {
                underlying, strike, ..
            } => write!(f, "Covered Call {} {:.0}", underlying, strike),
            Self::ProtectivePut {
                underlying, strike, ..
            } => write!(f, "Protective Put {} {:.0}", underlying, strike),
        }
    }
}

/// Manages options chains, positions, and strategy analysis.
pub struct OptionsManager {
    chains: HashMap<String, Vec<OptionsChain>>,
    positions: HashMap<Uuid, OptionsPosition>,
    risk_free_rate: f64,
}

impl Default for OptionsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OptionsManager {
    pub fn new() -> Self {
        Self {
            chains: HashMap::new(),
            positions: HashMap::new(),
            risk_free_rate: 0.05,
        }
    }

    /// Set the risk-free rate for pricing (default 5%).
    pub fn set_risk_free_rate(&mut self, rate: f64) {
        self.risk_free_rate = rate;
    }

    /// Store an options chain.
    pub fn store_chain(&mut self, chain: OptionsChain) {
        self.chains
            .entry(chain.underlying.clone())
            .or_default()
            .push(chain);
    }

    /// Get chains for an underlying.
    pub fn get_chains(&self, underlying: &str) -> Option<&Vec<OptionsChain>> {
        self.chains.get(underlying)
    }

    /// Add an options position.
    pub fn open_position(&mut self, position: OptionsPosition) -> Uuid {
        let id = position.id;
        self.positions.insert(id, position);
        id
    }

    /// Close an options position.
    pub fn close_position(&mut self, id: &Uuid) -> Result<OptionsPosition, BullShiftError> {
        self.positions
            .remove(id)
            .ok_or_else(|| BullShiftError::Trading(format!("Options position {} not found", id)))
    }

    /// List all open options positions.
    pub fn list_positions(&self) -> Vec<&OptionsPosition> {
        self.positions.values().collect()
    }

    /// Get portfolio Greeks (sum across all positions).
    pub fn portfolio_greeks(&self) -> Greeks {
        let mut total = Greeks::default();
        for pos in self.positions.values() {
            let sign = if pos.quantity >= 0 { 1.0 } else { -1.0 };
            let qty = (pos.quantity as f64).abs();
            total.delta += pos.contract.greeks.delta * qty * sign * pos.contract.multiplier;
            total.gamma += pos.contract.greeks.gamma * qty * sign * pos.contract.multiplier;
            total.theta += pos.contract.greeks.theta * qty * sign * pos.contract.multiplier;
            total.vega += pos.contract.greeks.vega * qty * sign * pos.contract.multiplier;
            total.rho += pos.contract.greeks.rho * qty * sign * pos.contract.multiplier;
        }
        total
    }

    /// Calculate Black-Scholes price for a European option.
    pub fn black_scholes(
        &self,
        option_type: &OptionType,
        spot: f64,
        strike: f64,
        time_to_expiry: f64,
        volatility: f64,
    ) -> f64 {
        if time_to_expiry <= 0.0 {
            // At expiration — intrinsic value
            return match option_type {
                OptionType::Call => (spot - strike).max(0.0),
                OptionType::Put => (strike - spot).max(0.0),
            };
        }

        if volatility <= 0.0 || !volatility.is_finite() {
            // Zero/invalid volatility — return intrinsic value
            return match option_type {
                OptionType::Call => (spot - strike).max(0.0),
                OptionType::Put => (strike - spot).max(0.0),
            };
        }

        let d1 = ((spot / strike).ln()
            + (self.risk_free_rate + volatility.powi(2) / 2.0) * time_to_expiry)
            / (volatility * time_to_expiry.sqrt());
        let d2 = d1 - volatility * time_to_expiry.sqrt();

        match option_type {
            OptionType::Call => {
                spot * norm_cdf(d1)
                    - strike * (-self.risk_free_rate * time_to_expiry).exp() * norm_cdf(d2)
            }
            OptionType::Put => {
                strike * (-self.risk_free_rate * time_to_expiry).exp() * norm_cdf(-d2)
                    - spot * norm_cdf(-d1)
            }
        }
    }

    /// Calculate Greeks for a given option using Black-Scholes.
    pub fn calculate_greeks(
        &self,
        option_type: &OptionType,
        spot: f64,
        strike: f64,
        time_to_expiry: f64,
        volatility: f64,
    ) -> Greeks {
        if time_to_expiry <= 0.0 {
            return Greeks::default();
        }

        let d1 = ((spot / strike).ln()
            + (self.risk_free_rate + volatility.powi(2) / 2.0) * time_to_expiry)
            / (volatility * time_to_expiry.sqrt());
        let d2 = d1 - volatility * time_to_expiry.sqrt();

        let n_d1 = norm_pdf(d1);

        let delta = match option_type {
            OptionType::Call => norm_cdf(d1),
            OptionType::Put => norm_cdf(d1) - 1.0,
        };

        let gamma = n_d1 / (spot * volatility * time_to_expiry.sqrt());

        let theta = match option_type {
            OptionType::Call => {
                -(spot * n_d1 * volatility) / (2.0 * time_to_expiry.sqrt())
                    - self.risk_free_rate
                        * strike
                        * (-self.risk_free_rate * time_to_expiry).exp()
                        * norm_cdf(d2)
            }
            OptionType::Put => {
                -(spot * n_d1 * volatility) / (2.0 * time_to_expiry.sqrt())
                    + self.risk_free_rate
                        * strike
                        * (-self.risk_free_rate * time_to_expiry).exp()
                        * norm_cdf(-d2)
            }
        };
        // Convert theta to per-day
        let theta = theta / 365.0;

        let vega = spot * n_d1 * time_to_expiry.sqrt() / 100.0;

        let rho = match option_type {
            OptionType::Call => {
                strike
                    * time_to_expiry
                    * (-self.risk_free_rate * time_to_expiry).exp()
                    * norm_cdf(d2)
                    / 100.0
            }
            OptionType::Put => {
                -strike
                    * time_to_expiry
                    * (-self.risk_free_rate * time_to_expiry).exp()
                    * norm_cdf(-d2)
                    / 100.0
            }
        };

        Greeks {
            delta,
            gamma,
            theta,
            vega,
            rho,
        }
    }

    /// Calculate max profit and max loss for a strategy.
    pub fn strategy_risk_profile(&self, strategy: &OptionsStrategy) -> (f64, f64) {
        match strategy {
            OptionsStrategy::LongOption { contract, quantity } => {
                let cost = contract.ask * contract.multiplier * *quantity as f64;
                let max_loss = cost;
                let max_profit = match contract.option_type {
                    OptionType::Call => f64::INFINITY,
                    OptionType::Put => {
                        (contract.strike * contract.multiplier * *quantity as f64) - cost
                    }
                };
                (max_profit, max_loss)
            }
            OptionsStrategy::CoveredCall { strike, shares, .. } => {
                let max_profit = *strike * *shares as f64; // Simplified
                let max_loss = f64::INFINITY; // Stock can go to 0
                (max_profit, max_loss)
            }
            OptionsStrategy::IronCondor {
                put_long,
                put_short,
                call_short,
                call_long,
                quantity,
                ..
            } => {
                let width = (call_long - call_short).max(put_short - put_long);
                let credit = 2.0; // Placeholder — would use actual premiums
                let max_profit = credit * 100.0 * *quantity as f64;
                let max_loss = (width - credit) * 100.0 * *quantity as f64;
                (max_profit, max_loss)
            }
            _ => (0.0, 0.0), // Other strategies need premium data
        }
    }
}

// --- Math helpers ---

/// Standard normal CDF approximation (Abramowitz & Stegun).
fn norm_cdf(x: f64) -> f64 {
    if x < -10.0 {
        return 0.0;
    }
    if x > 10.0 {
        return 1.0;
    }

    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x_abs = x.abs() / std::f64::consts::SQRT_2;
    let t = 1.0 / (1.0 + p * x_abs);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x_abs * x_abs).exp();

    0.5 * (1.0 + sign * y)
}

/// Standard normal PDF.
fn norm_pdf(x: f64) -> f64 {
    (-x * x / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black_scholes_call() {
        let mgr = OptionsManager::new();
        // AAPL at $150, strike $155, 30 days, 25% vol
        let price = mgr.black_scholes(&OptionType::Call, 150.0, 155.0, 30.0 / 365.0, 0.25);
        // Should be a positive value less than spot
        assert!(price > 0.0);
        assert!(price < 150.0);
    }

    #[test]
    fn test_black_scholes_put() {
        let mgr = OptionsManager::new();
        let price = mgr.black_scholes(&OptionType::Put, 150.0, 155.0, 30.0 / 365.0, 0.25);
        assert!(price > 0.0);
        // ITM put should be worth at least intrinsic
        assert!(price >= 4.0); // At least close to intrinsic (5.0)
    }

    #[test]
    fn test_black_scholes_at_expiry() {
        let mgr = OptionsManager::new();
        assert!(
            (mgr.black_scholes(&OptionType::Call, 150.0, 140.0, 0.0, 0.25) - 10.0).abs() < 0.01
        );
        assert!((mgr.black_scholes(&OptionType::Call, 130.0, 140.0, 0.0, 0.25)).abs() < 0.01);
        assert!((mgr.black_scholes(&OptionType::Put, 130.0, 140.0, 0.0, 0.25) - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_put_call_parity() {
        let mgr = OptionsManager::new();
        let s = 100.0;
        let k = 100.0;
        let t = 0.5;
        let v = 0.3;
        let call = mgr.black_scholes(&OptionType::Call, s, k, t, v);
        let put = mgr.black_scholes(&OptionType::Put, s, k, t, v);
        let parity = call - put - s + k * (-0.05 * t).exp();
        assert!(parity.abs() < 0.01, "Put-call parity violated: {}", parity);
    }

    #[test]
    fn test_greeks_call() {
        let mgr = OptionsManager::new();
        let greeks = mgr.calculate_greeks(&OptionType::Call, 150.0, 150.0, 30.0 / 365.0, 0.25);

        // ATM call delta should be ~0.5
        assert!((greeks.delta - 0.5).abs() < 0.1);
        // Gamma should be positive
        assert!(greeks.gamma > 0.0);
        // Theta should be negative (time decay)
        assert!(greeks.theta < 0.0);
        // Vega should be positive
        assert!(greeks.vega > 0.0);
    }

    #[test]
    fn test_greeks_put() {
        let mgr = OptionsManager::new();
        let greeks = mgr.calculate_greeks(&OptionType::Put, 150.0, 150.0, 30.0 / 365.0, 0.25);

        // ATM put delta should be ~-0.5
        assert!((greeks.delta + 0.5).abs() < 0.1);
        assert!(greeks.gamma > 0.0);
        assert!(greeks.theta < 0.0);
        assert!(greeks.vega > 0.0);
    }

    #[test]
    fn test_portfolio_greeks() {
        let mut mgr = OptionsManager::new();

        let contract = OptionsContract {
            symbol: "AAPL240315C150".to_string(),
            underlying: "AAPL".to_string(),
            option_type: OptionType::Call,
            strike: 150.0,
            expiration: Utc::now(),
            exercise_style: ExerciseStyle::American,
            multiplier: 100.0,
            bid: 5.0,
            ask: 5.50,
            last: 5.25,
            volume: 1000,
            open_interest: 5000,
            implied_volatility: 0.25,
            greeks: Greeks {
                delta: 0.55,
                gamma: 0.03,
                theta: -0.05,
                vega: 0.15,
                rho: 0.02,
            },
        };

        mgr.open_position(OptionsPosition {
            id: Uuid::new_v4(),
            contract,
            quantity: 2,
            avg_cost: 5.25,
            current_value: 5.50,
            unrealized_pnl: 50.0,
            opened_at: Utc::now(),
        });

        let greeks = mgr.portfolio_greeks();
        assert!((greeks.delta - 0.55 * 2.0 * 100.0).abs() < 0.01);
        assert!((greeks.gamma - 0.03 * 2.0 * 100.0).abs() < 0.01);
    }

    #[test]
    fn test_option_type_display() {
        assert_eq!(OptionType::Call.to_string(), "CALL");
        assert_eq!(OptionType::Put.to_string(), "PUT");
    }

    #[test]
    fn test_strategy_display() {
        let strat = OptionsStrategy::IronCondor {
            underlying: "SPY".to_string(),
            put_long: 400.0,
            put_short: 410.0,
            call_short: 440.0,
            call_long: 450.0,
            expiration: Utc::now(),
            quantity: 1,
        };
        assert_eq!(strat.to_string(), "Iron Condor SPY 410/440");
    }

    #[test]
    fn test_norm_cdf() {
        assert!((norm_cdf(0.0) - 0.5).abs() < 0.001);
        assert!(norm_cdf(-10.0) < 0.001);
        assert!(norm_cdf(10.0) > 0.999);
    }

    #[test]
    fn test_store_and_get_chain() {
        let mut mgr = OptionsManager::new();
        let chain = OptionsChain {
            underlying: "AAPL".to_string(),
            underlying_price: 150.0,
            expiration: Utc::now(),
            strikes: vec![],
        };
        mgr.store_chain(chain);
        assert!(mgr.get_chains("AAPL").is_some());
        assert!(mgr.get_chains("TSLA").is_none());
    }

    #[test]
    fn test_open_close_position() {
        let mut mgr = OptionsManager::new();
        let contract = OptionsContract {
            symbol: "AAPL240315C150".to_string(),
            underlying: "AAPL".to_string(),
            option_type: OptionType::Call,
            strike: 150.0,
            expiration: Utc::now(),
            exercise_style: ExerciseStyle::American,
            multiplier: 100.0,
            bid: 5.0,
            ask: 5.50,
            last: 5.25,
            volume: 0,
            open_interest: 0,
            implied_volatility: 0.25,
            greeks: Greeks::default(),
        };

        let id = mgr.open_position(OptionsPosition {
            id: Uuid::new_v4(),
            contract,
            quantity: 1,
            avg_cost: 5.25,
            current_value: 5.50,
            unrealized_pnl: 25.0,
            opened_at: Utc::now(),
        });

        assert_eq!(mgr.list_positions().len(), 1);
        mgr.close_position(&id).unwrap();
        assert_eq!(mgr.list_positions().len(), 0);
    }

    #[test]
    fn test_deep_itm_call() {
        let mgr = OptionsManager::new();
        // Deep ITM: spot=200, strike=100, 90 days, 30% vol
        let spot = 200.0;
        let strike = 100.0;
        let t = 90.0 / 365.0;
        let vol = 0.30;
        let price = mgr.black_scholes(&OptionType::Call, spot, strike, t, vol);
        let intrinsic = spot - strike; // 100.0

        // Deep ITM call should be very close to intrinsic + time value
        // At minimum it must be >= intrinsic (no arbitrage)
        assert!(
            price >= intrinsic * 0.99,
            "Deep ITM call price {} should be near intrinsic {}",
            price,
            intrinsic
        );
        // Should not exceed spot price
        assert!(price <= spot);
    }

    #[test]
    fn test_deep_otm_put() {
        let mgr = OptionsManager::new();
        // Deep OTM put: spot=200, strike=50, 30 days, 25% vol
        let price = mgr.black_scholes(&OptionType::Put, 200.0, 50.0, 30.0 / 365.0, 0.25);

        // Far OTM put should be nearly worthless
        assert!(
            price < 0.01,
            "Deep OTM put should be near zero, got {}",
            price
        );
        assert!(price >= 0.0, "Option price should never be negative");
    }

    #[test]
    fn test_greeks_sum_properties() {
        let mgr = OptionsManager::new();
        let spot = 100.0;
        let strike = 100.0;
        let t = 0.5;
        let vol = 0.30;

        let call_greeks = mgr.calculate_greeks(&OptionType::Call, spot, strike, t, vol);
        let put_greeks = mgr.calculate_greeks(&OptionType::Put, spot, strike, t, vol);

        // Put-call parity for delta: call_delta - put_delta = 1
        // (since put delta is negative, call_delta + |put_delta| = 1)
        let delta_sum = call_greeks.delta - put_greeks.delta;
        assert!(
            (delta_sum - 1.0).abs() < 0.02,
            "Call delta ({}) - Put delta ({}) = {} should be ~1.0",
            call_greeks.delta,
            put_greeks.delta,
            delta_sum
        );

        // Gamma should be the same for call and put at the same strike
        assert!(
            (call_greeks.gamma - put_greeks.gamma).abs() < 0.001,
            "Call gamma ({}) and put gamma ({}) should be equal",
            call_greeks.gamma,
            put_greeks.gamma
        );

        // Vega should be the same for call and put at the same strike
        assert!(
            (call_greeks.vega - put_greeks.vega).abs() < 0.001,
            "Call vega ({}) and put vega ({}) should be equal",
            call_greeks.vega,
            put_greeks.vega
        );
    }

    #[test]
    fn test_theta_negative() {
        let mgr = OptionsManager::new();
        // Test across various moneyness levels
        for &(spot, strike) in &[(100.0, 100.0), (110.0, 100.0), (90.0, 100.0)] {
            let call_greeks = mgr.calculate_greeks(&OptionType::Call, spot, strike, 0.25, 0.30);
            let put_greeks = mgr.calculate_greeks(&OptionType::Put, spot, strike, 0.25, 0.30);

            assert!(
                call_greeks.theta < 0.0,
                "Call theta should be negative for spot={}, strike={}, got {}",
                spot,
                strike,
                call_greeks.theta
            );
            assert!(
                put_greeks.theta < 0.0,
                "Put theta should be negative for spot={}, strike={}, got {}",
                spot,
                strike,
                put_greeks.theta
            );
        }
    }

    #[test]
    fn test_vega_positive() {
        let mgr = OptionsManager::new();
        // Test across ATM, ITM, OTM
        for &(spot, strike) in &[(100.0, 100.0), (120.0, 100.0), (80.0, 100.0)] {
            let call_greeks = mgr.calculate_greeks(&OptionType::Call, spot, strike, 0.5, 0.25);
            let put_greeks = mgr.calculate_greeks(&OptionType::Put, spot, strike, 0.5, 0.25);

            assert!(
                call_greeks.vega > 0.0,
                "Call vega should be positive for spot={}, strike={}, got {}",
                spot,
                strike,
                call_greeks.vega
            );
            assert!(
                put_greeks.vega > 0.0,
                "Put vega should be positive for spot={}, strike={}, got {}",
                spot,
                strike,
                put_greeks.vega
            );
        }

        // ATM vega should be highest
        let atm_vega = mgr
            .calculate_greeks(&OptionType::Call, 100.0, 100.0, 0.5, 0.25)
            .vega;
        let otm_vega = mgr
            .calculate_greeks(&OptionType::Call, 80.0, 100.0, 0.5, 0.25)
            .vega;
        assert!(
            atm_vega > otm_vega,
            "ATM vega ({}) should be greater than OTM vega ({})",
            atm_vega,
            otm_vega
        );
    }

    #[test]
    fn test_black_scholes_zero_volatility() {
        let mgr = OptionsManager::new();
        // Zero vol should return intrinsic value
        let call = mgr.black_scholes(&OptionType::Call, 150.0, 140.0, 0.5, 0.0);
        assert!((call - 10.0).abs() < 0.01, "Zero vol ITM call should be ~intrinsic, got {}", call);

        let otm_call = mgr.black_scholes(&OptionType::Call, 130.0, 140.0, 0.5, 0.0);
        assert!(otm_call.abs() < 0.01, "Zero vol OTM call should be ~0, got {}", otm_call);

        let put = mgr.black_scholes(&OptionType::Put, 130.0, 140.0, 0.5, 0.0);
        assert!((put - 10.0).abs() < 0.01, "Zero vol ITM put should be ~intrinsic, got {}", put);
    }

    #[test]
    fn test_black_scholes_negative_volatility() {
        let mgr = OptionsManager::new();
        let price = mgr.black_scholes(&OptionType::Call, 150.0, 140.0, 0.5, -0.25);
        assert!((price - 10.0).abs() < 0.01, "Negative vol should return intrinsic");
    }

    #[test]
    fn test_portfolio_greeks_short_position() {
        let mut mgr = OptionsManager::new();
        let contract = OptionsContract {
            symbol: "SPY250C450".to_string(),
            underlying: "SPY".to_string(),
            option_type: OptionType::Call,
            strike: 450.0,
            expiration: Utc::now(),
            exercise_style: ExerciseStyle::European,
            multiplier: 100.0,
            bid: 10.0,
            ask: 10.50,
            last: 10.25,
            volume: 500,
            open_interest: 2000,
            implied_volatility: 0.20,
            greeks: Greeks {
                delta: 0.50,
                gamma: 0.02,
                theta: -0.04,
                vega: 0.12,
                rho: 0.01,
            },
        };

        mgr.open_position(OptionsPosition {
            id: Uuid::new_v4(),
            contract,
            quantity: -3, // short position
            avg_cost: 10.25,
            current_value: 10.50,
            unrealized_pnl: -75.0,
            opened_at: Utc::now(),
        });

        let greeks = mgr.portfolio_greeks();
        // Short 3 contracts: delta = 0.50 * 3 * 100 * -1 = -150
        assert!((greeks.delta - (-150.0)).abs() < 0.01);
    }

    #[test]
    fn test_greeks_default() {
        let g = Greeks::default();
        assert_eq!(g.delta, 0.0);
        assert_eq!(g.gamma, 0.0);
        assert_eq!(g.theta, 0.0);
        assert_eq!(g.vega, 0.0);
        assert_eq!(g.rho, 0.0);
    }

    #[test]
    fn test_exercise_style_equality() {
        assert_eq!(ExerciseStyle::American, ExerciseStyle::American);
        assert_ne!(ExerciseStyle::American, ExerciseStyle::European);
    }

    #[test]
    fn test_close_nonexistent_position() {
        let mut mgr = OptionsManager::new();
        assert!(mgr.close_position(&Uuid::new_v4()).is_err());
    }

    #[test]
    fn test_empty_portfolio_greeks() {
        let mgr = OptionsManager::new();
        let greeks = mgr.portfolio_greeks();
        assert_eq!(greeks.delta, 0.0);
        assert_eq!(greeks.gamma, 0.0);
    }
}
