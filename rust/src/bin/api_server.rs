//! BullShift REST API server
//!
//! Wraps the BullShift trading engine as an HTTP API so external tools
//! (e.g. SecureYeoman MCP) can execute trades without FFI.
//!
//! # Configuration (env vars)
//!
//! | Variable           | Default   | Description                                    |
//! |--------------------|-----------|------------------------------------------------|
//! | `ALPACA_API_KEY`   | required  | Alpaca API key ID                              |
//! | `ALPACA_API_SECRET`| required  | Alpaca API secret key                          |
//! | `ALPACA_SANDBOX`   | `true`    | Set to `false` for live (non-paper) trading    |
//! | `BULLSHIFT_PORT`   | `8787`    | TCP port the server listens on                 |
//!
//! # Endpoints
//!
//! | Method | Path                        | Description                        |
//! |--------|-----------------------------|------------------------------------|
//! | GET    | /health                     | Health check                       |
//! | POST   | /v1/orders                  | Submit a trading order             |
//! | GET    | /v1/positions               | List open positions                |
//! | GET    | /v1/account                 | Get account details                |
//! | DELETE | /v1/orders/:id              | Cancel an open order               |
//! | GET    | /v1/market/:symbol          | Get market quote for a symbol      |
//! | GET    | /v1/algo/strategies         | List algo strategies + performance |
//! | GET    | /v1/algo/strategies/:id     | Get a single algo strategy         |
//! | GET    | /v1/algo/signals            | Get recent algo signals            |
//! | GET    | /v1/sentiment               | Get aggregated sentiment           |
//! | GET    | /v1/sentiment/:symbol       | Get sentiment for a symbol         |
//! | GET    | /v1/sentiment/signals       | Get recent sentiment signals       |
//! | GET    | /v1/alerts                  | List active alerts                 |
//! | POST   | /v1/alerts                  | Create an alert rule               |
//! | GET    | /v1/alerts/rules            | List all alert rules               |
//! | DELETE | /v1/alerts/rules/:id        | Delete an alert rule               |
//! | GET    | /v1/ai/providers            | List AI providers                  |
//! | POST   | /v1/ai/providers            | Add an AI provider                 |
//! | POST   | /v1/ai/providers/:id/configure | Store API key for provider      |
//! | POST   | /v1/ai/providers/:id/test   | Test provider connection           |
//! | POST   | /v1/ai/chat                 | Send a chat request                |

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use bullshift_core::ai_bridge::{AIProvider, AIProviderType, BearlyManaged};
use bullshift_core::algo::{AlgoEngine, AlgoParameters, AlgoStrategyType};
use bullshift_core::monitoring::{AlertCondition, AlertManager, AlertRule, AlertSeverity};
use bullshift_core::security::SecurityManager;
use bullshift_core::sentiment::SentimentRouter;
use bullshift_core::trading::api::{AlpacaApi, ApiOrderRequest, TradingApi, TradingCredentials};
use serde::Deserialize;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use uuid::Uuid;

struct AppState {
    api: AlpacaApi,
    ai: Mutex<BearlyManaged>,
    algo: Mutex<AlgoEngine>,
    sentiment: Mutex<SentimentRouter>,
    alerts: Mutex<AlertManager>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let api_key = std::env::var("ALPACA_API_KEY").expect("ALPACA_API_KEY env var is required");
    let api_secret =
        std::env::var("ALPACA_API_SECRET").expect("ALPACA_API_SECRET env var is required");

    // Default to sandbox — explicit opt-in required for live trading
    let sandbox = std::env::var("ALPACA_SANDBOX")
        .map(|v| v.to_lowercase() != "false")
        .unwrap_or(true);

    let port: u16 = std::env::var("BULLSHIFT_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8787);

    let credentials = TradingCredentials {
        api_key,
        api_secret,
        sandbox,
    };

    let security_manager = SecurityManager::new().expect("Failed to initialize SecurityManager");
    let bearly = BearlyManaged::new(security_manager);

    let state = Arc::new(AppState {
        api: AlpacaApi::new(credentials),
        ai: Mutex::new(bearly),
        algo: Mutex::new(AlgoEngine::new()),
        sentiment: Mutex::new(SentimentRouter::new()),
        alerts: Mutex::new(AlertManager::new()),
    });

    let app = Router::new()
        .route("/health", get(health_handler))
        // Trading endpoints
        .route("/v1/orders", post(submit_order_handler))
        .route("/v1/positions", get(get_positions_handler))
        .route("/v1/account", get(get_account_handler))
        .route("/v1/orders/:id", delete(cancel_order_handler))
        // Market data endpoint
        .route("/v1/market/:symbol", get(market_data_handler))
        // Algo strategy endpoints
        .route(
            "/v1/algo/strategies",
            get(list_strategies_handler).post(create_strategy_handler),
        )
        .route("/v1/algo/strategies/:id", get(get_strategy_handler))
        .route("/v1/algo/signals", get(recent_signals_handler))
        // Sentiment endpoints
        .route("/v1/sentiment", get(aggregate_sentiment_handler))
        .route("/v1/sentiment/:symbol", get(symbol_sentiment_handler))
        .route("/v1/sentiment/signals", get(sentiment_signals_handler))
        // Alert endpoints
        .route(
            "/v1/alerts",
            get(list_active_alerts_handler).post(create_alert_rule_handler),
        )
        .route("/v1/alerts/rules", get(list_alert_rules_handler))
        .route("/v1/alerts/rules/:id", delete(delete_alert_rule_handler))
        // AI provider endpoints
        .route("/v1/ai/providers", get(list_providers_handler))
        .route("/v1/ai/providers", post(add_provider_handler))
        .route(
            "/v1/ai/providers/:id/configure",
            post(configure_provider_handler),
        )
        .route("/v1/ai/providers/:id/test", post(test_provider_handler))
        .route("/v1/ai/chat", post(chat_handler))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    log::info!("BullShift API server listening on {}", addr);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    axum::serve(listener, app).await.expect("Server error");
}

// ---------------------------------------------------------------------------
// Health
// ---------------------------------------------------------------------------

async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok", "service": "bullshift-api" }))
}

// ---------------------------------------------------------------------------
// Trading endpoints
// ---------------------------------------------------------------------------

async fn submit_order_handler(
    State(state): State<Arc<AppState>>,
    Json(order): Json<ApiOrderRequest>,
) -> impl IntoResponse {
    match state.api.submit_order(order).await {
        Ok(resp) => match serde_json::to_value(resp) {
            Ok(val) => (StatusCode::OK, Json(val)).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("Serialization error: {}", e) })),
            )
                .into_response(),
        },
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("{}", e) })),
        )
            .into_response(),
    }
}

async fn get_positions_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.api.get_positions().await {
        Ok(positions) => match serde_json::to_value(positions) {
            Ok(val) => (StatusCode::OK, Json(val)).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("Serialization error: {}", e) })),
            )
                .into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("{}", e) })),
        )
            .into_response(),
    }
}

async fn get_account_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.api.get_account().await {
        Ok(account) => match serde_json::to_value(account) {
            Ok(val) => (StatusCode::OK, Json(val)).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("Serialization error: {}", e) })),
            )
                .into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("{}", e) })),
        )
            .into_response(),
    }
}

async fn cancel_order_handler(
    State(state): State<Arc<AppState>>,
    Path(order_id): Path<String>,
) -> impl IntoResponse {
    if order_id.contains('/') || order_id.contains("..") || order_id.len() > 64 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Invalid order ID" })),
        )
            .into_response();
    }
    match state.api.cancel_order(order_id).await {
        Ok(true) => (
            StatusCode::OK,
            Json(serde_json::json!({ "cancelled": true })),
        )
            .into_response(),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Order not found or already in a final state" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("{}", e) })),
        )
            .into_response(),
    }
}

// ---------------------------------------------------------------------------
// Market data endpoint
// ---------------------------------------------------------------------------

fn validate_symbol(symbol: &str) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if symbol.is_empty() || symbol.len() > 10 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Symbol must be 1-10 characters" })),
        ));
    }
    if !symbol
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Symbol contains invalid characters" })),
        ));
    }
    Ok(())
}

async fn market_data_handler(
    State(state): State<Arc<AppState>>,
    Path(symbol): Path<String>,
) -> impl IntoResponse {
    if let Err(e) = validate_symbol(&symbol) {
        return e.into_response();
    }
    match state.api.get_quote(&symbol).await {
        Ok(quote) => match serde_json::to_value(quote) {
            Ok(val) => (StatusCode::OK, Json(val)).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("Serialization error: {}", e) })),
            )
                .into_response(),
        },
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("{}", e) })),
        )
            .into_response(),
    }
}

// ---------------------------------------------------------------------------
// Algo strategy endpoints
// ---------------------------------------------------------------------------

async fn list_strategies_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let algo = state.algo.lock().await;
    let strategies: Vec<serde_json::Value> = algo
        .list_strategies()
        .iter()
        .map(|s| serde_json::to_value(s).unwrap_or_default())
        .collect();

    Json(serde_json::json!({ "strategies": strategies }))
}

#[derive(Deserialize)]
struct CreateStrategyRequest {
    name: String,
    strategy_type: String,
    #[serde(default)]
    parameters: Option<AlgoParameters>,
}

fn parse_strategy_type(s: &str) -> AlgoStrategyType {
    match s {
        "ma_crossover" | "MovingAverageCrossover" => AlgoStrategyType::MovingAverageCrossover,
        "mean_reversion" | "MeanReversion" => AlgoStrategyType::MeanReversion,
        "breakout" | "Breakout" => AlgoStrategyType::Breakout,
        "vwap" | "Vwap" => AlgoStrategyType::Vwap,
        "twap" | "Twap" => AlgoStrategyType::Twap,
        "grid" | "Grid" => AlgoStrategyType::Grid,
        "trailing_stop" | "TrailingStop" => AlgoStrategyType::TrailingStop,
        "pairs" | "PairsTrading" => AlgoStrategyType::PairsTrading,
        other => AlgoStrategyType::Custom(other.to_string()),
    }
}

async fn create_strategy_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateStrategyRequest>,
) -> impl IntoResponse {
    let mut algo = state.algo.lock().await;
    let strategy_type = parse_strategy_type(&req.strategy_type);
    let params = req.parameters.unwrap_or_default();
    let id = algo.add_strategy(&req.name, strategy_type, params);

    (
        StatusCode::CREATED,
        Json(serde_json::json!({ "id": id.to_string() })),
    )
        .into_response()
}

async fn get_strategy_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let strategy_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Invalid strategy ID" })),
            )
                .into_response()
        }
    };

    let algo = state.algo.lock().await;
    match algo.get_strategy(&strategy_id) {
        Some(strategy) => match serde_json::to_value(strategy) {
            Ok(val) => (StatusCode::OK, Json(val)).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("Serialization error: {}", e) })),
            )
                .into_response(),
        },
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Strategy not found" })),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
struct SignalsQuery {
    #[serde(default = "default_signal_limit")]
    limit: usize,
}

fn default_signal_limit() -> usize {
    50
}

const MAX_QUERY_LIMIT: usize = 1000;

fn clamp_limit(limit: usize) -> usize {
    limit.min(MAX_QUERY_LIMIT)
}

async fn recent_signals_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SignalsQuery>,
) -> impl IntoResponse {
    let algo = state.algo.lock().await;
    let signals: Vec<serde_json::Value> = algo
        .recent_signals(clamp_limit(query.limit))
        .iter()
        .map(|s| serde_json::to_value(s).unwrap_or_default())
        .collect();

    Json(serde_json::json!({ "signals": signals }))
}

// ---------------------------------------------------------------------------
// Sentiment endpoints
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct SentimentQuery {
    #[serde(default)]
    symbol: Option<String>,
}

async fn aggregate_sentiment_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SentimentQuery>,
) -> impl IntoResponse {
    let sentiment = state.sentiment.lock().await;

    if let Some(symbol) = &query.symbol {
        match sentiment.aggregate_sentiment(symbol) {
            Some(agg) => match serde_json::to_value(&agg) {
                Ok(val) => (StatusCode::OK, Json(val)).into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": format!("Serialization error: {}", e) })),
                )
                    .into_response(),
            },
            None => (
                StatusCode::OK,
                Json(serde_json::json!({ "symbol": symbol, "overall_score": 0.0, "signal_count": 0 })),
            )
                .into_response(),
        }
    } else {
        let sources: Vec<serde_json::Value> = sentiment
            .list_sources()
            .iter()
            .map(|s| serde_json::to_value(s).unwrap_or_default())
            .collect();
        let signals: Vec<serde_json::Value> = sentiment
            .recent_signals(20, None)
            .iter()
            .map(|s| serde_json::to_value(s).unwrap_or_default())
            .collect();

        Json(serde_json::json!({
            "sources": sources,
            "recent_signals": signals,
        }))
        .into_response()
    }
}

async fn symbol_sentiment_handler(
    State(state): State<Arc<AppState>>,
    Path(symbol): Path<String>,
) -> impl IntoResponse {
    let sentiment = state.sentiment.lock().await;
    let aggregate = sentiment.aggregate_sentiment(&symbol);
    let signals: Vec<serde_json::Value> = sentiment
        .signals_for_symbol(&symbol, 50)
        .iter()
        .map(|s| serde_json::to_value(s).unwrap_or_default())
        .collect();

    Json(serde_json::json!({
        "symbol": symbol,
        "aggregate": aggregate,
        "signals": signals,
    }))
}

#[derive(Deserialize)]
struct SentimentSignalsQuery {
    #[serde(default = "default_signal_limit")]
    limit: usize,
}

async fn sentiment_signals_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SentimentSignalsQuery>,
) -> impl IntoResponse {
    let sentiment = state.sentiment.lock().await;
    let signals: Vec<serde_json::Value> = sentiment
        .recent_signals(clamp_limit(query.limit), None)
        .iter()
        .map(|s| serde_json::to_value(s).unwrap_or_default())
        .collect();

    Json(serde_json::json!({ "signals": signals }))
}

// ---------------------------------------------------------------------------
// Alert endpoints
// ---------------------------------------------------------------------------

async fn list_active_alerts_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let alerts = state.alerts.lock().await;
    let active: Vec<serde_json::Value> = alerts
        .active_alerts()
        .iter()
        .map(|a| serde_json::to_value(a).unwrap_or_default())
        .collect();

    Json(serde_json::json!({ "alerts": active }))
}

#[derive(Deserialize)]
struct CreateAlertRuleRequest {
    name: String,
    metric_name: String,
    condition: String,
    threshold: f64,
    #[serde(default = "default_severity")]
    severity: String,
    #[serde(default = "default_cooldown")]
    cooldown_seconds: u64,
}

fn default_severity() -> String {
    "warning".to_string()
}
fn default_cooldown() -> u64 {
    300
}

fn parse_alert_condition(s: &str) -> AlertCondition {
    match s {
        "greater_than" | "gt" | ">" | "GreaterThan" => AlertCondition::GreaterThan,
        "less_than" | "lt" | "<" | "LessThan" => AlertCondition::LessThan,
        "equal_to" | "eq" | "=" | "EqualTo" => AlertCondition::EqualTo,
        _ => AlertCondition::GreaterThan,
    }
}

fn parse_alert_severity(s: &str) -> AlertSeverity {
    match s {
        "info" | "Info" => AlertSeverity::Info,
        "warning" | "Warning" => AlertSeverity::Warning,
        "critical" | "Critical" => AlertSeverity::Critical,
        _ => AlertSeverity::Warning,
    }
}

async fn create_alert_rule_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateAlertRuleRequest>,
) -> impl IntoResponse {
    let rule = AlertRule {
        id: Uuid::new_v4(),
        name: req.name,
        metric_name: req.metric_name,
        condition: parse_alert_condition(&req.condition),
        threshold: req.threshold,
        severity: parse_alert_severity(&req.severity),
        enabled: true,
        cooldown_seconds: req.cooldown_seconds,
    };

    let rule_id = rule.id;
    let mut alerts = state.alerts.lock().await;
    alerts.add_rule(rule);

    (
        StatusCode::CREATED,
        Json(serde_json::json!({ "id": rule_id.to_string() })),
    )
        .into_response()
}

async fn list_alert_rules_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let alerts = state.alerts.lock().await;
    let rules: Vec<serde_json::Value> = alerts
        .rules()
        .iter()
        .map(|r| serde_json::to_value(r).unwrap_or_default())
        .collect();

    Json(serde_json::json!({ "rules": rules }))
}

async fn delete_alert_rule_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let rule_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Invalid rule ID" })),
            )
                .into_response()
        }
    };

    let mut alerts = state.alerts.lock().await;
    if alerts.remove_rule(&rule_id) {
        (StatusCode::OK, Json(serde_json::json!({ "deleted": true }))).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Rule not found" })),
        )
            .into_response()
    }
}

// ---------------------------------------------------------------------------
// AI provider endpoints
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct AddProviderRequest {
    name: String,
    provider_type: String,
    api_endpoint: String,
    model_name: String,
    #[serde(default)]
    api_key: String,
    #[serde(default = "default_max_tokens")]
    max_tokens: u32,
    #[serde(default = "default_temperature")]
    temperature: f64,
}

fn default_max_tokens() -> u32 {
    4096
}
fn default_temperature() -> f64 {
    0.7
}

fn parse_provider_type(s: &str) -> AIProviderType {
    match s {
        "OpenAI" => AIProviderType::OpenAI,
        "Anthropic" => AIProviderType::Anthropic,
        "Ollama" => AIProviderType::Ollama,
        "LocalLLM" | "Local LLM" => AIProviderType::LocalLLM,
        "SecureYeoman" => AIProviderType::SecureYeoman,
        _ => AIProviderType::Custom,
    }
}

async fn list_providers_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let ai = state.ai.lock().await;
    let providers: Vec<serde_json::Value> = ai
        .get_providers()
        .iter()
        .map(|p| {
            serde_json::json!({
                "id": p.id.to_string(),
                "name": p.name,
                "provider_type": format!("{:?}", p.provider_type),
                "api_endpoint": p.api_endpoint,
                "model_name": p.model_name,
                "is_configured": p.is_configured,
                "is_active": p.is_active,
                "max_tokens": p.max_tokens,
                "temperature": p.temperature,
                "has_api_key": ai.has_encrypted_api_key(&p.id),
            })
        })
        .collect();

    Json(serde_json::json!({ "providers": providers }))
}

async fn add_provider_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AddProviderRequest>,
) -> impl IntoResponse {
    let provider = AIProvider {
        id: Uuid::new_v4(),
        name: req.name,
        provider_type: parse_provider_type(&req.provider_type),
        api_endpoint: req.api_endpoint,
        api_key: req.api_key,
        model_name: req.model_name,
        is_configured: false,
        is_active: false,
        max_tokens: req.max_tokens,
        temperature: req.temperature,
        created_at: chrono::Utc::now(),
        last_used: None,
    };

    let provider_id = provider.id;
    let mut ai = state.ai.lock().await;
    match ai.add_provider(provider).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(serde_json::json!({ "id": provider_id.to_string() })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("{}", e) })),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
struct ConfigureProviderRequest {
    api_key: String,
}

async fn configure_provider_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<ConfigureProviderRequest>,
) -> impl IntoResponse {
    let provider_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Invalid provider ID" })),
            )
                .into_response()
        }
    };

    let mut ai = state.ai.lock().await;
    match ai.update_provider_api_key(&provider_id, &req.api_key) {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "configured": true })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("{}", e) })),
        )
            .into_response(),
    }
}

async fn test_provider_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let provider_id = Uuid::parse_str(&id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Invalid provider ID" })),
        )
    })?;

    // Get the provider endpoint info while holding the lock briefly
    let (api_endpoint, provider_type) = {
        let ai = state.ai.lock().await;
        let provider = ai.get_provider(&provider_id).ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "Provider not found" })),
            )
        })?;
        (
            provider.api_endpoint.clone(),
            format!("{:?}", provider.provider_type),
        )
    };

    // Test connection without holding the lock
    let client = reqwest::Client::new();
    let test_url = match provider_type.as_str() {
        "OpenAI" => format!("{}/models", api_endpoint),
        "Anthropic" => format!("{}/v1/messages", api_endpoint),
        "Ollama" => format!("{}/api/tags", api_endpoint),
        "SecureYeoman" => format!("{}/api/v1/health", api_endpoint),
        _ => format!("{}/health", api_endpoint),
    };

    let connected = match client.get(&test_url).send().await {
        Ok(resp) => {
            resp.status().is_success() || resp.status() == reqwest::StatusCode::UNAUTHORIZED
        }
        Err(_) => false,
    };

    Ok(Json(serde_json::json!({ "connected": connected })))
}

#[derive(Deserialize)]
struct ChatRequest {
    provider_id: String,
    prompt: String,
}

async fn chat_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ChatRequest>,
) -> impl IntoResponse {
    let provider_id = match Uuid::parse_str(&req.provider_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Invalid provider ID" })),
            )
                .into_response()
        }
    };

    let ai = state.ai.lock().await;
    let provider = match ai.get_provider(&provider_id) {
        Some(p) => p.clone(),
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "Provider not found" })),
            )
                .into_response()
        }
    };

    // Resolve the decrypted API key and send
    let decrypted_key = match ai.resolve_api_key(&provider) {
        Ok(k) => k,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("{}", e) })),
            )
                .into_response()
        }
    };

    // Build a temporary provider with the decrypted key for the request
    let mut provider_with_key = provider;
    provider_with_key.api_key = decrypted_key;

    drop(ai); // release lock before async network call

    let client = reqwest::Client::new();
    let result = match provider_with_key.provider_type {
        AIProviderType::OpenAI => send_openai_chat(&client, &provider_with_key, &req.prompt).await,
        AIProviderType::Anthropic => {
            send_anthropic_chat(&client, &provider_with_key, &req.prompt).await
        }
        AIProviderType::Ollama => send_ollama_chat(&client, &provider_with_key, &req.prompt).await,
        AIProviderType::SecureYeoman => {
            send_secureyeoman_chat(&client, &provider_with_key, &req.prompt).await
        }
        _ => send_generic_chat(&client, &provider_with_key, &req.prompt).await,
    };

    match result {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("{}", e) })),
        )
            .into_response(),
    }
}

// ---------------------------------------------------------------------------
// Chat dispatch helpers
// ---------------------------------------------------------------------------

async fn send_openai_chat(
    client: &reqwest::Client,
    provider: &AIProvider,
    prompt: &str,
) -> Result<serde_json::Value, String> {
    let url = format!("{}/chat/completions", provider.api_endpoint);
    let body = serde_json::json!({
        "model": provider.model_name,
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": provider.max_tokens,
        "temperature": provider.temperature,
    });

    let resp = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", provider.api_key))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("OpenAI API error ({}): {}", status, text));
    }

    let data: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    let content = data["choices"]
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|choice| choice["message"]["content"].as_str())
        .unwrap_or("")
        .to_string();
    let tokens = data["usage"]["total_tokens"].as_u64().unwrap_or(0);

    Ok(serde_json::json!({
        "response": content,
        "tokens_used": tokens,
        "provider": "OpenAI",
    }))
}

async fn send_anthropic_chat(
    client: &reqwest::Client,
    provider: &AIProvider,
    prompt: &str,
) -> Result<serde_json::Value, String> {
    let url = format!("{}/v1/messages", provider.api_endpoint);
    let body = serde_json::json!({
        "model": provider.model_name,
        "max_tokens": provider.max_tokens,
        "temperature": provider.temperature,
        "messages": [{"role": "user", "content": prompt}],
    });

    let resp = client
        .post(&url)
        .header("x-api-key", &provider.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("Anthropic API error ({}): {}", status, text));
    }

    let data: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    let content = data["content"]
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|block| block["text"].as_str())
        .unwrap_or("")
        .to_string();
    let tokens = data["usage"]["input_tokens"].as_u64().unwrap_or(0)
        + data["usage"]["output_tokens"].as_u64().unwrap_or(0);

    Ok(serde_json::json!({
        "response": content,
        "tokens_used": tokens,
        "provider": "Anthropic",
    }))
}

async fn send_ollama_chat(
    client: &reqwest::Client,
    provider: &AIProvider,
    prompt: &str,
) -> Result<serde_json::Value, String> {
    let url = format!("{}/api/generate", provider.api_endpoint);
    let body = serde_json::json!({
        "model": provider.model_name,
        "prompt": prompt,
        "stream": false,
        "options": {
            "temperature": provider.temperature,
            "num_predict": provider.max_tokens,
        },
    });

    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("Ollama API error ({}): {}", status, text));
    }

    let data: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    let content = data["response"].as_str().unwrap_or("").to_string();
    let tokens =
        data["prompt_eval_count"].as_u64().unwrap_or(0) + data["eval_count"].as_u64().unwrap_or(0);

    Ok(serde_json::json!({
        "response": content,
        "tokens_used": tokens,
        "provider": "Ollama",
    }))
}

async fn send_secureyeoman_chat(
    client: &reqwest::Client,
    provider: &AIProvider,
    prompt: &str,
) -> Result<serde_json::Value, String> {
    let url = format!("{}/api/v1/chat", provider.api_endpoint);
    let body = serde_json::json!({
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": provider.max_tokens,
        "temperature": provider.temperature,
    });

    let mut req = client.post(&url).json(&body);
    if !provider.api_key.is_empty() {
        req = req.header("Authorization", format!("Bearer {}", provider.api_key));
    }

    let resp = req
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("SecureYeoman API error ({}): {}", status, text));
    }

    let data: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    let content = data["choices"][0]["message"]["content"]
        .as_str()
        .or_else(|| data["response"].as_str())
        .or_else(|| data["content"].as_str())
        .unwrap_or("")
        .to_string();
    let tokens = data["usage"]["total_tokens"].as_u64().unwrap_or(0);

    Ok(serde_json::json!({
        "response": content,
        "tokens_used": tokens,
        "provider": "SecureYeoman",
    }))
}

async fn send_generic_chat(
    client: &reqwest::Client,
    provider: &AIProvider,
    prompt: &str,
) -> Result<serde_json::Value, String> {
    let url = format!("{}/completions", provider.api_endpoint);
    let body = serde_json::json!({
        "prompt": prompt,
        "model": provider.model_name,
        "max_tokens": provider.max_tokens,
        "temperature": provider.temperature,
    });

    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("API error ({}): {}", status, text));
    }

    let data: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    let content = data["text"]
        .as_str()
        .or_else(|| data["response"].as_str())
        .or_else(|| data["completion"].as_str())
        .unwrap_or("")
        .to_string();

    Ok(serde_json::json!({
        "response": content,
        "tokens_used": 0,
        "provider": "Custom",
    }))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    fn make_test_state() -> Arc<AppState> {
        let credentials = TradingCredentials {
            api_key: "test-key".to_string(),
            api_secret: "test-secret".to_string(),
            sandbox: true,
        };
        let security_manager = SecurityManager::new().unwrap();
        let bearly = BearlyManaged::new(security_manager);

        Arc::new(AppState {
            api: AlpacaApi::new(credentials),
            ai: Mutex::new(bearly),
            algo: Mutex::new(AlgoEngine::new()),
            sentiment: Mutex::new(SentimentRouter::new()),
            alerts: Mutex::new(AlertManager::new()),
        })
    }

    fn make_app() -> Router {
        let state = make_test_state();

        Router::new()
            .route("/health", get(health_handler))
            .route("/v1/market/:symbol", get(market_data_handler))
            .route(
                "/v1/algo/strategies",
                get(list_strategies_handler).post(create_strategy_handler),
            )
            .route("/v1/algo/strategies/:id", get(get_strategy_handler))
            .route("/v1/algo/signals", get(recent_signals_handler))
            .route("/v1/sentiment", get(aggregate_sentiment_handler))
            .route("/v1/sentiment/:symbol", get(symbol_sentiment_handler))
            .route("/v1/sentiment/signals", get(sentiment_signals_handler))
            .route(
                "/v1/alerts",
                get(list_active_alerts_handler).post(create_alert_rule_handler),
            )
            .route("/v1/alerts/rules", get(list_alert_rules_handler))
            .route("/v1/alerts/rules/:id", delete(delete_alert_rule_handler))
            .with_state(state)
    }

    #[tokio::test]
    async fn test_health_returns_ok() {
        let app = make_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_strategies_empty() {
        let app = make_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/algo/strategies")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["strategies"], serde_json::json!([]));
    }

    #[tokio::test]
    async fn test_create_and_get_strategy() {
        let state = make_test_state();
        let app = Router::new()
            .route(
                "/v1/algo/strategies",
                get(list_strategies_handler).post(create_strategy_handler),
            )
            .route("/v1/algo/strategies/:id", get(get_strategy_handler))
            .with_state(state);

        // Create a strategy
        let create_body = serde_json::json!({
            "name": "Test MA Crossover",
            "strategy_type": "ma_crossover",
            "parameters": {
                "symbols": ["AAPL", "TSLA"],
                "max_position_size": 1000.0,
                "max_total_exposure": 10000.0,
                "stop_loss_pct": 0.02,
                "take_profit_pct": 0.05,
                "custom": {},
            }
        });

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/algo/strategies")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&create_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let strategy_id = json["id"].as_str().unwrap();

        // Get the strategy
        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/algo/strategies/{}", strategy_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["name"], "Test MA Crossover");
    }

    #[tokio::test]
    async fn test_get_strategy_not_found() {
        let app = make_app();
        let fake_id = Uuid::new_v4();
        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/algo/strategies/{}", fake_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_recent_signals_empty() {
        let app = make_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/algo/signals")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["signals"], serde_json::json!([]));
    }

    #[tokio::test]
    async fn test_sentiment_overview() {
        let app = make_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/sentiment")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["sources"].is_array());
        assert!(json["recent_signals"].is_array());
    }

    #[tokio::test]
    async fn test_symbol_sentiment() {
        let app = make_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/sentiment/AAPL")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["symbol"], "AAPL");
    }

    #[tokio::test]
    async fn test_sentiment_signals_empty() {
        let app = make_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/sentiment/signals")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_alerts_empty() {
        let app = make_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/alerts")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["alerts"], serde_json::json!([]));
    }

    #[tokio::test]
    async fn test_create_and_list_alert_rules() {
        let state = make_test_state();
        let app = Router::new()
            .route(
                "/v1/alerts",
                get(list_active_alerts_handler).post(create_alert_rule_handler),
            )
            .route("/v1/alerts/rules", get(list_alert_rules_handler))
            .route("/v1/alerts/rules/:id", delete(delete_alert_rule_handler))
            .with_state(state);

        // Create a rule
        let rule_body = serde_json::json!({
            "name": "High CPU",
            "metric_name": "cpu_usage",
            "condition": "greater_than",
            "threshold": 90.0,
            "severity": "critical",
            "cooldown_seconds": 60,
        });

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/alerts")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&rule_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let rule_id = json["id"].as_str().unwrap().to_string();

        // List rules
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/alerts/rules")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["rules"].as_array().unwrap().len(), 1);

        // Delete rule
        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(format!("/v1/alerts/rules/{}", rule_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_alert_rule_not_found() {
        let app = make_app();
        let fake_id = Uuid::new_v4();
        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(format!("/v1/alerts/rules/{}", fake_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_parse_strategy_types() {
        assert!(matches!(
            parse_strategy_type("ma_crossover"),
            AlgoStrategyType::MovingAverageCrossover
        ));
        assert!(matches!(
            parse_strategy_type("mean_reversion"),
            AlgoStrategyType::MeanReversion
        ));
        assert!(matches!(
            parse_strategy_type("breakout"),
            AlgoStrategyType::Breakout
        ));
        assert!(matches!(
            parse_strategy_type("vwap"),
            AlgoStrategyType::Vwap
        ));
        assert!(matches!(
            parse_strategy_type("twap"),
            AlgoStrategyType::Twap
        ));
        assert!(matches!(
            parse_strategy_type("grid"),
            AlgoStrategyType::Grid
        ));
        assert!(matches!(
            parse_strategy_type("trailing_stop"),
            AlgoStrategyType::TrailingStop
        ));
        assert!(matches!(
            parse_strategy_type("pairs"),
            AlgoStrategyType::PairsTrading
        ));
        assert!(matches!(
            parse_strategy_type("custom_foo"),
            AlgoStrategyType::Custom(_)
        ));
    }

    #[tokio::test]
    async fn test_parse_alert_conditions() {
        assert!(matches!(
            parse_alert_condition("greater_than"),
            AlertCondition::GreaterThan
        ));
        assert!(matches!(
            parse_alert_condition("gt"),
            AlertCondition::GreaterThan
        ));
        assert!(matches!(
            parse_alert_condition("less_than"),
            AlertCondition::LessThan
        ));
        assert!(matches!(
            parse_alert_condition("lt"),
            AlertCondition::LessThan
        ));
        assert!(matches!(
            parse_alert_condition("equal_to"),
            AlertCondition::EqualTo
        ));
        assert!(matches!(
            parse_alert_condition("eq"),
            AlertCondition::EqualTo
        ));
    }

    #[tokio::test]
    async fn test_parse_alert_severities() {
        assert!(matches!(parse_alert_severity("info"), AlertSeverity::Info));
        assert!(matches!(
            parse_alert_severity("warning"),
            AlertSeverity::Warning
        ));
        assert!(matches!(
            parse_alert_severity("critical"),
            AlertSeverity::Critical
        ));
        assert!(matches!(
            parse_alert_severity("unknown"),
            AlertSeverity::Warning
        ));
    }
}
