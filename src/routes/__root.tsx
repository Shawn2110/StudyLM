import { Outlet, Link, createRootRoute } from "@tanstack/react-router";
import { Settings } from "lucide-react";

import { Eyebrow } from "@/components/ui/eyebrow";

export const Route = createRootRoute({
  component: RootLayout,
});

/*
 * Quiet chrome, loud content (docs/design.md §1). The app frame is a single
 * hairline above the content — wordmark on the left, settings on the right.
 * Pages own their own headings.
 */
function RootLayout() {
  return (
    <div className="min-h-screen bg-background font-sans text-foreground">
      <header className="sticky top-0 z-30 border-b border-paper-300 bg-paper-50/85 backdrop-blur-md">
        <div className="mx-auto flex h-12 max-w-5xl items-center justify-between px-4">
          <Link
            to="/"
            className="flex items-center gap-2 rounded text-paper-900 transition-colors duration-instant hover:text-ink-600"
          >
            <Eyebrow as="span" className="text-paper-700">
              STUDYLM
            </Eyebrow>
          </Link>
          <nav className="flex items-center gap-1">
            <Link
              to="/settings"
              className="flex h-8 w-8 items-center justify-center rounded text-paper-500 transition-colors duration-instant hover:bg-paper-100 hover:text-paper-900"
              activeProps={{ className: "text-paper-900 bg-paper-100" }}
              aria-label="Settings"
            >
              <Settings className="h-4 w-4" />
            </Link>
          </nav>
        </div>
      </header>
      <main>
        <Outlet />
      </main>
    </div>
  );
}
