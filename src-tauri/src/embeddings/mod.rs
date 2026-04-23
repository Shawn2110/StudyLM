//! Local embedding model (Candle-based, `nomic-embed-text-v1.5`). Downloads
//! the weights on first run, caches under the shared HuggingFace cache, and
//! exposes a batched `embed(&[&str])` API that the ingestion and retrieval
//! pipelines share.

pub mod nomic;

pub use nomic::NomicEmbedder;
