//! Feature-level generation: chat (streaming, cited), study guide (map-reduce
//! over sections), flashcards (prep-mode-specific schema), podcast script.
//! Each entry point takes a `PrepMode` snapshot and routes to the matching
//! template under `prompts/`.
