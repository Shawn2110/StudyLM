import { useState } from "react";
import { createFileRoute } from "@tanstack/react-router";

import { cn } from "@/lib/utils";

export const Route = createFileRoute("/settings")({
  component: SettingsPage,
});

/*
 * Settings — docs/design.md §4.2.
 * Two-column nav-rail + content. Sections beyond "About" are stubs until
 * the relevant phase ships (Providers in Phase 1, etc.).
 */
const SECTIONS = [
  { id: "system", label: "System" },
  { id: "providers", label: "Providers" },
  { id: "appearance", label: "Appearance" },
  { id: "shortcuts", label: "Shortcuts" },
  { id: "about", label: "About" },
] as const;

type SectionId = (typeof SECTIONS)[number]["id"];

function SettingsPage() {
  const [active, setActive] = useState<SectionId>("about");

  return (
    <section className="grid h-full grid-cols-[200px_1fr] gap-12 px-8 py-8">
      <aside>
        <p className="mb-3 px-3 text-sm font-medium text-muted-foreground">
          Settings
        </p>
        <nav className="flex flex-col gap-px">
          {SECTIONS.map((s) => (
            <button
              key={s.id}
              type="button"
              onClick={() => setActive(s.id)}
              className={cn(
                "rounded-md px-3 py-1.5 text-left text-sm font-sans transition-colors duration-instant ease-enter",
                s.id === active
                  ? "bg-accent-soft text-text-strong"
                  : "text-muted-foreground hover:bg-surface-alt hover:text-text-strong",
              )}
            >
              {s.label}
            </button>
          ))}
        </nav>
      </aside>

      <div>
        {active === "about" && <AboutSection />}
        {active === "system" && <Stub label="System" phase="Phase 7" />}
        {active === "providers" && <Stub label="Providers" phase="Phase 1" />}
        {active === "appearance" && (
          <Stub label="Appearance" phase="later — token toggle" />
        )}
        {active === "shortcuts" && (
          <Stub label="Shortcuts" phase="later — when ⌘K palette lands" />
        )}
      </div>
    </section>
  );
}

function AboutSection() {
  return (
    <article className="space-y-4">
      <h1 className="text-2xl font-semibold leading-tight tracking-[-0.02em] text-text-strong">
        StudyLM
      </h1>
      <p className="max-w-prose text-sm leading-relaxed text-text">
        A local-first, BYOK study companion. Documents stay on your machine;
        you pick the LLM.
      </p>
      <dl className="grid grid-cols-[120px_1fr] gap-y-2 text-sm">
        <dt className="text-muted-foreground">Version</dt>
        <dd className="font-mono text-text-strong">0.1.0 — Phase 0+2</dd>
        <dt className="text-muted-foreground">License</dt>
        <dd className="text-text-strong">MIT (or Apache-2.0; not yet decided)</dd>
        <dt className="text-muted-foreground">Source</dt>
        <dd className="font-mono text-text-strong">
          github.com/Shawn2110/StudyLM
        </dd>
      </dl>
    </article>
  );
}

function Stub({ label, phase }: { label: string; phase: string }) {
  return (
    <article className="space-y-3">
      <h1 className="text-2xl font-semibold leading-tight tracking-[-0.02em] text-text-strong">
        {label}
      </h1>
      <p className="text-sm text-muted-foreground">
        Lands in <span className="font-mono text-text">{phase}</span>.
      </p>
    </article>
  );
}
