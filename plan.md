# Implementation Plan — StudyLM

**Status:** Draft v2 (desktop app)
**Target MVP:** 12 weeks solo · 6 weeks with a partner

Rust is more verbose than Python for equivalent logic, and desktop packaging adds real overhead (code signing, updater, multi-platform builds). Plan accordingly — this is a longer build than a web MVP.

## Phases

### Phase 0 — Skeleton (Week 1)

- Scaffold via `create-tauri-app`: Vite + React + TypeScript (strict).
- Add TailwindCSS + shadcn/ui.
- TanStack Router with two example routes (`/` and `/settings`).
- SQLite + sqlx with migrations; `keyring-rs` wrapper.
- First working `#[tauri::command]` round-trip from React to SQLite and back (e.g. `create_notebook`).
- `specta` + `tauri-specta` wired so Rust structs appear as TS types.

**Exit:** create a notebook in the UI, see it persisted in SQLite, reload the app, see it still there.

### Phase 1 — Provider setup (Week 2)

- Settings screen: pick provider (Anthropic / OpenAI / Google / OpenRouter / Ollama).
- API key entry, stored in keychain via Rust command.
- `LlmProvider` trait with minimal implementations (at least a `ping` method per provider).
- Ollama auto-detect: probe `localhost:11434`, list installed models.
- Error UX for invalid keys, missing models, unreachable Ollama.

**Exit:** user can switch between two providers, see key validation succeed, and the choice persists.

### Phase 2 — Ingestion (Weeks 3–4)

- pdfium-render integration; per-page text extraction.
- Recursive chunker: 800 tokens, 15% overlap, heading-aware.
- Candle with `nomic-embed-text-v1.5`; first-run model download with progress UI.
- FTS5 + sqlite-vec populated on ingest.
- Document list UI with status + error states.
- Graceful failure on corrupt PDFs; `lopdf` fallback.

**Exit:** drag a PDF onto the app, see it parsed + embedded, confirm vectors and FTS rows in the DB.

### Phase 3 — Chat with citations (Weeks 5–6)

- Prep-mode notebook creation wizard (all 6 fields).
- Hybrid retrieval: dense + FTS + RRF.
- System-prompt templates per prep mode.
- Streaming chat via Tauri events.
- Citation parsing in React; clickable pills.
- PDF viewer (`pdfjs-dist`) with page-jump on citation click.

**Exit:** ask a question, get a cited answer, click a citation, land on the correct page.

### Phase 4 — Study guide + flashcards (Weeks 7–8)

- Map-reduce guide generator (provider-capability-aware).
- Flashcard generator with prep-mode-specific schemas.
- JSON-fallback path for weak-tool-use providers.
- Flashcard review UI with Leitner-box progression.
- Regeneration flow when prep mode changes.

**Exit:** generate both outputs; verify they differ meaningfully across prep modes on the same source PDF.

### Phase 5 — Podcast (Week 9)

- Two-host script generation, prep-mode-aware.
- ElevenLabs integration (online path).
- Piper bundling + local path.
- Audio concatenation via `rodio`.
- Player UI with transcript + chapter markers.

**Exit:** generate a 5-minute podcast, play it in-app with transcript highlight.

### Phase 6 — Packaging & distribution (Weeks 10–11)

- GitHub Actions CI, three platforms.
- Apple Developer ID + macOS notarization in CI.
- Windows code-signing cert + signing in CI.
- Tauri Updater config + update-manifest generator.
- Release flow: tag → build artifacts → GitHub Release → updater picks up.
- First signed public beta.

**Exit:** a fresh machine downloads the installer, installs, opens, works. Auto-update tested with a second release.

### Phase 7 — Polish (Week 12)

- Onboarding: empty state, demo notebook, inline tooltips.
- Error surfaces: quota hit, offline, invalid key, model missing.
- Opt-in analytics via Plausible.
- Performance pass: cold-start time, memory baseline, ingest throughput.
- 20–30 private beta users.

**Exit:** someone unrelated installs, sets up a provider, and uses every feature without you on a call.

## Critical path

Ingestion → retrieval → chat. Everything else sits on top. Study guide and flashcards parallelize after ingestion. Podcast is a parallel track after prompts stabilize in Phase 3. Distribution (Phase 6) is a hard gate — don't underestimate it.

## Risks & mitigations

| Risk | Mitigation |
|---|---|
| Code signing stalls launch | Start Apple + Windows cert applications on Day 1 of Phase 0 (both take 1–4 weeks). |
| Candle is too slow on older hardware | Benchmark on a 4-core laptop in Phase 2. If it's unacceptable, fall back to Transformers.js in the WebView. |
| Tool-use quality varies across providers | Build JSON-fallback validation from day one. Test flashcard generation on Ollama `llama3.1:8b` as the worst case. |
| PDF edge cases (scans, complex layouts) | Ship a "parse quality: good / partial / poor" indicator; set expectations up front. |
| Users struggle with API-key setup | Offer a zero-key "Try it free" path via local Ollama + a small model. |
| Rust learning curve slows delivery | Keep the Rust surface minimal: commands, DB, LLM trait, ingestion. Push everything else into React. |
| Cross-platform bugs | Weekly smoke test on all three platforms starting Phase 2. |

## Cost estimates

**Dev:**
- ~$0 API spend if you develop against local Ollama.
- ~$50/month if you test cloud providers regularly.
- Apple Developer Program: ~$99/year.
- Windows OV cert: $250–500/year.

**Post-launch:** StudyLM itself is free to run (no servers). Users pay their LLM provider directly.

## What to skip in MVP

- Accounts, sync, collaboration.
- Mobile apps.
- Payments.
- Quiz mode.
- Cross-notebook search.
- Regional language support.

## Definition of done — per feature

1. Works on macOS, Windows, and Linux.
2. Degrades gracefully when LLM / Ollama / network is unavailable.
3. Covered by at least one Rust integration test + one Playwright UI test.
4. Instrumented with tracing spans.
5. Documented in `architecture.md` if the shape changed.
