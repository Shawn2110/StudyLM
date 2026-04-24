import { cn } from "@/lib/utils";
import type { Document, DocumentStatus } from "@/types/bindings";

/*
 * Source card per docs/design.md §6.3. Document-assigned stripe color
 * (hashed from filename), parse-sweep animation while non-terminal,
 * hairline border, quiet meta row.
 */

const STRIPE_COLORS = [
  "oklch(70% 0.115 65)",
  "oklch(72% 0.080 80)",
  "oklch(68% 0.090 45)",
  "oklch(66% 0.100 30)",
  "oklch(74% 0.075 95)",
  "oklch(62% 0.090 60)",
];

function stripeColor(filename: string): string {
  let hash = 0;
  for (let i = 0; i < filename.length; i += 1) {
    hash = (hash * 31 + filename.charCodeAt(i)) >>> 0;
  }
  return STRIPE_COLORS[hash % STRIPE_COLORS.length]!;
}

const STATUS_LABEL: Record<DocumentStatus, string> = {
  pending: "queued",
  parsing: "parsing",
  embedding: "embedding",
  ready: "ready",
  failed: "failed",
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
  const stripe = stripeColor(document.filename);
  const busy = isBusy(document.status);
  const pageInfo = document.page_count != null ? `${document.page_count} pp` : "—";
  const interactive = typeof onClick === "function";

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
        "group relative flex items-start gap-3 overflow-hidden rounded border bg-paper-100 px-3 py-3 transition-colors duration-instant ease-enter",
        selected
          ? "border-ink-500 bg-paper-200"
          : "border-paper-300 hover:bg-paper-200",
        interactive && "cursor-pointer",
      )}
    >
      <span
        aria-hidden="true"
        className={cn(
          "absolute left-0 top-0 h-full transition-all duration-instant",
          selected ? "w-1" : "w-[3px]",
        )}
        style={{ backgroundColor: stripe }}
      />
      {busy && (
        <span
          aria-hidden="true"
          className="pointer-events-none absolute inset-0 overflow-hidden"
        >
          <span
            className="absolute inset-y-0 left-0 w-1/3 animate-parse-sweep bg-gradient-to-r from-transparent via-ink-500/20 to-transparent"
          />
        </span>
      )}
      <div className="relative min-w-0 flex-1 pl-2">
        <h3
          className="truncate text-sm font-sans font-medium text-paper-900"
          title={document.filename}
        >
          {document.filename}
        </h3>
        <p className="mt-0.5 font-mono text-xs text-paper-500">
          {pageInfo} · {STATUS_LABEL[document.status]}
          {document.error ? ` · ${document.error}` : ""}
        </p>
      </div>
    </div>
  );
}
