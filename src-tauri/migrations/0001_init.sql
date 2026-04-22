CREATE TABLE notebook (
  id               TEXT PRIMARY KEY,
  title            TEXT NOT NULL,
  created_at       INTEGER NOT NULL,
  exam_type        TEXT NOT NULL,
  format           TEXT NOT NULL,
  subject          TEXT,
  duration_minutes INTEGER,
  exam_at          INTEGER,
  difficulty_focus TEXT
);

CREATE TABLE settings (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
