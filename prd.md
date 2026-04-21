# Product Requirements Document — StudyLM

**Status:** Draft v2 (desktop app)
**Last updated:** 2026-04-21

## 1. Summary

StudyLM is a **desktop study companion** that turns a student's own source material (textbooks, notes, lecture PDFs, papers) into a personalized revision tool. It adapts every generated output to **what the student is preparing for** — an internal test, viva, semester final, competitive entrance, or practical exam.

Three product pillars:

1. **Local-first.** Documents never leave the student's machine unless they choose an online LLM. Parsing, chunking, embedding, and search all run on-device.
2. **Bring your own LLM.** Users pick their provider — Ollama (fully local), Anthropic, OpenAI, Google, OpenRouter — and supply their own key. Keys are stored in the OS keychain, never in plaintext.
3. **Prep-mode aware.** Same source material, different flashcards / guides / podcasts depending on whether the student is preparing for a viva vs a final vs a competitive exam.

## 2. Problem

Indian students revising for exams today face three gaps:

- **Generic summarizers don't match exam format.** Viva prep needs probing questions. MCQ-heavy internals need distractors. Long-answer finals need structured essay outlines.
- **Cloud tools raise cost and privacy concerns.** Students upload copyrighted textbooks and personal notes to third-party servers. A local-first app sidesteps both.
- **"Bring your own key" is rarely first-class.** Most tools lock users into one provider. A student already paying for ChatGPT Plus or using free-tier Gemini shouldn't pay twice.

StudyLM closes all three gaps by running as a native desktop app with BYOK and prep-mode intelligence.

## 3. Target users

**Primary:** Indian undergraduate & postgraduate students (engineering, medical, commerce, humanities).
**Secondary:** Competitive exam aspirants (GATE, CAT, NEET, UPSC, CET).
**Tertiary:** Class 10 / 12 boards students, self-learners, MOOC students.

Technical profile: comfortable installing desktop apps, willing to paste an API key once. Not expected to use a terminal or run servers.

## 4. Preparation mode — the core differentiator

When a user creates a notebook (a study set), they specify:

| Field | Example values |
|---|---|
| Exam type | Internal · Mid-sem · End-sem · Viva · Practical · Assignment · Competitive · Custom |
| Format | MCQ · Short answer · Long answer · Oral · Numerical · Mixed |
| Subject | e.g. "Thermodynamics", "Constitutional Law" (free text) |
| Duration | Minutes the exam itself will take |
| Time remaining | Hours or days until the exam |
| Difficulty focus | Conceptual · Problem-solving · Memorization · Mixed |

The system uses this metadata to **rewrite prompts for every feature**. Same document, different output.

### How prep mode reshapes each feature

**Chat**
- *Viva:* examiner tone, probes for understanding before explaining.
- *MCQ:* concise answers, volunteers common distractors.
- *Long answer:* structured paragraphs with intro/body/conclusion.

**Study guide**
- *Viva:* potential questions + talking points + follow-ups.
- *Internal / MCQ:* definitions, formulas, quick-reference tables.
- *Final:* topic-wise notes with examples and inter-topic links.
- *Competitive:* problem patterns, shortcut techniques, common traps.

**Flashcards**
- *Viva:* Q&A with explanatory backs + follow-up prompts.
- *MCQ:* 4-option cards with distractor rationale.
- *Numerical:* problem on front, worked solution on back.
- *Memorization:* cloze-deletion cards.

**Podcast**
- *Short form (5–10 min)* for T-minus-hours crisis revision.
- *Long form (20–40 min)* for earlier revision with narrative + examples.
- *Viva-specific:* simulated examiner-student dialogue with probe questions.

## 5. Privacy & data model

- **Documents are stored on-device only**, in an app-managed folder (`~/Documents/StudyLM/` or OS-equivalent).
- **API keys live in the OS keychain** (macOS Keychain, Windows Credential Manager, Linux Secret Service). Never on disk in plaintext, never logged.
- **Local LLM path** (Ollama): nothing leaves the machine. Full offline mode.
- **Cloud LLM path:** only the user's query + retrieved chunks go to their chosen provider. StudyLM never sees or brokers traffic.
- **No telemetry by default.** Opt-in analytics (Plausible) can be enabled in settings.

## 6. User stories

Priority tiers: P0 = MVP, P1 = fast follow, P2 = later.

### P0 — must have for launch

- As a student, I can install StudyLM on macOS, Windows, or Linux from a signed installer.
- As a student, I can pick my LLM provider on first launch and paste an API key (or pick local Ollama).
- As a student, I can upload one or more PDFs into a notebook.
- As a student, I can specify my prep mode when creating a notebook.
- As a student, I can chat with my notebook and get answers cited to specific pages.
- As a student, I can generate a study guide and flashcards, both adapted to my prep mode.
- As a student, I can click a citation to jump to the source page.
- As a student, I can switch LLM providers later without losing my data.
- As a student, I can export a notebook (zip of PDFs + generated content) to back up or share.

### P1 — fast follow

- Podcast-style audio overview.
- Regenerate outputs after changing prep mode.
- Import from URL or YouTube transcript.
- Spaced-repetition flashcard review (Leitner, later SM-2 / FSRS).
- Auto-updater.

### P2 — later

- Practice quiz mode with timer + scoring.
- Day-by-day revision schedule tied to exam date.
- Regional language output (Hindi, Marathi).
- Cross-notebook search.
- LAN-sync mobile companion.

## 7. Success metrics

- **Activation:** % of installs where user creates a notebook + generates ≥1 output in first session. Target: 50%.
- **Week-2 retention** of activated users. Target: 35%.
- **Feature breadth:** % of notebooks using 2+ output types. Target: 60%.
- **Quality:** thumbs-up rate on generated outputs. Target: 70%.
- **BYOK mix:** rough 60/40 split between cloud-provider users and local-Ollama users — indicates healthy reach on both ends.

## 8. Out of scope for MVP

- Accounts, authentication, cloud sync.
- Real-time collaboration.
- Native mobile apps.
- LMS integrations (Moodle, Google Classroom).
- Payment / subscription (app is free; user pays their LLM provider directly).
- Voice cloning / custom voices.
- Content moderation (the user is the only one who sees their notebooks).

## 9. Distribution

- **Installers:** `.dmg` (macOS), `.msi` (Windows), `.AppImage` + `.deb` (Linux).
- **Code-signed:** Apple Developer ID + notarization for macOS; OV cert minimum (EV preferred) for Windows.
- **Auto-update** via Tauri Updater, from GitHub Releases.
- **Open source** under MIT or Apache 2.0. Free during MVP. A paid "Pro" tier (hosted sync, premium templates) is an explicit later decision, not an MVP concern.

## 10. Open questions

- Bundle the local embedding model (+80 MB installer) or download on first run?
- Hand-hold Ollama installation inside StudyLM, or just link to ollama.com?
- Wrong-API-key UX: retry loop, or fall back to read-only?
- Regional language: translate outputs only, or also support Hindi/Marathi source documents?
