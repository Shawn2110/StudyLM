import { FileText } from "lucide-react";

import { cn } from "@/lib/utils";
import type { Document, DocumentStatus } from "@/types/bindings";

/*
 * Source card per docs/design.md §6.3 (v2). 12 px radius, surface bg,
 * Lucide FileText icon (no v1 stripe), status pill, hover lift.
 */

const STATUS_LABEL: Record<DocumentStatus, string> = {
  pending: "Queued",
  parsing: "Parsing",
  embedding: "Embedding",
  ready: "Ready",
  failed: "Failed",
};

const STATUS_TONE: Record<DocumentStatus, string> = {
  pending: "bg-surface-alt text-muted-foreground",
  parsing: "bg-surface-alt text-muted-foreground",
  embedding: "bg-surface-alt text-muted-foreground",
  ready: "bg-success/15 text-success",
  failed: "bg-danger/15 text-danger",
};

function isBusy(status: DocumentStatus): boolean {
  return status === "pending" || status === "parsing" || status === "embedding";
}

export function SourceCard({
  document,
  selected = false,
  onClick,
}: {
  document: Document;
  selected?: boolean;
  onClick?: () => void;
}) {
  const busy = isBusy(document.status);
  const interactive = typeof onClick === "function";
  const pageInfo = document.page_count != null ? `${document.page_count} pages` : null;

  return (
    <div
      role={interactive ? "button" : undefined}
      tabIndex={interactive ? 0 : undefined}
      onClick={onClick}
      onKeyDown={
        interactive
          ? (e) => {
              if (e.key === "Enter" || e.key === " ") {
                e.preventDefault();
                onClick?.();
              }
            }
          : undefined
      }
      className={cn(
        "group relative flex h-full flex-col gap-3 overflow-hidden rounded-lg border bg-surface p-4 transition-all duration-snappy ease-enter",
        selected
          ? "border-accent bg-accent-soft"
          : "border-border-default hover:-translate-y-px hover:shadow-sm",
        interactive && "cursor-pointer",
      )}
    >
      {busy && (
        <span
          aria-hidden="true"
          className="pointer-events-none absolute inset-0 overflow-hidden"
        >
          <span className="absolute inset-y-0 left-0 w-1/3 animate-parse-sweep bg-gradient-to-r from-transparent via-accent/15 to-transparent" />
        </span>
      )}

      <div className="relative flex items-start gap-3">
        <span className="mt-0.5 flex h-9 w-9 shrink-0 items-center justify-center rounded-md bg-surface-alt text-muted-foreground">
          <FileText className="h-4 w-4" strokeWidth={1.75} />
        </span>
        <h3
          className="min-w-0 flex-1 truncate text-sm font-medium text-text-strong"
          title={document.filename}
        >
          {document.filename}
        </h3>
      </div>

      <div className="relative flex items-center justify-between text-xs">
        <span
          className={cn(
            "inline-flex items-center rounded-full px-2 py-0.5 text-[11px] font-medium",
            STATUS_TONE[document.status],
          )}
        >
          {STATUS_LABEL[document.status]}
        </span>
        {pageInfo && (
          <span className="font-mono text-muted-foreground">{pageInfo}</span>
        )}
      </div>

      {document.error && (
        <p className="relative truncate text-xs text-danger" title={document.error}>
          {document.error}
        </p>
      )}
    </div>
  );
}
