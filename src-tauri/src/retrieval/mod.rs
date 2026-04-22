//! Hybrid retrieval over the notebook's chunks: dense (sqlite-vec) + sparse
//! (FTS5 BM25) merged via Reciprocal Rank Fusion, with optional provider
//! rerank. Returns a ranked slice of chunks formatted for LLM context.
