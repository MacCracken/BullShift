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
//! | Method | Path                        | Description                  |
//! |--------|-----------------------------|------------------------------|
//! | GET    | /health                     | Health check                 |
//! | POST   | /v1/orders                  | Submit a trading order       |
//! | GET    | /v1/positions               | List open positions          |
//! | GET    | /v1/account                 | Get account details          |
//! | DELETE | /v1/orders/:id              | Cancel an open order         |
//! | GET    | /v1/ai/providers            | List AI providers            |
//! | POST   | /v1/ai/providers            | Add an AI provider           |
//! | POST   | /v1/ai/providers/:id/configure | Store API key for provider |
//! | POST   | /v1/ai/providers/:id/test   | Test provider connection     |
//! | POST   | /v1/ai/chat                 | Send a chat request          |

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use bullshift_core::ai_bridge::{AIProvider, AIProviderType, BearlyManaged};
use bullshift_core::security::SecurityManager;
use bullshift_core::trading::api::{AlpacaApi, ApiOrderRequest, TradingApi, TradingCredentials};
use serde::Deserialize;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use uuid::Uuid;

struct AppState {
    api: AlpacaApi,
    ai: Mutex<BearlyManaged>,
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
    });

    let app = Router::new()
        .route("/health", get(health_handler))
        // Trading endpoints
        .route("/v1/orders", post(submit_order_handler))
        .route("/v1/positions", get(get_positions_handler))
        .route("/v1/account", get(get_account_handler))
        .route("/v1/orders/:id", delete(cancel_order_handler))
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

    // Use the internal send helper via a direct HTTP call matching the provider type
    // For now, we forward through BearlyManaged's public interface
    drop(ai); // release lock before async network call

    // Re-acquire for the send — BearlyManaged::send_ai_request is private,
    // so we use execute_prompt or build a minimal approach.
    // Since send_ai_request is private, we need a public chat method.
    // For now, return a structured error indicating the chat endpoint needs
    // a public send method on BearlyManaged.

    // Actually, let's just make the HTTP call directly here using reqwest,
    // matching the provider type. This avoids needing to expose BearlyManaged internals.
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

    let content = data["choices"][0]["message"]["content"]
        .as_str()
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

    let content = data["content"][0]["text"]
        .as_str()
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

    fn make_app() -> Router {
        // Tests use a stub state — credentials are fake, network calls are not made
        // Integration tests against a live Alpaca sandbox should be run separately
        Router::new().route("/health", get(health_handler))
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
}
