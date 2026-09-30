//! Слой приложения: контракты хранения в терминах домена.

use crate::api::api_types::EventType;

/// Репозиторий типов встреч; имплементации живут в инфраструктуре.
pub trait EventTypesRepository: Send + Sync {
    fn list(&self) -> Vec<EventType>;
}
