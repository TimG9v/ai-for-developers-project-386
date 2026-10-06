//! Инфраструктура: in-memory хранилища типов встреч и слотов.
//! Данные не переживают перезапуск сервера (решение спеки — до введения БД).

use std::sync::Mutex;

use chrono::{DateTime, Utc};

use crate::api::api_types::{Booking, EventType, Slot};
use crate::domain::{BookingsRepository, EventTypesRepository, SlotsRepository};

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

    fn get(&self, id: &str) -> Option<Slot> {
        self.items
            .lock()
            .expect("slots lock")
            .iter()
            .find(|slot| slot.id == id)
            .cloned()
    }

    fn add(&self, slot: Slot) {
        self.items.lock().expect("slots lock").push(slot);
    }
}

/// Запись с интервалом её слота: интервал нужен атомарной проверке
/// занятости времени; времена слота неизменяемы, поэтому фиксируются
/// в момент записи (ADR 0003).
struct BookingRecord {
    booking: Booking,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

#[derive(Default)]
pub struct InMemoryBookings {
    items: Mutex<Vec<BookingRecord>>,
}

impl InMemoryBookings {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BookingsRepository for InMemoryBookings {
    fn try_add(&self, booking: Booking, slot: &Slot) -> bool {
        // Проверка занятости интервала и вставка — под одной блокировкой:
        // два одновременных запроса (в т.ч. на слоты разных типов с одним
        // временем) не создадут пересекающиеся записи.
        let mut items = self.items.lock().expect("bookings lock");
        let overlaps = items
            .iter()
            .any(|existing| intervals_overlap(existing.start, existing.end, slot));
        if overlaps {
            return false;
        }
        items.push(BookingRecord {
            booking,
            start: slot.start_date_time,
            end: slot.end_date_time,
        });
        true
    }

    fn contains_slot(&self, slot_id: &str) -> bool {
        self.items
            .lock()
            .expect("bookings lock")
            .iter()
            .any(|record| record.booking.slot_id == slot_id)
    }

    fn list(&self) -> Vec<Booking> {
        self.items
            .lock()
            .expect("bookings lock")
            .iter()
            .map(|record| record.booking.clone())
            .collect()
    }
}

/// Пересечение полуоткрытых интервалов [start, end): стык (end == start
/// соседнего) пересечением не считается.
fn intervals_overlap(start: DateTime<Utc>, end: DateTime<Utc>, slot: &Slot) -> bool {
    start < slot.end_date_time && slot.start_date_time < end
}
