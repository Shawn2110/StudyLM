import { useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Check, Circle } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";
import {
  deleteProviderKey,
  getProviderStatus,
  setActiveProvider,
  storeProviderKey,
} from "@/lib/commands";
import type {
  ProviderId,
  ProviderInfo,
  ProviderStatus,
} from "@/types/bindings";

/*
 * Provider configuration row used inside Settings → Providers. Each row
 * pings the provider on mount via a stored key (or just probes Ollama),
 * exposes a key input + Save button for cloud providers, and lets the user
 * mark a connected provider as the active one.
 */

export function ProviderRow({
  provider,
  active,
}: {
  provider: ProviderInfo;
  active: boolean;
}) {
  const queryClient = useQueryClient();
  const [pendingKey, setPendingKey] = useState("");

  const status = useQuery({
    queryKey: ["provider-status", provider.id],
    queryFn: () => getProviderStatus(provider.id),
    refetchOnWindowFocus: false,
  });

  const save = useMutation({
    mutationFn: async (key: string) => {
      await storeProviderKey(provider.id, key);
    },
    onSuccess: () => {
      setPendingKey("");
      queryClient.invalidateQueries({ queryKey: ["provider-status", provider.id] });
    },
  });

  const remove = useMutation({
    mutationFn: () => deleteProviderKey(provider.id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["provider-status", provider.id] });
      queryClient.invalidateQueries({ queryKey: ["active-provider"] });
    },
  });

  const activate = useMutation({
    mutationFn: () => setActiveProvider(provider.id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["active-provider"] });
    },
  });

  const stored = status.data;
  const connected = stored?.kind === "Connected";
  const modelCount = stored?.kind === "Connected" ? stored.detail.models.length : 0;
  const canActivate = connected && !active;

  return (
    <div
      className={cn(
        "rounded-lg border bg-surface p-4 transition-colors duration-snappy ease-enter",
        active ? "border-accent" : "border-border-default",
      )}
    >
      <header className="flex items-center justify-between gap-3">
        <div className="flex items-center gap-2">
          <ActiveDot active={active} />
          <h3 className="text-sm font-medium text-text-strong">{provider.label}</h3>
          {active && (
            <span className="rounded-full bg-accent-soft px-2 py-0.5 text-[10px] font-medium uppercase tracking-wider text-accent">
              Active
            </span>
          )}
        </div>
        <StatusBadge status={stored} loading={status.isLoading} />
      </header>

      {provider.needs_api_key && (
        <div className="mt-3 flex flex-col gap-2 sm:flex-row sm:items-center">
          <Input
            type="password"
            value={pendingKey}
            onChange={(e) => setPendingKey(e.target.value)}
            placeholder={connected ? "Replace stored key…" : `Paste your ${provider.label} API key`}
            className="flex-1"
          />
          <div className="flex gap-2">
            <Button
              type="button"
              size="sm"
              disabled={pendingKey.length === 0 || save.isPending}
              onClick={() => save.mutate(pendingKey)}
            >
              {save.isPending ? "Saving…" : connected ? "Replace key" : "Save & test"}
            </Button>
            {connected && (
              <Button
                type="button"
                size="sm"
                variant="ghost"
                disabled={remove.isPending}
                onClick={() => remove.mutate()}
              >
                {remove.isPending ? "Removing…" : "Remove key"}
              </Button>
            )}
          </div>
        </div>
      )}

      <footer className="mt-3 flex items-center justify-between text-xs text-muted-foreground">
        <StatusDetail status={stored} modelCount={modelCount} />
        {canActivate && (
          <Button
            type="button"
            size="sm"
            variant="secondary"
            disabled={activate.isPending}
            onClick={() => activate.mutate()}
          >
            {activate.isPending ? "Setting…" : "Make active"}
          </Button>
        )}
      </footer>
    </div>
  );
}

function ActiveDot({ active }: { active: boolean }) {
  if (active) {
    return (
      <span className="flex h-4 w-4 items-center justify-center rounded-full bg-accent text-accent-on">
        <Check className="h-3 w-3" strokeWidth={3} />
      </span>
    );
  }
  return <Circle className="h-4 w-4 text-muted-foreground" />;
}

function StatusBadge({
  status,
  loading,
}: {
  status: ProviderStatus | undefined;
  loading: boolean;
}) {
  if (loading) {
    return <Pill tone="muted">Checking…</Pill>;
  }
  if (!status) {
    return <Pill tone="muted">Unknown</Pill>;
  }
  switch (status.kind) {
    case "Connected":
      return <Pill tone="success">Connected</Pill>;
    case "NotConfigured":
      return <Pill tone="muted">Not configured</Pill>;
    case "InvalidKey":
      return <Pill tone="danger">Invalid key</Pill>;
    case "Unreachable":
      return <Pill tone="warning">Unreachable</Pill>;
    case "Error":
      return <Pill tone="danger">Error</Pill>;
  }
}

function StatusDetail({
  status,
  modelCount,
}: {
  status: ProviderStatus | undefined;
  modelCount: number;
}) {
  if (!status) return <span />;
  switch (status.kind) {
    case "Connected":
      return (
        <span>
          {modelCount} model{modelCount === 1 ? "" : "s"} available
        </span>
      );
    case "NotConfigured":
      return <span>No key stored.</span>;
    case "InvalidKey":
    case "Unreachable":
    case "Error":
      return <span className="truncate text-danger">{status.detail.message}</span>;
  }
}

function Pill({
  tone,
  children,
}: {
  tone: "muted" | "success" | "warning" | "danger";
  children: React.ReactNode;
}) {
  const toneMap: Record<typeof tone, string> = {
    muted: "bg-surface-alt text-muted-foreground",
    success: "bg-success/15 text-success",
    warning: "bg-warning/15 text-warning",
    danger: "bg-danger/15 text-danger",
  };
  return (
    <span
      className={cn(
        "inline-flex items-center rounded-full px-2 py-0.5 text-[11px] font-medium",
        toneMap[tone],
      )}
    >
      {children}
    </span>
  );
}

export function isProviderConnected(p: ProviderId, status?: ProviderStatus): boolean {
  if (!status) return false;
  return status.kind === "Connected" && p === p; // p comparison reserved for future per-provider checks
}
