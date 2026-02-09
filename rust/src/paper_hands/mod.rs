use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub var_95: f64, // Value at Risk 95%
    pub var_99: f64, // Value at Risk 99%
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
    pub fn create_portfolio(&mut self, name: String, initial_balance: f64) -> Result<Uuid, String> {
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

    pub fn execute_trade(&mut self, portfolio_id: Uuid, trade: PaperTrade) -> Result<(), String> {
        let portfolio = self.portfolios.get_mut(&portfolio_id)
            .ok_or("Portfolio not found")?;
        
        // Validate trade
        self.validate_trade(portfolio, &trade)?;
        
        // Execute trade logic
        self.process_trade(portfolio, trade)?;
        
        // Update portfolio metrics
        self.update_portfolio_metrics(portfolio);
        
        Ok(())
    }

    pub fn close_position(&mut self, portfolio_id: Uuid, symbol: &str, exit_price: f64) -> Result<(), String> {
        let portfolio = self.portfolios.get_mut(&portfolio_id)
            .ok_or("Portfolio not found")?;
        
        let position = portfolio.positions.get(symbol)
            .ok_or("Position not found")?;
        
        // Create closing trade
        let closing_trade = PaperTrade {
            id: Uuid::new_v4(),
            symbol: symbol.to_string(),
            side: if position.quantity > 0.0 { OrderSide::Sell } else { OrderSide::Buy },
            order_type: OrderType::Market,
            quantity: position.quantity.abs(),
            entry_price: position.entry_price,
            exit_price: Some(exit_price),
            stop_loss: None,
            take_profit: None,
            status: TradeStatus::Closed,
            entry_time: Utc::now(), // Would use actual entry time
            exit_time: Some(Utc::now()),
            pnl: Some(self.calculate_pnl(position, exit_price)),
            pnl_percentage: Some(self.calculate_pnl_percentage(position, exit_price)),
            fees: self.calculate_fees(position.quantity, exit_price),
            strategy: None,
            notes: None,
        };
        
        // Process closing trade
        self.process_trade(portfolio, closing_trade)?;
        
        // Remove position
        portfolio.positions.remove(symbol);
        
        // Update metrics
        self.update_portfolio_metrics(portfolio);
        
        Ok(())
    }

    // Backtesting
    pub fn run_backtest(&mut self, strategy_name: String, settings: SimulationSettings) -> Result<Uuid, String> {
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

    pub fn run_monte_carlo(&self, portfolio_id: Uuid, num_simulations: u32) -> Result<MonteCarloAnalysis, String> {
        let portfolio = self.portfolios.get(&portfolio_id)
            .ok_or("Portfolio not found")?;
        
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
    pub fn calculate_correlation_matrix(&self, portfolio_id: Uuid) -> Result<HashMap<String, HashMap<String, f64>>, String> {
        let portfolio = self.portfolios.get(&portfolio_id)
            .ok_or("Portfolio not found")?;
        
        let mut correlation_matrix = HashMap::new();
        let symbols: Vec<String> = portfolio.positions.keys().cloned().collect();
        
        for symbol_i in &symbols {
            let mut correlations = HashMap::new();
            
            for symbol_j in &symbols {
                let correlation = self.calculate_symbol_correlation(portfolio, symbol_i, symbol_j);
                correlations.insert(symbol_j.clone(), correlation);
            }
            
            correlation_matrix.insert(symbol_i.clone(), correlations);
        }
        
        Ok(correlation_matrix)
    }

    pub fn calculate_optimal_position_sizes(&self, portfolio_id: Uuid) -> Result<HashMap<String, f64>, String> {
        let portfolio = self.portfolios.get(&portfolio_id)
            .ok_or("Portfolio not found")?;
        
        let mut optimal_sizes = HashMap::new();
        
        for (symbol, position) in &portfolio.positions {
            // Kelly Criterion
            let win_rate = portfolio.performance_metrics.win_rate;
            let avg_win = portfolio.performance_metrics.average_win;
            let avg_loss = portfolio.performance_metrics.average_loss;
            
            let kelly_percentage = if avg_loss != 0.0 {
                (win_rate * avg_win - (1.0 - win_rate) * avg_loss.abs()) / avg_loss.abs()
            } else {
                0.0
            };
            
            // Apply position sizing limits
            let max_position = portfolio.current_balance * 0.25; // Max 25% per position
            let optimal_size = (kelly_percentage * portfolio.current_balance).min(max_position);
            
            optimal_sizes.insert(symbol.clone(), optimal_size);
        }
        
        Ok(optimal_sizes)
    }

    // Helper methods
    fn validate_trade(&self, portfolio: &PaperPortfolio, trade: &PaperTrade) -> Result<(), String> {
        // Check if sufficient balance
        let required_balance = trade.quantity * trade.entry_price + self.calculate_fees(trade.quantity, trade.entry_price);
        
        if required_balance > portfolio.available_balance {
            return Err("Insufficient balance for trade".to_string());
        }
        
        // Check position size limits
        let max_position_size = portfolio.current_balance * 0.5; // Max 50% per position
        if required_balance > max_position_size {
            return Err("Trade exceeds maximum position size".to_string());
        }
        
        Ok(())
    }

    fn process_trade(&mut self, portfolio: &mut PaperPortfolio, trade: PaperTrade) -> Result<(), String> {
        let fees = self.calculate_fees(trade.quantity, trade.entry_price);
        let total_cost = trade.quantity * trade.entry_price + fees;
        
        // Update balance
        portfolio.current_balance -= total_cost;
        portfolio.allocated_balance += total_cost;
        
        // Update or create position
        let symbol = trade.symbol.clone();
        if let Some(position) = portfolio.positions.get_mut(&symbol) {
            // Update existing position
            self.update_existing_position(position, &trade);
        } else {
            // Create new position
            portfolio.positions.insert(symbol.clone(), self.create_new_position(&trade));
        }
        
        // Add trade to history
        portfolio.trades.push(trade);
        portfolio.last_updated = Utc::now();
        
        Ok(())
    }

    fn update_existing_position(&self, position: &mut PaperPosition, trade: &PaperTrade) {
        let old_quantity = position.quantity;
        let old_avg_price = position.average_entry_price;
        
        // Calculate new average price
        let new_quantity = old_quantity + trade.quantity;
        let new_avg_price = (old_quantity * old_avg_price + trade.quantity * trade.entry_price) / new_quantity;
        
        position.quantity = new_quantity;
        position.average_entry_price = new_avg_price;
        position.trades_count += 1;
        position.last_updated = Utc::now();
        
        // Update unrealized P&L
        if let Some(current_price) = self.get_current_price(&position.symbol) {
            position.current_price = current_price;
            position.unrealized_pnl = self.calculate_unrealized_pnl(position, current_price);
            position.unrealized_pnl_percentage = position.unrealized_pnl / (position.quantity * position.average_entry_price);
        }
    }

    fn create_new_position(&self, trade: &PaperTrade) -> PaperPosition {
        let current_price = self.get_current_price(&trade.symbol).unwrap_or(trade.entry_price);
        
        PaperPosition {
            symbol: trade.symbol.clone(),
            quantity: trade.quantity,
            entry_price: trade.entry_price,
            current_price,
            unrealized_pnl: 0.0,
            unrealized_pnl_percentage: 0.0,
            realized_pnl: 0.0,
            total_fees: self.calculate_fees(trade.quantity, trade.entry_price),
            trades_count: 1,
            average_entry_price: trade.entry_price,
            last_updated: Utc::now(),
        }
    }

    fn calculate_pnl(&self, position: &PaperPosition, exit_price: f64) -> f64 {
        let pnl = (exit_price - position.entry_price) * position.quantity;
        pnl - position.total_fees
    }

    fn calculate_pnl_percentage(&self, position: &PaperPosition, exit_price: f64) -> f64 {
        let total_cost = position.entry_price * position.quantity;
        if total_cost == 0.0 {
            return 0.0;
        }
        ((exit_price - position.entry_price) * position.quantity) / total_cost
    }

    fn calculate_unrealized_pnl(&self, position: &PaperPosition, current_price: f64) -> f64 {
        (current_price - position.entry_price) * position.quantity
    }

    fn calculate_fees(&self, quantity: f64, price: f64) -> f64 {
        let trade_value = quantity * price;
        self.simulation_settings.commission_per_trade + 
        (trade_value * self.simulation_settings.commission_per_share)
    }

    fn update_portfolio_metrics(&mut self, portfolio_id: Uuid) {
        if let Some(portfolio) = self.portfolios.get_mut(&portfolio_id) {
            portfolio.performance_metrics = self.calculate_performance_metrics(portfolio);
            portfolio.risk_metrics = self.calculate_risk_metrics(portfolio);
        }
    }

    fn calculate_performance_metrics(&self, portfolio: &PaperPortfolio) -> PerformanceMetrics {
        let total_return = portfolio.current_balance - portfolio.initial_balance;
        let total_return_percentage = total_return / portfolio.initial_balance;
        
        let closed_trades: Vec<_> = portfolio.trades.iter()
            .filter(|t| t.status == TradeStatus::Closed)
            .collect();
        
        let winning_trades = closed_trades.iter()
            .filter(|t| t.pnl.unwrap_or(0.0) > 0.0)
            .count();
        
        let losing_trades = closed_trades.iter()
            .filter(|t| t.pnl.unwrap_or(0.0) < 0.0)
            .count();
        
        let win_rate = if closed_trades.is_empty() {
            0.0
        } else {
            winning_trades as f64 / closed_trades.len() as f64
        };
        
        let total_wins: f64 = closed_trades.iter()
            .filter(|t| t.pnl.unwrap_or(0.0) > 0.0)
            .map(|t| t.pnl.unwrap_or(0.0))
            .sum();
        
        let total_losses: f64 = closed_trades.iter()
            .filter(|t| t.pnl.unwrap_or(0.0) < 0.0)
            .map(|t| t.pnl.unwrap_or(0.0).abs())
            .sum();
        
        let profit_factor = if total_losses == 0.0 {
            if total_wins > 0.0 { f64::INFINITY } else { 0.0 }
        } else {
            total_wins / total_losses
        };
        
        PerformanceMetrics {
            total_return,
            total_return_percentage,
            annualized_return: 0.0, // Would calculate based on time period
            win_rate,
            profit_factor,
            sharpe_ratio: 0.0, // Would calculate using risk-free rate
            sortino_ratio: 0.0, // Would calculate using downside deviation
            max_drawdown: 0.0, // Would calculate from equity curve
            max_drawdown_percentage: 0.0,
            calmar_ratio: 0.0, // Would calculate using max drawdown
            total_trades: closed_trades.len() as u32,
            winning_trades: winning_trades as u32,
            losing_trades: losing_trades as u32,
            average_win: if winning_trades > 0 { total_wins / winning_trades as f64 } else { 0.0 },
            average_loss: if losing_trades > 0 { total_losses / losing_trades as f64 } else { 0.0 },
            largest_win: closed_trades.iter().map(|t| t.pnl.unwrap_or(0.0)).fold(0.0, f64::max),
            largest_loss: closed_trades.iter().map(|t| t.pnl.unwrap_or(0.0)).fold(0.0, f64::min),
            average_trade_duration: Duration::zero(), // Would calculate from trade durations
            best_trade_duration: Duration::zero(),
            worst_trade_duration: Duration::zero(),
        }
    }

    fn calculate_risk_metrics(&self, portfolio: &PaperPortfolio) -> RiskMetrics {
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

    fn simulate_trading(&mut self, strategy_name: String, settings: SimulationSettings, portfolio_id: Uuid) -> Result<BacktestResult, String> {
        // TODO: Implement actual trading simulation
        // For now, return a placeholder result
        
        let portfolio = self.portfolios.get(&portfolio_id).unwrap();
        
        Ok(BacktestResult {
            id: Uuid::new_v4(),
            strategy_name,
            settings,
            portfolio: portfolio.clone(),
            equity_curve: Vec::new(),
            trade_analysis: TradeAnalysis::default(),
            benchmark_comparison: BenchmarkComparison::default(),
            monte_carlo_analysis: MonteCarloAnalysis::default(),
            created_at: Utc::now(),
        })
    }

    fn run_single_simulation(&self, portfolio: &PaperPortfolio) -> Result<SimulationResult, String> {
        // TODO: Implement Monte Carlo simulation
        Ok(SimulationResult {
            final_value: portfolio.current_balance,
            total_return: portfolio.performance_metrics.total_return,
            max_drawdown: portfolio.performance_metrics.max_drawdown,
            sharpe_ratio: portfolio.performance_metrics.sharpe_ratio,
            win_rate: portfolio.performance_metrics.win_rate,
        })
    }

    fn calculate_monte_carlo_statistics(&self, simulations: Vec<SimulationResult>) -> MonteCarloAnalysis {
        // TODO: Implement Monte Carlo statistics calculation
        MonteCarloAnalysis {
            simulations,
            percentiles: HashMap::new(),
            probability_of_profit: 0.0,
            expected_shortfall: 0.0,
            var_95: 0.0,
            var_99: 0.0,
        }
    }

    fn calculate_symbol_correlation(&self, portfolio: &PaperPortfolio, symbol1: &str, symbol2: &str) -> f64 {
        // TODO: Implement correlation calculation
        0.0
    }

    fn get_current_price(&self, symbol: &str) -> Option<f64> {
        // TODO: Get current price from market data
        Some(150.0) // Placeholder
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