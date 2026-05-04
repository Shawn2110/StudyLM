# StudyLM

A **local-first, BYOK study companion** for students. Drop in your textbooks
and lecture PDFs, pick the LLM you want (Anthropic, OpenAI, Google,
OpenRouter, or fully-local Ollama), and get chat / study guides /
flashcards / podcasts that adapt to what you're actually preparing for —
viva vs. final, MCQ vs. long-answer, conceptual vs. problem-solving.

Built as a native desktop app on Tauri 2: ~10 MB binary, no Electron, no
servers. Documents and embeddings stay on your machine; only your prompts
+ retrieved chunks go to your chosen provider (or nothing leaves at all,
on the Ollama path).

## Status

| Phase | Scope | Status |
|---|---|---|
| Phase 0 | App skeleton, SQLite, keychain, notebook CRUD | ✅ done |
| Phase 1 | Provider setup (5 providers, key entry, ping) | ✅ done |
| Phase 2 | PDF ingestion → chunks → local embeddings → SQLite + FTS5 + sqlite-vec | ✅ done |
| Phase 3 | Chat with citations | ⏳ next |
| Phase 4 | Study guide + flashcards | — |
| Phase 5 | Podcast | — |
| Phase 6 | Packaging + signed installers | — |
| Phase 7 | Polish + private beta | — |

See [docs/plan.md](docs/plan.md) for the full roadmap.

## What's working today

- Create notebooks with a six-field prep mode (exam type, format, subject,
  duration, time remaining, difficulty focus).
- Drop a PDF into a notebook → it's parsed via `lopdf`, chunked at ~800
  tokens with 15% overlap, embedded locally with
  `nomic-embed-text-v1.5` via Candle, and written into SQLite with FTS5
  + sqlite-vec virtual tables for hybrid search.
- Configure any of the five LLM providers via Settings → Providers. Keys
  live in the OS keychain (Windows Credential Manager / macOS Keychain /
  Linux Secret Service). Live ping shows model counts before you commit.

## Stack

- **Shell:** Tauri 2 (Rust + WebView, no bundled Chromium)
- **Backend:** Rust 2021 — Tokio, sqlx, reqwest, tracing, thiserror,
  Candle, lopdf, sqlite-vec, keyring
- **Frontend:** React 18 + TypeScript (strict), Vite 5, TanStack Router
  (file-based), TanStack Query, TailwindCSS 3 + shadcn/ui
- **Storage:** SQLite with FTS5 + `sqlite-vec` (768-d float32 vectors)
- **Embeddings:** `nomic-embed-text-v1.5` via Candle, CPU-only, ~250 MB
  model downloaded on first ingest into the shared HuggingFace cache
- **IPC types:** `specta` + `tauri-specta` — Rust structs / enums become
  TypeScript at build time; never edit `src/types/bindings.ts` by hand

## Documentation

| File | Purpose |
|---|---|
| [docs/prd.md](docs/prd.md) | Product intent, target users, scope |
| [docs/architecture.md](docs/architecture.md) | Technical shape, schema, IPC surface |
| [docs/design.md](docs/design.md) | Visual + interaction design system (v2) |
| [docs/plan.md](docs/plan.md) | Phase-by-phase roadmap |
| [docs/agent.md](docs/agent.md) | Engineering manual for AI assistants + humans |
| [docs/TESTS.md](docs/TESTS.md) | How tests are organised |
| [docs/TESTS-EXPLAINED.md](docs/TESTS-EXPLAINED.md) | Why we test what we test |

## Local development

The Rust backend currently builds on **Linux** only. On Windows, develop
inside **WSL2 Ubuntu** (Smart App Control on Windows 11 blocks the
unsigned binaries cargo produces during build). Native Windows builds
will arrive in Phase 6 via signed CI.

### One-time setup (WSL2 Ubuntu 24.04 on Windows 11)

```bash
# Inside WSL Ubuntu
sudo apt-get update && sudo apt-get install -y \
  build-essential curl wget file pkg-config git ca-certificates \
  libwebkit2gtk-4.1-dev libxdo-dev libssl-dev \
  libayatana-appindicator3-dev librsvg2-dev libsqlite3-dev

# Rust (stable + msvc-equivalent linker for Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Node 22.x + pnpm
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo corepack enable
sudo corepack prepare pnpm@latest --activate

# Project deps
git clone https://github.com/Shawn2110/StudyLM.git ~/studylm
cd ~/studylm
pnpm install
```

### Run

```bash
cd ~/studylm
pnpm tauri dev
```

The app window appears on your Windows desktop via WSLg. The first PDF
you ingest will trigger the ~250 MB nomic model download; subsequent
ingests are fast.

### Verification scripts

```bash
pnpm typecheck                      # tsc --noEmit
pnpm lint                           # eslint
pnpm fmt                            # prettier --write
cd src-tauri && cargo clippy -- -D warnings
cd src-tauri && cargo test
```

## Project layout

```
studylm/
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── commands/   # #[tauri::command] handlers
│   │   ├── db/         # sqlx queries (notebooks, documents, chunks, settings)
│   │   ├── embeddings/ # Candle nomic-embed-text-v1.5
│   │   ├── ingestion/  # parse → chunk → embed pipeline
│   │   ├── llm/        # LlmProvider trait + 5 implementations
│   │   ├── keychain/   # OS keychain wrapper (keyring-rs)
│   │   ├── error.rs    # AppError enum (thiserror + Serialize + specta)
│   │   ├── lib.rs      # tauri::Builder, state, command registration
│   │   └── main.rs
│   ├── migrations/     # sqlx migrations (0001 init, 0002 documents)
│   ├── .sqlx/          # offline query cache (committed)
│   └── Cargo.toml
├── src/                # React frontend
│   ├── routes/         # TanStack Router file-based
│   ├── components/
│   │   ├── ui/         # shadcn primitives (Button, Dialog, Select, …)
│   │   ├── app/        # app shell (sidebar)
│   │   ├── notebook/   # notebook-specific components
│   │   ├── document/   # source card etc.
│   │   └── settings/   # provider row, providers panel
│   ├── lib/            # invoke wrappers, event listeners, format utils
│   ├── styles/         # tokens.css
│   └── types/          # GENERATED via specta — do not edit
├── docs/               # prd.md, architecture.md, design.md, plan.md, agent.md
└── README.md
```

## Privacy

- Documents stay on your machine, in
  `~/.local/share/com.studylm.app/notebooks/<id>/` (Linux) or the
  OS-equivalent app-data path.
- API keys live in the OS keychain. Never on disk in plaintext, never
  logged.
- On the **Ollama path**, nothing leaves your machine.
- On a cloud provider, only your prompt + retrieved chunks go to the
  provider you chose. StudyLM does not broker traffic, see your keys, or
  collect telemetry by default.

## Contributing

Coming soon. While the project is pre-MVP, the
[engineering manual in docs/agent.md](docs/agent.md) is the canonical
guide to coding conventions, commit style, and what to ask before
proposing changes.

## License

Not yet finalized. The plan is **MIT or Apache-2.0** — see
[docs/prd.md §9](docs/prd.md).
