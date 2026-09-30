"use client";

import { useState } from "react";

import { slotsList, type EventType, type Slot } from "@/src/client";

import { formatSlotInterval } from "@/lib/slot-time";

/**
 * Страница записи: гость выбирает тип встречи и видит календарь свободных
 * слотов этого типа. Окно 14 дней отсекает сервер — клиент рендерит ответ.
 */
export function BookingEventTypes({
  initialEventTypes,
}: {
  initialEventTypes: EventType[];
}) {
  const [selectedEventTypeId, setSelectedEventTypeId] = useState<string | null>(
    null,
  );
  const [slots, setSlots] = useState<Slot[]>([]);

  const selectedEventType = initialEventTypes.find(
    (eventType) => eventType.id === selectedEventTypeId,
  );

  async function chooseEventType(eventTypeId: string) {
    setSelectedEventTypeId(eventTypeId);
    const { data } = await slotsList({ query: { eventTypeId } });
    setSlots(data ?? []);
  }

  return (
    <div className="flex w-full max-w-2xl flex-col items-center gap-6">
      <ul className="flex w-full flex-col gap-4">
        {initialEventTypes.map((eventType) => (
          <li
            key={eventType.id}
            className="rounded-xl border bg-card p-6 text-card-foreground"
          >
            <div className="flex items-start justify-between gap-4">
              <div>
                <h2 className="text-xl font-semibold">{eventType.title}</h2>
                {eventType.description && (
                  <p className="text-muted-foreground">
                    {eventType.description}
                  </p>
                )}
                <p className="text-sm text-muted-foreground">
                  {eventType.durationMinutes} мин.
                </p>
              </div>
              <button
                type="button"
                onClick={() => void chooseEventType(eventType.id)}
                aria-label={`Выбрать ${eventType.title}`}
                className={`shrink-0 rounded-lg px-4 py-2 ${
                  eventType.id === selectedEventTypeId
                    ? "bg-primary text-primary-foreground"
                    : "border bg-background"
                }`}
              >
                {eventType.id === selectedEventTypeId ? "Выбран" : "Выбрать"}
              </button>
            </div>
          </li>
        ))}
      </ul>

      {initialEventTypes.length === 0 ? (
        <p className="text-muted-foreground">Пока нет доступных типов встреч</p>
      ) : selectedEventType === undefined ? (
        <p className="text-muted-foreground">
          Выберите тип встречи, чтобы увидеть свободные слоты
        </p>
      ) : slots.length === 0 ? (
        <p className="text-muted-foreground">
          У этого типа пока нет свободных слотов
        </p>
      ) : (
        <ul className="flex w-full flex-col gap-3" aria-label="Свободные слоты">
          {slots.map((slot) => (
            <li
              key={slot.id}
              className="rounded-xl border bg-card p-4 text-card-foreground"
            >
              {formatSlotInterval(
                new Date(slot.startDateTime),
                new Date(slot.endDateTime),
              )}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
