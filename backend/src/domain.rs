//! Слой приложения: контракты хранения в терминах домена.

use crate::api::api_types::EventType;

/// Репозиторий типов встреч; имплементации живут в инфраструктуре.
pub trait EventTypesRepository: Send + Sync {
    fn list(&self) -> Vec<EventType>;

    fn add(&self, event_type: EventType);
}

/// Причины отклонения типа встречи сервером.
#[derive(Debug)]
pub enum EventTypeValidationError {
    EmptyTitle,
    NonPositiveDuration,
}

/// Серверная валидация: название не пустое (пробелы по краям не считаются),
/// длительность положительная.
pub fn validate_event_type(event_type: &EventType) -> Result<(), EventTypeValidationError> {
    if event_type.title.trim().is_empty() {
        return Err(EventTypeValidationError::EmptyTitle);
    }
    if event_type.duration_minutes <= 0 {
        return Err(EventTypeValidationError::NonPositiveDuration);
    }
    Ok(())
}
