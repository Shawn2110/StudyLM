import { cn } from "@/lib/utils";
import type { Citation } from "@/lib/events";

/*
 * Inline citation pill rendered inside an assistant reply where the LLM
 * wrote `[42]`. Hovering reveals the source filename + page; clicking
 * (Phase 3 limited) scrolls to the matching source card. PDF jump-to-page
 * arrives in a later pass.
 */

export function CitationPill({
  citation,
  index,
  onClick,
}: {
  citation: Citation;
  index: number;
  onClick?: (citation: Citation) => void;
}) {
  const interactive = typeof onClick === "function";
  return (
    <button
      type="button"
      title={`${citation.document_filename} · p. ${citation.page}`}
      onClick={interactive ? () => onClick?.(citation) : undefined}
      tabIndex={interactive ? 0 : -1}
      className={cn(
        "inline-flex items-center align-baseline rounded-full bg-accent-soft px-1.5 py-0.5 font-mono text-[11px] font-medium leading-none text-accent transition-colors duration-instant ease-enter",
        interactive && "cursor-pointer hover:bg-accent hover:text-accent-on",
      )}
    >
      {index}
    </button>
  );
}
