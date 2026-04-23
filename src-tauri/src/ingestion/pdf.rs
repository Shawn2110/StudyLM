//! PDF → per-page text, using the pure-Rust `lopdf` crate. Enough for MVP
//! ingestion; pdfium-render arrives as a quality upgrade in a later pass and
//! keeps this same `parse_pdf(path) -> Vec<PageText>` signature.

use std::path::Path;

use lopdf::Document;

use crate::error::{AppError, AppResult};

/// One page's worth of extracted text. Page numbers are 1-indexed to match
/// lopdf's `get_pages()` and what users see in a PDF reader.
#[derive(Debug, Clone)]
pub struct PageText {
    pub page_number: u32,
    pub text: String,
}

/// Open the PDF at `path` and return extracted text for every page. Pages
/// whose text extraction fails are kept as empty strings rather than aborting
/// the whole document — Phase 2.5's UI surfaces parse-quality in the
/// document status line.
pub fn parse_pdf(path: &Path) -> AppResult<Vec<PageText>> {
    let doc = Document::load(path)
        .map_err(|e| AppError::Internal(format!("load pdf {}: {e}", path.display())))?;

    let pages = doc.get_pages();
    let mut out = Vec::with_capacity(pages.len());
    for (page_number, _object_id) in pages {
        let text = doc.extract_text(&[page_number]).unwrap_or_default();
        out.push(PageText { page_number, text });
    }

    Ok(out)
}
