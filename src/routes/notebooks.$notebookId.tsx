import { useEffect } from "react";
import { createFileRoute, Link } from "@tanstack/react-router";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { open } from "@tauri-apps/plugin-dialog";
import { ArrowLeft, Plus } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Eyebrow } from "@/components/ui/eyebrow";
import { SourceCard } from "@/components/document/source-card";
import { PrepModeBadgeOf } from "@/components/notebook/prep-mode-badge";
import { ingestDocument, listDocuments, listNotebooks } from "@/lib/commands";
import { onDocumentStatus } from "@/lib/events";

export const Route = createFileRoute("/notebooks/$notebookId")({
  component: NotebookDetail,
});

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
    onDocumentStatus((payload) => {
      if (payload.document_id) {
        queryClient.invalidateQueries({ queryKey: ["documents", notebookId] });
      }
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
    <section className="mx-auto max-w-[720px] px-4 pb-24 pt-10">
      <Link
        to="/"
        className="mb-4 inline-flex items-center gap-1 text-xs font-sans text-paper-500 transition-colors hover:text-paper-900"
      >
        <ArrowLeft className="h-3 w-3" /> Notebooks
      </Link>

      <header className="mb-8 flex items-start justify-between gap-4">
        <div className="min-w-0">
          {notebook && (
            <PrepModeBadgeOf source={notebook} className="mb-2" />
          )}
          <h1 className="font-serif text-[2.25rem] font-medium leading-tight tracking-[-0.02em] text-paper-900">
            {notebook?.title ?? "Notebook"}
          </h1>
        </div>
        <Button onClick={handleAddPdf} disabled={ingest.isPending}>
          <Plus className="h-4 w-4" />
          {ingest.isPending ? "Ingesting…" : "Add PDF"}
        </Button>
      </header>

      <section className="space-y-4">
        <Eyebrow>Sources</Eyebrow>

        {isLoading && (
          <p className="text-sm font-mono text-paper-500">Loading…</p>
        )}

        {error && (
          <p className="text-sm font-sans text-danger">
            {String((error as { message?: string })?.message ?? error)}
          </p>
        )}

        {ingest.isError && (
          <p className="text-sm font-sans text-danger">
            {String(
              (ingest.error as { message?: string })?.message ?? ingest.error,
            )}
          </p>
        )}

        {!isLoading && documents && documents.length === 0 && <EmptyState />}

        {documents && documents.length > 0 && (
          <ul className="space-y-2">
            {documents.map((doc) => (
              <li key={doc.id}>
                <SourceCard document={doc} />
              </li>
            ))}
          </ul>
        )}
      </section>
    </section>
  );
}

function EmptyState() {
  return (
    <div className="rounded border border-dashed border-paper-300 px-6 py-10 text-center">
      <h2 className="font-serif text-xl font-medium tracking-tight text-paper-900">
        No sources in this notebook.
      </h2>
      <p className="mx-auto mt-1 max-w-sm text-sm font-sans text-paper-500">
        Click <span className="font-mono text-paper-700">Add PDF</span> above to
        start building this notebook.
      </p>
    </div>
  );
}
