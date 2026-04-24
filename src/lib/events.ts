import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { DocumentStatus } from "@/types/bindings";

/*
 * Typed event listeners. Rust emitters are defined in
 * src-tauri/src/ingestion/pipeline.rs (`EVENT_DOCUMENT_STATUS`); the string
 * below must stay in sync with that constant.
 *
 * The payload shape mirrors `DocumentStatusPayload` in pipeline.rs. It is
 * kept in this file manually because tauri-specta's auto-generated bindings
 * cover #[tauri::command] inputs/outputs only — event payloads need
 * `collect_events!`, which we will wire in a later pass.
 */

export const DOCUMENT_STATUS_EVENT = "document-status";

export type DocumentStatusPayload = {
  document_id: string;
  status: DocumentStatus;
  error: string | null;
};

export async function onDocumentStatus(
  cb: (payload: DocumentStatusPayload) => void,
): Promise<UnlistenFn> {
  return listen<DocumentStatusPayload>(DOCUMENT_STATUS_EVENT, (event) => {
    cb(event.payload);
  });
}
