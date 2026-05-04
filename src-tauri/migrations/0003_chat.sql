-- Chat sessions and their messages. Per docs/architecture.md §4.
-- One notebook holds many chat sessions; one chat session holds many
-- messages. Citations are stored as JSON on the assistant message
-- (parsed from `[id]` markers in the streamed content).

CREATE TABLE chat (
  id          TEXT PRIMARY KEY,
  notebook_id TEXT NOT NULL REFERENCES notebook(id) ON DELETE CASCADE,
  title       TEXT,
  model_id    TEXT,
  provider    TEXT,
  created_at  INTEGER NOT NULL
);

CREATE INDEX idx_chat_notebook ON chat(notebook_id);

CREATE TABLE message (
  id             TEXT PRIMARY KEY,
  chat_id        TEXT NOT NULL REFERENCES chat(id) ON DELETE CASCADE,
  role           TEXT NOT NULL,         -- user|assistant
  content        TEXT NOT NULL,
  citations_json TEXT,                  -- JSON array of {chunk_id, document_id, page}
  created_at     INTEGER NOT NULL
);

CREATE INDEX idx_message_chat ON message(chat_id);
