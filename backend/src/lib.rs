pub mod api;
pub mod domain;
pub mod infra;

use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;

use crate::domain::EventTypesRepository;

/// Состояние приложения: репозитории, с которыми собран роутер.
#[derive(Clone)]
pub struct AppState {
    pub event_types: Arc<dyn EventTypesRepository>,
}

#[derive(Serialize)]
struct HealthStatus {
    status: &'static str,
}

/// Роутер с дефолтным in-memory состоянием.
pub fn app() -> Router {
    app_with_state(AppState {
        event_types: Arc::new(infra::InMemoryEventTypes::new()),
    })
}

/// Роутер с подставленным состоянием (тесты собирают его сами).
pub fn app_with_state(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/event-types", get(list_event_types))
        .with_state(state)
}

async fn health() -> Json<HealthStatus> {
    Json(HealthStatus { status: "ok" })
}

async fn list_event_types(State(state): State<AppState>) -> Json<Vec<api::api_types::EventType>> {
    Json(state.event_types.list())
}
