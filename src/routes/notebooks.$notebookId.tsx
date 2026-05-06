import { useEffect, useState } from "react";
import { createFileRoute } from "@tanstack/react-router";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { open } from "@tauri-apps/plugin-dialog";
import { FileText, MessageSquare, Plus } from "lucide-react";

import { Button } from "@/components/ui/button";
import { ChatPanel } from "@/components/chat/chat-panel";
import { SourceCard } from "@/components/document/source-card";
import { PrepModeBadgeOf } from "@/components/notebook/prep-mode-badge";
import { cn } from "@/lib/utils";
import { ingestDocument, listDocuments, listNotebooks } from "@/lib/commands";
import { onDocumentStatus } from "@/lib/events";

export const Route = createFileRoute("/notebooks/$notebookId")({
  component: NotebookDetail,
});

type Tab = "sources" | "chat";

const TABS: { id: Tab; label: string; icon: typeof FileText }[] = [
  { id: "sources", label: "Sources", icon: FileText },
  { id: "chat", label: "Chat", icon: MessageSquare },
];

/*
 * Notebook detail — docs/design.md §4.2.
 * Full-width main pane with tabs (Sources / Chat). Future tabs (Study
 * guide / Flashcards / Podcast) slot in alongside as their phases land.
 */
function NotebookDetail() {
  const { notebookId } = Route.useParams();
  const [tab, setTab] = useState<Tab>("sources");

  const { data: notebooks } = useQuery({
    queryKey: ["notebooks"],
    queryFn: listNotebooks,
  });
  const notebook = notebooks?.find((n) => n.id === notebookId);

  return (
    <section className="flex h-full flex-col px-8 py-6">
      <header className="mb-6 space-y-3">
        <div className="space-y-2">
          {notebook && <PrepModeBadgeOf source={notebook} />}
          <h1 className="text-3xl font-semibold leading-tight tracking-[-0.025em] text-text-strong">
            {notebook?.title ?? "Notebook"}
          </h1>
        </div>
        <nav className="flex gap-1 border-b border-border-default">
          {TABS.map((t) => {
            const active = tab === t.id;
            const Icon = t.icon;
            return (
              <button
                key={t.id}
                type="button"
                onClick={() => setTab(t.id)}
                className={cn(
                  "relative -mb-px inline-flex items-center gap-1.5 px-3 py-2 text-sm font-medium transition-colors duration-instant ease-enter",
                  active
                    ? "text-text-strong"
                    : "text-muted-foreground hover:text-text-strong",
                )}
              >
                <Icon className="h-4 w-4" />
                {t.label}
                {active && (
                  <span className="absolute inset-x-0 bottom-0 h-px bg-accent" />
                )}
              </button>
            );
          })}
        </nav>
      </header>

      <div className="min-h-0 flex-1">
        {tab === "sources" && <SourcesTab notebookId={notebookId} />}
        {tab === "chat" && <ChatPanel notebookId={notebookId} />}
      </div>
    </section>
  );
}

function SourcesTab({ notebookId }: { notebookId: string }) {
  const queryClient = useQueryClient();

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
    <section className="space-y-4">
      <div className="flex items-baseline justify-between">
        <p className="text-sm font-medium text-muted-foreground">
          {documents && documents.length > 0
            ? `${documents.length} source${documents.length === 1 ? "" : "s"}`
            : "Sources"}
        </p>
        <Button
          onClick={handleAddPdf}
          disabled={ingest.isPending}
          size="sm"
          variant="secondary"
        >
          <Plus className="h-4 w-4" />
          {ingest.isPending ? "Ingesting…" : "Add PDF"}
        </Button>
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
        <p className="text-sm text-danger">
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
            <li key={doc.id} id={`source-${doc.id}`}>
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
