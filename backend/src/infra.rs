//! Инфраструктура: in-memory хранилища типов встреч и слотов.
//! Данные не переживают перезапуск сервера (решение спеки — до введения БД).

use std::sync::Mutex;

use crate::api::api_types::{EventType, Slot};
use crate::domain::{EventTypesRepository, SlotsRepository};

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

    fn get(&self, id: &str) -> Option<EventType> {
        self.items
            .lock()
            .expect("event types lock")
            .iter()
            .find(|event_type| event_type.id == id)
            .cloned()
    }

    fn add(&self, event_type: EventType) {
        self.items
            .lock()
            .expect("event types lock")
            .push(event_type);
    }
}

#[derive(Default)]
pub struct InMemorySlots {
    items: Mutex<Vec<Slot>>,
}

impl InMemorySlots {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SlotsRepository for InMemorySlots {
    fn list(&self) -> Vec<Slot> {
        self.items.lock().expect("slots lock").clone()
    }

    fn add(&self, slot: Slot) {
        self.items.lock().expect("slots lock").push(slot);
    }
}
