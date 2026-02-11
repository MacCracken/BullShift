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

    pub fn calculate_optimal_position_sizes(&self, portfolio_id: Uuid) -> Result<HashMap<String, f64>, String> {
        let portfolio = self.portfolios.get(&portfolio_id)
            .ok_or("Portfolio not found")?;
        
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
        let symbol = &trade.symbol;
        
        // Update balance
        portfolio.current_balance -= total_cost;
        portfolio.allocated_balance += total_cost;
        
        // Update or create position
        match portfolio.positions.get_mut(symbol) {
            Some(position) => {
                // Update existing position
                self.update_existing_position(position, &trade);
            }
            None => {
                // Create new position - only clone symbol once
                let new_position = self.create_new_position(&trade);
                portfolio.positions.insert(trade.symbol.clone(), new_position);
            }
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
        let portfolio = self.portfolios.get(&portfolio_id)
            .ok_or("Portfolio not found")?;
        
        // Generate equity curve based on simulated trades
        let mut equity_curve = Vec::new();
        let mut portfolio_value = settings.initial_balance;
        let mut max_portfolio_value = portfolio_value;
        let mut max_drawdown = 0.0;
        
        let duration = settings.end_date.signed_duration_since(settings.start_date);
        let days = duration.num_days().max(1);
        
        // Generate daily equity points
        for day in 0..=days {
            let timestamp = settings.start_date + Duration::days(day);
            
            // Simulate daily returns using random walk with drift
            let daily_return = self.generate_daily_return(&strategy_name, day);
            portfolio_value *= (1.0 + daily_return);
            
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
        use std::f64::consts::PI;
        
        let base_return = match strategy_name.as_str() {
            "Momentum" => 0.0005,      // 0.05% daily
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
    
    fn generate_trade_analysis(&self, equity_curve: &[EquityPoint], settings: &SimulationSettings) -> TradeAnalysis {
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
        
        let total_return = equity_curve.last().map(|e| e.portfolio_value).unwrap_or(settings.initial_balance) - settings.initial_balance;
        let avg_win = if winning_trades > 0 { total_return * 0.8 / winning_trades as f64 } else { 0.0 };
        let avg_loss = if losing_trades > 0 { -total_return * 0.2 / losing_trades as f64 } else { 0.0 };
        
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
            let start_value = equity_curve.get(start_idx).map(|e| e.portfolio_value).unwrap_or(settings.initial_balance);
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
    
    fn generate_benchmark_comparison(&self, equity_curve: &[EquityPoint], settings: &SimulationSettings) -> BenchmarkComparison {
        // Generate simulated S&P 500 returns
        let mut benchmark_returns = Vec::new();
        let mut portfolio_returns = Vec::new();
        
        for i in 1..equity_curve.len() {
            let portfolio_return = (equity_curve[i].portfolio_value - equity_curve[i-1].portfolio_value) 
                / equity_curve[i-1].portfolio_value;
            // Benchmark has slightly lower returns with less volatility
            let benchmark_return = portfolio_return * 0.8 + 0.0002;
            
            portfolio_returns.push(portfolio_return);
            benchmark_returns.push(benchmark_return);
        }
        
        // Calculate correlation
        let correlation = self.calculate_correlation(&portfolio_returns, &benchmark_returns);
        
        // Calculate beta (portfolio sensitivity to benchmark)
        let benchmark_variance = self.calculate_variance(&benchmark_returns);
        let beta = if benchmark_variance > 0.0 {
            self.calculate_covariance(&portfolio_returns, &benchmark_returns) / benchmark_variance
        } else {
            1.0
        };
        
        // Calculate alpha (excess return)
        let avg_portfolio_return = portfolio_returns.iter().sum::<f64>() / portfolio_returns.len() as f64;
        let avg_benchmark_return = benchmark_returns.iter().sum::<f64>() / benchmark_returns.len() as f64;
        let alpha = avg_portfolio_return - beta * avg_benchmark_return;
        
        BenchmarkComparison {
            benchmark_name: "S&P 500".to_string(),
            benchmark_returns,
            portfolio_returns,
            correlation,
            beta,
            alpha,
            information_ratio: alpha / self.calculate_standard_deviation(&portfolio_returns),
            tracking_error: 0.0,
            up_capture: 1.0,
            down_capture: 0.8,
        }
    }

    fn run_single_simulation(&self, portfolio: &PaperPortfolio) -> Result<SimulationResult, String> {
        // Monte Carlo simulation using geometric Brownian motion
        let initial_value = portfolio.current_balance;
        let days = 252; // Trading days in a year
        
        // Calculate historical returns and volatility from portfolio trades
        let (mean_return, volatility) = self.calculate_return_statistics(portfolio);
        
        // Run simulation
        let mut current_value = initial_value;
        let mut max_value = initial_value;
        let mut max_drawdown = 0.0;
        let mut total_return = 0.0;
        
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
        let returns: Vec<f64> = trades.iter()
            .filter_map(|t| t.pnl_percentage)
            .collect();
        
        if returns.is_empty() {
            return (0.08, 0.15);
        }
        
        // Calculate mean
        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        
        // Calculate standard deviation
        let variance = returns.iter()
            .map(|r| (r - mean).powi(2))
            .sum::<f64>() / returns.len() as f64;
        let std_dev = variance.sqrt();
        
        (mean, std_dev)
    }

    fn calculate_monte_carlo_statistics(&self, simulations: Vec<SimulationResult>) -> MonteCarloAnalysis {
        if simulations.is_empty() {
            return MonteCarloAnalysis::default();
        }
        
        // Extract final values for percentile calculations
        let mut final_values: Vec<f64> = simulations.iter()
            .map(|s| s.final_value)
            .collect();
        final_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        // Calculate percentiles
        let mut percentiles = HashMap::new();
        for percentile in [5, 10, 25, 50, 75, 90, 95] {
            let index = ((percentile as f64 / 100.0) * (final_values.len() - 1) as f64) as usize;
            percentiles.insert(percentile, final_values[index.min(final_values.len() - 1)]);
        }
        
        // Calculate probability of profit
        let initial_value = simulations.first().map(|s| s.final_value).unwrap_or(1.0);
        let profitable_simulations = simulations.iter()
            .filter(|s| s.final_value > initial_value)
            .count();
        let probability_of_profit = profitable_simulations as f64 / simulations.len() as f64;
        
        // Calculate VaR (Value at Risk)
        let var_95_index = (0.05 * final_values.len() as f64) as usize;
        let var_99_index = (0.01 * final_values.len() as f64) as usize;
        let var_95 = final_values.get(var_95_index).copied().unwrap_or(0.0);
        let var_99 = final_values.get(var_99_index).copied().unwrap_or(0.0);
        
        // Calculate Expected Shortfall (CVaR)
        let expected_shortfall = final_values.iter()
            .take(var_95_index)
            .sum::<f64>() / var_95_index.max(1) as f64;
        
        MonteCarloAnalysis {
            simulations,
            percentiles,
            probability_of_profit,
            expected_shortfall,
            var_95,
            var_99,
        }
    }
    
    fn run_monte_carlo_simulation(&self, equity_curve: &[EquityPoint], num_simulations: u32) -> Result<MonteCarloAnalysis, String> {
        let mut simulations = Vec::new();
        
        // Calculate historical returns and volatility from equity curve
        let returns: Vec<f64> = equity_curve.windows(2)
            .map(|w| (w[1].portfolio_value - w[0].portfolio_value) / w[0].portfolio_value)
            .collect();
        
        if returns.is_empty() {
            return Ok(MonteCarloAnalysis::default());
        }
        
        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns.iter()
            .map(|r| (r - mean_return).powi(2))
            .sum::<f64>() / returns.len() as f64;
        let volatility = variance.sqrt();
        
        // Get initial value
        let initial_value = equity_curve.first()
            .map(|e| e.portfolio_value)
            .unwrap_or(100000.0);
        
        // Run multiple simulations
        for _ in 0..num_simulations {
            let mut current_value = initial_value;
            let mut max_value = current_value;
            let mut max_drawdown = 0.0;
            
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
            let annual_volatility = volatility * (252.0 as f64).sqrt();
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

    fn calculate_symbol_correlation(&self, portfolio: &PaperPortfolio, symbol1: &str, symbol2: &str) -> f64 {
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
        let trades1: Vec<&PaperTrade> = portfolio.trades
            .iter()
            .filter(|t| t.symbol == symbol1 && t.status == TradeStatus::Closed)
            .collect();
        let trades2: Vec<&PaperTrade> = portfolio.trades
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
        let returns1: Vec<f64> = trades1.iter()
            .filter_map(|t| t.pnl_percentage)
            .collect();
        let returns2: Vec<f64> = trades2.iter()
            .filter_map(|t| t.pnl_percentage)
            .collect();

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

        x.iter().zip(y.iter())
            .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
            .sum::<f64>() / n
    }

    fn calculate_variance(&self, data: &[f64]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let n = data.len() as f64;
        let mean = data.iter().sum::<f64>() / n;

        data.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / n
    }

    fn calculate_standard_deviation(&self, data: &[f64]) -> f64 {
        self.calculate_variance(data).sqrt()
    }

    fn get_current_price(&self, symbol: &str) -> Option<f64> {
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

    fn generate_symbol_base_price(&self, symbol: &str) -> f64 {
        // Generate a deterministic base price from symbol
        let hash = symbol.bytes().fold(0u64, |acc, b| {
            acc.wrapping_mul(31).wrapping_add(b as u64)
        });
        
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