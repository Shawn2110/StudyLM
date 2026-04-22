import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
  component: HomePage,
});

function HomePage() {
  return (
    <section>
      <h1 className="text-2xl font-semibold tracking-tight">Notebooks</h1>
      <p className="mt-2 text-sm text-muted-foreground">
        Create a notebook to start studying from your own material.
      </p>
    </section>
  );
}
