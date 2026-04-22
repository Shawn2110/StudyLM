//! Standalone binary that walks the same `commands_builder()` the runtime
//! uses and writes its full TypeScript surface (commands, events, structs,
//! enums, AppError) to `src/types/bindings.ts`. Wired into `pnpm dev` /
//! `pnpm build` via the `predev` / `prebuild` npm scripts so the generated
//! file is always fresh before the frontend reads it.

fn main() {
    let bindings_path = "../src/types/bindings.ts";
    studylm_lib::commands_builder()
        .export(specta_typescript::Typescript::default(), bindings_path)
        .expect("failed to write typescript bindings");
    println!("✓ wrote {bindings_path}");
}
