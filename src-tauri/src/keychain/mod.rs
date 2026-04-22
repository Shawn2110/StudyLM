//! Thin async wrapper over `keyring-rs` that stores API keys in the native
//! OS keychain (macOS Keychain, Windows Credential Manager, Linux Secret
//! Service). Key values are never logged and never cross the Rust ↔ React
//! boundary.
