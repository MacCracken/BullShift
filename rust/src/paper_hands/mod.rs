use crate::error::BullShiftError;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperTrade {
    pub id: Uuid,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: f64,
    pub entry_price: f64,
    pub exit_price: Option<f64>,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub status: TradeStatus,
    pub entry_time: DateTime<Utc>,
    pub exit_time: Option<DateTime<Utc>>,
    pub pnl: Option<f64>,
    pub pnl_percentage: Option<f64>,
    pub fees: f64,
    pub strategy: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
    TrailingStop,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TradeStatus {
    Pending,
    Open,
    Closed,
    Cancelled,
    PartiallyFilled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperPortfolio {
    pub id: Uuid,
    pub name: String,
    pub initial_balance: f64,
    pub current_balance: f64,
    pub allocated_balance: f64,
    pub available_balance: f64,
    pub positions: HashMap<String, PaperPosition>,
    pub trades: Vec<PaperTrade>,
    pub performance_metrics: PerformanceMetrics,
    pub risk_metrics: RiskMetrics,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperPosition {
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub unrealized_pnl: f64,
    pub unrealized_pnl_percentage: f64,
    pub realized_pnl: f64,
    pub total_fees: f64,
    pub trades_count: u32,
    pub average_entry_price: f64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub total_return: f64,
    pub total_return_percentage: f64,
    pub annualized_return: f64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub max_drawdown: f64,
    pub max_drawdown_percentage: f64,
    pub calmar_ratio: f64,
    pub total_trades: u32,
    pub winning_trades: u32,
    pub losing_trades: u32,
    pub average_win: f64,
    pub average_loss: f64,
    pub largest_win: f64,
    pub largest_loss: f64,
    pub average_trade_duration: Duration,
    pub best_trade_duration: Duration,
    pub worst_trade_duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMetrics {
    pub var_95: f64,  // Value at Risk 95%
    pub var_99: f64,  // Value at Risk 99%
    pub cvar_95: f64, // Conditional Value at Risk 95%
    pub beta: f64,
    pub correlation_to_market: f64,
    pub volatility: f64,
    pub downside_volatility: f64,
    pub upside_volatility: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    pub position_concentration: f64,
    pub sector_exposure: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationSettings {
    pub initial_balance: f64,
    pub commission_per_trade: f64,
    pub commission_per_share: f64,
    pub slippage_factor: f64,
    pub margin_requirement: f64,
    pub pattern_day_trader: bool,
    pub short_selling_enabled: bool,
    pub options_trading_enabled: bool,
    pub max_position_size: f64,
    pub max_portfolio_risk: f64,
    pub rebalance_frequency: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    pub id: Uuid,
    pub strategy_name: String,
    pub settings: SimulationSettings,
    pub portfolio: PaperPortfolio,
    pub equity_curve: Vec<EquityPoint>,
    pub trade_analysis: TradeAnalysis,
    pub benchmark_comparison: BenchmarkComparison,
    pub monte_carlo_analysis: MonteCarloAnalysis,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityPoint {
    pub timestamp: DateTime<Utc>,
    pub portfolio_value: f64,
    pub cash_balance: f64,
    pub positions_value: f64,
    pub drawdown: f64,
    pub drawdown_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeAnalysis {
    pub total_trades: u32,
    pub winning_trades: u32,
    pub losing_trades: u32,
    pub win_rate: f64,
    pub average_win: f64,
    pub average_loss: f64,
    pub profit_factor: f64,
    pub recovery_factor: f64,
    pub kelly_criterion: f64,
    pub trade_distribution: HashMap<String, u32>,
    pub monthly_returns: HashMap<String, f64>,
    pub rolling_returns: Vec<RollingReturn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollingReturn {
    pub period: String,
    pub return_value: f64,
    pub volatility: f64,
    pub sharpe_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkComparison {
    pub benchmark_name: String,
    pub benchmark_returns: Vec<f64>,
    pub portfolio_returns: Vec<f64>,
    pub correlation: f64,
    pub beta: f64,
    pub alpha: f64,
    pub information_ratio: f64,
    pub tracking_error: f64,
    pub up_capture: f64,
    pub down_capture: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonteCarloAnalysis {
    pub simulations: Vec<SimulationResult>,
    pub percentiles: HashMap<u8, f64>,
    pub probability_of_profit: f64,
    pub expected_shortfall: f64,
    pub var_95: f64,
    pub var_99: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    pub final_value: f64,
    pub total_return: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub win_rate: f64,
}

pub struct PaperHands {
    portfolios: HashMap<Uuid, PaperPortfolio>,
    simulation_settings: SimulationSettings,
    market_data: HashMap<String, Vec<MarketDataPoint>>,
    active_simulations: HashMap<Uuid, BacktestResult>,
}

impl Default for PaperHands {
    fn default() -> Self {
        Self::new()
    }
}

impl PaperHands {
    pub fn new() -> Self {
        Self {
            portfolios: HashMap::new(),
            simulation_settings: SimulationSettings::default(),
            market_data: HashMap::new(),
            active_simulations: HashMap::new(),
        }
    }

    // Portfolio Management
    pub fn create_portfolio(
        &mut self,
        name: String,
        initial_balance: f64,
    ) -> Result<Uuid, BullShiftError> {
        if !initial_balance.is_finite() || initial_balance <= 0.0 {
            return Err(BullShiftError::Validation(
                "Initial balance must be a positive finite number".to_string(),
            ));
        }
        let portfolio_id = Uuid::new_v4();

        let portfolio = PaperPortfolio {
            id: portfolio_id,
            name,
            initial_balance,
            current_balance: initial_balance,
            allocated_balance: 0.0,
            available_balance: initial_balance,
            positions: HashMap::new(),
            trades: Vec::new(),
            performance_metrics: PerformanceMetrics::default(),
            risk_metrics: RiskMetrics::default(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };

        self.portfolios.insert(portfolio_id, portfolio);
        log::info!("Created paper portfolio: {:?}", portfolio_id);
        Ok(portfolio_id)
    }

    pub fn execute_trade(
        &mut self,
        portfolio_id: Uuid,
        trade: PaperTrade,
    ) -> Result<(), BullShiftError> {
        {
            let portfolio = self
                .portfolios
                .get(&portfolio_id)
                .ok_or_else(|| BullShiftError::Portfolio("Portfolio not found".to_string()))?;
            Self::validate_trade_static(portfolio, &trade)?;
        }

        {
            let portfolio = self
                .portfolios
                .get_mut(&portfolio_id)
                .ok_or_else(|| BullShiftError::Portfolio("Portfolio not found".to_string()))?;
            Self::process_trade_static(
                &self.simulation_settings,
                &self.market_data,
                portfolio,
                trade,
            )?;
        }

        // Update portfolio metrics
        self.update_portfolio_metrics(portfolio_id);

        Ok(())
    }

    pub fn close_position(
        &mut self,
        portfolio_id: Uuid,
        symbol: &str,
        exit_price: f64,
    ) -> Result<(), BullShiftError> {
        let closing_trade = {
            let portfolio = self
                .portfolios
                .get(&portfolio_id)
                .ok_or_else(|| BullShiftError::Portfolio("Portfolio not found".to_string()))?;
            let position = portfolio
                .positions
                .get(symbol)
                .ok_or_else(|| BullShiftError::Portfolio("Position not found".to_string()))?;

            PaperTrade {
                id: Uuid::new_v4(),
                symbol: symbol.to_string(),
                side: if position.quantity > 0.0 {
                    OrderSide::Sell
                } else {
                    OrderSide::Buy
                },
                order_type: OrderType::Market,
                quantity: position.quantity.abs(),
                entry_price: position.entry_price,
                exit_price: Some(exit_price),
                stop_loss: None,
                take_profit: None,
                status: TradeStatus::Closed,
                entry_time: Utc::now(),
                exit_time: Some(Utc::now()),
                pnl: Some(Self::calculate_pnl_static(position, exit_price)),
                pnl_percentage: Some(Self::calculate_pnl_percentage_static(position, exit_price)),
                fees: Self::calculate_fees_static(
                    &self.simulation_settings,
                    position.quantity,
                    exit_price,
                ),
                strategy: None,
                notes: None,
            }
        };

        {
            let portfolio = self
                .portfolios
                .get_mut(&portfolio_id)
                .ok_or_else(|| BullShiftError::Portfolio("Portfolio not found".to_string()))?;
            Self::process_trade_static(
                &self.simulation_settings,
                &self.market_data,
                portfolio,
                closing_trade,
            )?;
            portfolio.positions.remove(symbol);
        }

        self.update_portfolio_metrics(portfolio_id);
        Ok(())
    }

    // Backtesting
    pub fn run_backtest(
        &mut self,
        strategy_name: String,
        settings: SimulationSettings,
    ) -> Result<Uuid, BullShiftError> {
        let backtest_id = Uuid::new_v4();

        // Create portfolio for backtest
        let portfolio_id = self.create_portfolio(
            format!("Backtest: {}", strategy_name),
            settings.initial_balance,
        )?;

        // Run simulation
        let result = self.simulate_trading(strategy_name, settings, portfolio_id)?;

        // Store result
        self.active_simulations.insert(backtest_id, result);

        log::info!("Completed backtest: {:?}", backtest_id);
        Ok(backtest_id)
    }

    pub fn run_monte_carlo(
        &self,
        portfolio_id: Uuid,
        num_simulations: u32,
    ) -> Result<MonteCarloAnalysis, BullShiftError> {
        let portfolio = self
            .portfolios
            .get(&portfolio_id)
            .ok_or_else(|| BullShiftError::Portfolio("Portfolio not found".to_string()))?;

        let mut simulations = Vec::new();

        for _ in 0..num_simulations {
            let simulation = self.run_single_simulation(portfolio)?;
            simulations.push(simulation);
        }

        // Calculate statistics
        let analysis = self.calculate_monte_carlo_statistics(simulations);

        Ok(analysis)
    }

    // Advanced Analytics
    pub fn calculate_correlation_matrix(
        &self,
        portfolio_id: Uuid,
    ) -> Result<HashMap<String, HashMap<String, f64>>, BullShiftError> {
        let portfolio = self
            .portfolios
            .get(&portfolio_id)
            .ok_or_else(|| BullShiftError::Portfolio("Portfolio not found".to_string()))?;

        let mut correlation_matrix = HashMap::new();
        let symbols: Vec<String> = portfolio.positions.keys().cloned().collect();

        for (i, symbol_i) in symbols.iter().enumerate() {
            let mut correlations = HashMap::with_capacity(symbols.len());

            for symbol_j in &symbols[i..] {
                // Calculate correlation only once for each pair
                let correlation = self.calculate_symbol_correlation(portfolio, symbol_i, symbol_j);
                correlations.insert(symbol_j.clone(), correlation);

                // Matrix is symmetric, so add reverse entry too (if not diagonal)
                if symbol_i != symbol_j {
                    correlation_matrix
                        .entry(symbol_j.clone())
                        .or_insert_with(HashMap::new)
                        .insert(symbol_i.clone(), correlation);
                }
            }

            correlation_matrix.insert(symbol_i.clone(), correlations);
        }

        Ok(correlation_matrix)
    }

    pub fn calculate_optimal_position_sizes(
        &self,
        portfolio_id: Uuid,
    ) -> Result<HashMap<String, f64>, BullShiftError> {
        let portfolio = self
            .portfolios
            .get(&portfolio_id)
            .ok_or_else(|| BullShiftError::Portfolio("Portfolio not found".to_string()))?;

        let mut optimal_sizes = HashMap::new();

        // Cache metrics to avoid repeated field access
        let win_rate = portfolio.performance_metrics.win_rate;
        let avg_win = portfolio.performance_metrics.average_win;
        let avg_loss = portfolio.performance_metrics.average_loss;
        let current_balance = portfolio.current_balance;

        // Pre-calculate Kelly percentage
        let kelly_percentage = if avg_loss != 0.0 {
            (win_rate * avg_win - (1.0 - win_rate) * avg_loss.abs()) / avg_loss.abs()
        } else {
            0.0
        };

        // Apply position sizing limits
        let max_position = current_balance * 0.25; // Max 25% per position
        let optimal_size = (kelly_percentage * current_balance).min(max_position);

        for symbol in portfolio.positions.keys() {
            optimal_sizes.insert(symbol.clone(), optimal_size);
        }

        Ok(optimal_sizes)
    }

    // Helper methods
    fn validate_trade_static(
        portfolio: &PaperPortfolio,
        trade: &PaperTrade,
    ) -> Result<(), BullShiftError> {
        let required_balance = trade.quantity * trade.entry_price;

        if required_balance > portfolio.available_balance {
            return Err(BullShiftError::Trading(
                "Insufficient balance for trade".to_string(),
            ));
        }

        let max_position_size = portfolio.current_balance * 0.5;
        if required_balance > max_position_size {
            return Err(BullShiftError::Trading(
                "Trade exceeds maximum position size".to_string(),
            ));
        }

        Ok(())
    }

    fn process_trade_static(
        settings: &SimulationSettings,
        market_data: &HashMap<String, Vec<MarketDataPoint>>,
        portfolio: &mut PaperPortfolio,
        trade: PaperTrade,
    ) -> Result<(), BullShiftError> {
        let fees = Self::calculate_fees_static(settings, trade.quantity, trade.entry_price);
        let total_cost = trade.quantity * trade.entry_price + fees;

        portfolio.current_balance -= total_cost;
        portfolio.allocated_balance += total_cost;

        let symbol = trade.symbol.clone();
        match portfolio.positions.get_mut(&symbol) {
            Some(position) => {
                Self::update_existing_position_static(market_data, position, &trade);
            }
            None => {
                let new_position = Self::create_new_position_static(settings, market_data, &trade);
                portfolio.positions.insert(symbol, new_position);
            }
        }

        portfolio.trades.push(trade);
        portfolio.last_updated = Utc::now();
        Ok(())
    }

    fn update_existing_position_static(
        market_data: &HashMap<String, Vec<MarketDataPoint>>,
        position: &mut PaperPosition,
        trade: &PaperTrade,
    ) {
        let old_quantity = position.quantity;
        let old_avg_price = position.average_entry_price;

        let new_quantity = old_quantity + trade.quantity;
        let new_avg_price =
            (old_quantity * old_avg_price + trade.quantity * trade.entry_price) / new_quantity;

        position.quantity = new_quantity;
        position.average_entry_price = new_avg_price;
        position.trades_count += 1;
        position.last_updated = Utc::now();

        if let Some(data) = market_data.get(&position.symbol) {
            if let Some(latest) = data.last() {
                position.current_price = latest.close;
                position.unrealized_pnl = (latest.close - position.entry_price) * position.quantity;
                let cost = position.quantity * position.average_entry_price;
                position.unrealized_pnl_percentage = if cost != 0.0 {
                    position.unrealized_pnl / cost
                } else {
                    0.0
                };
            }
        }
    }

    fn create_new_position_static(
        settings: &SimulationSettings,
        market_data: &HashMap<String, Vec<MarketDataPoint>>,
        trade: &PaperTrade,
    ) -> PaperPosition {
        let current_price = market_data
            .get(&trade.symbol)
            .and_then(|d| d.last())
            .map(|p| p.close)
            .unwrap_or(trade.entry_price);

        PaperPosition {
            symbol: trade.symbol.clone(),
            quantity: trade.quantity,
            entry_price: trade.entry_price,
            current_price,
            unrealized_pnl: 0.0,
            unrealized_pnl_percentage: 0.0,
            realized_pnl: 0.0,
            total_fees: Self::calculate_fees_static(settings, trade.quantity, trade.entry_price),
            trades_count: 1,
            average_entry_price: trade.entry_price,
            last_updated: Utc::now(),
        }
    }

    fn calculate_pnl_static(position: &PaperPosition, exit_price: f64) -> f64 {
        (exit_price - position.entry_price) * position.quantity - position.total_fees
    }

    fn calculate_pnl_percentage_static(position: &PaperPosition, exit_price: f64) -> f64 {
        let total_cost = position.entry_price * position.quantity;
        if total_cost == 0.0 {
            return 0.0;
        }
        ((exit_price - position.entry_price) * position.quantity) / total_cost
    }

    pub fn calculate_fees(&self, quantity: f64, price: f64) -> f64 {
        Self::calculate_fees_static(&self.simulation_settings, quantity, price)
    }

    fn calculate_fees_static(settings: &SimulationSettings, quantity: f64, price: f64) -> f64 {
        let trade_value = quantity * price;
        settings.commission_per_trade + (trade_value * settings.commission_per_share)
    }

    fn update_portfolio_metrics(&mut self, portfolio_id: Uuid) {
        if let Some(portfolio) = self.portfolios.get(&portfolio_id) {
            let perf = Self::calculate_performance_metrics_static(portfolio);
            let risk = Self::calculate_risk_metrics_static(portfolio);
            if let Some(portfolio) = self.portfolios.get_mut(&portfolio_id) {
                portfolio.performance_metrics = perf;
                portfolio.risk_metrics = risk;
            }
        }
    }

    fn calculate_performance_metrics_static(portfolio: &PaperPortfolio) -> PerformanceMetrics {
        let total_return = portfolio.current_balance - portfolio.initial_balance;
        let total_return_percentage = total_return / portfolio.initial_balance;

        let closed_trades: Vec<_> = portfolio
            .trades
            .iter()
            .filter(|t| t.status == TradeStatus::Closed)
            .collect();

        let winning_trades = closed_trades
            .iter()
            .filter(|t| t.pnl.unwrap_or(0.0) > 0.0)
            .count();

        let losing_trades = closed_trades
            .iter()
            .filter(|t| t.pnl.unwrap_or(0.0) < 0.0)
            .count();

        let win_rate = if closed_trades.is_empty() {
            0.0
        } else {
            winning_trades as f64 / closed_trades.len() as f64
        };

        let total_wins: f64 = closed_trades
            .iter()
            .filter(|t| t.pnl.unwrap_or(0.0) > 0.0)
            .map(|t| t.pnl.unwrap_or(0.0))
            .sum();

        let total_losses: f64 = closed_trades
            .iter()
            .filter(|t| t.pnl.unwrap_or(0.0) < 0.0)
            .map(|t| t.pnl.unwrap_or(0.0).abs())
            .sum();

        let profit_factor = if total_losses == 0.0 {
            if total_wins > 0.0 {
                f64::INFINITY
            } else {
                0.0
            }
        } else {
            total_wins / total_losses
        };

        PerformanceMetrics {
            total_return,
            total_return_percentage,
            annualized_return: 0.0, // Would calculate based on time period
            win_rate,
            profit_factor,
            sharpe_ratio: 0.0,  // Would calculate using risk-free rate
            sortino_ratio: 0.0, // Would calculate using downside deviation
            max_drawdown: 0.0,  // Would calculate from equity curve
            max_drawdown_percentage: 0.0,
            calmar_ratio: 0.0, // Would calculate using max drawdown
            total_trades: closed_trades.len() as u32,
            winning_trades: winning_trades as u32,
            losing_trades: losing_trades as u32,
            average_win: if winning_trades > 0 {
                total_wins / winning_trades as f64
            } else {
                0.0
            },
            average_loss: if losing_trades > 0 {
                total_losses / losing_trades as f64
            } else {
                0.0
            },
            largest_win: closed_trades
                .iter()
                .map(|t| t.pnl.unwrap_or(0.0))
                .fold(0.0, f64::max),
            largest_loss: closed_trades
                .iter()
                .map(|t| t.pnl.unwrap_or(0.0))
                .fold(0.0, f64::min),
            average_trade_duration: Duration::zero(), // Would calculate from trade durations
            best_trade_duration: Duration::zero(),
            worst_trade_duration: Duration::zero(),
        }
    }

    fn calculate_risk_metrics_static(_portfolio: &PaperPortfolio) -> RiskMetrics {
        // Simplified risk metrics calculation
        RiskMetrics {
            var_95: 0.0, // Would calculate from historical returns
            var_99: 0.0,
            cvar_95: 0.0,
            beta: 0.0, // Would calculate relative to market
            correlation_to_market: 0.0,
            volatility: 0.0, // Would calculate from returns
            downside_volatility: 0.0,
            upside_volatility: 0.0,
            skewness: 0.0,
            kurtosis: 0.0,
            position_concentration: 0.0, // Would calculate concentration ratio
            sector_exposure: HashMap::new(),
        }
    }

    fn simulate_trading(
        &mut self,
        strategy_name: String,
        settings: SimulationSettings,
        portfolio_id: Uuid,
    ) -> Result<BacktestResult, BullShiftError> {
        let portfolio = self
            .portfolios
            .get(&portfolio_id)
            .ok_or_else(|| BullShiftError::Portfolio("Portfolio not found".to_string()))?;

        // Generate equity curve based on simulated trades
        let mut equity_curve = Vec::new();
        let mut portfolio_value: f64 = settings.initial_balance;
        let mut max_portfolio_value: f64 = portfolio_value;
        let mut max_drawdown: f64 = 0.0;

        let duration = settings.end_date.signed_duration_since(settings.start_date);
        let days = duration.num_days().max(1);

        // Generate daily equity points
        for day in 0..=days {
            let timestamp = settings.start_date + Duration::days(day);

            // Simulate daily returns using random walk with drift
            let daily_return = self.generate_daily_return(&strategy_name, day);
            portfolio_value *= 1.0 + daily_return;

            // Calculate drawdown
            if portfolio_value > max_portfolio_value {
                max_portfolio_value = portfolio_value;
            }
            let drawdown = max_portfolio_value - portfolio_value;
            let drawdown_pct = if max_portfolio_value > 0.0 {
                drawdown / max_portfolio_value
            } else {
                0.0
            };
            max_drawdown = max_drawdown.max(drawdown);

            equity_curve.push(EquityPoint {
                timestamp,
                portfolio_value,
                cash_balance: portfolio_value * 0.1, // 10% cash
                positions_value: portfolio_value * 0.9,
                drawdown,
                drawdown_percentage: drawdown_pct,
            });
        }

        // Generate simulated trade analysis
        let trade_analysis = self.generate_trade_analysis(&equity_curve, &settings);

        // Generate benchmark comparison
        let benchmark_comparison = self.generate_benchmark_comparison(&equity_curve, &settings);

        // Run Monte Carlo analysis
        let monte_carlo_analysis = self.run_monte_carlo_simulation(&equity_curve, 1000)?;

        Ok(BacktestResult {
            id: Uuid::new_v4(),
            strategy_name,
            settings,
            portfolio: portfolio.clone(),
            equity_curve,
            trade_analysis,
            benchmark_comparison,
            monte_carlo_analysis,
            created_at: Utc::now(),
        })
    }

    fn generate_daily_return(&self, strategy_name: &str, day: i64) -> f64 {
        // Different strategies have different return characteristics
        let base_return = match strategy_name {
            "Momentum" => 0.0005, // 0.05% daily
            "Mean Reversion" => 0.0003,
            "Trend Following" => 0.0004,
            "Breakout" => 0.0006,
            _ => 0.0002,
        };

        // Add some randomness and cyclical patterns
        let random_component = (day as f64 * 0.1).sin() * 0.002;
        let noise = ((day * 17) % 100) as f64 / 10000.0 - 0.005;

        base_return + random_component + noise
    }

    fn generate_trade_analysis(
        &self,
        equity_curve: &[EquityPoint],
        settings: &SimulationSettings,
    ) -> TradeAnalysis {
        let total_days = equity_curve.len() as u32;
        let trades_per_day = 2; // Assume 2 trades per day
        let total_trades = total_days * trades_per_day;

        // Calculate returns from equity curve
        let mut returns = Vec::new();
        for i in 1..equity_curve.len() {
            let prev = equity_curve[i - 1].portfolio_value;
            let curr = equity_curve[i].portfolio_value;
            returns.push((curr - prev) / prev);
        }

        // Simulate win rate and trade distribution
        let win_rate = 0.55; // 55% win rate
        let winning_trades = (total_trades as f64 * win_rate) as u32;
        let losing_trades = total_trades - winning_trades;

        let total_return = equity_curve
            .last()
            .map(|e| e.portfolio_value)
            .unwrap_or(settings.initial_balance)
            - settings.initial_balance;
        let avg_win = if winning_trades > 0 {
            total_return * 0.8 / winning_trades as f64
        } else {
            0.0
        };
        let avg_loss = if losing_trades > 0 {
            -total_return * 0.2 / losing_trades as f64
        } else {
            0.0
        };

        let profit_factor = if avg_loss.abs() > 0.0 {
            (winning_trades as f64 * avg_win) / (losing_trades as f64 * avg_loss.abs())
        } else {
            0.0
        };

        // Generate monthly returns
        let mut monthly_returns = HashMap::new();
        for (i, point) in equity_curve.iter().enumerate().step_by(30) {
            let month_key = point.timestamp.format("%Y-%m").to_string();
            let start_idx = i.saturating_sub(30);
            let start_value = equity_curve
                .get(start_idx)
                .map(|e| e.portfolio_value)
                .unwrap_or(settings.initial_balance);
            let monthly_return = (point.portfolio_value - start_value) / start_value;
            monthly_returns.insert(month_key, monthly_return);
        }

        TradeAnalysis {
            total_trades,
            winning_trades,
            losing_trades,
            win_rate,
            average_win: avg_win,
            average_loss: avg_loss,
            profit_factor,
            recovery_factor: 0.0, // Would calculate from equity curve
            kelly_criterion: if avg_loss.abs() > 0.0 {
                win_rate - ((1.0 - win_rate) / (avg_win / avg_loss.abs()))
            } else {
                0.0
            },
            trade_distribution: HashMap::new(),
            monthly_returns,
            rolling_returns: Vec::new(),
        }
    }

    fn generate_benchmark_comparison(
        &self,
        equity_curve: &[EquityPoint],
        _settings: &SimulationSettings,
    ) -> BenchmarkComparison {
        let mut benchmark_returns = Vec::new();
        let mut portfolio_returns = Vec::new();

        for i in 1..equity_curve.len() {
            let portfolio_return = (equity_curve[i].portfolio_value
                - equity_curve[i - 1].portfolio_value)
                / equity_curve[i - 1].portfolio_value;
            let benchmark_return = portfolio_return * 0.8 + 0.0002;
            portfolio_returns.push(portfolio_return);
            benchmark_returns.push(benchmark_return);
        }

        let correlation = self.calculate_correlation(&portfolio_returns, &benchmark_returns);
        let benchmark_variance = self.calculate_variance(&benchmark_returns);
        let beta = if benchmark_variance > 0.0 {
            self.calculate_covariance(&portfolio_returns, &benchmark_returns) / benchmark_variance
        } else {
            1.0
        };

        let n = portfolio_returns.len().max(1) as f64;
        let avg_portfolio_return = portfolio_returns.iter().sum::<f64>() / n;
        let avg_benchmark_return = benchmark_returns.iter().sum::<f64>() / n;
        let alpha = avg_portfolio_return - beta * avg_benchmark_return;
        let std_dev = self.calculate_standard_deviation(&portfolio_returns);
        let information_ratio = if std_dev > 0.0 { alpha / std_dev } else { 0.0 };

        BenchmarkComparison {
            benchmark_name: "S&P 500".to_string(),
            benchmark_returns,
            portfolio_returns,
            correlation,
            beta,
            alpha,
            information_ratio,
            tracking_error: 0.0,
            up_capture: 1.0,
            down_capture: 0.8,
        }
    }

    fn run_single_simulation(
        &self,
        portfolio: &PaperPortfolio,
    ) -> Result<SimulationResult, BullShiftError> {
        // Monte Carlo simulation using geometric Brownian motion
        let initial_value = portfolio.current_balance;
        let days = 252; // Trading days in a year

        // Calculate historical returns and volatility from portfolio trades
        let (mean_return, volatility) = self.calculate_return_statistics(portfolio);

        // Run simulation
        let mut current_value = initial_value;
        let mut max_value = initial_value;
        let mut max_drawdown: f64 = 0.0;
        let mut total_return: f64 = 0.0;

        for _ in 0..days {
            // Generate random shock using Box-Muller transform
            let u1 = rand::random::<f64>();
            let u2 = rand::random::<f64>();
            let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();

            // Apply geometric Brownian motion
            let daily_return = mean_return / days as f64 + volatility * z / (days as f64).sqrt();
            current_value *= 1.0 + daily_return;

            // Track max drawdown
            if current_value > max_value {
                max_value = current_value;
            }
            let drawdown = (max_value - current_value) / max_value;
            max_drawdown = max_drawdown.max(drawdown);

            total_return = (current_value - initial_value) / initial_value;
        }

        // Calculate Sharpe ratio (simplified, assuming risk-free rate of 2%)
        let risk_free_rate = 0.02;
        let excess_return = total_return - risk_free_rate;
        let sharpe_ratio = if volatility > 0.0 {
            excess_return / volatility
        } else {
            0.0
        };

        // Estimate win rate based on return distribution
        let win_rate = if total_return > 0.0 { 0.55 } else { 0.45 };

        Ok(SimulationResult {
            final_value: current_value,
            total_return: current_value - initial_value,
            max_drawdown,
            sharpe_ratio,
            win_rate,
        })
    }

    fn calculate_return_statistics(&self, portfolio: &PaperPortfolio) -> (f64, f64) {
        let trades = &portfolio.trades;
        if trades.is_empty() {
            return (0.08, 0.15); // Default: 8% annual return, 15% volatility
        }

        // Calculate returns from trades
        let returns: Vec<f64> = trades.iter().filter_map(|t| t.pnl_percentage).collect();

        if returns.is_empty() {
            return (0.08, 0.15);
        }

        // Calculate mean
        let mean = returns.iter().sum::<f64>() / returns.len() as f64;

        // Calculate standard deviation
        let variance =
            returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / returns.len() as f64;
        let std_dev = variance.sqrt();

        (mean, std_dev)
    }

    fn calculate_monte_carlo_statistics(
        &self,
        simulations: Vec<SimulationResult>,
    ) -> MonteCarloAnalysis {
        if simulations.is_empty() {
            return MonteCarloAnalysis::default();
        }

        // Extract final values for percentile calculations, filtering NaN/Inf
        let mut final_values: Vec<f64> = simulations
            .iter()
            .map(|s| s.final_value)
            .filter(|v| v.is_finite())
            .collect();
        if final_values.is_empty() {
            return MonteCarloAnalysis::default();
        }
        final_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Calculate percentiles
        let mut percentiles = HashMap::new();
        let max_idx = final_values.len() - 1;
        for percentile in [5, 10, 25, 50, 75, 90, 95] {
            let index = ((percentile as f64 / 100.0) * max_idx as f64) as usize;
            percentiles.insert(percentile, final_values[index.min(max_idx)]);
        }

        // Calculate probability of profit
        let initial_value = simulations.first().map(|s| s.final_value).unwrap_or(1.0);
        let profitable_simulations = simulations
            .iter()
            .filter(|s| s.final_value > initial_value)
            .count();
        let probability_of_profit = profitable_simulations as f64 / simulations.len() as f64;

        // Calculate VaR (Value at Risk)
        let var_95_index = (0.05 * final_values.len() as f64) as usize;
        let var_99_index = (0.01 * final_values.len() as f64) as usize;
        let var_95 = final_values.get(var_95_index).copied().unwrap_or(0.0);
        let var_99 = final_values.get(var_99_index).copied().unwrap_or(0.0);

        // Calculate Expected Shortfall (CVaR)
        let expected_shortfall =
            final_values.iter().take(var_95_index).sum::<f64>() / var_95_index.max(1) as f64;

        MonteCarloAnalysis {
            simulations,
            percentiles,
            probability_of_profit,
            expected_shortfall,
            var_95,
            var_99,
        }
    }

    fn run_monte_carlo_simulation(
        &self,
        equity_curve: &[EquityPoint],
        num_simulations: u32,
    ) -> Result<MonteCarloAnalysis, BullShiftError> {
        let mut simulations = Vec::new();

        // Calculate historical returns and volatility from equity curve
        let returns: Vec<f64> = equity_curve
            .windows(2)
            .map(|w| (w[1].portfolio_value - w[0].portfolio_value) / w[0].portfolio_value)
            .collect();

        if returns.is_empty() {
            return Ok(MonteCarloAnalysis::default());
        }

        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns
            .iter()
            .map(|r| (r - mean_return).powi(2))
            .sum::<f64>()
            / returns.len() as f64;
        let volatility = variance.sqrt();

        // Get initial value
        let initial_value = equity_curve
            .first()
            .map(|e| e.portfolio_value)
            .unwrap_or(100000.0);

        // Run multiple simulations
        for _ in 0..num_simulations {
            let mut current_value = initial_value;
            let mut max_value = current_value;
            let mut max_drawdown: f64 = 0.0;

            // Simulate future path
            for _ in 0..returns.len() {
                // Random walk with historical characteristics
                let random_shock = self.generate_random_shock();
                let daily_return = mean_return + volatility * random_shock;
                current_value *= 1.0 + daily_return;

                // Update max drawdown
                if current_value > max_value {
                    max_value = current_value;
                }
                let drawdown = (max_value - current_value) / max_value;
                max_drawdown = max_drawdown.max(drawdown);
            }

            // Calculate metrics
            let total_return = current_value - initial_value;
            let risk_free_rate = 0.02;
            let days = returns.len() as f64;
            let annual_return = (1.0 + total_return / initial_value).powf(252.0 / days) - 1.0;
            let annual_volatility = volatility * 252.0_f64.sqrt();
            let sharpe_ratio = if annual_volatility > 0.0 {
                (annual_return - risk_free_rate) / annual_volatility
            } else {
                0.0
            };

            simulations.push(SimulationResult {
                final_value: current_value,
                total_return,
                max_drawdown,
                sharpe_ratio,
                win_rate: if total_return > 0.0 { 0.55 } else { 0.45 },
            });
        }

        Ok(self.calculate_monte_carlo_statistics(simulations))
    }

    fn generate_random_shock(&self) -> f64 {
        // Box-Muller transform for normal distribution
        let u1 = rand::random::<f64>();
        let u2 = rand::random::<f64>();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }

    fn calculate_symbol_correlation(
        &self,
        portfolio: &PaperPortfolio,
        symbol1: &str,
        symbol2: &str,
    ) -> f64 {
        // Get positions for both symbols
        let pos1 = match portfolio.positions.get(symbol1) {
            Some(p) => p,
            None => return 0.0,
        };
        let pos2 = match portfolio.positions.get(symbol2) {
            Some(p) => p,
            None => return 0.0,
        };

        // Get trades for both symbols
        let trades1: Vec<&PaperTrade> = portfolio
            .trades
            .iter()
            .filter(|t| t.symbol == symbol1 && t.status == TradeStatus::Closed)
            .collect();
        let trades2: Vec<&PaperTrade> = portfolio
            .trades
            .iter()
            .filter(|t| t.symbol == symbol2 && t.status == TradeStatus::Closed)
            .collect();

        // If no closed trades, use price-based correlation
        if trades1.is_empty() || trades2.is_empty() {
            // Generate synthetic returns based on position characteristics
            let returns1 = self.generate_synthetic_returns(pos1);
            let returns2 = self.generate_synthetic_returns(pos2);
            return self.calculate_correlation(&returns1, &returns2);
        }

        // Calculate returns from trades
        let returns1: Vec<f64> = trades1.iter().filter_map(|t| t.pnl_percentage).collect();
        let returns2: Vec<f64> = trades2.iter().filter_map(|t| t.pnl_percentage).collect();

        // Need at least 2 data points for correlation
        if returns1.len() < 2 || returns2.len() < 2 {
            return 0.0;
        }

        // Align returns by taking minimum length
        let min_len = returns1.len().min(returns2.len());
        let aligned_returns1: Vec<f64> = returns1.into_iter().take(min_len).collect();
        let aligned_returns2: Vec<f64> = returns2.into_iter().take(min_len).collect();

        self.calculate_correlation(&aligned_returns1, &aligned_returns2)
    }

    fn generate_synthetic_returns(&self, position: &PaperPosition) -> Vec<f64> {
        // Generate synthetic daily returns based on position's unrealized P&L
        let days = 30; // Assume 30 days
        let avg_daily_return = position.unrealized_pnl_percentage / days as f64;
        let volatility = 0.02; // Assume 2% daily volatility

        (0..days)
            .map(|i| {
                let trend = avg_daily_return;
                let noise = ((i * 7) % 100) as f64 / 1000.0 - 0.05;
                trend + noise * volatility
            })
            .collect()
    }

    fn calculate_correlation(&self, x: &[f64], y: &[f64]) -> f64 {
        if x.len() != y.len() || x.len() < 2 {
            return 0.0;
        }

        let n = x.len() as f64;
        let mean_x = x.iter().sum::<f64>() / n;
        let mean_y = y.iter().sum::<f64>() / n;

        let mut numerator = 0.0;
        let mut sum_sq_x = 0.0;
        let mut sum_sq_y = 0.0;

        for i in 0..x.len() {
            let diff_x = x[i] - mean_x;
            let diff_y = y[i] - mean_y;
            numerator += diff_x * diff_y;
            sum_sq_x += diff_x * diff_x;
            sum_sq_y += diff_y * diff_y;
        }

        let denominator = (sum_sq_x * sum_sq_y).sqrt();
        if denominator > 0.0 {
            numerator / denominator
        } else {
            0.0
        }
    }

    fn calculate_covariance(&self, x: &[f64], y: &[f64]) -> f64 {
        if x.len() != y.len() || x.is_empty() {
            return 0.0;
        }

        let n = x.len() as f64;
        let mean_x = x.iter().sum::<f64>() / n;
        let mean_y = y.iter().sum::<f64>() / n;

        x.iter()
            .zip(y.iter())
            .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
            .sum::<f64>()
            / n
    }

    fn calculate_variance(&self, data: &[f64]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let n = data.len() as f64;
        let mean = data.iter().sum::<f64>() / n;

        data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n
    }

    fn calculate_standard_deviation(&self, data: &[f64]) -> f64 {
        self.calculate_variance(data).sqrt()
    }

    pub fn get_current_price(&self, symbol: &str) -> Option<f64> {
        // Try to get price from cached market data
        if let Some(price_data) = self.market_data.get(symbol) {
            if let Some(latest) = price_data.last() {
                return Some(latest.close);
            }
        }

        // If no market data available, generate a realistic price based on symbol hash
        // This ensures consistent pricing for the same symbol
        let base_price = self.generate_symbol_base_price(symbol);

        // Add some realistic variation based on time
        let time_factor = (Utc::now().timestamp() % 86400) as f64 / 86400.0;
        let variation = (time_factor - 0.5) * 0.02; // ±1% variation

        Some(base_price * (1.0 + variation))
    }

    pub fn generate_symbol_base_price(&self, symbol: &str) -> f64 {
        // Generate a deterministic base price from symbol
        let hash = symbol
            .bytes()
            .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));

        // Map hash to a price range ($10 - $500)
        let min_price = 10.0;
        let max_price = 500.0;
        let normalized = (hash % 1000) as f64 / 1000.0;

        min_price + (max_price - min_price) * normalized
    }

    /// Fetch market data for a symbol (public API for data streaming integration)
    pub fn fetch_market_data(&mut self, symbol: &str, data: Vec<MarketDataPoint>) {
        self.market_data.insert(symbol.to_string(), data);
        log::info!("Updated market data for {}", symbol);
    }

    /// Get latest market data for a symbol
    pub fn get_market_data(&self, symbol: &str) -> Option<&Vec<MarketDataPoint>> {
        self.market_data.get(symbol)
    }

    /// Clear old market data to manage memory
    pub fn clear_old_market_data(&mut self, days_to_keep: i64) {
        let cutoff = Utc::now() - Duration::days(days_to_keep);

        for (_, data) in self.market_data.iter_mut() {
            data.retain(|point| point.timestamp >= cutoff);
        }

        log::info!("Cleared market data older than {} days", days_to_keep);
    }

    // Public interface methods
    pub fn get_portfolio(&self, portfolio_id: &Uuid) -> Option<&PaperPortfolio> {
        self.portfolios.get(portfolio_id)
    }

    pub fn get_portfolios(&self) -> Vec<&PaperPortfolio> {
        self.portfolios.values().collect()
    }

    pub fn get_backtest_result(&self, backtest_id: &Uuid) -> Option<&BacktestResult> {
        self.active_simulations.get(backtest_id)
    }

    pub fn get_backtest_results(&self) -> Vec<&BacktestResult> {
        self.active_simulations.values().collect()
    }
}

// Default implementations
impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            initial_balance: 100000.0,
            commission_per_trade: 5.0,
            commission_per_share: 0.005,
            slippage_factor: 0.001,
            margin_requirement: 0.5,
            pattern_day_trader: false,
            short_selling_enabled: true,
            options_trading_enabled: false,
            max_position_size: 0.25,
            max_portfolio_risk: 0.02,
            rebalance_frequency: "monthly".to_string(),
            start_date: Utc::now() - Duration::days(365),
            end_date: Utc::now(),
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_return: 0.0,
            total_return_percentage: 0.0,
            annualized_return: 0.0,
            win_rate: 0.0,
            profit_factor: 0.0,
            sharpe_ratio: 0.0,
            sortino_ratio: 0.0,
            max_drawdown: 0.0,
            max_drawdown_percentage: 0.0,
            calmar_ratio: 0.0,
            total_trades: 0,
            winning_trades: 0,
            losing_trades: 0,
            average_win: 0.0,
            average_loss: 0.0,
            largest_win: 0.0,
            largest_loss: 0.0,
            average_trade_duration: Duration::zero(),
            best_trade_duration: Duration::zero(),
            worst_trade_duration: Duration::zero(),
        }
    }
}

impl Default for RiskMetrics {
    fn default() -> Self {
        Self {
            var_95: 0.0,
            var_99: 0.0,
            cvar_95: 0.0,
            beta: 0.0,
            correlation_to_market: 0.0,
            volatility: 0.0,
            downside_volatility: 0.0,
            upside_volatility: 0.0,
            skewness: 0.0,
            kurtosis: 0.0,
            position_concentration: 0.0,
            sector_exposure: HashMap::new(),
        }
    }
}

impl Default for TradeAnalysis {
    fn default() -> Self {
        Self {
            total_trades: 0,
            winning_trades: 0,
            losing_trades: 0,
            win_rate: 0.0,
            average_win: 0.0,
            average_loss: 0.0,
            profit_factor: 0.0,
            recovery_factor: 0.0,
            kelly_criterion: 0.0,
            trade_distribution: HashMap::new(),
            monthly_returns: HashMap::new(),
            rolling_returns: Vec::new(),
        }
    }
}

impl Default for BenchmarkComparison {
    fn default() -> Self {
        Self {
            benchmark_name: "S&P 500".to_string(),
            benchmark_returns: Vec::new(),
            portfolio_returns: Vec::new(),
            correlation: 0.0,
            beta: 0.0,
            alpha: 0.0,
            information_ratio: 0.0,
            tracking_error: 0.0,
            up_capture: 0.0,
            down_capture: 0.0,
        }
    }
}

impl Default for MonteCarloAnalysis {
    fn default() -> Self {
        Self {
            simulations: Vec::new(),
            percentiles: HashMap::new(),
            probability_of_profit: 0.0,
            expected_shortfall: 0.0,
            var_95: 0.0,
            var_99: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataPoint {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_portfolio_valid() {
        let mut engine = PaperHands::new();
        let id = engine
            .create_portfolio("Test".to_string(), 100_000.0)
            .unwrap();
        let portfolio = engine.get_portfolio(&id).unwrap();
        assert_eq!(portfolio.name, "Test");
        assert_eq!(portfolio.initial_balance, 100_000.0);
        assert_eq!(portfolio.current_balance, 100_000.0);
        assert_eq!(portfolio.available_balance, 100_000.0);
        assert_eq!(portfolio.allocated_balance, 0.0);
        assert!(portfolio.positions.is_empty());
        assert!(portfolio.trades.is_empty());
    }

    #[test]
    fn test_create_portfolio_invalid_balance_zero() {
        let mut engine = PaperHands::new();
        assert!(engine.create_portfolio("Zero".to_string(), 0.0).is_err());
    }

    #[test]
    fn test_create_portfolio_invalid_balance_negative() {
        let mut engine = PaperHands::new();
        assert!(engine.create_portfolio("Neg".to_string(), -1000.0).is_err());
    }

    #[test]
    fn test_create_portfolio_invalid_balance_nan() {
        let mut engine = PaperHands::new();
        assert!(engine
            .create_portfolio("NaN".to_string(), f64::NAN)
            .is_err());
    }

    #[test]
    fn test_create_portfolio_invalid_balance_infinity() {
        let mut engine = PaperHands::new();
        assert!(engine
            .create_portfolio("Inf".to_string(), f64::INFINITY)
            .is_err());
    }

    #[test]
    fn test_create_multiple_portfolios() {
        let mut engine = PaperHands::new();
        let id1 = engine.create_portfolio("P1".to_string(), 50_000.0).unwrap();
        let id2 = engine.create_portfolio("P2".to_string(), 75_000.0).unwrap();
        assert_ne!(id1, id2);
        let all = engine.get_portfolios();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_get_portfolio_not_found() {
        let engine = PaperHands::new();
        assert!(engine.get_portfolio(&Uuid::new_v4()).is_none());
    }

    #[test]
    fn test_close_position_portfolio_not_found() {
        let mut engine = PaperHands::new();
        let result = engine.close_position(Uuid::new_v4(), "AAPL", 150.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_close_position_symbol_not_found() {
        let mut engine = PaperHands::new();
        let id = engine
            .create_portfolio("Test".to_string(), 100_000.0)
            .unwrap();
        let result = engine.close_position(id, "NONEXISTENT", 100.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_simulation_settings_default() {
        let settings = SimulationSettings::default();
        assert_eq!(settings.initial_balance, 100_000.0);
        assert_eq!(settings.commission_per_trade, 5.0);
        assert_eq!(settings.commission_per_share, 0.005);
        assert!(!settings.pattern_day_trader);
        assert!(settings.short_selling_enabled);
        assert!(!settings.options_trading_enabled);
    }

    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();
        assert_eq!(metrics.total_return, 0.0);
        assert_eq!(metrics.win_rate, 0.0);
        assert_eq!(metrics.total_trades, 0);
        assert_eq!(metrics.winning_trades, 0);
    }

    #[test]
    fn test_risk_metrics_default() {
        let metrics = RiskMetrics::default();
        assert_eq!(metrics.var_95, 0.0);
        assert_eq!(metrics.beta, 0.0);
        assert!(metrics.sector_exposure.is_empty());
    }

    #[test]
    fn test_monte_carlo_analysis_default() {
        let analysis = MonteCarloAnalysis::default();
        assert!(analysis.simulations.is_empty());
        assert!(analysis.percentiles.is_empty());
        assert_eq!(analysis.probability_of_profit, 0.0);
    }

    #[test]
    fn test_trade_analysis_default() {
        let analysis = TradeAnalysis::default();
        assert_eq!(analysis.total_trades, 0);
        assert_eq!(analysis.win_rate, 0.0);
        assert!(analysis.monthly_returns.is_empty());
    }

    #[test]
    fn test_benchmark_comparison_default() {
        let bench = BenchmarkComparison::default();
        assert_eq!(bench.benchmark_name, "S&P 500");
        assert!(bench.benchmark_returns.is_empty());
        assert_eq!(bench.beta, 0.0);
    }

    #[test]
    fn test_fetch_and_get_market_data() {
        let mut engine = PaperHands::new();
        let data = vec![MarketDataPoint {
            timestamp: Utc::now(),
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 103.0,
            volume: 1_000_000,
        }];
        engine.fetch_market_data("AAPL", data);
        let result = engine.get_market_data("AAPL");
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 1);
        assert_eq!(result.unwrap()[0].close, 103.0);
    }

    #[test]
    fn test_get_market_data_not_found() {
        let engine = PaperHands::new();
        assert!(engine.get_market_data("NONEXISTENT").is_none());
    }

    #[test]
    fn test_clear_old_market_data() {
        let mut engine = PaperHands::new();
        let old_point = MarketDataPoint {
            timestamp: Utc::now() - Duration::days(100),
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 103.0,
            volume: 500,
        };
        let new_point = MarketDataPoint {
            timestamp: Utc::now(),
            open: 110.0,
            high: 115.0,
            low: 109.0,
            close: 113.0,
            volume: 600,
        };
        engine.fetch_market_data("AAPL", vec![old_point, new_point]);
        engine.clear_old_market_data(30);
        let data = engine.get_market_data("AAPL").unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].close, 113.0);
    }

    #[test]
    fn test_generate_symbol_base_price_deterministic() {
        let engine = PaperHands::new();
        let price1 = engine.generate_symbol_base_price("AAPL");
        let price2 = engine.generate_symbol_base_price("AAPL");
        assert_eq!(price1, price2);
    }

    #[test]
    fn test_generate_symbol_base_price_range() {
        let engine = PaperHands::new();
        let price = engine.generate_symbol_base_price("TSLA");
        assert!((10.0..=500.0).contains(&price));
    }

    #[test]
    fn test_generate_symbol_base_price_differs_by_symbol() {
        let engine = PaperHands::new();
        let price_a = engine.generate_symbol_base_price("AAPL");
        let price_b = engine.generate_symbol_base_price("GOOG");
        // Different symbols should produce different prices (not guaranteed but very likely)
        assert_ne!(price_a, price_b);
    }

    #[test]
    fn test_calculate_correlation_identical() {
        let engine = PaperHands::new();
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let corr = engine.calculate_correlation(&data, &data);
        assert!((corr - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_correlation_inverse() {
        let engine = PaperHands::new();
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![5.0, 4.0, 3.0, 2.0, 1.0];
        let corr = engine.calculate_correlation(&x, &y);
        assert!((corr - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_correlation_too_short() {
        let engine = PaperHands::new();
        let corr = engine.calculate_correlation(&[1.0], &[2.0]);
        assert_eq!(corr, 0.0);
    }

    #[test]
    fn test_calculate_correlation_mismatched_lengths() {
        let engine = PaperHands::new();
        let corr = engine.calculate_correlation(&[1.0, 2.0, 3.0], &[1.0, 2.0]);
        assert_eq!(corr, 0.0);
    }

    #[test]
    fn test_calculate_variance_empty() {
        let engine = PaperHands::new();
        assert_eq!(engine.calculate_variance(&[]), 0.0);
    }

    #[test]
    fn test_calculate_variance_constant() {
        let engine = PaperHands::new();
        let data = vec![5.0, 5.0, 5.0, 5.0];
        assert!(engine.calculate_variance(&data).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_variance_known() {
        let engine = PaperHands::new();
        // Variance of [1, 2, 3, 4, 5] = 2.0
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert!((engine.calculate_variance(&data) - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_standard_deviation() {
        let engine = PaperHands::new();
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let std = engine.calculate_standard_deviation(&data);
        assert!((std - 2.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_covariance_identical() {
        let engine = PaperHands::new();
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let cov = engine.calculate_covariance(&data, &data);
        // Covariance with self = variance
        assert!((cov - engine.calculate_variance(&data)).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_covariance_empty() {
        let engine = PaperHands::new();
        assert_eq!(engine.calculate_covariance(&[], &[]), 0.0);
    }

    #[test]
    fn test_calculate_covariance_mismatched() {
        let engine = PaperHands::new();
        assert_eq!(engine.calculate_covariance(&[1.0, 2.0], &[1.0]), 0.0);
    }

    #[test]
    fn test_get_current_price_from_market_data() {
        let mut engine = PaperHands::new();
        let data = vec![MarketDataPoint {
            timestamp: Utc::now(),
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 150.0,
            volume: 1000,
        }];
        engine.fetch_market_data("AAPL", data);
        let price = engine.get_current_price("AAPL");
        assert!(price.is_some());
        assert_eq!(price.unwrap(), 150.0);
    }

    #[test]
    fn test_get_current_price_generated() {
        let engine = PaperHands::new();
        let price = engine.get_current_price("AAPL");
        assert!(price.is_some());
        let p = price.unwrap();
        assert!(p > 0.0 && p.is_finite());
    }

    #[test]
    fn test_generate_daily_return_by_strategy() {
        let engine = PaperHands::new();
        // Momentum should have higher base return than default
        let momentum_r = engine.generate_daily_return("Momentum", 0);
        let default_r = engine.generate_daily_return("Unknown", 0);
        // Both should be small daily values
        assert!(momentum_r.abs() < 0.1);
        assert!(default_r.abs() < 0.1);
    }

    #[test]
    fn test_calculate_return_statistics_empty_portfolio() {
        let engine = PaperHands::new();
        let portfolio = PaperPortfolio {
            id: Uuid::new_v4(),
            name: "Empty".to_string(),
            initial_balance: 100_000.0,
            current_balance: 100_000.0,
            allocated_balance: 0.0,
            available_balance: 100_000.0,
            positions: HashMap::new(),
            trades: Vec::new(),
            performance_metrics: PerformanceMetrics::default(),
            risk_metrics: RiskMetrics::default(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };
        let (mean, vol) = engine.calculate_return_statistics(&portfolio);
        // Should return defaults
        assert_eq!(mean, 0.08);
        assert_eq!(vol, 0.15);
    }

    #[test]
    fn test_calculate_monte_carlo_statistics_empty() {
        let engine = PaperHands::new();
        let analysis = engine.calculate_monte_carlo_statistics(Vec::new());
        assert!(analysis.simulations.is_empty());
        assert_eq!(analysis.probability_of_profit, 0.0);
    }

    #[test]
    fn test_calculate_monte_carlo_statistics_single() {
        let engine = PaperHands::new();
        let sims = vec![SimulationResult {
            final_value: 110_000.0,
            total_return: 10_000.0,
            max_drawdown: 0.05,
            sharpe_ratio: 1.5,
            win_rate: 0.55,
        }];
        let analysis = engine.calculate_monte_carlo_statistics(sims);
        assert!(!analysis.percentiles.is_empty());
    }

    #[test]
    fn test_calculate_monte_carlo_statistics_nan_filtered() {
        let engine = PaperHands::new();
        let sims = vec![
            SimulationResult {
                final_value: f64::NAN,
                total_return: 0.0,
                max_drawdown: 0.0,
                sharpe_ratio: 0.0,
                win_rate: 0.0,
            },
            SimulationResult {
                final_value: f64::INFINITY,
                total_return: 0.0,
                max_drawdown: 0.0,
                sharpe_ratio: 0.0,
                win_rate: 0.0,
            },
        ];
        let analysis = engine.calculate_monte_carlo_statistics(sims);
        // All values should be filtered, returning default
        assert!(analysis.percentiles.is_empty());
    }

    #[test]
    fn test_calculate_performance_metrics_no_trades() {
        let _engine = PaperHands::new();
        let portfolio = PaperPortfolio {
            id: Uuid::new_v4(),
            name: "Empty".to_string(),
            initial_balance: 100_000.0,
            current_balance: 100_000.0,
            allocated_balance: 0.0,
            available_balance: 100_000.0,
            positions: HashMap::new(),
            trades: Vec::new(),
            performance_metrics: PerformanceMetrics::default(),
            risk_metrics: RiskMetrics::default(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };
        let metrics = PaperHands::calculate_performance_metrics_static(&portfolio);
        assert_eq!(metrics.total_return, 0.0);
        assert_eq!(metrics.win_rate, 0.0);
        assert_eq!(metrics.total_trades, 0);
    }

    #[test]
    fn test_calculate_fees() {
        let engine = PaperHands::new();
        // Default: $5 per trade + 0.005 per share value
        let fees = engine.calculate_fees(100.0, 50.0);
        // 5.0 + (100 * 50 * 0.005) = 5.0 + 25.0 = 30.0
        assert!((fees - 30.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_order_side_variants() {
        let buy = OrderSide::Buy;
        let sell = OrderSide::Sell;
        assert!(matches!(buy, OrderSide::Buy));
        assert!(matches!(sell, OrderSide::Sell));
    }

    #[test]
    fn test_order_type_variants() {
        assert!(matches!(OrderType::Market, OrderType::Market));
        assert!(matches!(OrderType::Limit, OrderType::Limit));
        assert!(matches!(OrderType::Stop, OrderType::Stop));
        assert!(matches!(OrderType::StopLimit, OrderType::StopLimit));
        assert!(matches!(OrderType::TrailingStop, OrderType::TrailingStop));
    }

    #[test]
    fn test_trade_status_variants() {
        assert_eq!(TradeStatus::Closed, TradeStatus::Closed);
        assert_ne!(TradeStatus::Open, TradeStatus::Closed);
        assert_ne!(TradeStatus::Pending, TradeStatus::Open);
    }

    #[test]
    fn test_get_backtest_results_empty() {
        let engine = PaperHands::new();
        assert!(engine.get_backtest_results().is_empty());
    }

    #[test]
    fn test_get_backtest_result_not_found() {
        let engine = PaperHands::new();
        assert!(engine.get_backtest_result(&Uuid::new_v4()).is_none());
    }

    #[test]
    fn test_run_backtest() {
        let mut engine = PaperHands::new();
        let settings = SimulationSettings {
            initial_balance: 50_000.0,
            start_date: Utc::now() - Duration::days(30),
            end_date: Utc::now(),
            ..SimulationSettings::default()
        };
        let result = engine.run_backtest("Momentum".to_string(), settings);
        assert!(result.is_ok());
        let backtest_id = result.unwrap();
        let bt = engine.get_backtest_result(&backtest_id);
        assert!(bt.is_some());
        assert_eq!(bt.unwrap().strategy_name, "Momentum");
        assert!(!bt.unwrap().equity_curve.is_empty());
    }

    #[test]
    fn test_correlation_matrix_empty_portfolio() {
        let mut engine = PaperHands::new();
        let id = engine
            .create_portfolio("Test".to_string(), 100_000.0)
            .unwrap();
        let matrix = engine.calculate_correlation_matrix(id).unwrap();
        assert!(matrix.is_empty());
    }

    #[test]
    fn test_optimal_position_sizes_empty() {
        let mut engine = PaperHands::new();
        let id = engine
            .create_portfolio("Test".to_string(), 100_000.0)
            .unwrap();
        let sizes = engine.calculate_optimal_position_sizes(id).unwrap();
        assert!(sizes.is_empty());
    }

    #[test]
    fn test_monte_carlo_portfolio_not_found() {
        let engine = PaperHands::new();
        assert!(engine.run_monte_carlo(Uuid::new_v4(), 10).is_err());
    }
}
