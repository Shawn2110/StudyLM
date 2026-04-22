import { useState } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";

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
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { createNotebook } from "@/lib/commands";

const examTypes = [
  "internal",
  "midsem",
  "endsem",
  "viva",
  "practical",
  "assignment",
  "competitive",
  "custom",
] as const;

const formats = ["mcq", "short", "long", "oral", "numerical", "mixed"] as const;

const difficultyFocuses = [
  "conceptual",
  "problem_solving",
  "memorization",
  "mixed",
] as const;

const schema = z.object({
  exam_type: z.enum(examTypes),
  format: z.enum(formats),
  subject: z.string().optional(),
  duration_minutes: z.string().optional(),
  exam_at: z.string().optional(),
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
      exam_at: "",
    },
  });

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
    const examAt = values.exam_at
      ? Math.floor(new Date(values.exam_at).getTime() / 1000)
      : null;
    mutation.mutate({
      exam_type: values.exam_type,
      format: values.format,
      subject: values.subject?.trim() ? values.subject.trim() : null,
      duration_minutes: Number.isFinite(minutes) ? minutes : null,
      exam_at: Number.isFinite(examAt) ? examAt : null,
      difficulty_focus: values.difficulty_focus ?? null,
    });
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button>Create notebook</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>New notebook</DialogTitle>
          <DialogDescription>
            Set the prep mode. You can change it later if you regenerate.
          </DialogDescription>
        </DialogHeader>
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            className="space-y-4"
          >
            <div className="grid grid-cols-2 gap-4">
              <FormField
                control={form.control}
                name="exam_type"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Exam type</FormLabel>
                    <Select
                      onValueChange={field.onChange}
                      defaultValue={field.value}
                    >
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {examTypes.map((t) => (
                          <SelectItem key={t} value={t}>
                            {t}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="format"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Format</FormLabel>
                    <Select
                      onValueChange={field.onChange}
                      defaultValue={field.value}
                    >
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {formats.map((f) => (
                          <SelectItem key={f} value={f}>
                            {f}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>
            <FormField
              control={form.control}
              name="subject"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Subject (optional)</FormLabel>
                  <FormControl>
                    <Input placeholder="Thermodynamics" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div className="grid grid-cols-2 gap-4">
              <FormField
                control={form.control}
                name="duration_minutes"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Duration (min, optional)</FormLabel>
                    <FormControl>
                      <Input
                        type="number"
                        inputMode="numeric"
                        placeholder="180"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="exam_at"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Exam date (optional)</FormLabel>
                    <FormControl>
                      <Input type="datetime-local" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>
            <FormField
              control={form.control}
              name="difficulty_focus"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Difficulty focus (optional)</FormLabel>
                  <Select
                    onValueChange={field.onChange}
                    value={field.value ?? undefined}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Pick one" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {difficultyFocuses.map((d) => (
                        <SelectItem key={d} value={d}>
                          {d.replace("_", " ")}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />
            {mutation.isError && (
              <p className="text-sm text-destructive">
                {String((mutation.error as { message?: string })?.message ?? mutation.error)}
              </p>
            )}
            <DialogFooter>
              <Button
                type="button"
                variant="ghost"
                onClick={() => setOpen(false)}
              >
                Cancel
              </Button>
              <Button type="submit" disabled={mutation.isPending}>
                {mutation.isPending ? "Creating…" : "Create"}
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
}
