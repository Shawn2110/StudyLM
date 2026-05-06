import { useEffect, useRef, useState, type KeyboardEvent } from "react";
import { useQuery } from "@tanstack/react-query";
import { Send } from "lucide-react";

import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { cn } from "@/lib/utils";
import { getActiveProvider, getProviderStatus } from "@/lib/commands";
import type { ModelInfo } from "@/types/bindings";

/*
 * Composer with the model picker pinned to the right of the chat box, per
 * the user direction. textarea grows vertically up to 8 rows; Enter
 * submits, Shift+Enter inserts a newline.
 */
export function ChatComposer({
  selectedModel,
  onSelectedModelChange,
  onSend,
  busy,
}: {
  selectedModel: string | null;
  onSelectedModelChange: (modelId: string) => void;
  onSend: (text: string) => void;
  busy: boolean;
}) {
  const [text, setText] = useState("");
  const textRef = useRef<HTMLTextAreaElement | null>(null);

  const active = useQuery({
    queryKey: ["active-provider"],
    queryFn: getActiveProvider,
    refetchOnWindowFocus: false,
  });

  const status = useQuery({
    queryKey: ["provider-status", active.data],
    queryFn: () => getProviderStatus(active.data!),
    enabled: active.data != null,
    refetchOnWindowFocus: false,
  });

  const models: ModelInfo[] =
    status.data?.kind === "Connected" ? status.data.detail.models : [];

  // Default the selected model to the first available once they load.
  useEffect(() => {
    if (!selectedModel && models.length > 0 && models[0]) {
      onSelectedModelChange(models[0].id);
    }
  }, [selectedModel, models, onSelectedModelChange]);

  // Auto-grow textarea.
  useEffect(() => {
    const el = textRef.current;
    if (!el) return;
    el.style.height = "auto";
    el.style.height = `${Math.min(el.scrollHeight, 192)}px`; // cap ~8 rows
  }, [text]);

  function handleKeyDown(e: KeyboardEvent<HTMLTextAreaElement>) {
    if (e.key === "Enter" && !e.shiftKey && !e.metaKey && !e.ctrlKey) {
      e.preventDefault();
      submit();
    }
  }

  function submit() {
    const trimmed = text.trim();
    if (!trimmed || busy || !selectedModel) return;
    onSend(trimmed);
    setText("");
  }

  const disabled = busy || !active.data || !selectedModel;
  const placeholder = !active.data
    ? "Configure a provider in Settings → Providers first."
    : models.length === 0
      ? "No models available — check your provider key."
      : "Ask anything about this notebook…";

  return (
    <div className="rounded-xl border border-border-default bg-surface p-2 shadow-sm">
      <div className="flex items-stretch gap-2">
        <textarea
          ref={textRef}
          value={text}
          onChange={(e) => setText(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder={placeholder}
          disabled={disabled}
          rows={2}
          className={cn(
            "min-h-[44px] flex-1 resize-none bg-transparent px-2 py-2 text-sm leading-relaxed text-text-strong placeholder:text-muted-foreground focus:outline-none",
          )}
        />
        <div className="flex w-[180px] flex-col justify-between gap-2">
          <Select
            value={selectedModel ?? undefined}
            onValueChange={onSelectedModelChange}
            disabled={models.length === 0}
          >
            <SelectTrigger className="h-8 text-xs">
              <SelectValue placeholder="Model" />
            </SelectTrigger>
            <SelectContent>
              {models.map((m) => (
                <SelectItem key={m.id} value={m.id} className="text-xs">
                  {m.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          <Button
            type="button"
            onClick={submit}
            disabled={disabled || text.trim().length === 0}
            className="h-8"
          >
            <Send className="h-3.5 w-3.5" />
            {busy ? "Sending…" : "Send"}
          </Button>
        </div>
      </div>
    </div>
  );
}
