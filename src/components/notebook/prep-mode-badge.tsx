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
 * Prep-mode pill, docs/design.md §7.1. Carries the exam_type, difficulty
 * focus, and time-remaining (when known). Renders at the top-left of every
 * generated artifact — chat transcript, study guide, flashcard set,
 * podcast — as the visual promise that the output is tailored.
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
        "inline-flex items-center rounded-full border border-ink-500 px-2 py-0.5 font-mono text-[11px] uppercase leading-none tracking-[0.08em] text-ink-600",
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
