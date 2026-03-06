use std::collections::HashMap;

// ---------------------------------------------------------------------------
// IndicatorType
// ---------------------------------------------------------------------------

/// Categorizes indicators by their analytical purpose.
#[derive(Debug, Clone, PartialEq)]
pub enum IndicatorType {
    Trend,
    Momentum,
    Volatility,
    Volume,
    Oscillator,
    Custom(String),
}

impl std::fmt::Display for IndicatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndicatorType::Trend => write!(f, "Trend"),
            IndicatorType::Momentum => write!(f, "Momentum"),
            IndicatorType::Volatility => write!(f, "Volatility"),
            IndicatorType::Volume => write!(f, "Volume"),
            IndicatorType::Oscillator => write!(f, "Oscillator"),
            IndicatorType::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

// ---------------------------------------------------------------------------
// IndicatorParams
// ---------------------------------------------------------------------------

/// A thin wrapper around `HashMap<String, f64>` that provides a builder API
/// for configuring indicator parameters.
#[derive(Debug, Clone, Default)]
pub struct IndicatorParams {
    params: HashMap<String, f64>,
}

impl IndicatorParams {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    /// Builder-style setter.
    pub fn with(mut self, key: &str, value: f64) -> Self {
        self.params.insert(key.to_string(), value);
        self
    }

    pub fn get(&self, key: &str) -> Option<f64> {
        self.params.get(key).copied()
    }

    pub fn get_or(&self, key: &str, default: f64) -> f64 {
        self.params.get(key).copied().unwrap_or(default)
    }
}

// ---------------------------------------------------------------------------
// Indicator trait
// ---------------------------------------------------------------------------

/// Common interface every indicator must implement.
pub trait Indicator {
    /// Full human-readable name (e.g. "Simple Moving Average").
    fn name(&self) -> &str;

    /// Abbreviated name (e.g. "SMA").
    fn short_name(&self) -> &str;

    /// The analytical category of this indicator.
    fn indicator_type(&self) -> IndicatorType;

    /// Minimum number of data points before the indicator produces output.
    fn min_periods(&self) -> usize;

    /// Feed a new data point into the indicator.
    fn update(&mut self, value: f64);

    /// Current indicator value, or `None` if not enough data has been fed.
    fn value(&self) -> Option<f64>;

    /// Whether enough data has been fed.
    fn is_ready(&self) -> bool;

    /// Clear all internal state.
    fn reset(&mut self);
}

// ---------------------------------------------------------------------------
// SMA — Simple Moving Average
// ---------------------------------------------------------------------------

pub struct Sma {
    period: usize,
    buffer: Vec<f64>,
    index: usize,
    count: usize,
    sum: f64,
}

impl Sma {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            buffer: vec![0.0; period],
            index: 0,
            count: 0,
            sum: 0.0,
        }
    }
}

impl Indicator for Sma {
    fn name(&self) -> &str {
        "Simple Moving Average"
    }
    fn short_name(&self) -> &str {
        "SMA"
    }
    fn indicator_type(&self) -> IndicatorType {
        IndicatorType::Trend
    }
    fn min_periods(&self) -> usize {
        self.period
    }

    fn update(&mut self, value: f64) {
        if self.count >= self.period {
            self.sum -= self.buffer[self.index];
        }
        self.buffer[self.index] = value;
        self.sum += value;
        self.index = (self.index + 1) % self.period;
        if self.count < self.period {
            self.count += 1;
        }
    }

    fn value(&self) -> Option<f64> {
        if self.is_ready() {
            Some(self.sum / self.period as f64)
        } else {
            None
        }
    }

    fn is_ready(&self) -> bool {
        self.count >= self.period
    }

    fn reset(&mut self) {
        self.buffer = vec![0.0; self.period];
        self.index = 0;
        self.count = 0;
        self.sum = 0.0;
    }
}

// ---------------------------------------------------------------------------
// EMA — Exponential Moving Average
// ---------------------------------------------------------------------------

pub struct Ema {
    period: usize,
    multiplier: f64,
    current: Option<f64>,
    count: usize,
    /// Accumulates sum of the first `period` values to seed the EMA.
    seed_sum: f64,
}

impl Ema {
    pub fn new(period: usize) -> Self {
        let multiplier = 2.0 / (period as f64 + 1.0);
        Self {
            period,
            multiplier,
            current: None,
            count: 0,
            seed_sum: 0.0,
        }
    }
}

impl Indicator for Ema {
    fn name(&self) -> &str {
        "Exponential Moving Average"
    }
    fn short_name(&self) -> &str {
        "EMA"
    }
    fn indicator_type(&self) -> IndicatorType {
        IndicatorType::Trend
    }
    fn min_periods(&self) -> usize {
        self.period
    }

    fn update(&mut self, value: f64) {
        self.count += 1;
        if self.count < self.period {
            self.seed_sum += value;
        } else if self.count == self.period {
            self.seed_sum += value;
            self.current = Some(self.seed_sum / self.period as f64);
        } else if let Some(prev) = self.current {
            self.current = Some((value - prev) * self.multiplier + prev);
        }
    }

    fn value(&self) -> Option<f64> {
        self.current
    }

    fn is_ready(&self) -> bool {
        self.count >= self.period
    }

    fn reset(&mut self) {
        self.current = None;
        self.count = 0;
        self.seed_sum = 0.0;
    }
}

// ---------------------------------------------------------------------------
// RSI — Relative Strength Index
// ---------------------------------------------------------------------------

pub struct Rsi {
    period: usize,
    count: usize,
    prev_value: Option<f64>,
    avg_gain: f64,
    avg_loss: f64,
    /// Accumulates initial gains during the seed window.
    seed_gains: f64,
    /// Accumulates initial losses during the seed window.
    seed_losses: f64,
}

impl Rsi {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            count: 0,
            prev_value: None,
            avg_gain: 0.0,
            avg_loss: 0.0,
            seed_gains: 0.0,
            seed_losses: 0.0,
        }
    }
}

impl Indicator for Rsi {
    fn name(&self) -> &str {
        "Relative Strength Index"
    }
    fn short_name(&self) -> &str {
        "RSI"
    }
    fn indicator_type(&self) -> IndicatorType {
        IndicatorType::Momentum
    }
    fn min_periods(&self) -> usize {
        // Need period + 1 values (period changes).
        self.period + 1
    }

    fn update(&mut self, value: f64) {
        if let Some(prev) = self.prev_value {
            let change = value - prev;
            let gain = if change > 0.0 { change } else { 0.0 };
            let loss = if change < 0.0 { -change } else { 0.0 };

            self.count += 1;
            let p = self.period as f64;

            if self.count < self.period {
                self.seed_gains += gain;
                self.seed_losses += loss;
            } else if self.count == self.period {
                self.seed_gains += gain;
                self.seed_losses += loss;
                self.avg_gain = self.seed_gains / p;
                self.avg_loss = self.seed_losses / p;
            } else {
                self.avg_gain = (self.avg_gain * (p - 1.0) + gain) / p;
                self.avg_loss = (self.avg_loss * (p - 1.0) + loss) / p;
            }
        }
        self.prev_value = Some(value);
    }

    fn value(&self) -> Option<f64> {
        if !self.is_ready() {
            return None;
        }
        if self.avg_loss == 0.0 {
            return Some(100.0);
        }
        let rs = self.avg_gain / self.avg_loss;
        Some(100.0 - 100.0 / (1.0 + rs))
    }

    fn is_ready(&self) -> bool {
        self.count >= self.period
    }

    fn reset(&mut self) {
        self.count = 0;
        self.prev_value = None;
        self.avg_gain = 0.0;
        self.avg_loss = 0.0;
        self.seed_gains = 0.0;
        self.seed_losses = 0.0;
    }
}

// ---------------------------------------------------------------------------
// MACD — Moving Average Convergence Divergence
// ---------------------------------------------------------------------------

pub struct Macd {
    fast_ema: Ema,
    slow_ema: Ema,
    signal_ema: Ema,
    count: usize,
    slow_period: usize,
    signal_period: usize,
}

impl Macd {
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        Self {
            fast_ema: Ema::new(fast_period),
            slow_ema: Ema::new(slow_period),
            signal_ema: Ema::new(signal_period),
            count: 0,
            slow_period,
            signal_period,
        }
    }

    /// The MACD line: fast EMA minus slow EMA.
    pub fn macd_line(&self) -> Option<f64> {
        match (self.fast_ema.value(), self.slow_ema.value()) {
            (Some(f), Some(s)) => Some(f - s),
            _ => None,
        }
    }

    /// The signal line: EMA of the MACD line.
    pub fn signal_line(&self) -> Option<f64> {
        self.signal_ema.value()
    }

    /// Histogram: MACD line minus signal line.
    pub fn histogram(&self) -> Option<f64> {
        match (self.macd_line(), self.signal_line()) {
            (Some(m), Some(s)) => Some(m - s),
            _ => None,
        }
    }
}

impl Indicator for Macd {
    fn name(&self) -> &str {
        "Moving Average Convergence Divergence"
    }
    fn short_name(&self) -> &str {
        "MACD"
    }
    fn indicator_type(&self) -> IndicatorType {
        IndicatorType::Trend
    }
    fn min_periods(&self) -> usize {
        self.slow_period + self.signal_period - 1
    }

    fn update(&mut self, value: f64) {
        self.fast_ema.update(value);
        self.slow_ema.update(value);
        self.count += 1;

        // Once both EMAs are ready, feed the MACD line into the signal EMA.
        if let Some(macd) = self.macd_line() {
            self.signal_ema.update(macd);
        }
    }

    fn value(&self) -> Option<f64> {
        self.macd_line()
    }

    fn is_ready(&self) -> bool {
        self.signal_ema.is_ready()
    }

    fn reset(&mut self) {
        self.fast_ema.reset();
        self.slow_ema.reset();
        self.signal_ema.reset();
        self.count = 0;
    }
}

// ---------------------------------------------------------------------------
// BollingerBands
// ---------------------------------------------------------------------------

pub struct BollingerBands {
    period: usize,
    num_std_dev: f64,
    buffer: Vec<f64>,
    index: usize,
    count: usize,
    sum: f64,
}

impl BollingerBands {
    pub fn new(period: usize, num_std_dev: f64) -> Self {
        Self {
            period,
            num_std_dev,
            buffer: vec![0.0; period],
            index: 0,
            count: 0,
            sum: 0.0,
        }
    }

    /// Upper Bollinger Band.
    pub fn upper(&self) -> Option<f64> {
        self.middle().map(|m| m + self.num_std_dev * self.std_dev())
    }

    /// Middle band (SMA).
    pub fn middle(&self) -> Option<f64> {
        if self.is_ready() {
            Some(self.sum / self.period as f64)
        } else {
            None
        }
    }

    /// Lower Bollinger Band.
    pub fn lower(&self) -> Option<f64> {
        self.middle().map(|m| m - self.num_std_dev * self.std_dev())
    }

    fn std_dev(&self) -> f64 {
        let mean = self.sum / self.period as f64;
        let variance: f64 = self
            .buffer
            .iter()
            .map(|v| {
                let diff = v - mean;
                diff * diff
            })
            .sum::<f64>()
            / self.period as f64;
        variance.sqrt()
    }
}

impl Indicator for BollingerBands {
    fn name(&self) -> &str {
        "Bollinger Bands"
    }
    fn short_name(&self) -> &str {
        "BB"
    }
    fn indicator_type(&self) -> IndicatorType {
        IndicatorType::Volatility
    }
    fn min_periods(&self) -> usize {
        self.period
    }

    fn update(&mut self, value: f64) {
        if self.count >= self.period {
            self.sum -= self.buffer[self.index];
        }
        self.buffer[self.index] = value;
        self.sum += value;
        self.index = (self.index + 1) % self.period;
        if self.count < self.period {
            self.count += 1;
        }
    }

    fn value(&self) -> Option<f64> {
        self.middle()
    }

    fn is_ready(&self) -> bool {
        self.count >= self.period
    }

    fn reset(&mut self) {
        self.buffer = vec![0.0; self.period];
        self.index = 0;
        self.count = 0;
        self.sum = 0.0;
    }
}

// ---------------------------------------------------------------------------
// ATR — Average True Range
// ---------------------------------------------------------------------------

pub struct Atr {
    period: usize,
    count: usize,
    prev_close: Option<f64>,
    atr_value: Option<f64>,
    /// Accumulates true-range values for the initial seed window.
    seed_sum: f64,
}

impl Atr {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            count: 0,
            prev_close: None,
            atr_value: None,
            seed_sum: 0.0,
        }
    }

    /// Feed high, low, close data. This is the primary update method for ATR.
    pub fn update_hlc(&mut self, high: f64, low: f64, close: f64) {
        let tr = if let Some(pc) = self.prev_close {
            let hl = high - low;
            let hc = (high - pc).abs();
            let lc = (low - pc).abs();
            hl.max(hc).max(lc)
        } else {
            high - low
        };

        self.prev_close = Some(close);
        self.count += 1;
        let p = self.period as f64;

        if self.count < self.period {
            self.seed_sum += tr;
        } else if self.count == self.period {
            self.seed_sum += tr;
            self.atr_value = Some(self.seed_sum / p);
        } else if let Some(prev_atr) = self.atr_value {
            self.atr_value = Some((prev_atr * (p - 1.0) + tr) / p);
        }
    }
}

impl Indicator for Atr {
    fn name(&self) -> &str {
        "Average True Range"
    }
    fn short_name(&self) -> &str {
        "ATR"
    }
    fn indicator_type(&self) -> IndicatorType {
        IndicatorType::Volatility
    }
    fn min_periods(&self) -> usize {
        self.period
    }

    /// No-op for ATR — use [`Atr::update_hlc`] instead.
    fn update(&mut self, _value: f64) {
        // ATR requires high/low/close data; single-value update is a no-op.
    }

    fn value(&self) -> Option<f64> {
        self.atr_value
    }

    fn is_ready(&self) -> bool {
        self.count >= self.period
    }

    fn reset(&mut self) {
        self.count = 0;
        self.prev_close = None;
        self.atr_value = None;
        self.seed_sum = 0.0;
    }
}

// ---------------------------------------------------------------------------
// Stochastic Oscillator
// ---------------------------------------------------------------------------

pub struct Stochastic {
    k_period: usize,
    d_period: usize,
    highs: Vec<f64>,
    lows: Vec<f64>,
    closes: Vec<f64>,
    k_values: Vec<f64>,
}

impl Stochastic {
    pub fn new(k_period: usize, d_period: usize) -> Self {
        Self {
            k_period,
            d_period,
            highs: Vec::new(),
            lows: Vec::new(),
            closes: Vec::new(),
            k_values: Vec::new(),
        }
    }

    /// Feed a high/low/close bar and update the oscillator.
    pub fn update_hlc(&mut self, high: f64, low: f64, close: f64) {
        self.highs.push(high);
        self.lows.push(low);
        self.closes.push(close);

        if self.highs.len() >= self.k_period {
            let start = self.highs.len() - self.k_period;
            let highest = self.highs[start..]
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max);
            let lowest = self.lows[start..]
                .iter()
                .copied()
                .fold(f64::INFINITY, f64::min);

            let k = if (highest - lowest).abs() < f64::EPSILON {
                50.0
            } else {
                (close - lowest) / (highest - lowest) * 100.0
            };
            self.k_values.push(k);
        }
    }

    /// The fast %K value.
    pub fn k_value(&self) -> Option<f64> {
        self.k_values.last().copied()
    }

    /// The %D value (SMA of the last `d_period` %K values).
    pub fn d_value(&self) -> Option<f64> {
        if self.k_values.len() >= self.d_period {
            let start = self.k_values.len() - self.d_period;
            let sum: f64 = self.k_values[start..].iter().sum();
            Some(sum / self.d_period as f64)
        } else {
            None
        }
    }
}

impl Indicator for Stochastic {
    fn name(&self) -> &str {
        "Stochastic Oscillator"
    }
    fn short_name(&self) -> &str {
        "STOCH"
    }
    fn indicator_type(&self) -> IndicatorType {
        IndicatorType::Oscillator
    }
    fn min_periods(&self) -> usize {
        self.k_period + self.d_period - 1
    }

    /// Convenience single-value update: treats value as close with high=low=close.
    fn update(&mut self, value: f64) {
        self.update_hlc(value, value, value);
    }

    fn value(&self) -> Option<f64> {
        self.k_value()
    }

    fn is_ready(&self) -> bool {
        self.k_values.len() >= self.d_period
    }

    fn reset(&mut self) {
        self.highs.clear();
        self.lows.clear();
        self.closes.clear();
        self.k_values.clear();
    }
}

// ---------------------------------------------------------------------------
// IndicatorRegistry
// ---------------------------------------------------------------------------

type IndicatorFactory = Box<dyn Fn(&IndicatorParams) -> Box<dyn Indicator + Send> + Send>;

/// A registry of named indicator factories. Pre-registers all built-in
/// indicators and allows custom indicators to be registered at runtime.
pub struct IndicatorRegistry {
    factories: HashMap<String, IndicatorFactory>,
}

impl IndicatorRegistry {
    /// Create a new registry pre-loaded with all built-in indicators.
    pub fn new() -> Self {
        let mut reg = Self {
            factories: HashMap::new(),
        };

        reg.factories.insert(
            "SMA".to_string(),
            Box::new(|params: &IndicatorParams| {
                let period = params.get_or("period", 14.0) as usize;
                Box::new(Sma::new(period))
            }),
        );

        reg.factories.insert(
            "EMA".to_string(),
            Box::new(|params: &IndicatorParams| {
                let period = params.get_or("period", 14.0) as usize;
                Box::new(Ema::new(period))
            }),
        );

        reg.factories.insert(
            "RSI".to_string(),
            Box::new(|params: &IndicatorParams| {
                let period = params.get_or("period", 14.0) as usize;
                Box::new(Rsi::new(period))
            }),
        );

        reg.factories.insert(
            "MACD".to_string(),
            Box::new(|params: &IndicatorParams| {
                let fast = params.get_or("fast_period", 12.0) as usize;
                let slow = params.get_or("slow_period", 26.0) as usize;
                let signal = params.get_or("signal_period", 9.0) as usize;
                Box::new(Macd::new(fast, slow, signal))
            }),
        );

        reg.factories.insert(
            "BB".to_string(),
            Box::new(|params: &IndicatorParams| {
                let period = params.get_or("period", 20.0) as usize;
                let std_dev = params.get_or("num_std_dev", 2.0);
                Box::new(BollingerBands::new(period, std_dev))
            }),
        );

        reg.factories.insert(
            "ATR".to_string(),
            Box::new(|params: &IndicatorParams| {
                let period = params.get_or("period", 14.0) as usize;
                Box::new(Atr::new(period))
            }),
        );

        reg.factories.insert(
            "STOCH".to_string(),
            Box::new(|params: &IndicatorParams| {
                let k = params.get_or("k_period", 14.0) as usize;
                let d = params.get_or("d_period", 3.0) as usize;
                Box::new(Stochastic::new(k, d))
            }),
        );

        reg
    }

    /// Create an indicator by name.
    pub fn create(&self, name: &str, params: &IndicatorParams) -> Option<Box<dyn Indicator + Send>> {
        self.factories.get(name).map(|f| f(params))
    }

    /// Register a custom indicator factory.
    pub fn register_custom(
        &mut self,
        name: &str,
        factory: IndicatorFactory,
    ) {
        self.factories.insert(name.to_string(), factory);
    }

    /// Return sorted names of all available indicators.
    pub fn available_indicators(&self) -> Vec<String> {
        let mut names: Vec<String> = self.factories.keys().cloned().collect();
        names.sort();
        names
    }

    /// Number of registered indicator factories.
    pub fn indicator_count(&self) -> usize {
        self.factories.len()
    }
}

impl Default for IndicatorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indicator_type_display() {
        assert_eq!(IndicatorType::Trend.to_string(), "Trend");
        assert_eq!(IndicatorType::Momentum.to_string(), "Momentum");
        assert_eq!(IndicatorType::Volatility.to_string(), "Volatility");
        assert_eq!(IndicatorType::Volume.to_string(), "Volume");
        assert_eq!(IndicatorType::Oscillator.to_string(), "Oscillator");
        assert_eq!(
            IndicatorType::Custom("MyInd".to_string()).to_string(),
            "Custom(MyInd)"
        );
    }

    #[test]
    fn test_sma_basic() {
        let mut sma = Sma::new(3);
        sma.update(1.0);
        sma.update(2.0);
        sma.update(3.0);
        assert!(sma.is_ready());
        let v = sma.value().unwrap();
        assert!((v - 2.0).abs() < 1e-10);

        sma.update(4.0);
        let v = sma.value().unwrap();
        assert!((v - 3.0).abs() < 1e-10); // (2+3+4)/3

        sma.update(5.0);
        let v = sma.value().unwrap();
        assert!((v - 4.0).abs() < 1e-10); // (3+4+5)/3
    }

    #[test]
    fn test_sma_not_ready() {
        let mut sma = Sma::new(3);
        assert!(!sma.is_ready());
        assert!(sma.value().is_none());

        sma.update(1.0);
        assert!(!sma.is_ready());

        sma.update(2.0);
        assert!(!sma.is_ready());
        assert!(sma.value().is_none());
    }

    #[test]
    fn test_ema_basic() {
        let mut ema = Ema::new(3);
        ema.update(2.0);
        ema.update(4.0);
        assert!(!ema.is_ready());

        ema.update(6.0);
        assert!(ema.is_ready());
        // Seed = (2+4+6)/3 = 4.0
        assert!((ema.value().unwrap() - 4.0).abs() < 1e-10);

        // multiplier = 2/(3+1) = 0.5
        ema.update(8.0);
        // EMA = (8 - 4) * 0.5 + 4 = 6.0
        assert!((ema.value().unwrap() - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_rsi_basic() {
        let mut rsi = Rsi::new(14);
        // Feed 20 values so it has enough data
        let data = [
            44.0, 44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89,
            46.03, 45.61, 46.28, 46.28, 46.00, 46.03, 46.41, 46.22,
        ];
        for &v in &data {
            rsi.update(v);
        }
        assert!(rsi.is_ready());
        let val = rsi.value().unwrap();
        assert!(val >= 0.0 && val <= 100.0);
    }

    #[test]
    fn test_rsi_overbought() {
        let mut rsi = Rsi::new(14);
        // Feed strictly increasing values: all gains, no losses.
        for i in 0..30 {
            rsi.update(i as f64);
        }
        assert!(rsi.is_ready());
        let val = rsi.value().unwrap();
        // Should approach 100
        assert!(val > 95.0, "RSI should approach 100 with all gains, got {val}");
    }

    #[test]
    fn test_macd_basic() {
        let mut macd = Macd::new(12, 26, 9);
        // Feed enough data
        for i in 0..50 {
            macd.update(100.0 + (i as f64) * 0.5);
        }
        assert!(macd.is_ready());
        let ml = macd.macd_line();
        let sl = macd.signal_line();
        let hist = macd.histogram();
        assert!(ml.is_some());
        assert!(sl.is_some());
        assert!(hist.is_some());
        // With an uptrend the fast EMA > slow EMA so MACD line should be positive.
        assert!(ml.unwrap() > 0.0);
    }

    #[test]
    fn test_bollinger_bands() {
        let mut bb = BollingerBands::new(5, 2.0);
        let data = [20.0, 22.0, 21.0, 23.0, 24.0];
        for &v in &data {
            bb.update(v);
        }
        assert!(bb.is_ready());
        let upper = bb.upper().unwrap();
        let middle = bb.middle().unwrap();
        let lower = bb.lower().unwrap();
        assert!(upper > middle, "upper ({upper}) should be > middle ({middle})");
        assert!(middle > lower, "middle ({middle}) should be > lower ({lower})");
        // Middle should be mean = 22.0
        assert!((middle - 22.0).abs() < 1e-10);
    }

    #[test]
    fn test_atr_basic() {
        let mut atr = Atr::new(3);
        // Day 1
        atr.update_hlc(48.70, 47.79, 48.16);
        assert!(!atr.is_ready());
        // Day 2
        atr.update_hlc(48.72, 48.14, 48.61);
        // Day 3
        atr.update_hlc(48.90, 48.39, 48.75);
        assert!(atr.is_ready());
        let val = atr.value().unwrap();
        assert!(val > 0.0);
    }

    #[test]
    fn test_stochastic_basic() {
        let mut stoch = Stochastic::new(5, 3);
        let bars: Vec<(f64, f64, f64)> = vec![
            (130.0, 120.0, 125.0),
            (132.0, 121.0, 127.0),
            (135.0, 123.0, 130.0),
            (133.0, 122.0, 128.0),
            (136.0, 124.0, 134.0),
            (138.0, 126.0, 135.0),
            (137.0, 125.0, 133.0),
        ];
        for &(h, l, c) in &bars {
            stoch.update_hlc(h, l, c);
        }
        assert!(stoch.is_ready());
        let k = stoch.k_value().unwrap();
        let d = stoch.d_value().unwrap();
        assert!(k >= 0.0 && k <= 100.0, "%K out of range: {k}");
        assert!(d >= 0.0 && d <= 100.0, "%D out of range: {d}");
    }

    #[test]
    fn test_indicator_params_builder() {
        let params = IndicatorParams::new()
            .with("period", 20.0)
            .with("num_std_dev", 2.5);
        assert_eq!(params.get("period"), Some(20.0));
        assert_eq!(params.get("num_std_dev"), Some(2.5));
        assert_eq!(params.get("missing"), None);
        assert_eq!(params.get_or("missing", 42.0), 42.0);
        assert_eq!(params.get_or("period", 10.0), 20.0);
    }

    #[test]
    fn test_indicator_registry_create() {
        let registry = IndicatorRegistry::new();
        assert_eq!(registry.indicator_count(), 7);

        let params = IndicatorParams::new().with("period", 5.0);
        let mut sma = registry.create("SMA", &params).expect("SMA should exist");
        assert_eq!(sma.short_name(), "SMA");
        assert_eq!(sma.min_periods(), 5);

        for i in 1..=5 {
            sma.update(i as f64);
        }
        assert!(sma.is_ready());
        assert!((sma.value().unwrap() - 3.0).abs() < 1e-10);

        // Non-existent indicator returns None.
        assert!(registry.create("NOPE", &params).is_none());
    }

    #[test]
    fn test_indicator_registry_custom() {
        let mut registry = IndicatorRegistry::new();
        let initial = registry.indicator_count();

        registry.register_custom(
            "CUSTOM_SMA",
            Box::new(|params: &IndicatorParams| {
                let period = params.get_or("period", 10.0) as usize;
                Box::new(Sma::new(period))
            }),
        );

        assert_eq!(registry.indicator_count(), initial + 1);
        let names = registry.available_indicators();
        assert!(names.contains(&"CUSTOM_SMA".to_string()));

        let params = IndicatorParams::new().with("period", 3.0);
        let ind = registry.create("CUSTOM_SMA", &params);
        assert!(ind.is_some());
        assert_eq!(ind.unwrap().min_periods(), 3);
    }

    #[test]
    fn test_indicator_reset() {
        let mut sma = Sma::new(3);
        sma.update(1.0);
        sma.update(2.0);
        sma.update(3.0);
        assert!(sma.is_ready());

        sma.reset();
        assert!(!sma.is_ready());
        assert!(sma.value().is_none());

        // After reset, should work again from scratch.
        sma.update(10.0);
        sma.update(20.0);
        sma.update(30.0);
        assert!(sma.is_ready());
        assert!((sma.value().unwrap() - 20.0).abs() < 1e-10);
    }
}
