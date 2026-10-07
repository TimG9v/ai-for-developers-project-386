//! Слой приложения: контракты хранения и правила домена.

use chrono::{DateTime, Duration, Utc};

use crate::api::api_types::{Booking, EventType, Slot};

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

    fn get(&self, id: &str) -> Option<Slot>;

    fn add(&self, slot: Slot);
}

/// Репозиторий записей; имплементации живут в инфраструктуре.
pub trait BookingsRepository: Send + Sync {
    /// Атомарный insert-if-absent: записи не пересекаются по интервалу
    /// времени слота — ни дважды на одном слоте, ни между слотами разных
    /// типов встреч (ADR 0003). Возвращает false, если интервал занят.
    fn try_add(&self, booking: Booking, slot: &Slot) -> bool;

    /// Занят ли слот какой-либо записью.
    fn contains_slot(&self, slot_id: &str) -> bool;

    /// Занят ли интервал времени какой-либо записью (ADR 0003): true —
    /// пересекается с интервалом хотя бы одной записи.
    fn is_interval_taken(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> bool;

    /// Все записи — ракурс владельца (история 4).
    fn list(&self) -> Vec<Booking>;
}

/// Слот виден в календаре записи, только если начинается в окне 14 дней:
/// не в прошлом и не позже 14-го дня включительно.
pub fn is_within_booking_window(start: DateTime<Utc>, now: DateTime<Utc>) -> bool {
    start >= now && start <= now + Duration::days(BOOKING_WINDOW_DAYS)
}

/// Шаг сетки начала слота — 30 минут (обязательное требование проекта).
const SLOT_GRID_SECONDS: i64 = 30 * 60;

/// Начало слота на 30-минутной сетке: …:00 / …:30, секунды и доли — ноль.
/// Суб-секунды проверяются отдельно: timestamp() их отбрасывает.
pub fn is_on_grid(start: DateTime<Utc>) -> bool {
    start.timestamp_subsec_nanos() == 0 && start.timestamp().rem_euclid(SLOT_GRID_SECONDS) == 0
}

/// Причины отклонения слота сервером.
#[derive(Debug)]
pub enum SlotValidationError {
    EndBeforeStart,
    OutsideBookingWindow,
    DurationMismatch,
    OffGridStart,
}

/// Серверная валидация слота: интервал непустой, начало в окне 14 дней,
/// длительность интервала равна длительности типа встречи
/// (словарь: «длительность слота определяется его типом встречи»),
/// начало на 30-минутной сетке.
pub fn validate_slot(
    slot: &Slot,
    now: DateTime<Utc>,
    event_type: &EventType,
) -> Result<(), SlotValidationError> {
    if slot.end_date_time <= slot.start_date_time {
        return Err(SlotValidationError::EndBeforeStart);
    }
    if !is_within_booking_window(slot.start_date_time, now) {
        return Err(SlotValidationError::OutsideBookingWindow);
    }
    let actual_minutes = (slot.end_date_time - slot.start_date_time).num_minutes();
    if actual_minutes != i64::from(event_type.duration_minutes) {
        return Err(SlotValidationError::DurationMismatch);
    }
    if !is_on_grid(slot.start_date_time) {
        return Err(SlotValidationError::OffGridStart);
    }
    Ok(())
}

/// Причины отклонения записи гостя сервером.
#[derive(Debug)]
pub enum BookingValidationError {
    EmptyGuestName,
    EmptyGuestEmail,
    SlotOutsideWindow,
}

/// Серверная валидация записи: имя и email гостя обязательны (история 13),
/// слот записи — в окне 14 дней (история 16: правило едино для всех).
/// Занятость слота проверяется отдельно и атомарно (try_add).
pub fn validate_booking(
    booking: &Booking,
    slot: &Slot,
    now: DateTime<Utc>,
) -> Result<(), BookingValidationError> {
    if booking.guest_name.trim().is_empty() {
        return Err(BookingValidationError::EmptyGuestName);
    }
    if booking.guest_email.trim().is_empty() {
        return Err(BookingValidationError::EmptyGuestEmail);
    }
    if !is_within_booking_window(slot.start_date_time, now) {
        return Err(BookingValidationError::SlotOutsideWindow);
    }
    Ok(())
}
