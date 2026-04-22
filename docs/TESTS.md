# StudyLM — Phase 0 Test Plan

Every check that must pass before Phase 0 is considered done. Run from the
project root inside WSL Ubuntu unless a step says otherwise.

## Prerequisites

- WSL2 Ubuntu 24.04 with the apt deps from the setup notes
  (`libwebkit2gtk-4.1-dev`, `libdbus-1-dev`, `libsqlite3-dev`, etc.)
- Rust stable (`rustup default stable`)
- Node 22 + pnpm 10
- Project dependencies installed (`pnpm install --frozen-lockfile`)
- `src-tauri/.sqlx/` is committed (the sqlx offline cache); CI sets
  `SQLX_OFFLINE=true` so `query!`/`query_as!` compile without `DATABASE_URL`

## Static checks (no app launch)

### 1. Frontend typecheck

```bash
pnpm typecheck
```

Pre-script `pretypecheck` regenerates `src/types/bindings.ts` from Rust, then
`tsc --noEmit` runs. **Expected:** exit code 0, no TS errors printed.

### 2. Frontend lint

```bash
pnpm lint
```

Runs `eslint .` against the flat config in `eslint.config.js`. **Expected:**
exit 0.

### 3. Prettier check

```bash
pnpm fmt:check
```

**Expected:** exit 0. If it fails, run `pnpm fmt` to auto-format.

### 4. Rust clippy (strict)

```bash
cd src-tauri && SQLX_OFFLINE=true cargo clippy -- -D warnings
```

**Expected:** exit 0. `-D warnings` makes any clippy warning a hard error.

### 5. Rust unit tests

```bash
cd src-tauri && SQLX_OFFLINE=true cargo test
```

**Expected:** exit 0. No unit tests yet, so the expected line is
`test result: ok. 0 passed`.

### 6. Vitest stub

```bash
pnpm test
```

**Expected:** prints `no tests yet — add vitest in Phase 1` and exits 0. Real
vitest suites land in Phase 1 alongside provider code.

## Build checks

### 7. Frontend production build

```bash
pnpm build
```

`tsc && vite build`. **Expected:** exit 0; `dist/index.html`,
`dist/assets/index-*.js`, `dist/assets/index-*.css` all present.

### 8. Bindings generation

```bash
cd src-tauri && SQLX_OFFLINE=true cargo run --bin generate-bindings
```

**Expected:** prints `✓ wrote ../src/types/bindings.ts`. Re-running and then
`git diff src/types/bindings.ts` should show no changes — the file is
deterministic from the Rust command surface.

## Behavioural checks (manual, run in order)

### 9. App launches without errors

```bash
pnpm tauri dev
```

**Expected:** Rust compiles (~3 min cold; <30 s warm), Vite starts on port
1420, a window titled **StudyLM** appears (in WSL: routed through WSLg onto
the Windows desktop). The terminal shows `info ... sqlite ready` from the
`tracing` subscriber.

Leave this command running for steps 10–12.

### 10. Create a notebook from the dialog

In the open window:

1. Click **Create notebook**.
2. Fill at least `Exam type` and `Format` (defaults to `internal` / `mcq`),
   optionally `Subject`, `Duration`, `Exam date`, `Difficulty focus`.
3. Click **Create**.

**Expected:** the dialog closes immediately and the notebook appears in the
list with the chosen subject (or `<ExamType> · <Format>` if subject was
blank).

### 11. SQLite file exists at the OS app-data path

In a second terminal:

```bash
ls -la ~/.local/share/com.studylm.app/
```

**Expected:** `studylm.db` is present and non-empty. (On macOS the path is
`~/Library/Application Support/com.studylm.app/`; on Windows it's
`%APPDATA%/com.studylm.app/`.)

```bash
sqlite3 ~/.local/share/com.studylm.app/studylm.db 'SELECT id, title, exam_type, format FROM notebook;'
```

**Expected:** the row created in step 10 is returned.

### 12. Persistence across restart

1. Close the app window (or `Ctrl+C` the `pnpm tauri dev` terminal).
2. Re-run `pnpm tauri dev`.
3. Wait for the window to reopen.

**Expected:** the notebook from step 10 is still in the list.

## CI dry run (optional)

Pushing to a branch + opening a PR triggers `.github/workflows/ci.yml` on
`macos-latest`, `windows-latest`, and `ubuntu-latest`. Each runner runs the
same check set: `pnpm typecheck`, `pnpm test`, `cargo clippy -- -D warnings`,
`cargo test`. **Expected:** all three jobs green.
