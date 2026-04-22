//! Local embedding model (Candle-based, `nomic-embed-text-v1.5`). Downloads
//! the weights on first run, caches in the app-data folder, exposes a batched
//! `embed(&[&str])` API that the ingestion and retrieval pipelines share.
