-- Documents, chunks, and FTS5 mirror. The vec0 (sqlite-vec) virtual table is
-- created at runtime in db::init_pool because sqlx-cli's SQLite binary does
-- not have the sqlite-vec extension auto-loaded — only our app's binary does.
-- See docs/architecture.md §4.

CREATE TABLE document (
  id          TEXT PRIMARY KEY,
  notebook_id TEXT NOT NULL REFERENCES notebook(id) ON DELETE CASCADE,
  filename    TEXT NOT NULL,
  source_type TEXT NOT NULL,            -- pdf|url|md|text
  source_url  TEXT,
  local_path  TEXT NOT NULL,            -- absolute path inside app-managed folder
  page_count  INTEGER,
  status      TEXT NOT NULL,            -- pending|parsing|embedding|ready|failed
  error       TEXT,
  created_at  INTEGER NOT NULL
);

CREATE INDEX idx_document_notebook ON document(notebook_id);

CREATE TABLE chunk (
  id            INTEGER PRIMARY KEY,
  document_id   TEXT NOT NULL REFERENCES document(id) ON DELETE CASCADE,
  page          INTEGER NOT NULL,
  chunk_idx     INTEGER NOT NULL,
  text          TEXT NOT NULL,
  token_count   INTEGER NOT NULL,
  headings_json TEXT
);

CREATE INDEX idx_chunk_document ON chunk(document_id);

-- Full-text search mirror, kept in sync via triggers on `chunk`.
CREATE VIRTUAL TABLE chunk_fts USING fts5(
  text,
  content='chunk',
  content_rowid='id'
);

CREATE TRIGGER chunk_ai AFTER INSERT ON chunk BEGIN
  INSERT INTO chunk_fts(rowid, text) VALUES (new.id, new.text);
END;

CREATE TRIGGER chunk_ad AFTER DELETE ON chunk BEGIN
  INSERT INTO chunk_fts(chunk_fts, rowid, text) VALUES('delete', old.id, old.text);
END;

CREATE TRIGGER chunk_au AFTER UPDATE ON chunk BEGIN
  INSERT INTO chunk_fts(chunk_fts, rowid, text) VALUES('delete', old.id, old.text);
  INSERT INTO chunk_fts(rowid, text) VALUES (new.id, new.text);
END;
