pub mod api;
pub mod domain;
pub mod infra;

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Query, State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::api::api_types::{EventType, Slot};
use crate::domain::{EventTypesRepository, SlotsRepository};

/// Состояние приложения: репозитории, с которыми собран роутер.
#[derive(Clone)]
pub struct AppState {
    pub event_types: Arc<dyn EventTypesRepository>,
    pub slots: Arc<dyn SlotsRepository>,
}

#[derive(Serialize)]
struct HealthStatus {
    status: &'static str,
}

/// Роутер с дефолтным in-memory состоянием.
pub fn app() -> Router {
    app_with_state(AppState {
        event_types: Arc::new(infra::InMemoryEventTypes::new()),
        slots: Arc::new(infra::InMemorySlots::new()),
    })
}

/// Роутер с подставленным состоянием (тесты собирают его сами).
pub fn app_with_state(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route(
            "/event-types",
            get(list_event_types).post(create_event_type),
        )
        .route("/slots", get(list_slots).post(create_slot))
        .with_state(state)
}

async fn health() -> Json<HealthStatus> {
    Json(HealthStatus { status: "ok" })
}

async fn list_event_types(State(state): State<AppState>) -> Json<Vec<EventType>> {
    Json(state.event_types.list())
}

/// Создание типа встречи владельцем. Невалидный ввод — контрактный 400:
/// и нераспарсиваемое тело, и нарушение правил домена.
async fn create_event_type(
    State(state): State<AppState>,
    event_type: Result<Json<EventType>, JsonRejection>,
) -> Response {
    let event_type = match event_type {
        Ok(Json(event_type)) => event_type,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };
    if domain::validate_event_type(&event_type).is_err() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    state.event_types.add(event_type.clone());
    Json(event_type).into_response()
}

#[derive(Deserialize)]
struct SlotsQuery {
    #[serde(rename = "eventTypeId")]
    event_type_id: Option<String>,
}

/// Календарь записи: только свободные слоты выбранного типа в окне 14 дней.
/// Записей пока нет (#17) — «свободный» означает «существует и в окне».
async fn list_slots(State(state): State<AppState>, query: Query<SlotsQuery>) -> Json<Vec<Slot>> {
    let now: DateTime<Utc> = Utc::now();
    let slots = state
        .slots
        .list()
        .into_iter()
        .filter(|slot| {
            query
                .event_type_id
                .as_deref()
                .is_none_or(|id| slot.event_type_id == id)
        })
        .filter(|slot| domain::is_within_booking_window(slot.start_date_time, now))
        .collect();
    Json(slots)
}

/// Публикация слота владельцем. Несуществующий тип — контрактный 404,
/// слот вне окна или с пустым интервалом — контрактный 400.
async fn create_slot(
    State(state): State<AppState>,
    slot: Result<Json<Slot>, JsonRejection>,
) -> Response {
    let slot = match slot {
        Ok(Json(slot)) => slot,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };
    if state.event_types.get(&slot.event_type_id).is_none() {
        return StatusCode::NOT_FOUND.into_response();
    }
    if domain::validate_slot(&slot, Utc::now()).is_err() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    state.slots.add(slot.clone());
    Json(slot).into_response()
}
