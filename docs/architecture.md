# StudyLM — Architecture

**Status:** Draft v2 (desktop app)
**Last updated:** 2026-04-21

## 1. Overview

StudyLM is a Tauri 2 desktop application with two major layers:

- **Rust backend** (in `src-tauri/`) — PDF parsing, chunking, local embeddings, SQLite + sqlite-vec storage, keychain access, LLM API calls, async job orchestration.
- **React frontend** (in `src/`) — UI only. Chat streaming display, PDF viewer, flashcard review, settings.

The two halves communicate over Tauri's IPC: React invokes Rust **commands** (request/response) and listens to Rust **events** (streams, progress updates).

There is **no server**, no cloud backend, no auth. All data lives in a single SQLite file on the user's disk. API keys live in the OS keychain. LLM traffic goes directly from Rust to the user's chosen provider.

## 2. Stack

| Layer | Choice | Rationale |
|---|---|---|
| Shell | **Tauri 2** | Native-weight binaries (~5–10 MB), Rust backend, WebView frontend. Not Electron — no bundled Chromium. |
| Backend | **Rust** (edition 2021) | One language for PDF parsing, embeddings, SQLite, keychain, HTTP. No runtime overhead. |
| Async runtime | **Tokio** | Every I/O function is async. |
| Frontend build | **Vite** | Fast HMR, minimal config. |
| UI | **React 18 + TypeScript (strict)** | Familiar, large ecosystem. |
| Routing | **TanStack Router** | Type-safe, file-based, no server dependency. |
| Styling | **TailwindCSS + shadcn/ui** | Fast to style, consistent component library. |
| Streaming LLM UI | **Vercel AI SDK** (client-side only) | Used purely for its streaming-display hooks. Actual LLM calls happen in Rust. |
| Storage | **SQLite + sqlite-vec + FTS5** | Single file; vectors, text, and metadata live in the same DB. Embedded, zero-setup. |
| DB client | **sqlx** | Compile-time checked queries, async. |
| PDF parsing | **pdfium-render** (Rust bindings to Chromium's pdfium) | Reliable text + layout extraction. `lopdf` as pure-Rust fallback. |
| Local embeddings | **Candle** running `nomic-embed-text-v1.5` | Rust ML inference, no Python dependency. ~200 MB model, ~50 ms per chunk on CPU. |
| LLM providers | Anthropic, OpenAI, Google, OpenRouter, Ollama | User picks one; key in keychain. |
| Keychain | **keyring-rs** | macOS Keychain, Windows Credential Manager, Linux Secret Service. |
| HTTP | **reqwest** | Async, SSE streaming support. |
| IPC types | **specta** + **tauri-specta** | Generates TypeScript types from Rust structs at build time. |
| Logging | **tracing** + `tracing-subscriber` | Structured logs to local file + console in dev. |
| TTS (P1) | ElevenLabs (HTTP) or Piper (bundled, local) | User picks online quality vs fully-offline. |
| Packaging | Tauri bundler | Produces `.dmg`, `.msi`, `.AppImage`, `.deb`. |
| Updates | Tauri Updater | Signed update manifests from GitHub Releases. |

## 3. Repo layout

```
studylm/
├── src-tauri/                # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── error.rs          # single AppError enum (Serialize)
│   │   ├── commands/         # #[tauri::command] functions called from React
│   │   ├── ingestion/        # PDF parse → chunk → embed
│   │   ├── retrieval/        # hybrid search (vec + FTS5 + optional rerank)
│   │   ├── generation/       # chat, guide, flashcards, podcast
│   │   ├── llm/              # LlmProvider trait + implementations
│   │   ├── embeddings/       # Candle-based local embedder
│   │   ├── db/               # sqlx queries, connection pool
│   │   ├── keychain/         # keyring wrapper
│   │   ├── prompts/          # prompt templates per prep mode (Tera)
│   │   └── events/           # typed event emitters
│   ├── migrations/           # SQL migrations
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                      # React frontend
│   ├── routes/               # TanStack Router file-based routes
│   ├── components/
│   ├── hooks/
│   ├── lib/                  # invoke() wrappers, typed event listeners
│   ├── types/                # GENERATED from Rust via specta — do not edit
│   └── main.tsx
├── vite.config.ts
├── tailwind.config.ts
├── package.json
├── prd.md
├── architecture.md
├── plan.md
└── agent.md
```

## 4. Data model (SQLite)

```sql
CREATE TABLE notebook (
  id               TEXT PRIMARY KEY,
  title            TEXT NOT NULL,
  created_at       INTEGER NOT NULL,
  -- prep mode
  exam_type        TEXT NOT NULL,  -- internal|midsem|endsem|viva|practical|competitive|custom
  format           TEXT NOT NULL,  -- mcq|short|long|oral|numerical|mixed
  subject          TEXT,
  duration_minutes INTEGER,
  exam_at          INTEGER,
  difficulty_focus TEXT            -- conceptual|problemsolving|memorization|mixed
);

CREATE TABLE document (
  id          TEXT PRIMARY KEY,
  notebook_id TEXT NOT NULL REFERENCES notebook(id) ON DELETE CASCADE,
  filename    TEXT NOT NULL,
  source_type TEXT NOT NULL,       -- pdf|url|md|text
  source_url  TEXT,
  local_path  TEXT NOT NULL,       -- absolute path in app-managed folder
  page_count  INTEGER,
  status      TEXT NOT NULL,       -- pending|parsing|embedding|ready|failed
  error       TEXT,
  created_at  INTEGER NOT NULL
);

CREATE TABLE chunk (
  id            INTEGER PRIMARY KEY,
  document_id   TEXT NOT NULL REFERENCES document(id) ON DELETE CASCADE,
  page          INTEGER NOT NULL,
  chunk_idx     INTEGER NOT NULL,
  text          TEXT NOT NULL,
  token_count   INTEGER NOT NULL,
  headings_json TEXT
);

-- full-text search
CREATE VIRTUAL TABLE chunk_fts USING fts5(
  text, content='chunk', content_rowid='id'
);

-- vector search (sqlite-vec)
CREATE VIRTUAL TABLE chunk_vec USING vec0(
  embedding float[768]
);

CREATE TABLE chat (
  id          TEXT PRIMARY KEY,
  notebook_id TEXT NOT NULL REFERENCES notebook(id) ON DELETE CASCADE,
  created_at  INTEGER NOT NULL
);

CREATE TABLE message (
  id             TEXT PRIMARY KEY,
  chat_id        TEXT NOT NULL REFERENCES chat(id) ON DELETE CASCADE,
  role           TEXT NOT NULL,  -- user|assistant
  content        TEXT NOT NULL,
  citations_json TEXT,
  created_at     INTEGER NOT NULL
);

CREATE TABLE study_guide (
  id                 TEXT PRIMARY KEY,
  notebook_id        TEXT NOT NULL REFERENCES notebook(id) ON DELETE CASCADE,
  prep_mode_snapshot TEXT NOT NULL,  -- JSON
  markdown           TEXT NOT NULL,
  created_at         INTEGER NOT NULL
);

CREATE TABLE flashcard_set (
  id                 TEXT PRIMARY KEY,
  notebook_id        TEXT NOT NULL REFERENCES notebook(id) ON DELETE CASCADE,
  prep_mode_snapshot TEXT NOT NULL,
  created_at         INTEGER NOT NULL
);

CREATE TABLE flashcard (
  id                TEXT PRIMARY KEY,
  set_id            TEXT NOT NULL REFERENCES flashcard_set(id) ON DELETE CASCADE,
  front             TEXT NOT NULL,
  back              TEXT NOT NULL,
  extras_json       TEXT,            -- MCQ options, follow-ups, etc.
  topic             TEXT,
  difficulty        TEXT,
  review_state_json TEXT             -- Leitner / SM-2 state
);

CREATE TABLE podcast (
  id                 TEXT PRIMARY KEY,
  notebook_id        TEXT NOT NULL REFERENCES notebook(id) ON DELETE CASCADE,
  prep_mode_snapshot TEXT NOT NULL,
  local_path         TEXT NOT NULL,
  duration_seconds   INTEGER,
  transcript         TEXT,
  created_at         INTEGER NOT NULL
);

CREATE TABLE settings (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
```

The database file lives at the OS-appropriate app-data path (`tauri::api::path::app_data_dir()`).

## 5. Ingestion pipeline (Rust)

```
React invokes: ingest_document(notebook_id, path)
      ↓
Tauri command spawns a Tokio task, returns job_id immediately
      ↓
Task:
  1. Copy PDF to app-managed folder; insert document row (status=parsing).
  2. pdfium-render → per-page text + bounding-box metadata. Fall back to lopdf on failure.
  3. Recursive splitter → 800-token chunks, 15% overlap, heading-aware.
  4. Insert chunk rows + FTS5 entries.
  5. Candle embedding pass (batch 32) → write to vec0 table.
  6. Update document status → ready. Emit `document.ready` event.
```

Benchmark target: a 300-page PDF in ~30 s parse + ~60 s embed on a modern laptop.

## 6. LLM provider abstraction

Rust trait in `src-tauri/src/llm/`:

```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn chat_stream(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<ToolSpec>>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatChunk>> + Send>>>;

    fn capabilities(&self) -> Capabilities;  // context window, tool-use quality, cost/token
}
```

Implementations:

- `AnthropicProvider` — Messages API, Sonnet 4.5 default.
- `OpenAIProvider` — Chat Completions + function calling.
- `GoogleProvider` — Gemini API.
- `OpenRouterProvider` — delegates to any underlying model.
- `OllamaProvider` — `localhost:11434`, auto-detects installed models.

`capabilities()` is what makes generation provider-aware: map-reduce engages when the chosen model's context window can't fit the document, and tool-use flows switch to a JSON-prompting fallback for providers with weak tool support (most local models).

Keys are fetched from the keychain lazily per session, never passed to the frontend.

## 7. Retrieval (Rust)

Hybrid search per query:

1. **Dense:** embed query with Candle (same model as chunks) → sqlite-vec cosine top-40.
2. **Sparse:** FTS5 BM25 on `chunk_fts` → top-40.
3. **Fusion:** Reciprocal Rank Fusion (k=60) → merged top-40.
4. **Rerank:** if the chosen provider exposes a rerank endpoint (Voyage, Cohere-via-OpenRouter) and the user has a key, rerank to top-5. Otherwise take top-5 from fusion.
5. **Format** each chunk: `<source id="{chunk.id}" doc="{filename}" page="{page}">{text}</source>`.

Context sent to the LLM: 8k tokens default, configurable up to the provider's limit.

## 8. Generation

### 8.1 Chat

Rust builds system prompt from (prep-mode snapshot, retrieved sources) and calls the provider's `chat_stream`. Deltas are emitted as `chat.{id}.delta` events. React renders via Vercel AI SDK streaming hooks.

Citation post-processing: after the stream completes, React parses `[id]` markers and renders them as clickable pills that open the PDF viewer at the cited page.

### 8.2 Study guide

Map-reduce when document size exceeds provider context:

- **Map:** per section (~5k tokens) → JSON (`{concept, definition, why_it_matters, related}`), using the provider's cheap tier (Haiku / gpt-4o-mini / `llama3.1:8b`).
- **Reduce:** section JSONs + prep-mode snapshot → final Markdown from the top tier.

### 8.3 Flashcards

Tool-use / structured output per section. Schema is prep-mode-specific (viva → `follow_ups[]`; MCQ → `options[]` + `correct` + `explanation`; etc.).

Providers with unreliable tool use (most small local models) fall back to strict-JSON prompting + post-parse validation + retry-on-malformed-JSON.

Dedup: embed `front` field, skip cards with cosine > 0.9 against existing cards in the same set.

### 8.4 Podcast

1. Script generation (user's chosen LLM) → dialog between two named hosts.
2. TTS — either online (ElevenLabs, user's key) or local (Piper, bundled binary).
3. Audio concatenation via `rodio`; saved as MP3 in the app-data folder.

## 9. Tauri command surface (selected)

```rust
#[tauri::command] async fn create_notebook(prep_mode: PrepMode) -> Result<Notebook>;
#[tauri::command] async fn list_notebooks() -> Result<Vec<Notebook>>;
#[tauri::command] async fn ingest_document(notebook_id: String, path: PathBuf) -> Result<JobId>;
#[tauri::command] async fn list_documents(notebook_id: String) -> Result<Vec<Document>>;

#[tauri::command] async fn send_chat_message(chat_id: String, text: String) -> Result<JobId>;
// response streams via event "chat.{job_id}.delta"

#[tauri::command] async fn generate_study_guide(notebook_id: String) -> Result<JobId>;
#[tauri::command] async fn generate_flashcards(notebook_id: String) -> Result<JobId>;
#[tauri::command] async fn generate_podcast(notebook_id: String) -> Result<JobId>;

#[tauri::command] async fn set_provider(provider: ProviderConfig) -> Result<()>;
#[tauri::command] async fn get_provider_status() -> Result<ProviderStatus>;
#[tauri::command] async fn store_api_key(provider: String, key: String) -> Result<()>;  // → keychain
```

All command argument and return types are exported to TypeScript via `specta` / `tauri-specta` so the React layer has matching types without hand-maintenance.

## 10. Distribution

- GitHub Actions CI, three runners (macos-latest, windows-latest, ubuntu-latest).
- Artifacts: notarized `.dmg`, signed `.msi`, `.AppImage`, `.deb`.
- Tauri Updater config points at `https://github.com/<user>/studylm/releases/latest/updater.json`.
- Code signing:
  - macOS: Apple Developer ID + notarization (notarytool). ~$99/year.
  - Windows: OV cert minimum, EV preferred. ~$250–500/year.
  - Linux: no signing; AppImage SHA-256 published alongside.

## 11. Key design decisions

- **No Python anywhere.** All heavy lifting in Rust. Avoids bundling a Python runtime; avoids the two-languages/two-error-styles trap.
- **sqlite-vec over LanceDB / DuckDB / Chroma.** One file; backup is copy-paste; notebooks map cleanly to rows.
- **Local embeddings always, even when the LLM is cloud.** Embeddings are cheap; sending chunks to a cloud embedding API adds latency and cost for no quality win when Candle does it in ~50 ms per chunk on-device.
- **Prompts as Rust string constants, templated with Tera.** File-system templates complicate bundling. One module per feature, prep-mode branches inside each.
- **No agentic framework (no LangGraph / LangChain).** MVP features are single-turn with structured output. Revisit when we add a revision-planner feature that genuinely needs multi-step loops.
- **Vercel AI SDK on the frontend for streaming UI only.** LLM calls originate in Rust. The SDK provides the render-as-tokens-arrive hooks.

## 12. Performance targets

- **Cold start to first paint:** < 400 ms.
- **PDF ingest (300 pages):** < 2 min end-to-end.
- **Chat first-token latency:** < 600 ms on cloud LLMs, < 2 s on Ollama local.
- **Memory footprint idle:** < 200 MB.
- **Installer size:** < 30 MB (Candle model downloaded on first run, not bundled).
