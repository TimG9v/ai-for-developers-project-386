//! Инфраструктура: in-memory хранилище типов встреч.
//! Данные не переживают перезапуск сервера (решение спеки — до введения БД).

use std::sync::Mutex;

use crate::api::api_types::EventType;
use crate::domain::EventTypesRepository;

#[derive(Default)]
pub struct InMemoryEventTypes {
    items: Mutex<Vec<EventType>>,
}

impl InMemoryEventTypes {
    pub fn new() -> Self {
        Self::default()
    }
}

impl EventTypesRepository for InMemoryEventTypes {
    fn list(&self) -> Vec<EventType> {
        self.items.lock().expect("event types lock").clone()
    }

    fn add(&self, event_type: EventType) {
        self.items
            .lock()
            .expect("event types lock")
            .push(event_type);
    }
}
