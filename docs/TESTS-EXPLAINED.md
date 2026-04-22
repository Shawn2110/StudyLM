# Phase 0 Tests — Explained

Companion to [TESTS.md](TESTS.md). For each check there, this doc says **what
it really validates**, **why we picked it for Phase 0**, and **how to read a
failure** when it goes red. Read this when a test fails or before changing
the test plan, not as part of every test run.

## How the test plan is shaped

Phase 0's ambition is small on purpose: prove the skeleton is alive. There is
no ingestion, no LLM, no retrieval yet. So the tests fall into three thin
layers, and each test in TESTS.md belongs to exactly one:

- **Static** — type & lint checks. Fast, deterministic, run on every commit.
- **Build** — compile the app end-to-end. Catches dependency drift, broken
  generated files, and configuration mistakes.
- **Behavioural** — actually launch the app and exercise the
  create-notebook → persist → reopen path. The only thing in Phase 0 that
  proves the IPC, DB, and React layer are wired together correctly.

Anything below the first behavioural check has to pass before behaviour is
worth testing. CI runs the static + build layers automatically; the
behavioural layer is manual until we have Playwright in Phase 7.

## Per-test detail

### 1. `pnpm typecheck`

**Validates:** `tsc --noEmit` against the strict tsconfig, and (via
`pretypecheck`) that `cargo run --bin generate-bindings` still emits
well-formed TS for every `#[tauri::command]` and `specta::Type`.

**Why Phase 0 cares:** the React ↔ Rust boundary in StudyLM is enforced by
generated types. If the binding-gen step breaks or the Rust `Notebook` /
`PrepMode` change without the frontend updating, this test catches it before
the user ever clicks anything.

**Reading failures:**

- `Cannot find module '@/types/bindings'` — the generator didn't run; check
  that the `predev`/`prebuild`/`pretypecheck` script is wired and
  `cargo run --bin generate-bindings` works on its own.
- TS errors in `src/routes/...` — usually a renamed Rust field that the
  frontend still references; update the call site.

### 2. `pnpm lint`

**Validates:** ESLint flat config (`eslint.config.js`) over all `src/`
TypeScript. We use the typescript-eslint recommended preset only — no React
plugins yet, since plugin-react-hooks v7's flat config story was unstable
when we wired this up.

**Why Phase 0 cares:** prevents the obvious foot-guns (unused vars, `any`
without justification) before they accumulate.

**Reading failures:** usually self-explanatory. If a generated file is being
linted, add it to `eslint.config.js`'s `ignores`.

### 3. `pnpm fmt:check`

**Validates:** Prettier formatting matches `.prettierrc`. Generated/
machine-owned files are excluded via `.prettierignore`.

**Why Phase 0 cares:** keeps diffs small. CI fails if any file deviates from
the canonical format, which prevents whitespace-only churn in PRs.

**Reading failures:** run `pnpm fmt` to auto-format.

### 4. `cargo clippy -- -D warnings`

**Validates:** clippy's default lints, with warnings promoted to hard errors.
This is the **strictest** check we run — it catches `.unwrap()` (against
agent.md), unused imports, and a long tail of patterns the Rust community
considers smells.

**Why Phase 0 cares:** the Rust surface is small enough now that we can
afford zero clippy warnings. Letting one slip in becomes "well, what's one
more" by Phase 4.

**Reading failures:**

- `clippy::needless_pass_by_value` and similar — usually a one-line fix
  (`&T` instead of `T`).
- Anything in `src-tauri/src/db/notebooks.rs` after editing SQL — re-run
  `cargo sqlx prepare` (see `.env.example`) and commit the updated
  `.sqlx/*.json`.

### 5. `cargo test`

**Validates:** every `#[test]` and `#[tokio::test]` in the Rust crate. Empty
in Phase 0; placeholder so CI has a hook for Phase 1+.

**Reading failures:** when this starts having real tests, integration tests
that touch SQLite must use `sqlite::memory:` per agent.md.

### 6. `pnpm test`

**Validates:** stub. Prints "no tests yet" so CI doesn't fail. Will become
`vitest run` in Phase 1.

### 7. `pnpm build`

**Validates:** the full frontend production pipeline (`tsc` then `vite
build`). Different from `pnpm dev` — dev uses esbuild + HMR, build uses
Rollup with full tree-shaking, so it surfaces things dev doesn't (circular
imports, dynamic-import edge cases).

**Why Phase 0 cares:** if `pnpm build` fails, `pnpm tauri build` will fail
later, and we'd rather find that out now.

**Reading failures:** if Rollup complains about a CSS import order, check
`src/index.css` is imported from `src/main.tsx` (not from a route file).

### 8. `cargo run --bin generate-bindings`

**Validates:** the dedicated bin compiles, the same `commands_builder()`
that Tauri runs at startup hands a Builder to `tauri-specta`, and the export
to `../src/types/bindings.ts` succeeds.

**Why Phase 0 cares:** this is the contract between Rust and TypeScript. The
moment it diverges from `commands_builder()` in `lib.rs`, the frontend's
type safety is a lie. Re-running and getting a clean diff means the file is
deterministic and committed correctly.

**Reading failures:**

- `error: failed to determine which binary to run` — `default-run` missing
  from `Cargo.toml`'s `[package]` section. Should be `default-run = "studylm"`.
- TypeScript syntax errors in the output — usually a specta version mismatch.
  Pin specta + tauri-specta to the same `=2.0.0-rc.X` line.

### 9. `pnpm tauri dev` opens a window

**Validates:** the integration of every layer. The Rust binary boots, the
`setup` closure resolves `app_data_dir`, opens the SQLite pool, runs
`0001_init.sql`, mounts tauri-specta events, and the WebView loads the Vite
dev server.

**What success looks like:** terminal prints
`info ... sqlite ready path=/home/shawn/.local/share/com.studylm.app/studylm.db`
and a window titled "StudyLM" appears. In WSL the window is rendered by GTK
and routed onto the Windows desktop via WSLg — it will look slightly
different from the production Windows WebView2 build, but the React tree and
IPC are identical.

**Reading failures:**

- Window doesn't appear, no errors in terminal: WSLg may not be running.
  Run `wsl --shutdown` from PowerShell, then `wsl` again.
- `error: failed to bind to port 1420`: kill any leftover `vite` from a
  previous run.
- `migrate error: ...`: delete the dev DB at
  `~/.local/share/com.studylm.app/studylm.db` and retry.

### 10. Create a notebook from the dialog

**Validates:** the full create path — React form → zod validation →
TanStack Query mutation → `commands.createNotebook` IPC → Rust handler →
`db::notebooks::create` → `sqlx::query!` INSERT → `query_as!` SELECT in the
list refresh → React re-renders.

If this works, almost every Phase 0 wire is exercised in a single click.

**Reading failures:**

- Dialog stays open after submit, error text in red: the `mutation.error`
  is shown verbatim. If it says `Db("UNIQUE constraint failed")`, you double-
  clicked Create. If it says `InvalidInput`, the form didn't validate (zod
  schema mismatch).
- New notebook doesn't appear: TanStack Query didn't invalidate the list.
  Check that `queryKey: ["notebooks"]` is the same in `index.tsx` and the
  `onSuccess` of the mutation.

### 11. SQLite file exists at the OS app-data path

**Validates:** the path resolution works (`Manager::path().app_data_dir()`
returns the right OS-specific location and we joined `studylm.db` correctly),
and that sqlx actually wrote rows there.

**Why Phase 0 cares:** this is the proof that "documents stay on the user's
machine" from the PRD is more than aspirational — the file is
inspectable, backupable, and lives where the user would expect.

### 12. Persistence across restart

**Validates:** `0001_init.sql` is committed (not a temp memory DB),
`init_pool` is reusing the same file path on restart, and migrations are
idempotent (`sqlx::migrate!` skips already-applied migrations).

**Why Phase 0 cares:** a notebook that disappears after restart means we're
silently using `:memory:` somewhere, or we're writing to a temp directory.
Either is a launch-blocker.

## When a test ends up wrong

Tests are not load-bearing forever. If TESTS.md ever asks for something
that's no longer the right shape (e.g. we move from `app_data_dir()` to a
user-configurable path in Phase 7), update both files together — don't
let one drift from the other.
