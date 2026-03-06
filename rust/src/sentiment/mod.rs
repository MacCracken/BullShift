use crate::bullrunnr::{NewsArticle, NewsCategory, NewsSource, SentimentAnalysis, SentimentLabel};
use crate::error::BullShiftError;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Identifies the origin of a sentiment signal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SentimentSourceType {
    SecureYeoman,
    RssFeed,
    Webhook,
    RedditApi,
    TwitterApi,
    CustomApi,
}

impl std::fmt::Display for SentimentSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SecureYeoman => write!(f, "SecureYeoman"),
            Self::RssFeed => write!(f, "RSS"),
            Self::Webhook => write!(f, "Webhook"),
            Self::RedditApi => write!(f, "Reddit"),
            Self::TwitterApi => write!(f, "Twitter"),
            Self::CustomApi => write!(f, "Custom"),
        }
    }
}

/// Configuration for a registered sentiment source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentSourceConfig {
    pub id: Uuid,
    pub name: String,
    pub source_type: SentimentSourceType,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub enabled: bool,
    pub poll_interval_secs: u64,
    pub symbols_filter: Vec<String>,
}

/// A normalized sentiment signal from any source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentSignal {
    pub id: Uuid,
    pub source_type: SentimentSourceType,
    pub source_name: String,
    pub symbol: Option<String>,
    pub headline: String,
    pub content: String,
    pub sentiment_score: f64,
    pub confidence: f64,
    pub url: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub raw_data: Option<serde_json::Value>,
}

/// Routes sentiment data from multiple sources (SecureYeoman event bus +
/// independent feeds) into BullRunnr's analysis pipeline.
///
/// Sources include:
/// - SecureYeoman event bus (consolidated Twitter/X, Reddit, webhooks)
/// - RSS feeds (direct, no SecureYeoman dependency)
/// - Webhook receivers (direct)
/// - Reddit API (direct)
/// - Twitter/X API (direct)
/// - Custom API endpoints
pub struct SentimentRouter {
    sources: HashMap<Uuid, SentimentSourceConfig>,
    recent_signals: std::collections::VecDeque<SentimentSignal>,
    max_signals: usize,
}

impl Default for SentimentRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl SentimentRouter {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            recent_signals: std::collections::VecDeque::with_capacity(1000),
            max_signals: 1000,
        }
    }

    /// Register a new sentiment source.
    pub fn add_source(&mut self, config: SentimentSourceConfig) -> Uuid {
        let id = config.id;
        self.sources.insert(id, config);
        id
    }

    /// Remove a sentiment source.
    pub fn remove_source(&mut self, id: &Uuid) -> bool {
        self.sources.remove(id).is_some()
    }

    /// List all registered sources.
    pub fn list_sources(&self) -> Vec<&SentimentSourceConfig> {
        self.sources.values().collect()
    }

    /// Get a source by ID.
    pub fn get_source(&self, id: &Uuid) -> Option<&SentimentSourceConfig> {
        self.sources.get(id)
    }

    /// Enable or disable a source.
    pub fn set_source_enabled(&mut self, id: &Uuid, enabled: bool) -> bool {
        if let Some(source) = self.sources.get_mut(id) {
            source.enabled = enabled;
            true
        } else {
            false
        }
    }

    /// Ingest a signal from any source.
    pub fn ingest_signal(&mut self, signal: SentimentSignal) {
        if self.recent_signals.len() >= self.max_signals {
            self.recent_signals.pop_front();
        }
        self.recent_signals.push_back(signal);
    }

    /// Get recent signals, optionally filtered by source type.
    pub fn recent_signals(
        &self,
        limit: usize,
        source_filter: Option<&SentimentSourceType>,
    ) -> Vec<&SentimentSignal> {
        self.recent_signals
            .iter()
            .rev()
            .filter(|s| {
                source_filter
                    .map(|f| std::mem::discriminant(&s.source_type) == std::mem::discriminant(f))
                    .unwrap_or(true)
            })
            .take(limit)
            .collect()
    }

    /// Get signals for a specific symbol.
    pub fn signals_for_symbol(&self, symbol: &str, limit: usize) -> Vec<&SentimentSignal> {
        self.recent_signals
            .iter()
            .rev()
            .filter(|s| s.symbol.as_deref() == Some(symbol))
            .take(limit)
            .collect()
    }

    /// Calculate aggregate sentiment for a symbol across all sources.
    pub fn aggregate_sentiment(&self, symbol: &str) -> Option<AggregateSentiment> {
        let signals: Vec<&SentimentSignal> = self
            .recent_signals
            .iter()
            .filter(|s| s.symbol.as_deref() == Some(symbol))
            .collect();

        if signals.is_empty() {
            return None;
        }

        let total_weight: f64 = signals.iter().map(|s| s.confidence).sum();
        let weighted_score: f64 = signals
            .iter()
            .map(|s| s.sentiment_score * s.confidence)
            .sum::<f64>()
            / total_weight;

        let mut source_breakdown: HashMap<String, SourceSentiment> = HashMap::new();
        for signal in &signals {
            let key = signal.source_type.to_string();
            let entry = source_breakdown
                .entry(key.clone())
                .or_insert(SourceSentiment {
                    source: key,
                    score: 0.0,
                    count: 0,
                    confidence: 0.0,
                });
            entry.score += signal.sentiment_score;
            entry.count += 1;
            entry.confidence += signal.confidence;
        }

        // Average out per-source scores
        for entry in source_breakdown.values_mut() {
            if entry.count > 0 {
                entry.score /= entry.count as f64;
                entry.confidence /= entry.count as f64;
            }
        }

        Some(AggregateSentiment {
            symbol: symbol.to_string(),
            overall_score: weighted_score,
            signal_count: signals.len(),
            source_breakdown,
        })
    }
}

/// Aggregated sentiment across all sources for a symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateSentiment {
    pub symbol: String,
    pub overall_score: f64,
    pub signal_count: usize,
    pub source_breakdown: HashMap<String, SourceSentiment>,
}

/// Per-source sentiment summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceSentiment {
    pub source: String,
    pub score: f64,
    pub count: usize,
    pub confidence: f64,
}

// --- NewsSource implementations for BullRunnr integration ---

/// Fetches sentiment signals from SecureYeoman's event bus and converts
/// them to NewsArticle format for BullRunnr's pipeline.
pub struct SecureYeomanEventSource {
    base_url: String,
    api_key: Option<String>,
}

impl SecureYeomanEventSource {
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        Self { base_url, api_key }
    }
}

#[async_trait::async_trait]
impl NewsSource for SecureYeomanEventSource {
    async fn fetch_articles(&self, client: &Client) -> Result<Vec<NewsArticle>, BullShiftError> {
        let url = format!(
            "{}/api/v1/integrations/bullshift/feed",
            self.base_url
        );

        let mut req = client.get(&url);
        if let Some(ref key) = self.api_key {
            req = req.header("x-api-key", key);
        }

        match req.send().await {
            Ok(resp) if resp.status().is_success() => {
                let items: Vec<EventBusFeedItem> = resp.json().await.map_err(|e| {
                    BullShiftError::Api(format!("Failed to parse SecureYeoman feed: {}", e))
                })?;

                Ok(items.into_iter().map(|item| item.into_article()).collect())
            }
            Ok(resp) => {
                log::warn!("SecureYeoman feed returned {}", resp.status());
                Ok(vec![])
            }
            Err(e) => {
                log::warn!("Failed to fetch SecureYeoman feed: {}", e);
                Ok(vec![]) // Graceful degradation
            }
        }
    }

    async fn search_articles(
        &self,
        client: &Client,
        query: &str,
        symbols: &[String],
    ) -> Result<Vec<NewsArticle>, BullShiftError> {
        let url = format!(
            "{}/api/v1/integrations/bullshift/feed/search",
            self.base_url
        );

        let body = serde_json::json!({
            "query": query,
            "symbols": symbols,
        });

        let mut req = client.post(&url).json(&body);
        if let Some(ref key) = self.api_key {
            req = req.header("x-api-key", key);
        }

        match req.send().await {
            Ok(resp) if resp.status().is_success() => {
                let items: Vec<EventBusFeedItem> = resp.json().await.map_err(|e| {
                    BullShiftError::Api(format!("Failed to parse search results: {}", e))
                })?;
                Ok(items.into_iter().map(|item| item.into_article()).collect())
            }
            Ok(_) | Err(_) => Ok(vec![]),
        }
    }
}

/// RSS feed source that can subscribe to any RSS/Atom feed.
pub struct RssFeedSource {
    name: String,
    feed_url: String,
    symbols_hint: Vec<String>,
}

impl RssFeedSource {
    pub fn new(name: String, feed_url: String, symbols_hint: Vec<String>) -> Self {
        Self {
            name,
            feed_url,
            symbols_hint,
        }
    }
}

#[async_trait::async_trait]
impl NewsSource for RssFeedSource {
    async fn fetch_articles(&self, client: &Client) -> Result<Vec<NewsArticle>, BullShiftError> {
        let resp = client
            .get(&self.feed_url)
            .header("User-Agent", "BullShift/2026.3.5")
            .send()
            .await
            .map_err(|e| BullShiftError::Network(format!("RSS fetch failed: {}", e)))?;

        if !resp.status().is_success() {
            return Ok(vec![]);
        }

        let body = resp
            .text()
            .await
            .map_err(|e| BullShiftError::Network(format!("RSS read failed: {}", e)))?;

        Ok(parse_rss_to_articles(&body, &self.name, &self.symbols_hint))
    }

    async fn search_articles(
        &self,
        _client: &Client,
        _query: &str,
        _symbols: &[String],
    ) -> Result<Vec<NewsArticle>, BullShiftError> {
        // RSS feeds don't support search — return empty
        Ok(vec![])
    }
}

/// Webhook-received articles stored in memory, exposed as a NewsSource.
pub struct WebhookSource {
    articles: std::sync::Arc<std::sync::Mutex<Vec<NewsArticle>>>,
}

impl Default for WebhookSource {
    fn default() -> Self {
        Self::new()
    }
}

impl WebhookSource {
    pub fn new() -> Self {
        Self {
            articles: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    /// Push a new article received via webhook.
    pub fn push_article(&self, article: NewsArticle) {
        let mut articles = self.articles.lock().unwrap_or_else(|e| e.into_inner());
        if articles.len() >= 500 {
            articles.drain(..100); // Evict oldest batch
        }
        articles.push(article);
    }

    /// Get the shared article store for use by webhook handlers.
    pub fn store(&self) -> std::sync::Arc<std::sync::Mutex<Vec<NewsArticle>>> {
        self.articles.clone()
    }
}

#[async_trait::async_trait]
impl NewsSource for WebhookSource {
    async fn fetch_articles(&self, _client: &Client) -> Result<Vec<NewsArticle>, BullShiftError> {
        let articles = self.articles.lock().unwrap_or_else(|e| e.into_inner());
        Ok(articles.clone())
    }

    async fn search_articles(
        &self,
        _client: &Client,
        query: &str,
        _symbols: &[String],
    ) -> Result<Vec<NewsArticle>, BullShiftError> {
        let articles = self.articles.lock().unwrap_or_else(|e| e.into_inner());
        let query_lower = query.to_lowercase();
        Ok(articles
            .iter()
            .filter(|a| {
                a.title.to_lowercase().contains(&query_lower)
                    || a.content.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect())
    }
}

// --- Internal helpers ---

/// Feed item from SecureYeoman's event bus.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EventBusFeedItem {
    id: Option<String>,
    title: String,
    content: String,
    source: String,
    url: Option<String>,
    symbols: Vec<String>,
    timestamp: Option<DateTime<Utc>>,
    sentiment_score: Option<f64>,
}

impl EventBusFeedItem {
    fn into_article(self) -> NewsArticle {
        let score = self.sentiment_score.unwrap_or(0.0);
        let label = if score > 0.3 {
            SentimentLabel::Bullish
        } else if score < -0.3 {
            SentimentLabel::Bearish
        } else {
            SentimentLabel::Neutral
        };

        NewsArticle {
            id: self.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            title: self.title,
            content: self.content,
            source: format!("SecureYeoman:{}", self.source),
            author: None,
            url: self.url.unwrap_or_default(),
            published_at: self.timestamp.unwrap_or_else(Utc::now),
            symbols: self.symbols,
            sentiment: SentimentAnalysis {
                overall: label,
                score,
                confidence: 0.7, // Event bus items have moderate confidence
                aspects: HashMap::new(),
            },
            relevance_score: 0.8, // Event bus items are pre-filtered for relevance
            category: NewsCategory::BreakingNews,
        }
    }
}

/// Simple RSS/XML parser that extracts <item> entries.
/// This is a basic implementation — production would use a proper XML parser.
fn parse_rss_to_articles(
    xml: &str,
    source_name: &str,
    symbols_hint: &[String],
) -> Vec<NewsArticle> {
    let mut articles = Vec::new();

    // Simple tag extraction (handles basic RSS 2.0 format)
    for item_block in xml.split("<item>").skip(1) {
        let end = item_block.find("</item>").unwrap_or(item_block.len());
        let item = &item_block[..end];

        let title = extract_tag(item, "title").unwrap_or_default();
        let description = extract_tag(item, "description").unwrap_or_default();
        let link = extract_tag(item, "link").unwrap_or_default();

        if title.is_empty() {
            continue;
        }

        // Check if any hint symbols appear in the title or description
        let combined = format!("{} {}", title, description).to_uppercase();
        let matched_symbols: Vec<String> = symbols_hint
            .iter()
            .filter(|s| combined.contains(&s.to_uppercase()))
            .cloned()
            .collect();

        articles.push(NewsArticle {
            id: Uuid::new_v4().to_string(),
            title,
            content: description,
            source: format!("RSS:{}", source_name),
            author: None,
            url: link,
            published_at: Utc::now(),
            symbols: matched_symbols,
            sentiment: SentimentAnalysis {
                overall: SentimentLabel::Neutral,
                score: 0.0,
                confidence: 0.0, // Needs analysis by BullRunnr
                aspects: HashMap::new(),
            },
            relevance_score: 0.0, // Will be scored by BullRunnr
            category: NewsCategory::MarketAnalysis,
        });
    }

    articles
}

fn extract_tag(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let start = xml.find(&open)? + open.len();
    let end = xml.find(&close)?;
    if start >= end {
        return None;
    }
    let content = &xml[start..end];
    // Strip CDATA if present
    let content = content
        .strip_prefix("<![CDATA[")
        .and_then(|s| s.strip_suffix("]]>"))
        .unwrap_or(content);
    Some(content.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentiment_router_add_remove_source() {
        let mut router = SentimentRouter::new();
        let config = SentimentSourceConfig {
            id: Uuid::new_v4(),
            name: "Test RSS".to_string(),
            source_type: SentimentSourceType::RssFeed,
            endpoint: "https://example.com/feed.xml".to_string(),
            api_key: None,
            enabled: true,
            poll_interval_secs: 300,
            symbols_filter: vec!["AAPL".to_string()],
        };

        let id = router.add_source(config);
        assert_eq!(router.list_sources().len(), 1);

        assert!(router.remove_source(&id));
        assert_eq!(router.list_sources().len(), 0);
    }

    #[test]
    fn test_ingest_and_query_signals() {
        let mut router = SentimentRouter::new();

        router.ingest_signal(SentimentSignal {
            id: Uuid::new_v4(),
            source_type: SentimentSourceType::RssFeed,
            source_name: "TestRSS".to_string(),
            symbol: Some("AAPL".to_string()),
            headline: "Apple rallies".to_string(),
            content: "Stock up 5%".to_string(),
            sentiment_score: 0.8,
            confidence: 0.9,
            url: None,
            timestamp: Utc::now(),
            raw_data: None,
        });

        router.ingest_signal(SentimentSignal {
            id: Uuid::new_v4(),
            source_type: SentimentSourceType::SecureYeoman,
            source_name: "SY".to_string(),
            symbol: Some("AAPL".to_string()),
            headline: "Bearish tweet".to_string(),
            content: "Analysts downgrade".to_string(),
            sentiment_score: -0.5,
            confidence: 0.6,
            url: None,
            timestamp: Utc::now(),
            raw_data: None,
        });

        let all = router.recent_signals(10, None);
        assert_eq!(all.len(), 2);

        let rss_only = router.recent_signals(10, Some(&SentimentSourceType::RssFeed));
        assert_eq!(rss_only.len(), 1);

        let aapl = router.signals_for_symbol("AAPL", 10);
        assert_eq!(aapl.len(), 2);
    }

    #[test]
    fn test_aggregate_sentiment() {
        let mut router = SentimentRouter::new();

        // Bullish signal with high confidence
        router.ingest_signal(SentimentSignal {
            id: Uuid::new_v4(),
            source_type: SentimentSourceType::RssFeed,
            source_name: "RSS".to_string(),
            symbol: Some("TSLA".to_string()),
            headline: "Tesla up".to_string(),
            content: "".to_string(),
            sentiment_score: 0.8,
            confidence: 0.9,
            url: None,
            timestamp: Utc::now(),
            raw_data: None,
        });

        // Bearish signal with low confidence
        router.ingest_signal(SentimentSignal {
            id: Uuid::new_v4(),
            source_type: SentimentSourceType::TwitterApi,
            source_name: "Twitter".to_string(),
            symbol: Some("TSLA".to_string()),
            headline: "Short Tesla".to_string(),
            content: "".to_string(),
            sentiment_score: -0.3,
            confidence: 0.4,
            url: None,
            timestamp: Utc::now(),
            raw_data: None,
        });

        let agg = router.aggregate_sentiment("TSLA").unwrap();
        assert_eq!(agg.signal_count, 2);
        // Weighted average: (0.8*0.9 + (-0.3)*0.4) / (0.9 + 0.4) = 0.6461...
        assert!(agg.overall_score > 0.4);
        assert!(agg.source_breakdown.contains_key("RSS"));
        assert!(agg.source_breakdown.contains_key("Twitter"));
    }

    #[test]
    fn test_aggregate_sentiment_no_signals() {
        let router = SentimentRouter::new();
        assert!(router.aggregate_sentiment("NVDA").is_none());
    }

    #[test]
    fn test_rss_parser() {
        let xml = r#"<?xml version="1.0"?>
<rss version="2.0">
  <channel>
    <title>Test Feed</title>
    <item>
      <title>AAPL hits new high</title>
      <description>Apple stock reaches all-time high today</description>
      <link>https://example.com/1</link>
    </item>
    <item>
      <title>Market update</title>
      <description>Broad market rally continues</description>
      <link>https://example.com/2</link>
    </item>
  </channel>
</rss>"#;

        let articles = parse_rss_to_articles(xml, "TestFeed", &["AAPL".to_string()]);
        assert_eq!(articles.len(), 2);
        assert_eq!(articles[0].title, "AAPL hits new high");
        assert_eq!(articles[0].symbols, vec!["AAPL".to_string()]);
        assert!(articles[0].source.contains("RSS:TestFeed"));
        // Second article doesn't mention AAPL
        assert!(articles[1].symbols.is_empty());
    }

    #[test]
    fn test_source_enable_disable() {
        let mut router = SentimentRouter::new();
        let config = SentimentSourceConfig {
            id: Uuid::new_v4(),
            name: "Test".to_string(),
            source_type: SentimentSourceType::Webhook,
            endpoint: "".to_string(),
            api_key: None,
            enabled: true,
            poll_interval_secs: 60,
            symbols_filter: vec![],
        };
        let id = router.add_source(config);

        assert!(router.get_source(&id).unwrap().enabled);
        router.set_source_enabled(&id, false);
        assert!(!router.get_source(&id).unwrap().enabled);
    }

    #[test]
    fn test_webhook_source_push_and_fetch() {
        let source = WebhookSource::new();
        source.push_article(NewsArticle {
            id: "wh-1".to_string(),
            title: "Breaking news".to_string(),
            content: "Important update".to_string(),
            source: "webhook".to_string(),
            author: None,
            url: "".to_string(),
            published_at: Utc::now(),
            symbols: vec!["AAPL".to_string()],
            sentiment: SentimentAnalysis {
                overall: SentimentLabel::Neutral,
                score: 0.0,
                confidence: 0.0,
                aspects: HashMap::new(),
            },
            relevance_score: 0.0,
            category: NewsCategory::BreakingNews,
        });

        let store = source.store();
        let articles = store.lock().unwrap();
        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].title, "Breaking news");
    }

    #[test]
    fn test_sentiment_source_type_display() {
        assert_eq!(SentimentSourceType::SecureYeoman.to_string(), "SecureYeoman");
        assert_eq!(SentimentSourceType::RssFeed.to_string(), "RSS");
        assert_eq!(SentimentSourceType::Webhook.to_string(), "Webhook");
    }
}
