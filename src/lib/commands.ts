import { commands, type AppError, type Notebook, type PrepMode } from "@/types/bindings";

type Result<T, E> = { status: "ok"; data: T } | { status: "error"; error: E };

async function unwrap<T>(p: Promise<Result<T, AppError>>): Promise<T> {
  const r = await p;
  if (r.status === "error") {
    throw r.error;
  }
  return r.data;
}

export async function listNotebooks(): Promise<Notebook[]> {
  return unwrap(commands.listNotebooks());
}

export async function createNotebook(prepMode: PrepMode): Promise<Notebook> {
  return unwrap(commands.createNotebook(prepMode));
}
