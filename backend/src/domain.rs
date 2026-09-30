//! Слой приложения: контракты хранения и правила домена.

use chrono::{DateTime, Duration, Utc};

use crate::api::api_types::{EventType, Slot};

/// Окно записи — константа от текущей даты (решение спеки; не настраивается).
pub const BOOKING_WINDOW_DAYS: i64 = 14;

/// Репозиторий типов встреч; имплементации живут в инфраструктуре.
pub trait EventTypesRepository: Send + Sync {
    fn list(&self) -> Vec<EventType>;

    fn get(&self, id: &str) -> Option<EventType>;

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

/// Репозиторий слотов; имплементации живут в инфраструктуре.
pub trait SlotsRepository: Send + Sync {
    fn list(&self) -> Vec<Slot>;

    fn add(&self, slot: Slot);
}

/// Слот виден в календаре записи, только если начинается в окне 14 дней:
/// не в прошлом и не позже 14-го дня включительно.
pub fn is_within_booking_window(start: DateTime<Utc>, now: DateTime<Utc>) -> bool {
    start >= now && start <= now + Duration::days(BOOKING_WINDOW_DAYS)
}

/// Причины отклонения слота сервером.
#[derive(Debug)]
pub enum SlotValidationError {
    EndBeforeStart,
    OutsideBookingWindow,
}

/// Серверная валидация слота: интервал непустой, начало в окне 14 дней.
pub fn validate_slot(slot: &Slot, now: DateTime<Utc>) -> Result<(), SlotValidationError> {
    if slot.end_date_time <= slot.start_date_time {
        return Err(SlotValidationError::EndBeforeStart);
    }
    if !is_within_booking_window(slot.start_date_time, now) {
        return Err(SlotValidationError::OutsideBookingWindow);
    }
    Ok(())
}
