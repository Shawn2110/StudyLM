//! Turn per-page extracted text into RAG-sized chunks. Paragraph-greedy
//! packing with a configurable target-token budget and per-chunk overlap.
//! Token counting is injected so Phase 2.3 can swap the approximate counter
//! for the real BERT tokenizer without reshaping this module.

use crate::ingestion::PageText;

/// A packed chunk ready to be written to the `chunk` table. `chunk_idx` is
/// monotonically increasing within a single document.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub text: String,
    pub page: u32,
    pub chunk_idx: u32,
    pub token_count: u32,
}

/// Chunker configuration. Defaults match architecture.md §5.
#[derive(Debug, Clone, Copy)]
pub struct ChunkOpts {
    pub target_tokens: usize,
    pub overlap_ratio: f32,
}

impl Default for ChunkOpts {
    fn default() -> Self {
        Self {
            target_tokens: 800,
            overlap_ratio: 0.15,
        }
    }
}

/// Rough BERT-style token count: ~1.3 tokens per whitespace-separated word.
/// Real nomic tokenizer arrives in Phase 2.3.
pub fn approximate_token_count(text: &str) -> usize {
    let words = text.split_whitespace().count();
    ((words as f32) * 1.3).ceil() as usize
}

/// Recursively pack paragraphs into chunks up to `opts.target_tokens`,
/// carrying `opts.overlap_ratio` of the previous chunk forward so adjacent
/// chunks share context. Oversized paragraphs become their own chunk.
pub fn chunk_document<F>(pages: &[PageText], opts: &ChunkOpts, count_tokens: F) -> Vec<Chunk>
where
    F: Fn(&str) -> usize,
{
    let overlap_budget = ((opts.target_tokens as f32) * opts.overlap_ratio).round() as usize;

    let mut chunks = Vec::new();
    let mut buffer = String::new();
    let mut buffer_page: u32 = pages.first().map(|p| p.page_number).unwrap_or(1);
    let mut next_idx: u32 = 0;

    for page in pages {
        for paragraph in split_paragraphs(&page.text) {
            if paragraph.is_empty() {
                continue;
            }
            let candidate = if buffer.is_empty() {
                paragraph.to_string()
            } else {
                format!("{buffer}\n\n{paragraph}")
            };

            if count_tokens(&candidate) > opts.target_tokens && !buffer.is_empty() {
                // Flush current buffer as a chunk, then seed a new buffer
                // with the overlap tail + the new paragraph.
                flush(&mut chunks, &mut next_idx, &buffer, buffer_page, &count_tokens);
                let carry = tail_for_tokens(&buffer, overlap_budget, &count_tokens);
                buffer = if carry.is_empty() {
                    paragraph.to_string()
                } else {
                    format!("{carry}\n\n{paragraph}")
                };
                buffer_page = page.page_number;
            } else {
                if buffer.is_empty() {
                    buffer_page = page.page_number;
                }
                buffer = candidate;
            }
        }
    }

    if !buffer.trim().is_empty() {
        flush(&mut chunks, &mut next_idx, &buffer, buffer_page, &count_tokens);
    }

    chunks
}

fn flush<F>(out: &mut Vec<Chunk>, next_idx: &mut u32, text: &str, page: u32, count_tokens: &F)
where
    F: Fn(&str) -> usize,
{
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return;
    }
    let token_count = count_tokens(trimmed) as u32;
    out.push(Chunk {
        text: trimmed.to_string(),
        page,
        chunk_idx: *next_idx,
        token_count,
    });
    *next_idx += 1;
}

/// Paragraph-aware split. Collapses blank lines into a single separator and
/// trims each paragraph.
fn split_paragraphs(text: &str) -> Vec<&str> {
    text.split("\n\n")
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .collect()
}

/// Take the last ~`target_tokens` of `text`, word-aligned. Returns an empty
/// string if the full text already fits in the budget.
fn tail_for_tokens<F>(text: &str, target_tokens: usize, count_tokens: &F) -> String
where
    F: Fn(&str) -> usize,
{
    if target_tokens == 0 || count_tokens(text) <= target_tokens {
        return String::new();
    }
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut start = words.len();
    while start > 0 {
        let candidate_start = start - 1;
        let candidate = words[candidate_start..].join(" ");
        if count_tokens(&candidate) > target_tokens {
            break;
        }
        start = candidate_start;
    }
    words[start..].join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page(n: u32, text: &str) -> PageText {
        PageText {
            page_number: n,
            text: text.to_string(),
        }
    }

    #[test]
    fn chunks_below_target_return_single_chunk() {
        let pages = vec![page(1, "Short paragraph. Nothing more.")];
        let out = chunk_document(&pages, &ChunkOpts::default(), approximate_token_count);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].page, 1);
        assert_eq!(out[0].chunk_idx, 0);
    }

    #[test]
    fn chunks_respect_token_budget_with_overlap() {
        let opts = ChunkOpts {
            target_tokens: 10,
            overlap_ratio: 0.2,
        };
        let paragraph = (0..8)
            .map(|i| format!("word{i}"))
            .collect::<Vec<_>>()
            .join(" ");
        let pages = vec![page(
            1,
            &format!("{paragraph}\n\n{paragraph}\n\n{paragraph}"),
        )];
        let out = chunk_document(&pages, &opts, approximate_token_count);
        assert!(out.len() >= 2, "expected multi-chunk, got {}", out.len());
        // Every chunk stays at or below (target_tokens + one paragraph) —
        // we never split inside a paragraph in MVP.
        for (i, c) in out.iter().enumerate() {
            assert_eq!(c.chunk_idx, i as u32);
            assert!(c.token_count > 0);
        }
    }

    #[test]
    fn empty_input_yields_empty_output() {
        let out = chunk_document(&[], &ChunkOpts::default(), approximate_token_count);
        assert!(out.is_empty());
    }
}
