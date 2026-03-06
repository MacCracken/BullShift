use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentumScore {
    pub symbol: String,
    pub score: f64,
    pub volume_spike: f64,
    pub price_momentum: f64,
    pub social_sentiment: f64,
    pub trend_strength: TrendStrength,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendStrength {
    Weak,
    Moderate,
    Strong,
    Explosive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeAnalysis {
    pub symbol: String,
    pub current_volume: f64,
    pub avg_volume_20d: f64,
    pub volume_ratio: f64,
    pub volume_spike_detected: bool,
    pub unusual_volume_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceMomentum {
    pub symbol: String,
    pub current_price: f64,
    pub sma_20: f64,
    pub sma_50: f64,
    pub rsi_14: f64,
    pub macd_signal: f64,
    pub price_change_1d: f64,
    pub price_change_5d: f64,
    pub momentum_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSentiment {
    pub symbol: String,
    pub mentions_count: i64,
    pub sentiment_score: f64, // -1.0 (bearish) to 1.0 (bullish)
    pub buzz_score: f64,
    pub influencer_score: f64,
    pub news_volume: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAlert {
    pub symbol: String,
    pub alert_type: AlertType,
    pub message: String,
    pub confidence: f64,
    pub timestamp: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    VolumeSpike,
    PriceBreakout,
    MomentumShift,
    SocialBuzz,
    TrendReversal,
}

pub struct TrendSetter {
    momentum_scores: HashMap<String, MomentumScore>,
    volume_analysis: HashMap<String, VolumeAnalysis>,
    price_momentum: HashMap<String, PriceMomentum>,
    social_sentiment: HashMap<String, SocialSentiment>,
    active_alerts: Vec<TrendAlert>,
}

impl TrendSetter {
    pub fn new() -> Self {
        Self {
            momentum_scores: HashMap::new(),
            volume_analysis: HashMap::new(),
            price_momentum: HashMap::new(),
            social_sentiment: HashMap::new(),
            active_alerts: Vec::new(),
        }
    }

    pub fn analyze_momentum(&mut self, symbol: String, market_data: &MarketDataUpdate) -> MomentumScore {
        // Analyze volume
        let volume_analysis = self.analyze_volume(&symbol, &market_data);
        
        // Analyze price momentum
        let price_momentum = self.analyze_price_momentum(&symbol, &market_data);
        
        // Analyze social sentiment
        let social_sentiment = self.analyze_social_sentiment(&symbol);
        
        // Calculate composite momentum score
        let volume_weight = 0.3;
        let price_weight = 0.4;
        let social_weight = 0.3;
        
        let volume_score = volume_analysis.unusual_volume_score.min(1.0);
        let price_score = price_momentum.momentum_score;
        let social_score = (social_sentiment.sentiment_score + 1.0) / 2.0; // Normalize to 0-1
        
        let composite_score = (volume_score * volume_weight) + 
                             (price_score * price_weight) + 
                             (social_score * social_weight);
        
        // Determine trend strength
        let trend_strength = match composite_score {
            score if score >= 0.8 => TrendStrength::Explosive,
            score if score >= 0.6 => TrendStrength::Strong,
            score if score >= 0.4 => TrendStrength::Moderate,
            _ => TrendStrength::Weak,
        };
        
        let symbol_string = symbol.to_string();
        
        let momentum_score = MomentumScore {
            symbol: symbol_string.clone(),
            score: composite_score,
            volume_spike: volume_analysis.volume_ratio,
            price_momentum: price_momentum.momentum_score,
            social_sentiment: social_sentiment.sentiment_score,
            trend_strength,
            timestamp: Utc::now(),
        };
        
        // Store results - use single clone for all insertions
        self.momentum_scores.insert(symbol_string.clone(), momentum_score);
        self.volume_analysis.insert(symbol_string.clone(), volume_analysis);
        self.price_momentum.insert(symbol_string.clone(), price_momentum);
        self.social_sentiment.insert(symbol_string, social_sentiment);
        
        // Generate alerts if needed
        self.generate_alerts(&momentum_score);
        
        momentum_score
    }

    fn analyze_volume(&self, symbol: &str, market_data: &MarketDataUpdate) -> VolumeAnalysis {
        // Calculate volume ratio vs 20-day average
        let avg_volume_20d = self.get_historical_avg_volume(symbol, 20);
        let current_volume = market_data.volume;
        let volume_ratio = current_volume / avg_volume_20d;
        
        // Detect unusual volume
        let volume_spike_detected = volume_ratio > 2.0; // 2x average volume
        let unusual_volume_score = (volume_ratio - 1.0).min(5.0) / 5.0; // Normalize to 0-1
        
        VolumeAnalysis {
            symbol: symbol.to_string(),
            current_volume,
            avg_volume_20d,
            volume_ratio,
            volume_spike_detected,
            unusual_volume_score,
        }
    }

    fn analyze_price_momentum(&self, symbol: &str, market_data: &MarketDataUpdate) -> PriceMomentum {
        let current_price = market_data.price;
        
        // Calculate moving averages
        let sma_20 = self.calculate_sma(symbol, 20);
        let sma_50 = self.calculate_sma(symbol, 50);
        
        // Calculate RSI
        let rsi_14 = self.calculate_rsi(symbol, 14);
        
        // Calculate MACD
        let macd_signal = self.calculate_macd_signal(symbol);
        
        // Calculate price changes
        let price_change_1d = self.get_price_change(symbol, 1);
        let price_change_5d = self.get_price_change(symbol, 5);
        
        // Calculate momentum score
        let price_vs_sma20 = (current_price - sma_20) / sma_20;
        let sma_trend = (sma_20 - sma_50) / sma_50;
        let rsi_momentum = (rsi_14 - 50.0) / 50.0;
        
        let momentum_score = (price_vs_sma20 * 0.3 + 
                              sma_trend * 0.3 + 
                              rsi_momentum * 0.2 + 
                              macd_signal * 0.2).clamp(-1.0, 1.0);
        
        PriceMomentum {
            symbol: symbol.to_string(),
            current_price,
            sma_20,
            sma_50,
            rsi_14,
            macd_signal,
            price_change_1d,
            price_change_5d,
            momentum_score,
        }
    }

    fn analyze_social_sentiment(&self, symbol: &str) -> SocialSentiment {
        // In a real implementation, this would connect to Twitter API, Reddit API, etc.
        // For now, simulate social sentiment data
        
        let mentions_count = self.get_social_mentions(symbol);
        let sentiment_score = self.calculate_sentiment_score(symbol);
        let buzz_score = (mentions_count as f64 / 1000.0).min(1.0);
        let influencer_score = self.get_influencer_sentiment(symbol);
        let news_volume = self.get_news_volume(symbol);
        
        SocialSentiment {
            symbol: symbol.to_string(),
            mentions_count,
            sentiment_score,
            buzz_score,
            influencer_score,
            news_volume,
        }
    }

    fn generate_alerts(&mut self, momentum_score: &MomentumScore) {
        let symbol = momentum_score.symbol.clone();
        let now = Utc::now();
        
        // Volume spike alert
        if momentum_score.volume_spike > 3.0 {
            self.active_alerts.push(TrendAlert {
                symbol: symbol.clone(),
                alert_type: AlertType::VolumeSpike,
                message: format!("Unusual volume spike detected: {:.1}x average", momentum_score.volume_spike),
                confidence: 0.8,
                timestamp: now,
                expires_at: now + Duration::hours(4),
            });
        }
        
        // Strong momentum alert
        if momentum_score.score > 0.7 {
            self.active_alerts.push(TrendAlert {
                symbol: symbol.clone(),
                alert_type: AlertType::MomentumShift,
                message: format!("Strong momentum detected: {:.2}", momentum_score.score),
                confidence: momentum_score.score,
                timestamp: now,
                expires_at: now + Duration::hours(2),
            });
        }
        
        // Social buzz alert
        if momentum_score.social_sentiment > 0.6 {
            self.active_alerts.push(TrendAlert {
                symbol,
                alert_type: AlertType::SocialBuzz,
                message: format!("High social sentiment: {:.2}", momentum_score.social_sentiment),
                confidence: 0.7,
                timestamp: now,
                expires_at: now + Duration::hours(6),
            });
        }
    }

    // Helper methods (simplified for example)
    fn get_historical_avg_volume(&self, _symbol: &str, _days: u32) -> f64 {
        1000000.0 // Default 1M shares
    }

    fn calculate_sma(&self, _symbol: &str, _period: u32) -> f64 {
        150.0 // Default SMA
    }

    fn calculate_rsi(&self, _symbol: &str, _period: u32) -> f64 {
        55.0 // Default RSI
    }

    fn calculate_macd_signal(&self, _symbol: &str) -> f64 {
        0.1 // Default MACD signal
    }

    fn get_price_change(&self, _symbol: &str, _days: u32) -> f64 {
        0.02 // Default 2% change
    }

    fn get_social_mentions(&self, _symbol: &str) -> i64 {
        500 // Default mentions
    }

    fn calculate_sentiment_score(&self, _symbol: &str) -> f64 {
        0.3 // Default positive sentiment
    }

    fn get_influencer_sentiment(&self, _symbol: &str) -> f64 {
        0.4 // Default influencer score
    }

    fn get_news_volume(&self, _symbol: &str) -> i64 {
        25 // Default news articles
    }

    // Public interface methods
    pub fn get_top_momentum_stocks(&self, limit: usize) -> Vec<&MomentumScore> {
        let mut scores: Vec<&MomentumScore> = self.momentum_scores.values().collect();
        scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        scores.into_iter().take(limit).collect()
    }

    pub fn get_active_alerts(&self) -> &[TrendAlert] {
        &self.active_alerts
    }

    pub fn get_momentum_score(&self, symbol: &str) -> Option<&MomentumScore> {
        self.momentum_scores.get(symbol)
    }

    pub fn clear_expired_alerts(&mut self) {
        let now = Utc::now();
        self.active_alerts.retain(|alert| alert.expires_at > now);
    }
}

#[derive(Debug, Clone)]
pub struct MarketDataUpdate {
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub timestamp: DateTime<Utc>,
}