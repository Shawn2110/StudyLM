import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { DocumentStatus } from "@/types/bindings";

/*
 * Typed event listeners. Rust emitters are defined in
 * src-tauri/src/ingestion/pipeline.rs (`EVENT_DOCUMENT_STATUS`) and
 * src-tauri/src/generation/chat.rs (`EVENT_CHAT_STREAM`); the strings
 * below must stay in sync with those constants.
 *
 * Event payload types are mirrored manually here because tauri-specta's
 * generated bindings cover #[tauri::command] inputs/outputs only — event
 * payloads need `collect_events!`, which we will wire later.
 */

export const DOCUMENT_STATUS_EVENT = "document-status";
export const CHAT_STREAM_EVENT = "chat-stream";

export type DocumentStatusPayload = {
  document_id: string;
  status: DocumentStatus;
  error: string | null;
};

export type Citation = {
  chunk_id: number;
  document_id: string;
  document_filename: string;
  page: number;
};

export type ChatStreamPayload =
  | { kind: "delta"; chat_id: string; text: string }
  | {
      kind: "done";
      chat_id: string;
      message_id: string;
      citations: Citation[];
    }
  | { kind: "error"; chat_id: string; message: string };

export async function onDocumentStatus(
  cb: (payload: DocumentStatusPayload) => void,
): Promise<UnlistenFn> {
  return listen<DocumentStatusPayload>(DOCUMENT_STATUS_EVENT, (event) => {
    cb(event.payload);
  });
}

export async function onChatStream(
  cb: (payload: ChatStreamPayload) => void,
): Promise<UnlistenFn> {
  return listen<ChatStreamPayload>(CHAT_STREAM_EVENT, (event) => {
    cb(event.payload);
  });
}
