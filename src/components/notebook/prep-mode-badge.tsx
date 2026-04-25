import { cn } from "@/lib/utils";
import {
  formatDifficulty,
  formatExamType,
  formatFormat,
  relativeTime,
} from "@/lib/format";
import type {
  DifficultyFocus,
  ExamType,
  Format,
  Notebook,
  PrepMode,
} from "@/types/bindings";

/*
 * Prep-mode pill, docs/design.md §6.6 (v2). Renders at the top-left of
 * every generated artifact (chat, study guide, flashcards, podcast) and on
 * notebook headers — the visual promise that the output is tailored.
 * v2 swap: accent-soft background + accent text (was bordered transparent).
 */

type PrepModeBadgeProps = {
  examType: ExamType;
  format?: Format | null;
  difficultyFocus?: DifficultyFocus | null;
  examAt?: number | null;
  className?: string;
};

export function PrepModeBadge({
  examType,
  format,
  difficultyFocus,
  examAt,
  className,
}: PrepModeBadgeProps) {
  const pieces: string[] = [formatExamType(examType)];
  if (format) pieces.push(formatFormat(format));
  if (difficultyFocus) pieces.push(formatDifficulty(difficultyFocus));
  if (examAt != null) pieces.push(relativeTime(examAt));

  const display = pieces.slice(0, 3).join(" · ");
  const full = pieces.join(" · ");

  return (
    <span
      title={pieces.length > 3 ? full : undefined}
      className={cn(
        "inline-flex items-center rounded-full bg-accent-soft px-2 py-0.5 font-mono text-[11px] font-medium uppercase leading-none tracking-[0.04em] text-accent",
        className,
      )}
    >
      {display.toUpperCase()}
    </span>
  );
}

/** Convenience wrapper accepting a full Notebook or PrepMode object. */
export function PrepModeBadgeOf({
  source,
  className,
}: {
  source: Pick<Notebook | PrepMode, "exam_type" | "format" | "difficulty_focus" | "exam_at">;
  className?: string;
}) {
  return (
    <PrepModeBadge
      examType={source.exam_type}
      format={source.format}
      difficultyFocus={source.difficulty_focus}
      examAt={source.exam_at}
      className={className}
    />
  );
}
