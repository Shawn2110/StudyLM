import { useState } from "react";
import { createFileRoute } from "@tanstack/react-router";

import { Eyebrow } from "@/components/ui/eyebrow";
import { cn } from "@/lib/utils";

export const Route = createFileRoute("/settings")({
  component: SettingsPage,
});

/*
 * Settings — docs/design.md §8.8.
 * Two-column nav-rail + content. Sections beyond "About" are stubs until
 * the relevant phase ships (Providers in Phase 1, Shortcuts when the
 * command palette lands, etc.).
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
    <section className="mx-auto grid max-w-5xl grid-cols-[180px_1fr] gap-12 px-4 pb-24 pt-12">
      <aside>
        <Eyebrow className="mb-4 block">Settings</Eyebrow>
        <nav className="flex flex-col gap-px">
          {SECTIONS.map((s) => (
            <button
              key={s.id}
              type="button"
              onClick={() => setActive(s.id)}
              className={cn(
                "rounded px-2 py-1.5 text-left text-sm font-sans transition-colors duration-instant ease-enter",
                s.id === active
                  ? "bg-paper-200 text-paper-900"
                  : "text-paper-500 hover:bg-paper-100 hover:text-paper-900",
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
      <h1 className="font-serif text-[1.75rem] font-medium leading-tight tracking-[-0.015em] text-paper-900">
        StudyLM
      </h1>
      <p className="max-w-prose text-sm font-sans leading-relaxed text-paper-700">
        A local-first, BYOK study companion. Documents stay on your machine;
        you pick the LLM.
      </p>
      <dl className="grid grid-cols-[120px_1fr] gap-y-1 font-mono text-xs text-paper-500">
        <dt>Version</dt>
        <dd>0.1.0 — Phase 0</dd>
        <dt>License</dt>
        <dd>MIT (or Apache-2.0; not yet decided)</dd>
        <dt>Source</dt>
        <dd>github.com/Shawn2110/StudyLM</dd>
      </dl>
    </article>
  );
}

function Stub({ label, phase }: { label: string; phase: string }) {
  return (
    <article className="space-y-3">
      <h1 className="font-serif text-[1.75rem] font-medium leading-tight tracking-[-0.015em] text-paper-900">
        {label}
      </h1>
      <p className="text-sm font-sans text-paper-500">
        Lands in <span className="font-mono text-paper-700">{phase}</span>.
      </p>
    </article>
  );
}
