import { Fragment, useMemo } from "react";

import { CitationPill } from "@/components/chat/citation-pill";
import { cn } from "@/lib/utils";
import type { Citation } from "@/lib/events";
import type { Message } from "@/types/bindings";

/*
 * One chat message rendered as an editorial block (no iMessage-style
 * left/right alignment). Role is signalled by a small label above the
 * body. Assistant replies parse `[N]` markers into CitationPill chips.
 */

export function MessageBubble({
  message,
  streaming = false,
  onCitationClick,
}: {
  message: Pick<Message, "role" | "content" | "citations_json">;
  streaming?: boolean;
  onCitationClick?: (c: Citation) => void;
}) {
  const citations = useMemo<Citation[]>(() => {
    if (!message.citations_json) return [];
    try {
      return JSON.parse(message.citations_json) as Citation[];
    } catch {
      return [];
    }
  }, [message.citations_json]);

  const segments = useMemo(
    () => splitOnCitations(message.content),
    [message.content],
  );

  return (
    <article className="space-y-1.5">
      <p
        className={cn(
          "text-xs font-medium uppercase tracking-wider",
          message.role === "user" ? "text-muted-foreground" : "text-accent",
        )}
      >
        {message.role === "user" ? "You" : "Assistant"}
      </p>
      <div className="whitespace-pre-wrap text-[0.9375rem] leading-relaxed text-text">
        {segments.map((seg, i) => {
          if (seg.kind === "text") {
            return <Fragment key={i}>{seg.text}</Fragment>;
          }
          const cite = citations.find((c) => c.chunk_id === seg.chunkId);
          if (!cite) {
            return <Fragment key={i}>{seg.raw}</Fragment>;
          }
          const idx = citations.indexOf(cite) + 1;
          return (
            <CitationPill
              key={i}
              citation={cite}
              index={idx}
              onClick={onCitationClick}
            />
          );
        })}
        {streaming && (
          <span
            className="ml-0.5 inline-block h-[1.1em] w-[2px] -translate-y-[1px] animate-pulse bg-accent align-middle"
            aria-hidden="true"
          />
        )}
      </div>
    </article>
  );
}

type Segment =
  | { kind: "text"; text: string }
  | { kind: "cite"; chunkId: number; raw: string };

function splitOnCitations(content: string): Segment[] {
  const out: Segment[] = [];
  let i = 0;
  let textStart = 0;
  while (i < content.length) {
    if (content[i] === "[") {
      let j = i + 1;
      while (j < content.length && /[0-9]/.test(content[j]!)) j += 1;
      if (j > i + 1 && content[j] === "]") {
        if (textStart < i) {
          out.push({ kind: "text", text: content.slice(textStart, i) });
        }
        const chunkId = Number.parseInt(content.slice(i + 1, j), 10);
        out.push({ kind: "cite", chunkId, raw: content.slice(i, j + 1) });
        i = j + 1;
        textStart = i;
        continue;
      }
    }
    i += 1;
  }
  if (textStart < content.length) {
    out.push({ kind: "text", text: content.slice(textStart) });
  }
  return out;
}
