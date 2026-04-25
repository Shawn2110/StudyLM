import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { BookOpen } from "lucide-react";

import { listNotebooks } from "@/lib/commands";

export const Route = createFileRoute("/")({
  component: HomePage,
});

/*
 * Home — docs/design.md §4.1.
 * The notebook list itself moved into the sidebar in v2; this route is
 * the welcome state shown in the main pane when no notebook is selected.
 */
function HomePage() {
  const { data: notebooks } = useQuery({
    queryKey: ["notebooks"],
    queryFn: listNotebooks,
  });

  const hasNotebooks = notebooks && notebooks.length > 0;

  return (
    <section className="flex h-full flex-col items-center justify-center px-8 py-12">
      <div className="flex max-w-md flex-col items-center text-center">
        <BookOpen className="h-8 w-8 text-muted-foreground" strokeWidth={1.5} />
        <h1 className="mt-4 text-2xl font-semibold tracking-[-0.02em] text-text-strong">
          {hasNotebooks ? "Pick a notebook" : "Welcome to StudyLM"}
        </h1>
        <p className="mt-2 text-sm leading-relaxed text-muted-foreground">
          {hasNotebooks
            ? "Choose a notebook from the sidebar, or create a new one to get started."
            : "Create a notebook from the sidebar, drop in your study material, and let StudyLM help you prepare."}
        </p>
      </div>
    </section>
  );
}
