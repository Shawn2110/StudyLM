import { Outlet, Link, createRootRoute } from "@tanstack/react-router";

export const Route = createRootRoute({
  component: RootLayout,
});

function RootLayout() {
  return (
    <div className="min-h-screen bg-background text-foreground">
      <header className="border-b">
        <nav className="mx-auto flex h-12 max-w-5xl items-center gap-4 px-4 text-sm">
          <Link
            to="/"
            className="font-semibold tracking-tight"
            activeProps={{ className: "font-semibold tracking-tight" }}
          >
            StudyLM
          </Link>
          <div className="ml-auto flex items-center gap-3">
            <Link
              to="/"
              className="text-muted-foreground hover:text-foreground"
              activeProps={{ className: "text-foreground" }}
            >
              Notebooks
            </Link>
            <Link
              to="/settings"
              className="text-muted-foreground hover:text-foreground"
              activeProps={{ className: "text-foreground" }}
            >
              Settings
            </Link>
          </div>
        </nav>
      </header>
      <main className="mx-auto max-w-5xl px-4 py-6">
        <Outlet />
      </main>
    </div>
  );
}
