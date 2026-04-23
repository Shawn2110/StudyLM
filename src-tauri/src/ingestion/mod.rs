//! PDF parse → chunk → embed pipeline. Runs inside Tokio tasks kicked off by
//! commands; emits `document.ready` events when a document finishes indexing.
//! Starts with `lopdf` for Phase 2; pdfium-render arrives as a quality
//! upgrade in a later pass behind the same `parse_pdf` signature.

pub mod chunk;
pub mod pdf;
pub mod pipeline;

pub use chunk::{approximate_token_count, chunk_document, Chunk, ChunkOpts};
pub use pdf::{parse_pdf, PageText};
pub use pipeline::{ingest_pdf, EVENT_DOCUMENT_STATUS};
