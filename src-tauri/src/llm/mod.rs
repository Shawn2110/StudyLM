//! `LlmProvider` trait plus per-provider implementations (Anthropic, OpenAI,
//! Google, OpenRouter, Ollama). Every LLM call in StudyLM goes through this
//! trait—HTTP, streaming, retries, and capability introspection live here and
//! nowhere else.
