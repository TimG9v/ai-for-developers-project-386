import { eventTypesList } from "@/src/client";

// Список типов встреч меняется в рантайме (in-memory хранилище) —
// статический пререндер запёк бы пустой список на этапе сборки.
export const dynamic = "force-dynamic";

export default async function BookingPage() {
  const { data } = await eventTypesList();
  const eventTypes = data ?? [];

  return (
    <main className="mx-auto flex w-full max-w-6xl flex-1 flex-col items-center justify-center gap-4 px-6 py-32">
      <h1 className="text-3xl font-semibold tracking-tight">Страница записи</h1>
      <p className="text-muted-foreground">Выберите тип встречи</p>
      {eventTypes.length === 0 ? (
        <p className="text-muted-foreground">Пока нет доступных типов встреч</p>
      ) : (
        <ul className="flex w-full max-w-2xl flex-col gap-4">
          {eventTypes.map((eventType) => (
            <li
              key={eventType.id}
              className="rounded-xl border bg-card p-6 text-card-foreground"
            >
              <h2 className="text-xl font-semibold">{eventType.title}</h2>
              {eventType.description && (
                <p className="text-muted-foreground">
                  {eventType.description}
                </p>
              )}
              <p className="text-sm text-muted-foreground">
                {eventType.durationMinutes} мин.
              </p>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
