//! Parse `[id]` citation markers out of an assistant reply and pair them
//! with the retrieved chunks they reference. Only ids the model actually
//! used + that we sent are kept — invented ids are dropped silently.

use std::collections::HashSet;

use crate::db::models::Citation;
use crate::retrieval::RetrievedChunk;

/// Scan `text` for tokens of the form `[12345]` and return the set of
/// integer ids found. Brackets containing anything other than digits are
/// ignored.
pub fn extract_ids(text: &str) -> HashSet<i64> {
    let mut out = HashSet::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'[' {
            let start = i + 1;
            let mut end = start;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if end > start && end < bytes.len() && bytes[end] == b']' {
                if let Ok(s) = std::str::from_utf8(&bytes[start..end]) {
                    if let Ok(id) = s.parse::<i64>() {
                        out.insert(id);
                    }
                }
                i = end + 1;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// Given the assistant's full reply text and the chunks we retrieved for
/// this turn, return the subset of chunks the reply cited.
pub fn collect(text: &str, sources: &[RetrievedChunk]) -> Vec<Citation> {
    let cited = extract_ids(text);
    sources
        .iter()
        .filter(|c| cited.contains(&c.chunk_id))
        .map(|c| Citation {
            chunk_id: c.chunk_id,
            document_id: c.document_id.clone(),
            document_filename: c.document_filename.clone(),
            page: c.page,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chunk(id: i64) -> RetrievedChunk {
        RetrievedChunk {
            chunk_id: id,
            document_id: format!("doc-{id}"),
            document_filename: format!("file-{id}.pdf"),
            page: id,
            text: String::new(),
        }
    }

    #[test]
    fn extracts_numeric_ids() {
        let ids = extract_ids("First law [42] and second law [17] differ.");
        assert!(ids.contains(&42));
        assert!(ids.contains(&17));
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn ignores_non_numeric_brackets() {
        let ids = extract_ids("see [appendix] and [12].");
        assert_eq!(ids.len(), 1);
        assert!(ids.contains(&12));
    }

    #[test]
    fn collect_filters_to_known_sources() {
        let sources = vec![chunk(1), chunk(2), chunk(3)];
        let cites = collect("uses [2] and [99] (invented).", &sources);
        assert_eq!(cites.len(), 1);
        assert_eq!(cites[0].chunk_id, 2);
    }
}
