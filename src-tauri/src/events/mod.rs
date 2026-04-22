//! Typed event emitters for Rust → React messages. Long-running commands
//! return a `JobId` synchronously and then emit `job.{id}.progress`,
//! `job.{id}.done`, and streaming variants (`chat.{id}.delta`) via these
//! helpers. Frontends listen through `src/lib/events.ts`.
