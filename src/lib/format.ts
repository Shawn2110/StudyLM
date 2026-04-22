import type { ExamType, Format, DifficultyFocus } from "@/types/bindings";

const EXAM_TYPE_LABEL: Record<ExamType, string> = {
  internal: "Internal",
  midsem: "Mid-Sem",
  endsem: "End-Sem",
  viva: "Viva",
  practical: "Practical",
  assignment: "Assignment",
  competitive: "Competitive",
  custom: "Custom",
};

const FORMAT_LABEL: Record<Format, string> = {
  mcq: "MCQ",
  short: "Short",
  long: "Long",
  oral: "Oral",
  numerical: "Numerical",
  mixed: "Mixed",
};

const DIFFICULTY_LABEL: Record<DifficultyFocus, string> = {
  conceptual: "Conceptual",
  problem_solving: "Problem-Solving",
  memorization: "Memorization",
  mixed: "Mixed",
};

export const formatExamType = (e: ExamType) => EXAM_TYPE_LABEL[e];
export const formatFormat = (f: Format) => FORMAT_LABEL[f];
export const formatDifficulty = (d: DifficultyFocus) => DIFFICULTY_LABEL[d];

/** Compact relative time for both past ("created 2h ago") and future
 *  ("in 3 days"). Input is unix epoch seconds. */
export function relativeTime(unixSeconds: number, now: number = Date.now() / 1000): string {
  const diff = unixSeconds - now;
  const abs = Math.abs(diff);
  const future = diff > 0;

  const minute = 60;
  const hour = 60 * minute;
  const day = 24 * hour;
  const week = 7 * day;

  let value: number;
  let unit: string;
  if (abs < minute) {
    value = Math.round(abs);
    unit = value === 1 ? "second" : "seconds";
  } else if (abs < hour) {
    value = Math.round(abs / minute);
    unit = value === 1 ? "minute" : "minutes";
  } else if (abs < day) {
    value = Math.round(abs / hour);
    unit = value === 1 ? "hour" : "hours";
  } else if (abs < week) {
    value = Math.round(abs / day);
    unit = value === 1 ? "day" : "days";
  } else {
    value = Math.round(abs / week);
    unit = value === 1 ? "week" : "weeks";
  }
  return future ? `in ${value} ${unit}` : `${value} ${unit} ago`;
}
