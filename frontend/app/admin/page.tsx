import { eventTypesList } from "@/src/client";

import { AdminEventTypes } from "@/components/admin-event-types";

// Список типов встреч меняется в рантайме (in-memory хранилище).
export const dynamic = "force-dynamic";

export default async function AdminPage() {
  const { data } = await eventTypesList();

  return (
    <main className="mx-auto flex w-full max-w-6xl flex-1 flex-col items-center gap-6 px-6 py-16">
      <h1 className="text-3xl font-semibold tracking-tight">Админка</h1>
      <AdminEventTypes initialEventTypes={data ?? []} />
    </main>
  );
}
