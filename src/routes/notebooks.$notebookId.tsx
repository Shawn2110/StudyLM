import { useEffect } from "react";
import { createFileRoute } from "@tanstack/react-router";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { open } from "@tauri-apps/plugin-dialog";
import { FileText, Plus } from "lucide-react";

import { Button } from "@/components/ui/button";
import { SourceCard } from "@/components/document/source-card";
import { PrepModeBadgeOf } from "@/components/notebook/prep-mode-badge";
import { ingestDocument, listDocuments, listNotebooks } from "@/lib/commands";
import { onDocumentStatus } from "@/lib/events";

export const Route = createFileRoute("/notebooks/$notebookId")({
  component: NotebookDetail,
});

/*
 * Notebook detail — docs/design.md §4.2 (v2).
 * Full-width main pane, internal padding only. Sources rendered as a
 * responsive grid (3 across at desktop, 2 at md, 1 at narrow).
 */
function NotebookDetail() {
  const { notebookId } = Route.useParams();
  const queryClient = useQueryClient();

  const { data: notebooks } = useQuery({
    queryKey: ["notebooks"],
    queryFn: listNotebooks,
  });
  const notebook = notebooks?.find((n) => n.id === notebookId);

  const { data: documents, isLoading, error } = useQuery({
    queryKey: ["documents", notebookId],
    queryFn: () => listDocuments(notebookId),
  });

  const ingest = useMutation({
    mutationFn: (path: string) => ingestDocument(notebookId, path),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["documents", notebookId] });
    },
  });

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let active = true;
    onDocumentStatus(() => {
      queryClient.invalidateQueries({ queryKey: ["documents", notebookId] });
    }).then((fn) => {
      if (active) unlisten = fn;
      else fn();
    });
    return () => {
      active = false;
      unlisten?.();
    };
  }, [queryClient, notebookId]);

  async function handleAddPdf() {
    const picked = await open({
      filters: [{ name: "PDF", extensions: ["pdf"] }],
      multiple: false,
    });
    if (typeof picked === "string") {
      ingest.mutate(picked);
    }
  }

  return (
    <section className="flex h-full flex-col px-8 py-6">
      <header className="mb-8 flex items-start justify-between gap-6">
        <div className="min-w-0 space-y-2">
          {notebook && <PrepModeBadgeOf source={notebook} />}
          <h1 className="text-3xl font-semibold leading-tight tracking-[-0.025em] text-text-strong">
            {notebook?.title ?? "Notebook"}
          </h1>
        </div>
        <Button onClick={handleAddPdf} disabled={ingest.isPending}>
          <Plus className="h-4 w-4" />
          {ingest.isPending ? "Ingesting…" : "Add PDF"}
        </Button>
      </header>

      <div className="mb-3 flex items-baseline justify-between">
        <p className="text-sm font-medium text-muted-foreground">Sources</p>
        {documents && documents.length > 0 && (
          <p className="font-mono text-xs text-muted-foreground">
            {documents.length} source{documents.length === 1 ? "" : "s"}
          </p>
        )}
      </div>

      {isLoading && (
        <p className="font-mono text-sm text-muted-foreground">Loading…</p>
      )}

      {error && (
        <p className="text-sm text-danger">
          {String((error as { message?: string })?.message ?? error)}
        </p>
      )}

      {ingest.isError && (
        <p className="mb-3 text-sm text-danger">
          {String(
            (ingest.error as { message?: string })?.message ?? ingest.error,
          )}
        </p>
      )}

      {!isLoading && documents && documents.length === 0 && (
        <EmptyState onAdd={handleAddPdf} pending={ingest.isPending} />
      )}

      {documents && documents.length > 0 && (
        <ul className="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
          {documents.map((doc) => (
            <li key={doc.id}>
              <SourceCard document={doc} />
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}

function EmptyState({
  onAdd,
  pending,
}: {
  onAdd: () => void;
  pending: boolean;
}) {
  return (
    <div className="mt-12 flex flex-col items-center text-center">
      <FileText className="h-8 w-8 text-muted-foreground" strokeWidth={1.5} />
      <p className="mt-3 text-sm text-text">No sources in this notebook yet.</p>
      <Button className="mt-4" onClick={onAdd} disabled={pending}>
        <Plus className="h-4 w-4" />
        {pending ? "Ingesting…" : "Add a PDF"}
      </Button>
    </div>
  );
}
