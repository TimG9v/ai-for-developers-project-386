import { Button } from "@/components/ui/button";

export default function Home() {
  return (
    <main className="flex flex-1 flex-col items-center justify-center gap-6 py-32">
      <h1 className="text-3xl font-semibold tracking-tight">Calendar</h1>
      <Button>Create event</Button>
    </main>
  );
}
