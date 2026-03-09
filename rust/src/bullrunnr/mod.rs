use crate::error::BullShiftError;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsArticle {
    pub id: String,
    pub title: String,
    pub content: String,
    pub source: String,
    pub author: Option<String>,
    pub url: String,
    pub published_at: DateTime<Utc>,
    pub symbols: Vec<String>,
    pub sentiment: SentimentAnalysis,
    pub relevance_score: f64,
    pub category: NewsCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentAnalysis {
    pub overall: SentimentLabel,
    pub score: f64,      // -1.0 (very bearish) to 1.0 (very bullish)
    pub confidence: f64, // 0.0 to 1.0
    pub aspects: HashMap<String, AspectSentiment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SentimentLabel {
    VeryBearish,
    Bearish,
    Neutral,
    Bullish,
    VeryBullish,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AspectSentiment {
    pub sentiment: SentimentLabel,
    pub score: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NewsCategory {
    Earnings,
    MergersAcquisitions,
    Regulatory,
    MarketAnalysis,
    EconomicData,
    CompanyNews,
    SectorNews,
    BreakingNews,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsStream {
    pub articles: Vec<NewsArticle>,
    pub symbol_sentiment: HashMap<String, SymbolSentiment>,
    pub market_sentiment: MarketSentiment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolSentiment {
    pub symbol: String,
    pub sentiment_score: f64,
    pub article_count: i32,
    pub recent_articles: Vec<String>, // Article IDs
    pub sentiment_trend: SentimentTrend,
    pub buzz_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SentimentTrend {
    Improving,
    Declining,
    Stable,
    Volatile,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketSentiment {
    pub overall_score: f64,
    pub bullish_count: i32,
    pub bearish_count: i32,
    pub neutral_count: i32,
    pub total_articles: i32,
    pub fear_greed_index: f64,
}

pub struct BullRunnr {
    client: Client,
    news_sources: Vec<Box<dyn NewsSource + Send + Sync>>,
    sentiment_analyzer: Box<dyn SentimentAnalyzer + Send + Sync>,
    article_cache: HashMap<String, NewsArticle>,
    symbol_sentiment: HashMap<String, SymbolSentiment>,
    market_sentiment: MarketSentiment,
}

impl Default for BullRunnr {
    fn default() -> Self {
        Self::new()
    }
}

impl BullRunnr {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            news_sources: vec![
                Box::new(AlphaVantageNews::new()),
                Box::new(NewsAPI::new()),
                Box::new(TwitterNews::new()),
            ],
            sentiment_analyzer: Box::new(VaderSentimentAnalyzer::new()),
            article_cache: HashMap::new(),
            symbol_sentiment: HashMap::new(),
            market_sentiment: MarketSentiment {
                overall_score: 0.0,
                bullish_count: 0,
                bearish_count: 0,
                neutral_count: 0,
                total_articles: 0,
                fear_greed_index: 50.0,
            },
        }
    }

    pub async fn fetch_latest_news(&mut self) -> Result<NewsStream, BullShiftError> {
        let mut all_articles = Vec::new();

        // Fetch from all sources
        for source in &self.news_sources {
            match source.fetch_articles(&self.client).await {
                Ok(mut articles) => {
                    // Analyze sentiment for each article
                    for article in &mut articles {
                        article.sentiment = self.sentiment_analyzer.analyze(&article.content);
                        article.relevance_score = self.calculate_relevance_score(article);
                    }
                    all_articles.extend(articles);
                }
                Err(e) => {
                    log::warn!("Failed to fetch from source: {}", e);
                }
            }
        }

        // Sort by relevance and recency
        all_articles.sort_by(|a, b| {
            b.relevance_score
                .partial_cmp(&a.relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(b.published_at.cmp(&a.published_at))
        });

        // Update caches
        self.update_article_cache(&all_articles);
        self.update_symbol_sentiment(&all_articles);
        self.update_market_sentiment(&all_articles);

        // Take ownership of sentiment data to avoid cloning
        let symbol_sentiment = std::mem::take(&mut self.symbol_sentiment);
        let market_sentiment = std::mem::take(&mut self.market_sentiment);

        Ok(NewsStream {
            articles: all_articles,
            symbol_sentiment,
            market_sentiment,
        })
    }

    pub async fn search_news(
        &mut self,
        query: &str,
        symbols: &[String],
    ) -> Result<Vec<NewsArticle>, BullShiftError> {
        let mut search_results = Vec::new();

        for source in &self.news_sources {
            match source.search_articles(&self.client, query, symbols).await {
                Ok(mut articles) => {
                    for article in &mut articles {
                        article.sentiment = self.sentiment_analyzer.analyze(&article.content);
                        article.relevance_score = self.calculate_relevance_score(article);
                    }
                    search_results.extend(articles);
                }
                Err(e) => {
                    log::warn!("Search failed for source: {}", e);
                }
            }
        }

        search_results.sort_by(|a, b| {
            b.relevance_score
                .partial_cmp(&a.relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(search_results)
    }

    fn calculate_relevance_score(&self, article: &NewsArticle) -> f64 {
        let mut score = 0.0;

        // Recency factor (newer articles are more relevant)
        let hours_old = (Utc::now() - article.published_at).num_hours() as f64;
        let recency_score = (1.0 / (1.0 + hours_old / 24.0)).min(1.0);
        score += recency_score * 0.3;

        // Source credibility
        let source_score = self.get_source_credibility(&article.source);
        score += source_score * 0.2;

        // Symbol relevance
        let symbol_score = if !article.symbols.is_empty() {
            1.0
        } else {
            0.5
        };
        score += symbol_score * 0.3;

        // Content length (longer articles might be more detailed)
        let length_score = (article.content.len() as f64 / 1000.0).min(1.0);
        score += length_score * 0.2;

        score.min(1.0)
    }

    fn get_source_credibility(&self, source: &str) -> f64 {
        match source.to_lowercase().as_str() {
            "reuters" | "bloomberg" | "associated press" => 1.0,
            "cnbc" | "marketwatch" | "yahoo finance" => 0.8,
            "seeking alpha" | "the motley fool" => 0.7,
            _ => 0.5,
        }
    }

    fn update_article_cache(&mut self, articles: &[NewsArticle]) {
        // Batch insert to minimize rehashing
        let estimated_size = self.article_cache.len() + articles.len();
        if estimated_size > self.article_cache.capacity() {
            self.article_cache
                .reserve(estimated_size - self.article_cache.capacity());
        }

        // Use drain iterator if we own the articles, otherwise clone
        for article in articles {
            self.article_cache
                .insert(article.id.clone(), article.clone());
        }

        // Remove old articles only when we exceed the limit by 10% to avoid frequent cleanup
        if self.article_cache.len() > 1100 {
            // Use a more efficient approach with binary heap or sorted vec
            let mut ids_with_dates: Vec<_> = self
                .article_cache
                .iter()
                .map(|(id, article)| (article.published_at, id.clone()))
                .collect();

            // Sort by date (oldest first)
            ids_with_dates.sort_by(|a, b| a.0.cmp(&b.0));

            // Remove oldest articles
            let to_remove = self.article_cache.len() - 1000;
            for item in ids_with_dates.iter().take(to_remove) {
                self.article_cache.remove(&item.1);
            }
        }
    }

    fn update_symbol_sentiment(&mut self, articles: &[NewsArticle]) {
        // Reset symbol sentiment
        self.symbol_sentiment.clear();

        // Group articles by symbol using references only
        let mut symbol_articles: HashMap<&str, Vec<&NewsArticle>> = HashMap::new();

        for article in articles {
            for symbol in &article.symbols {
                symbol_articles
                    .entry(symbol.as_str())
                    .or_default()
                    .push(article);
            }
        }

        // Calculate sentiment for each symbol
        for (symbol, symbol_articles) in symbol_articles {
            let avg_sentiment = if !symbol_articles.is_empty() {
                let sum: f64 = symbol_articles.iter().map(|a| a.sentiment.score).sum();
                sum / symbol_articles.len() as f64
            } else {
                0.0
            };

            let buzz_score = (symbol_articles.len() as f64 / 10.0).min(1.0);

            let sentiment_scores: Vec<f64> =
                symbol_articles.iter().map(|a| a.sentiment.score).collect();
            let sentiment_trend = self.calculate_sentiment_trend(&sentiment_scores);

            self.symbol_sentiment.insert(
                symbol.to_string(),
                SymbolSentiment {
                    symbol: symbol.to_string(),
                    sentiment_score: avg_sentiment,
                    article_count: symbol_articles.len() as i32,
                    recent_articles: symbol_articles.iter().map(|a| a.id.clone()).collect(),
                    sentiment_trend,
                    buzz_score,
                },
            );
        }
    }

    fn calculate_sentiment_trend(&self, scores: &[f64]) -> SentimentTrend {
        if scores.len() < 3 {
            return SentimentTrend::Stable;
        }

        let first_half = &scores[..scores.len() / 2];
        let second_half = &scores[scores.len() / 2..];

        let first_avg = first_half.iter().sum::<f64>() / first_half.len() as f64;
        let second_avg = second_half.iter().sum::<f64>() / second_half.len() as f64;

        let difference = second_avg - first_avg;

        if difference > 0.2 {
            SentimentTrend::Improving
        } else if difference < -0.2 {
            SentimentTrend::Declining
        } else {
            SentimentTrend::Stable
        }
    }

    fn update_market_sentiment(&mut self, articles: &[NewsArticle]) {
        let sentiment_scores: Vec<f64> = articles.iter().map(|a| a.sentiment.score).collect();

        let overall_score = if sentiment_scores.is_empty() {
            0.0
        } else {
            sentiment_scores.iter().sum::<f64>() / sentiment_scores.len() as f64
        };

        let (bullish_count, bearish_count, neutral_count) =
            sentiment_scores
                .iter()
                .fold((0, 0, 0), |(bull, bear, neutral), &score| {
                    if score > 0.1 {
                        (bull + 1, bear, neutral)
                    } else if score < -0.1 {
                        (bull, bear + 1, neutral)
                    } else {
                        (bull, bear, neutral + 1)
                    }
                });

        // Calculate Fear & Greed Index (simplified)
        let fear_greed_index = ((overall_score + 1.0) * 50.0).clamp(0.0, 100.0);

        self.market_sentiment = MarketSentiment {
            overall_score,
            bullish_count,
            bearish_count,
            neutral_count,
            total_articles: articles.len() as i32,
            fear_greed_index,
        };
    }

    // Public interface methods
    pub fn get_symbol_sentiment(&self, symbol: &str) -> Option<&SymbolSentiment> {
        self.symbol_sentiment.get(symbol)
    }

    pub fn get_market_sentiment(&self) -> &MarketSentiment {
        &self.market_sentiment
    }

    pub fn get_top_sentiment_movers(&self, limit: usize) -> Vec<&SymbolSentiment> {
        let mut symbols: Vec<_> = self.symbol_sentiment.values().collect();
        symbols.sort_by(|a, b| {
            b.buzz_score
                .partial_cmp(&a.buzz_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        symbols.into_iter().take(limit).collect()
    }
}

// Traits for news sources and sentiment analysis
#[async_trait::async_trait]
pub trait NewsSource {
    async fn fetch_articles(&self, client: &Client) -> Result<Vec<NewsArticle>, BullShiftError>;
    async fn search_articles(
        &self,
        client: &Client,
        query: &str,
        symbols: &[String],
    ) -> Result<Vec<NewsArticle>, BullShiftError>;
}

#[async_trait::async_trait]
pub trait SentimentAnalyzer {
    fn analyze(&self, text: &str) -> SentimentAnalysis;
}

// Mock implementations
pub struct AlphaVantageNews;

impl Default for AlphaVantageNews {
    fn default() -> Self {
        Self::new()
    }
}

impl AlphaVantageNews {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl NewsSource for AlphaVantageNews {
    async fn fetch_articles(&self, _client: &Client) -> Result<Vec<NewsArticle>, BullShiftError> {
        // Mock implementation
        Ok(vec![])
    }

    async fn search_articles(
        &self,
        _client: &Client,
        _query: &str,
        _symbols: &[String],
    ) -> Result<Vec<NewsArticle>, BullShiftError> {
        Ok(vec![])
    }
}

pub struct NewsAPI;

impl Default for NewsAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl NewsAPI {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl NewsSource for NewsAPI {
    async fn fetch_articles(&self, _client: &Client) -> Result<Vec<NewsArticle>, BullShiftError> {
        Ok(vec![])
    }

    async fn search_articles(
        &self,
        _client: &Client,
        _query: &str,
        _symbols: &[String],
    ) -> Result<Vec<NewsArticle>, BullShiftError> {
        Ok(vec![])
    }
}

pub struct TwitterNews;

impl Default for TwitterNews {
    fn default() -> Self {
        Self::new()
    }
}

impl TwitterNews {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl NewsSource for TwitterNews {
    async fn fetch_articles(&self, _client: &Client) -> Result<Vec<NewsArticle>, BullShiftError> {
        Ok(vec![])
    }

    async fn search_articles(
        &self,
        _client: &Client,
        _query: &str,
        _symbols: &[String],
    ) -> Result<Vec<NewsArticle>, BullShiftError> {
        Ok(vec![])
    }
}

pub struct VaderSentimentAnalyzer;

impl Default for VaderSentimentAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl VaderSentimentAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

impl SentimentAnalyzer for VaderSentimentAnalyzer {
    fn analyze(&self, text: &str) -> SentimentAnalysis {
        // Simplified sentiment analysis
        let positive_words = [
            "good",
            "great",
            "excellent",
            "bullish",
            "growth",
            "profit",
            "rally",
        ];
        let negative_words = [
            "bad", "terrible", "awful", "bearish", "decline", "loss", "crash",
        ];

        let text_lower = text.to_lowercase();
        let words: Vec<&str> = text_lower.split_whitespace().collect();

        let positive_count = words
            .iter()
            .filter(|&&w| positive_words.contains(&w))
            .count() as f64;
        let negative_count = words
            .iter()
            .filter(|&&w| negative_words.contains(&w))
            .count() as f64;

        let score = (positive_count - negative_count) / (words.len() as f64 + 1.0);
        let score = score.clamp(-1.0, 1.0);

        let overall = if score > 0.3 {
            SentimentLabel::Bullish
        } else if score < -0.3 {
            SentimentLabel::Bearish
        } else {
            SentimentLabel::Neutral
        };

        SentimentAnalysis {
            overall,
            score,
            confidence: score.abs(),
            aspects: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentiment_label_variants() {
        // Verify all SentimentLabel variants exist and can be constructed
        let _very_bearish = SentimentLabel::VeryBearish;
        let _bearish = SentimentLabel::Bearish;
        let _neutral = SentimentLabel::Neutral;
        let _bullish = SentimentLabel::Bullish;
        let _very_bullish = SentimentLabel::VeryBullish;

        // Verify Debug is implemented
        assert!(format!("{:?}", SentimentLabel::VeryBearish).contains("VeryBearish"));
        assert!(format!("{:?}", SentimentLabel::Bearish).contains("Bearish"));
        assert!(format!("{:?}", SentimentLabel::Neutral).contains("Neutral"));
        assert!(format!("{:?}", SentimentLabel::Bullish).contains("Bullish"));
        assert!(format!("{:?}", SentimentLabel::VeryBullish).contains("VeryBullish"));
    }

    #[test]
    fn test_news_category_variants() {
        // Verify all NewsCategory variants exist and can be constructed
        let categories = vec![
            NewsCategory::Earnings,
            NewsCategory::MergersAcquisitions,
            NewsCategory::Regulatory,
            NewsCategory::MarketAnalysis,
            NewsCategory::EconomicData,
            NewsCategory::CompanyNews,
            NewsCategory::SectorNews,
            NewsCategory::BreakingNews,
        ];

        assert_eq!(categories.len(), 8);
        assert!(format!("{:?}", NewsCategory::Earnings).contains("Earnings"));
        assert!(format!("{:?}", NewsCategory::Regulatory).contains("Regulatory"));
        assert!(format!("{:?}", NewsCategory::MarketAnalysis).contains("MarketAnalysis"));
    }

    #[test]
    fn test_vader_positive_sentiment() {
        let analyzer = VaderSentimentAnalyzer::new();
        let result = analyzer.analyze("great amazing excellent");
        // "great" and "excellent" are in the positive word list
        assert!(
            result.score > 0.0,
            "Expected positive score, got {}",
            result.score
        );
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_vader_negative_sentiment() {
        let analyzer = VaderSentimentAnalyzer::new();
        let result = analyzer.analyze("terrible crash disaster");
        // "terrible" and "crash" are in the negative word list
        assert!(
            result.score < 0.0,
            "Expected negative score, got {}",
            result.score
        );
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_vader_neutral_sentiment() {
        let analyzer = VaderSentimentAnalyzer::new();
        let result = analyzer.analyze("the stock traded today");
        // No positive or negative words in the list
        assert!(
            result.score.abs() < 0.01,
            "Expected near-zero score for neutral text, got {}",
            result.score
        );
    }

    fn make_test_article(id: &str, symbol: &str) -> NewsArticle {
        NewsArticle {
            id: id.to_string(),
            title: format!("Test article {}", id),
            content: "Some neutral content here".to_string(),
            source: "test".to_string(),
            author: None,
            url: "https://example.com".to_string(),
            published_at: Utc::now(),
            symbols: vec![symbol.to_string()],
            sentiment: SentimentAnalysis {
                overall: SentimentLabel::Neutral,
                score: 0.0,
                confidence: 0.5,
                aspects: HashMap::new(),
            },
            relevance_score: 0.5,
            category: NewsCategory::CompanyNews,
        }
    }

    #[test]
    fn test_bullrunnr_add_and_get_articles() {
        let mut runnr = BullRunnr::new();
        let articles = vec![
            make_test_article("1", "AAPL"),
            make_test_article("2", "TSLA"),
            make_test_article("3", "GOOG"),
        ];

        // Add articles via update_article_cache (internal method accessible in tests)
        runnr.update_article_cache(&articles);

        // Verify articles were cached
        assert_eq!(runnr.article_cache.len(), 3);
        assert!(runnr.article_cache.contains_key("1"));
        assert!(runnr.article_cache.contains_key("2"));
        assert!(runnr.article_cache.contains_key("3"));
    }

    #[test]
    fn test_bullrunnr_symbol_filter() {
        let mut runnr = BullRunnr::new();
        let articles = vec![
            make_test_article("1", "AAPL"),
            make_test_article("2", "TSLA"),
            make_test_article("3", "AAPL"),
        ];

        // Update symbol sentiment from articles
        runnr.update_symbol_sentiment(&articles);

        // Verify AAPL has 2 articles and TSLA has 1
        let aapl_sentiment = runnr.get_symbol_sentiment("AAPL");
        assert!(aapl_sentiment.is_some());
        assert_eq!(aapl_sentiment.unwrap().article_count, 2);

        let tsla_sentiment = runnr.get_symbol_sentiment("TSLA");
        assert!(tsla_sentiment.is_some());
        assert_eq!(tsla_sentiment.unwrap().article_count, 1);

        // Verify that a non-existent symbol returns None
        assert!(runnr.get_symbol_sentiment("MSFT").is_none());
    }

    #[test]
    fn test_market_sentiment_default() {
        let sentiment = MarketSentiment::default();
        assert_eq!(sentiment.overall_score, 0.0);
        assert_eq!(sentiment.bullish_count, 0);
        assert_eq!(sentiment.bearish_count, 0);
        assert_eq!(sentiment.neutral_count, 0);
        assert_eq!(sentiment.total_articles, 0);
        assert_eq!(sentiment.fear_greed_index, 0.0);
    }
}
