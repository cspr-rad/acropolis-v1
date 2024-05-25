use crate::{state::StateType, verifier::Receipt};
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
    Path(gov_key): Path<Vec<u8>>,
) -> impl IntoResponse {
    let state = state.lock().unwrap();
    match k256::ecdsa::VerifyingKey::from_encoded_point(&EncodedPoint::from_bytes(gov_key).unwrap())
    {
        Ok(key) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!(state.fetch_census_votes(key))),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({"Error": e.to_string()})),
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
