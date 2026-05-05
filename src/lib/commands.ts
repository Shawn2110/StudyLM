import {
  commands,
  type AppError,
  type Chat,
  type Document,
  type Message,
  type Notebook,
  type PrepMode,
  type ProviderId,
  type ProviderInfo,
  type ProviderStatus,
} from "@/types/bindings";

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

export async function listDocuments(notebookId: string): Promise<Document[]> {
  return unwrap(commands.listDocuments(notebookId));
}

export async function ingestDocument(
  notebookId: string,
  path: string,
): Promise<string> {
  return unwrap(commands.ingestDocument(notebookId, path));
}

export async function listProviders(): Promise<ProviderInfo[]> {
  return unwrap(commands.listProviders());
}

export async function validateProviderKey(
  provider: ProviderId,
  apiKey: string,
): Promise<ProviderStatus> {
  return unwrap(commands.validateProviderKey(provider, apiKey));
}

export async function getProviderStatus(
  provider: ProviderId,
): Promise<ProviderStatus> {
  return unwrap(commands.getProviderStatus(provider));
}

export async function storeProviderKey(
  provider: ProviderId,
  apiKey: string,
): Promise<void> {
  await unwrap(commands.storeProviderKey(provider, apiKey));
}

export async function deleteProviderKey(provider: ProviderId): Promise<void> {
  await unwrap(commands.deleteProviderKey(provider));
}

export async function setActiveProvider(
  provider: ProviderId | null,
): Promise<void> {
  await unwrap(commands.setActiveProvider(provider));
}

export async function getActiveProvider(): Promise<ProviderId | null> {
  return unwrap(commands.getActiveProvider());
}

export async function listChats(notebookId: string): Promise<Chat[]> {
  return unwrap(commands.listChats(notebookId));
}

export async function createChat(notebookId: string): Promise<Chat> {
  return unwrap(commands.createChat(notebookId));
}

export async function listMessages(chatId: string): Promise<Message[]> {
  return unwrap(commands.listMessages(chatId));
}

export async function sendChatMessage(
  chatId: string,
  userText: string,
  modelId: string,
): Promise<string> {
  return unwrap(commands.sendChatMessage(chatId, userText, modelId));
}
