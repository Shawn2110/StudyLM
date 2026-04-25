import { useState, type KeyboardEvent } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Plus } from "lucide-react";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
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
 * Prep-mode wizard, docs/design.md §4.3 (v2). 520 px modal, plain UI labels
 * (no SMALL CAPS), radio pills for the enum fields, hours/days toggle for
 * time-remaining, ⌘+Return to submit.
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
        <Button className="w-full justify-start">
          <Plus className="h-4 w-4" />
          New notebook
        </Button>
      </DialogTrigger>
      <DialogContent className="max-w-[520px]">
        <DialogHeader>
          <DialogTitle>New notebook</DialogTitle>
          <DialogDescription>
            Set the prep mode. You can change it later if you regenerate.
          </DialogDescription>
          <PrepModeBadge
            examType={watched.exam_type}
            format={watched.format}
            difficultyFocus={watched.difficulty_focus}
            examAt={examAtFromForm}
            className="mt-2 self-start"
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
                <Field label="Exam type">
                  <PillGroup
                    value={field.value}
                    options={examTypes}
                    label={formatExamType}
                    onChange={field.onChange}
                  />
                </Field>
              )}
            />

            <FormField
              control={form.control}
              name="format"
              render={({ field }) => (
                <Field label="Format">
                  <PillGroup
                    value={field.value}
                    options={formats}
                    label={formatFormat}
                    onChange={field.onChange}
                  />
                </Field>
              )}
            />

            <FormField
              control={form.control}
              name="subject"
              render={({ field }) => (
                <Field label="Subject">
                  <FormItem className="space-y-1">
                    <FormControl>
                      <Input placeholder="Thermodynamics" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                </Field>
              )}
            />

            <Field label="Time remaining">
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
            </Field>

            <FormField
              control={form.control}
              name="duration_minutes"
              render={({ field }) => (
                <Field label="Exam duration">
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
                </Field>
              )}
            />

            <FormField
              control={form.control}
              name="difficulty_focus"
              render={({ field }) => (
                <Field label="Focus">
                  <PillGroup
                    value={field.value ?? null}
                    options={difficultyFocuses}
                    label={formatDifficulty}
                    onChange={field.onChange}
                    allowEmpty
                  />
                </Field>
              )}
            />

            {mutation.isError && (
              <p className="text-sm text-danger">
                {String((mutation.error as { message?: string })?.message ?? mutation.error)}
              </p>
            )}

            <DialogFooter className="!mt-6 items-center">
              <p className="mr-auto font-mono text-xs text-muted-foreground">
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

function Field({
  label,
  children,
}: {
  label: string;
  children: React.ReactNode;
}) {
  return (
    <div className="space-y-1.5">
      <p className="text-sm font-medium text-text-strong">{label}</p>
      {children}
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
              "rounded-full border px-3 py-1 text-xs font-medium transition-all duration-instant ease-enter",
              selected
                ? "border-accent bg-accent text-accent-on shadow-sm"
                : "border-border-default bg-surface text-text hover:border-border-strong hover:text-text-strong",
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
    <div className="inline-flex items-center rounded-md border border-border-default bg-surface-alt p-0.5">
      {units.map((u) => {
        const active = value === u;
        return (
          <button
            key={u}
            type="button"
            onClick={() => onChange(u)}
            className={cn(
              "rounded-sm px-3 py-1 text-xs font-medium transition-all duration-instant ease-enter",
              active
                ? "bg-surface text-text-strong shadow-sm"
                : "text-muted-foreground hover:text-text-strong",
            )}
          >
            {u}
          </button>
        );
      })}
    </div>
  );
}
