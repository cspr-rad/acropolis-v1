use crate::{state::StateType, verifier::Receipt};
use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode, routing::get, routing::post, Json, Router};

// Router configuring all accessible API endpoints
pub fn app_router() -> Router<StateType> {
    let mut router = Router::new();
    // Add default endpoints
    router = router
        .route("/ping", get(ping))
        .route("/submit_receipt", post(submit_receipt));
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
