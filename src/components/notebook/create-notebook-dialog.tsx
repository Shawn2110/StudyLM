import { useEffect, useState, type KeyboardEvent } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Eyebrow } from "@/components/ui/eyebrow";
import { PrepModeBadge } from "@/components/notebook/prep-mode-badge";
import { cn } from "@/lib/utils";
import { createNotebook } from "@/lib/commands";
import {
  formatDifficulty,
  formatExamType,
  formatFormat,
} from "@/lib/format";
import type { DifficultyFocus, ExamType, Format } from "@/types/bindings";

/*
 * Prep-mode wizard, docs/design.md §7.5.
 * One modal, one tall form, all six fields visible. Eyebrow label left,
 * input right. exam_type + format use radio pills. Time-remaining is
 * hours-or-days toggle + number. Live preview pill on the right.
 * Submit on ⌘/Ctrl + Return.
 */

const examTypes = [
  "internal",
  "midsem",
  "endsem",
  "viva",
  "practical",
  "assignment",
  "competitive",
  "custom",
] as const satisfies readonly ExamType[];

const formats = [
  "mcq",
  "short",
  "long",
  "oral",
  "numerical",
  "mixed",
] as const satisfies readonly Format[];

const difficultyFocuses = [
  "conceptual",
  "problem_solving",
  "memorization",
  "mixed",
] as const satisfies readonly DifficultyFocus[];

type TimeUnit = "hours" | "days";

const schema = z.object({
  exam_type: z.enum(examTypes),
  format: z.enum(formats),
  subject: z.string().optional(),
  duration_minutes: z.string().optional(),
  remaining_value: z.string().optional(),
  remaining_unit: z.enum(["hours", "days"] as const),
  difficulty_focus: z.enum(difficultyFocuses).optional(),
});

type FormValues = z.infer<typeof schema>;

export function CreateNotebookDialog() {
  const [open, setOpen] = useState(false);
  const queryClient = useQueryClient();

  const form = useForm<FormValues>({
    resolver: zodResolver(schema),
    defaultValues: {
      exam_type: "internal",
      format: "mcq",
      subject: "",
      duration_minutes: "",
      remaining_value: "",
      remaining_unit: "days",
    },
  });

  const watched = form.watch();

  const examAtFromForm = (() => {
    if (!watched.remaining_value) return null;
    const n = Number.parseInt(watched.remaining_value, 10);
    if (!Number.isFinite(n) || n <= 0) return null;
    const seconds = watched.remaining_unit === "hours" ? n * 3600 : n * 86400;
    return Math.floor(Date.now() / 1000) + seconds;
  })();

  const mutation = useMutation({
    mutationFn: createNotebook,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["notebooks"] });
      setOpen(false);
      form.reset();
    },
  });

  function onSubmit(values: FormValues) {
    const minutes = values.duration_minutes
      ? Number.parseInt(values.duration_minutes, 10)
      : null;
    mutation.mutate({
      exam_type: values.exam_type,
      format: values.format,
      subject: values.subject?.trim() ? values.subject.trim() : null,
      duration_minutes: Number.isFinite(minutes) ? minutes : null,
      exam_at: examAtFromForm,
      difficulty_focus: values.difficulty_focus ?? null,
    });
  }

  function handleKeyDown(e: KeyboardEvent<HTMLFormElement>) {
    if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
      e.preventDefault();
      void form.handleSubmit(onSubmit)();
    }
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button>+ New notebook</Button>
      </DialogTrigger>
      <DialogContent className="max-w-[560px]">
        <DialogHeader>
          <DialogTitle>New notebook</DialogTitle>
          <PrepModeBadge
            examType={watched.exam_type}
            format={watched.format}
            difficultyFocus={watched.difficulty_focus}
            examAt={examAtFromForm}
            className="self-start"
          />
        </DialogHeader>

        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            onKeyDown={handleKeyDown}
            className="space-y-5"
          >
            <FormField
              control={form.control}
              name="exam_type"
              render={({ field }) => (
                <WizardRow label="Exam type">
                  <PillGroup
                    value={field.value}
                    options={examTypes}
                    label={formatExamType}
                    onChange={field.onChange}
                  />
                </WizardRow>
              )}
            />

            <FormField
              control={form.control}
              name="format"
              render={({ field }) => (
                <WizardRow label="Format">
                  <PillGroup
                    value={field.value}
                    options={formats}
                    label={formatFormat}
                    onChange={field.onChange}
                  />
                </WizardRow>
              )}
            />

            <FormField
              control={form.control}
              name="subject"
              render={({ field }) => (
                <WizardRow label="Subject">
                  <FormItem className="space-y-1">
                    <FormControl>
                      <Input placeholder="Thermodynamics" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                </WizardRow>
              )}
            />

            <WizardRow label="Time remaining">
              <div className="flex gap-2">
                <FormField
                  control={form.control}
                  name="remaining_value"
                  render={({ field }) => (
                    <FormItem className="flex-1 space-y-1">
                      <FormControl>
                        <Input
                          type="number"
                          inputMode="numeric"
                          placeholder="3"
                          {...field}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name="remaining_unit"
                  render={({ field }) => (
                    <ToggleUnit
                      value={field.value as TimeUnit}
                      onChange={(v) => field.onChange(v)}
                    />
                  )}
                />
              </div>
            </WizardRow>

            <FormField
              control={form.control}
              name="duration_minutes"
              render={({ field }) => (
                <WizardRow label="Exam duration">
                  <FormItem className="space-y-1">
                    <FormControl>
                      <Input
                        type="number"
                        inputMode="numeric"
                        placeholder="180 minutes"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                </WizardRow>
              )}
            />

            <FormField
              control={form.control}
              name="difficulty_focus"
              render={({ field }) => (
                <WizardRow label="Focus">
                  <PillGroup
                    value={field.value ?? null}
                    options={difficultyFocuses}
                    label={formatDifficulty}
                    onChange={field.onChange}
                    allowEmpty
                  />
                </WizardRow>
              )}
            />

            {mutation.isError && (
              <p className="text-sm font-sans text-danger">
                {String((mutation.error as { message?: string })?.message ?? mutation.error)}
              </p>
            )}

            <DialogFooter className="!mt-6 items-center">
              <p className="mr-auto font-mono text-xs text-paper-500">
                ⌘ + Return to create
              </p>
              <Button
                type="button"
                variant="ghost"
                onClick={() => setOpen(false)}
              >
                Cancel
              </Button>
              <Button type="submit" disabled={mutation.isPending}>
                {mutation.isPending ? "Creating…" : "Create notebook"}
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
}

function WizardRow({
  label,
  children,
}: {
  label: string;
  children: React.ReactNode;
}) {
  return (
    <div className="grid grid-cols-[100px_1fr] items-start gap-4">
      <Eyebrow className="pt-2">{label}</Eyebrow>
      <div>{children}</div>
    </div>
  );
}

function PillGroup<T extends string>({
  value,
  options,
  label,
  onChange,
  allowEmpty,
}: {
  value: T | null | undefined;
  options: readonly T[];
  label: (v: T) => string;
  onChange: (v: T | undefined) => void;
  allowEmpty?: boolean;
}) {
  return (
    <div className="flex flex-wrap gap-1.5">
      {options.map((opt) => {
        const selected = value === opt;
        return (
          <button
            key={opt}
            type="button"
            onClick={() => onChange(allowEmpty && selected ? undefined : opt)}
            className={cn(
              "rounded-full border px-3 py-1 text-xs font-sans transition-colors duration-instant ease-enter",
              selected
                ? "border-ink-500 bg-ink-500 text-paper-50"
                : "border-paper-300 bg-paper-50 text-paper-700 hover:border-paper-400 hover:text-paper-900",
            )}
          >
            {label(opt)}
          </button>
        );
      })}
    </div>
  );
}

function ToggleUnit({
  value,
  onChange,
}: {
  value: TimeUnit;
  onChange: (v: TimeUnit) => void;
}) {
  const units: TimeUnit[] = ["hours", "days"];
  return (
    <div className="inline-flex items-center rounded border border-paper-300 bg-paper-200 p-0.5">
      {units.map((u) => {
        const active = value === u;
        return (
          <button
            key={u}
            type="button"
            onClick={() => onChange(u)}
            className={cn(
              "rounded-sm px-3 py-1 text-xs font-sans transition-colors duration-instant ease-enter",
              active
                ? "bg-paper-50 text-paper-900 shadow-[inset_0_0_0_1px_var(--paper-300)]"
                : "text-paper-500 hover:text-paper-900",
            )}
          >
            {u}
          </button>
        );
      })}
    </div>
  );
}

// Reset live-preview state on dialog close so reopening starts fresh.
export function useResetOnClose(open: boolean, reset: () => void) {
  useEffect(() => {
    if (!open) reset();
  }, [open, reset]);
}
