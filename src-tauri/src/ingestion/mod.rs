//! PDF parse → chunk → embed pipeline. Runs inside Tokio tasks kicked off by
//! commands; emits `document.ready` events when a document finishes indexing.
//! Uses `pdfium-render` for extraction and falls back to `lopdf` for quirky
//! PDFs.
