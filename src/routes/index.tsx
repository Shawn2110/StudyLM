import { createFileRoute, Link } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";

import { CreateNotebookDialog } from "@/components/notebook/create-notebook-dialog";
import { Eyebrow } from "@/components/ui/eyebrow";
import { listNotebooks } from "@/lib/commands";
import {
  formatExamType,
  formatFormat,
  relativeTime,
} from "@/lib/format";
import type { Notebook } from "@/types/bindings";

export const Route = createFileRoute("/")({
  component: HomePage,
});

/*
 * Notebook list — docs/design.md §8.2.
 * Centered 720 px column, Fraunces "Notebooks" header, hairline rows
 * (not cards). Eyebrow + Fraunces title + Geist Mono meta line per row.
 */
function HomePage() {
  const { data: notebooks, isLoading, error } = useQuery({
    queryKey: ["notebooks"],
    queryFn: listNotebooks,
  });

  return (
    <section className="mx-auto max-w-[720px] px-4 pb-24 pt-12">
      <header className="mb-10 flex items-end justify-between gap-4">
        <h1 className="font-serif text-[2.25rem] font-medium leading-tight tracking-[-0.02em] text-paper-900">
          Notebooks
        </h1>
        <CreateNotebookDialog />
      </header>

      {isLoading && (
        <p className="text-sm font-mono text-paper-500">Loading notebooks…</p>
      )}

      {error && (
        <p className="text-sm font-sans text-danger">
          {String((error as { message?: string })?.message ?? error)}
        </p>
      )}

      {!isLoading && !error && notebooks && notebooks.length === 0 && (
        <EmptyState />
      )}

      {notebooks && notebooks.length > 0 && (
        <ul className="border-t border-paper-300">
          {notebooks.map((nb) => (
            <NotebookRow key={nb.id} notebook={nb} />
          ))}
        </ul>
      )}
    </section>
  );
}

function NotebookRow({ notebook }: { notebook: Notebook }) {
  const eyebrow = [
    notebook.subject,
    formatExamType(notebook.exam_type),
    notebook.exam_at != null ? relativeTime(notebook.exam_at) : null,
  ]
    .filter((piece): piece is string => Boolean(piece))
    .join(" · ");

  const meta = [
    "0 sources",
    `created ${relativeTime(notebook.created_at)}`,
    formatFormat(notebook.format),
  ].join(" · ");

  return (
    <li className="border-b border-paper-300">
      <Link
        to="/notebooks/$notebookId"
        params={{ notebookId: notebook.id }}
        className="block cursor-pointer px-1 py-5 text-left transition-colors duration-instant ease-enter hover:bg-paper-100"
      >
        <Eyebrow className="font-mono normal-case tracking-[0.08em]">
          {eyebrow}
        </Eyebrow>
        <h2 className="mt-1 font-serif text-[1.375rem] font-medium leading-snug tracking-[-0.01em] text-paper-900">
          {notebook.title}
        </h2>
        <p className="mt-1 font-mono text-xs text-paper-500">{meta}</p>
      </Link>
    </li>
  );
}

function EmptyState() {
  return (
    <div className="border-t border-paper-300 pt-16 text-center">
      <h2 className="font-serif text-2xl font-medium tracking-tight text-paper-900">
        Nothing here yet.
      </h2>
      <p className="mx-auto mt-2 max-w-md text-sm font-sans text-paper-500">
        Create a notebook and drop in your study material to get started.
      </p>
    </div>
  );
}
