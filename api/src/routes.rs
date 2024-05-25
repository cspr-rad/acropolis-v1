use crate::{state::StateType, utils, verifier::Receipt};
use axum::response::IntoResponse;
use axum::{
    extract::{Path, State},
    http::header,
    http::StatusCode,
    routing::get,
    routing::post,
    Json, Router,
};
use k256::EncodedPoint;
use serde_json::json;

// Router configuring all accessible API endpoints
pub fn app_router() -> Router<StateType> {
    let mut router = Router::new();
    // Add default endpoints
    router = router
        .route("/ping", get(ping))
        .route("/submit_receipt", post(submit_receipt))
        .route("/fetch_elections", get(fetch_elections))
        .route("/fetch_votes/:gov_key", get(fetch_votes));
    // add 404 error handler
    router = router.fallback(handler_404);
    router
}

async fn submit_receipt(
    State(state): State<StateType>,
    Json(body): Json<Receipt>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.process_receipt(body);
    (StatusCode::OK, "Receipt received")
}

async fn fetch_elections(State(state): State<StateType>) -> impl IntoResponse {
    let state = state.lock().unwrap();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        Json(json!(state.get_all_gov_keys())),
    )
}

async fn fetch_votes(
    State(state): State<StateType>,
    Path(gov_key): Path<String>,
) -> impl IntoResponse {
    let state = state.lock().unwrap();

    let gov_key_bytes = match utils::from_hex_with_prefix(&gov_key) {
        Ok(bytes) => bytes,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                [(header::CONTENT_TYPE, "application/json")],
                Json(json!({"error": "Invalid hex encoding"})),
            )
        }
    };

    let encoded_point = match EncodedPoint::from_bytes(&gov_key_bytes) {
        Ok(point) => point,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                [(header::CONTENT_TYPE, "application/json")],
                Json(json!({"error": "Invalid encoded point"})),
            )
        }
    };

    match k256::ecdsa::VerifyingKey::from_encoded_point(&encoded_point) {
        Ok(key) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!(state.fetch_census_votes(key))),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({"error": e.to_string()})),
        ),
    }
}

// Ping endpoint for debugging - TODO return DateTime of API server
async fn ping() -> &'static str {
    "Pong!"
}

// 404 - TODO return response in JSON
async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource could not be found.",
    )
}
