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
//! | Method | Path               | Description              |
//! |--------|--------------------|--------------------------|
//! | GET    | /health            | Health check             |
//! | POST   | /v1/orders         | Submit a trading order   |
//! | GET    | /v1/positions      | List open positions      |
//! | GET    | /v1/account        | Get account details      |
//! | DELETE | /v1/orders/:id     | Cancel an open order     |

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use bullshift_core::trading::api::{AlpacaApi, ApiOrderRequest, TradingApi, TradingCredentials};
use std::sync::Arc;
use tokio::net::TcpListener;

struct AppState {
    api: AlpacaApi,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let api_key = std::env::var("ALPACA_API_KEY")
        .expect("ALPACA_API_KEY env var is required");
    let api_secret = std::env::var("ALPACA_API_SECRET")
        .expect("ALPACA_API_SECRET env var is required");

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

    let state = Arc::new(AppState {
        api: AlpacaApi::new(credentials),
    });

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/v1/orders", post(submit_order_handler))
        .route("/v1/positions", get(get_positions_handler))
        .route("/v1/account", get(get_account_handler))
        .route("/v1/orders/:id", delete(cancel_order_handler))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    log::info!("BullShift API server listening on {}", addr);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    axum::serve(listener, app).await.expect("Server error");
}

async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok", "service": "bullshift-api" }))
}

async fn submit_order_handler(
    State(state): State<Arc<AppState>>,
    Json(order): Json<ApiOrderRequest>,
) -> impl IntoResponse {
    match state.api.submit_order(order).await {
        Ok(resp) => match serde_json::to_value(resp) {
            Ok(val) => (StatusCode::OK, Json(val)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": format!("Serialization error: {}", e) }))).into_response(),
        },
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": e })),
        )
            .into_response(),
    }
}

async fn get_positions_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.api.get_positions().await {
        Ok(positions) => match serde_json::to_value(positions) {
            Ok(val) => (StatusCode::OK, Json(val)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": format!("Serialization error: {}", e) }))).into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )
            .into_response(),
    }
}

async fn get_account_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.api.get_account().await {
        Ok(account) => match serde_json::to_value(account) {
            Ok(val) => (StatusCode::OK, Json(val)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": format!("Serialization error: {}", e) }))).into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
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
            Json(serde_json::json!({ "error": e })),
        )
            .into_response(),
    }
}

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
