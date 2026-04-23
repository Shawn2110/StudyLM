//! Local embedding model (Candle-based, `nomic-embed-text-v1.5`). Downloads
//! the weights on first run, caches under the shared HuggingFace cache, and
//! exposes a batched `embed(&[&str])` API that the ingestion and retrieval
//! pipelines share.

pub mod nomic;

use std::sync::Arc;

pub use nomic::NomicEmbedder;

use crate::error::AppResult;

/// Shared embedder slot held in Tauri state. Lazily initialized on first
/// ingest so the ~250 MB model download does not block app startup.
pub type EmbedderSlot = Arc<tokio::sync::OnceCell<Arc<NomicEmbedder>>>;

/// Build an empty slot ready to be managed by Tauri state.
pub fn new_slot() -> EmbedderSlot {
    Arc::new(tokio::sync::OnceCell::new())
}

/// Return the shared embedder, downloading and loading it on first call.
pub async fn get_or_init(slot: &EmbedderSlot) -> AppResult<Arc<NomicEmbedder>> {
    let arc = slot
        .get_or_try_init(|| async { NomicEmbedder::load().await.map(Arc::new) })
        .await?;
    Ok(Arc::clone(arc))
}
