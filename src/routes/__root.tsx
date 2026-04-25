import { Outlet, createRootRoute } from "@tanstack/react-router";

import { Sidebar } from "@/components/app/sidebar";

export const Route = createRootRoute({
  component: RootLayout,
});

/*
 * App shell — docs/design.md §4.1.
 * Slim top bar, persistent left sidebar, full-width main pane. Routes own
 * their own internal padding and max-widths.
 */
function RootLayout() {
  return (
    <div className="flex h-screen flex-col bg-bg font-sans text-text">
      <header className="flex h-topbar shrink-0 items-center border-b border-border-default bg-surface px-4">
        <span className="font-sans text-sm font-semibold tracking-tight text-text-strong">
          StudyLM
        </span>
      </header>
      <div className="flex min-h-0 flex-1">
        <Sidebar />
        <main className="min-w-0 flex-1 overflow-y-auto">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
