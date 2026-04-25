import { Link, useMatchRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { Settings as SettingsIcon } from "lucide-react";

import { CreateNotebookDialog } from "@/components/notebook/create-notebook-dialog";
import { cn } from "@/lib/utils";
import { listNotebooks } from "@/lib/commands";

/*
 * Persistent left sidebar — docs/design.md §4.1.
 * Holds the create-notebook action, the notebook list, and a settings
 * link pinned to the bottom.
 */
export function Sidebar() {
  const matchRoute = useMatchRoute();
  const { data: notebooks } = useQuery({
    queryKey: ["notebooks"],
    queryFn: listNotebooks,
  });

  const activeSettings = matchRoute({ to: "/settings" }) !== false;

  return (
    <aside
      aria-label="Notebooks"
      className="flex h-full w-sidebar shrink-0 flex-col gap-4 border-r border-border-default bg-surface px-3 py-4"
    >
      <CreateNotebookDialog />

      <div className="flex min-h-0 flex-1 flex-col">
        <p className="px-3 pb-2 text-xs font-medium text-muted-foreground">
          Notebooks
        </p>
        <div className="flex-1 overflow-y-auto">
          {notebooks && notebooks.length === 0 && (
            <p className="px-3 py-2 text-xs text-muted-foreground">
              No notebooks yet.
            </p>
          )}
          <ul className="flex flex-col gap-px">
            {notebooks?.map((nb) => {
              const active =
                matchRoute({
                  to: "/notebooks/$notebookId",
                  params: { notebookId: nb.id },
                }) !== false;
              return (
                <li key={nb.id}>
                  <Link
                    to="/notebooks/$notebookId"
                    params={{ notebookId: nb.id }}
                    title={nb.title}
                    className={cn(
                      "block truncate rounded-md px-3 py-1.5 text-sm font-sans transition-colors duration-instant ease-enter",
                      active
                        ? "bg-accent-soft text-text-strong"
                        : "text-muted-foreground hover:bg-surface-alt hover:text-text-strong",
                    )}
                  >
                    {nb.title}
                  </Link>
                </li>
              );
            })}
          </ul>
        </div>
      </div>

      <Link
        to="/settings"
        className={cn(
          "flex items-center gap-2 rounded-md px-3 py-2 text-sm font-sans transition-colors duration-instant ease-enter",
          activeSettings
            ? "bg-accent-soft text-text-strong"
            : "text-muted-foreground hover:bg-surface-alt hover:text-text-strong",
        )}
      >
        <SettingsIcon className="h-4 w-4" />
        Settings
      </Link>
    </aside>
  );
}
