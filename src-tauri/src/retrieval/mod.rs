//! Hybrid search per docs/architecture.md §7.
//!
//! 1. Dense — embed the query with the same Candle model that wrote
//!    chunk vectors, then do a top-K KNN over `chunk_vec`.
//! 2. Sparse — FTS5 BM25 over `chunk_fts`, top-K.
//! 3. Fusion — Reciprocal Rank Fusion (k=60) across the two lists.
//! 4. Take top-N from the fused list and hydrate with chunk + document
//!    metadata, ready for the generation prompt.

use std::collections::HashMap;
use std::sync::Arc;

use sqlx::SqlitePool;

use crate::db::chunks::{self, ChunkWithDocument};
use crate::embeddings::NomicEmbedder;
use crate::error::AppResult;

/// Tunable parameters for the hybrid pipeline. Defaults match
/// architecture §7.
#[derive(Debug, Clone, Copy)]
pub struct RetrievalOpts {
    pub dense_k: i64,
    pub fts_k: i64,
    pub rrf_k: f32,
    pub top_n: usize,
}

impl Default for RetrievalOpts {
    fn default() -> Self {
        Self {
            dense_k: 40,
            fts_k: 40,
            rrf_k: 60.0,
            top_n: 5,
        }
    }
}

/// One retrieved chunk, ready to format into the generation prompt.
#[derive(Debug, Clone)]
pub struct RetrievedChunk {
    pub chunk_id: i64,
    pub document_id: String,
    pub document_filename: String,
    pub page: i64,
    pub text: String,
}

impl From<ChunkWithDocument> for RetrievedChunk {
    fn from(c: ChunkWithDocument) -> Self {
        Self {
            chunk_id: c.id,
            document_id: c.document_id,
            document_filename: c.document_filename,
            page: c.page,
            text: c.text,
        }
    }
}

/// Run the full hybrid pipeline for `query` against `notebook_id`.
/// The query embedding step is CPU-bound and runs on a blocking thread.
pub async fn retrieve(
    pool: &SqlitePool,
    embedder: Arc<NomicEmbedder>,
    notebook_id: &str,
    query: &str,
    opts: RetrievalOpts,
) -> AppResult<Vec<RetrievedChunk>> {
    // 1. Dense — embed the query, then KNN against vec0.
    let query_text = query.to_string();
    let query_vec_rows = tokio::task::spawn_blocking(move || embedder.embed(&[&query_text]))
        .await
        .map_err(|e| crate::error::AppError::Internal(format!("embed join: {e}")))??;
    let query_vec = query_vec_rows
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::AppError::Internal("empty query embedding".into()))?;
    let dense_ids = chunks::dense_search(pool, notebook_id, &query_vec, opts.dense_k).await?;

    // 2. Sparse — FTS5 BM25.
    let fts_query = sanitize_fts_query(query);
    let fts_ids = if fts_query.is_empty() {
        Vec::new()
    } else {
        chunks::fts_search(pool, notebook_id, &fts_query, opts.fts_k).await?
    };

    // 3. RRF fuse.
    let fused = rrf_fuse(&dense_ids, &fts_ids, opts.rrf_k);
    let top_ids: Vec<i64> = fused.iter().take(opts.top_n).map(|(id, _)| *id).collect();

    // 4. Hydrate with chunk + document metadata.
    let rows = chunks::fetch_chunks(pool, &top_ids).await?;
    Ok(rows.into_iter().map(RetrievedChunk::from).collect())
}

/// Format retrieved chunks into the source block expected by the
/// generation prompt: each chunk is wrapped in a `<source …>` tag the LLM
/// is asked to cite by id.
pub fn format_sources(chunks: &[RetrievedChunk]) -> String {
    let mut out = String::new();
    for c in chunks {
        out.push_str(&format!(
            "<source id=\"{}\" doc=\"{}\" page=\"{}\">\n{}\n</source>\n",
            c.chunk_id,
            escape_attr(&c.document_filename),
            c.page,
            c.text.trim()
        ));
    }
    out
}

fn rrf_fuse(dense: &[i64], sparse: &[i64], k: f32) -> Vec<(i64, f32)> {
    let mut scores: HashMap<i64, f32> = HashMap::new();
    for (rank, id) in dense.iter().enumerate() {
        *scores.entry(*id).or_insert(0.0) += 1.0 / (k + (rank as f32) + 1.0);
    }
    for (rank, id) in sparse.iter().enumerate() {
        *scores.entry(*id).or_insert(0.0) += 1.0 / (k + (rank as f32) + 1.0);
    }
    let mut pairs: Vec<(i64, f32)> = scores.into_iter().collect();
    pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    pairs
}

/// FTS5 query syntax is finicky — colons, parens, and quotes can produce
/// a `SyntaxError`. For MVP we tokenise the query into alphanumeric words
/// and OR them. Phrase matching can come later behind a UI toggle.
fn sanitize_fts_query(raw: &str) -> String {
    let words: Vec<String> = raw
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 1)
        .map(|w| w.to_lowercase())
        .collect();
    if words.is_empty() {
        return String::new();
    }
    words.join(" OR ")
}

fn escape_attr(s: &str) -> String {
    s.replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rrf_fuse_combines_rankings() {
        let dense = vec![10, 20, 30];
        let sparse = vec![20, 10, 40];
        let fused = rrf_fuse(&dense, &sparse, 60.0);
        let top_two: std::collections::HashSet<i64> =
            fused.iter().take(2).map(|(id, _)| *id).collect();
        let expected: std::collections::HashSet<i64> = [10, 20].into_iter().collect();
        assert_eq!(top_two, expected);
    }

    #[test]
    fn sanitize_fts_drops_punctuation() {
        assert_eq!(
            sanitize_fts_query("what is gibbs free energy?"),
            "what OR is OR gibbs OR free OR energy"
        );
        assert_eq!(sanitize_fts_query(" \" '"), "");
    }

    #[test]
    fn format_sources_wraps_each_chunk() {
        let chunks = vec![RetrievedChunk {
            chunk_id: 7,
            document_id: "doc-1".into(),
            document_filename: "Thermo.pdf".into(),
            page: 47,
            text: "A short body.".into(),
        }];
        let out = format_sources(&chunks);
        assert!(out.contains("id=\"7\""));
        assert!(out.contains("doc=\"Thermo.pdf\""));
        assert!(out.contains("page=\"47\""));
        assert!(out.contains("A short body."));
    }
}
