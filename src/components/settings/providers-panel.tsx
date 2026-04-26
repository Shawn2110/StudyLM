import { useQuery } from "@tanstack/react-query";

import { ProviderRow } from "@/components/settings/provider-row";
import { getActiveProvider, listProviders } from "@/lib/commands";

/*
 * Settings → Providers panel. Lists all five LLM providers, surfaces
 * each one's stored-key status, and lets the user pick which is active.
 */
export function ProvidersPanel() {
  const providers = useQuery({
    queryKey: ["providers"],
    queryFn: listProviders,
    refetchOnWindowFocus: false,
  });
  const active = useQuery({
    queryKey: ["active-provider"],
    queryFn: getActiveProvider,
    refetchOnWindowFocus: false,
  });

  return (
    <article className="space-y-6">
      <header className="space-y-1">
        <h1 className="text-2xl font-semibold leading-tight tracking-[-0.02em] text-text-strong">
          Providers
        </h1>
        <p className="max-w-prose text-sm leading-relaxed text-muted-foreground">
          Pick the LLM you want StudyLM to use. Keys live in your OS keychain
          (Windows Credential Manager / macOS Keychain / Linux Secret Service)
          and never touch disk in plaintext. Ollama is detected on{" "}
          <span className="font-mono text-text">localhost:11434</span> with no
          key required.
        </p>
      </header>

      {providers.isLoading && (
        <p className="text-sm text-muted-foreground">Loading providers…</p>
      )}

      {providers.error && (
        <p className="text-sm text-danger">
          {String(
            (providers.error as { message?: string })?.message ?? providers.error,
          )}
        </p>
      )}

      <div className="grid gap-3">
        {providers.data?.map((p) => (
          <ProviderRow
            key={p.id}
            provider={p}
            active={active.data === p.id}
          />
        ))}
      </div>
    </article>
  );
}
