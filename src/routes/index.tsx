import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";

import { CreateNotebookDialog } from "@/components/notebook/create-notebook-dialog";
import { listNotebooks } from "@/lib/commands";

export const Route = createFileRoute("/")({
  component: HomePage,
});

function HomePage() {
  const { data: notebooks, isLoading, error } = useQuery({
    queryKey: ["notebooks"],
    queryFn: listNotebooks,
  });

  return (
    <section>
      <div className="mb-6 flex items-center justify-between">
        <h1 className="text-2xl font-semibold tracking-tight">Notebooks</h1>
        <CreateNotebookDialog />
      </div>

      {isLoading && (
        <p className="text-sm text-muted-foreground">Loading…</p>
      )}

      {error && (
        <p className="text-sm text-destructive">
          {String((error as { message?: string })?.message ?? error)}
        </p>
      )}

      {!isLoading && notebooks && notebooks.length === 0 && (
        <p className="text-sm text-muted-foreground">
          No notebooks yet. Create one to get started.
        </p>
      )}

      {notebooks && notebooks.length > 0 && (
        <ul className="space-y-2">
          {notebooks.map((nb) => (
            <li key={nb.id} className="rounded-md border p-4">
              <div className="font-medium">{nb.title}</div>
              <div className="text-sm text-muted-foreground">
                {nb.exam_type} · {nb.format}
                {nb.subject ? ` · ${nb.subject}` : null}
              </div>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
