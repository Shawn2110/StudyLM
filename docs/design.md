# Design — StudyLM

**Status:** Draft v1
**Last updated:** 2026-04-21

This document defines StudyLM's visual and interaction design. It is the source of truth for component styling, typography, color, motion, and layout. When architecture, behavior, or accessibility decisions touch visual design, they belong here too.

## 1. Philosophy

StudyLM is a serious tool for a serious moment — a student preparing for an exam that matters. The design should feel like a **well-made reading room**, not a productivity app. Warm, quiet, focused; substantive but never stuffy; modern but rooted in the typographic traditions of books and journals rather than SaaS dashboards.

The tone is scholarly-editorial. Pages have the weight of content, not the airiness of a landing page. Rules, margins, and typographic hierarchy do the work that drop-shadows and gradients do in generic AI products.

Three non-negotiables:

1. **Text is the interface.** A student will spend 80% of their time reading and reading from a PDF. Typography, contrast, and line length are load-bearing. If a choice makes long reading worse, it loses — even if it looks good in a screenshot.
2. **Quiet chrome, loud content.** The app frame fades; the notebook content dominates. Buttons, nav, and toolbars are restrained enough that generated output stands out on the page like ink on paper.
3. **Desktop-native.** Keyboard shortcuts, command palette, window chrome, native menus, drag-to-resize panes. This app should never feel like a website stuffed in a WebView.

## 2. What we take from NotebookLM — and what we don't

**Borrow:**

- The three-pane workspace metaphor — sources on the left, workspace in the middle, generated output on the right.
- The "notebook as the primary container" mental model.
- Citations as first-class UI, clickable back to source pages.
- The "generate" actions (guide, flashcards, audio) as a clearly grouped affordance.

**Deliberately don't borrow:**

- Material Design language. Google's Material aesthetic (rounded rectangles, blue-500 accents, Roboto, card-with-shadow everywhere) is generic and overused.
- Chip-shaped buttons for everything. NotebookLM leans hard on pill-shaped buttons; we use rectangular buttons with a light border and reserve pills for semantic meaning (status, prep mode, citations).
- The "corporate neutral" voice in copy. Our empty states, errors, and onboarding have a human voice.
- Google-account gravity. We are local-first, BYOK. The UI should never ask the user to sign in to anything.
- Overused illustration styles (flat 3D isometric characters, duotone blobs).

## 3. Design tokens

All tokens exposed as CSS custom properties in `src/styles/tokens.css`. Tailwind's theme reads from these. Dark mode is not an afterthought — every token has a paired dark value, tested at first launch.

### 3.1 Color

Warm paper in light mode; deep warm ink in dark. The accent is a single confident ochre — scholarly rather than corporate. Semantic colors (success, warning, danger) are desaturated to sit alongside the warm neutrals without screaming.

```css
:root {
  /* neutrals — warm, paper-like */
  --paper-50:  oklch(98.5% 0.006 80);   /* page background */
  --paper-100: oklch(96.0% 0.009 80);   /* panel surface */
  --paper-200: oklch(92.0% 0.012 80);   /* raised surface, input bg */
  --paper-300: oklch(86.0% 0.015 80);   /* hairline */
  --paper-400: oklch(72.0% 0.015 80);   /* muted chrome */
  --paper-500: oklch(58.0% 0.013 80);   /* secondary text */
  --paper-700: oklch(36.0% 0.015 80);   /* body text */
  --paper-900: oklch(20.0% 0.018 80);   /* display text, strong headings */

  /* accent — scholarly ochre */
  --ink-400: oklch(70.0% 0.115 65);
  --ink-500: oklch(62.0% 0.125 60);     /* primary accent */
  --ink-600: oklch(54.0% 0.135 55);     /* hover, active */
  --ink-700: oklch(46.0% 0.130 52);     /* pressed */

  /* semantic — desaturated to live among the neutrals */
  --success-500: oklch(60.0% 0.090 160);
  --warning-500: oklch(72.0% 0.130 80);
  --danger-500:  oklch(58.0% 0.150 28);

  /* focus ring — always visible, never ugly */
  --focus: oklch(62.0% 0.160 60 / 0.55);
}

:root[data-theme='dark'] {
  --paper-50:  oklch(16.0% 0.010 75);
  --paper-100: oklch(19.5% 0.011 75);
  --paper-200: oklch(23.5% 0.013 75);
  --paper-300: oklch(30.0% 0.015 75);
  --paper-400: oklch(45.0% 0.015 75);
  --paper-500: oklch(62.0% 0.012 75);
  --paper-700: oklch(82.0% 0.010 75);
  --paper-900: oklch(95.0% 0.008 75);

  --ink-400: oklch(68.0% 0.120 68);
  --ink-500: oklch(74.0% 0.125 65);
  --ink-600: oklch(80.0% 0.130 62);
  --ink-700: oklch(86.0% 0.125 60);

  --success-500: oklch(70.0% 0.100 160);
  --warning-500: oklch(78.0% 0.130 80);
  --danger-500:  oklch(68.0% 0.155 28);
}
```

**Usage rules:**

- `--paper-50` is the only page background. `--paper-100` is for the three workspace panes. `--paper-200` is for input and flashcard surfaces.
- `--ink-500` is the only default accent — links, primary buttons, focus states, the active citation pill, the selected notebook in the sidebar.
- `--ink-600` is hover; `--ink-700` is active/pressed. Do not introduce new accent shades.
- Body text uses `--paper-700` on `--paper-50` — this deliberately lands ~11:1 contrast, above AAA.
- `--paper-500` is for secondary text (metadata, timestamps, byline-style labels). Never use it for anything a user must read carefully.
- Status colors are **only** for status. Never as decoration.

### 3.2 Typography

Three families, each doing exactly one job. Variable fonts so we ship one file per family.

```css
:root {
  --font-display: 'Fraunces', 'Iowan Old Style', 'Palatino', serif;
  --font-ui:      'Geist', 'Inter', system-ui, sans-serif;
  --font-mono:    'Geist Mono', 'JetBrains Mono', ui-monospace, monospace;
}
```

- **Fraunces** — variable serif with optical sizes and a soft axis. Used for notebook titles, study-guide headings, the empty-state poetry, any time text *is* content rather than chrome. Opsz 144 at display sizes, 11 at small sizes.
- **Geist** — for every UI element: buttons, menu items, input labels, body copy in chat bubbles. Tight tracking at large sizes, loose at small.
- **Geist Mono** — for page numbers, model names, keyboard shortcuts, file paths, code blocks. Also used in prep-mode metadata to give it an index-card feel.

**Type scale** — modular, 1.2 ratio. All sizes in rem, line-height scales with size:

| Role | Size | Line-height | Tracking | Family |
|---|---|---|---|---|
| Display — notebook title | 2.25 rem | 1.1 | -0.02em | Fraunces 500, opsz 120 |
| H1 — guide heading | 1.75 rem | 1.2 | -0.015em | Fraunces 500, opsz 80 |
| H2 — guide subheading | 1.375 rem | 1.3 | -0.01em | Fraunces 500, opsz 40 |
| H3 — section label | 1.0 rem | 1.4 | 0 | Geist 600 |
| Body — content | 1.0 rem | 1.65 | 0 | Geist 400 |
| UI — buttons, menu | 0.875 rem | 1.4 | 0 | Geist 500 |
| Metadata | 0.75 rem | 1.4 | 0.02em | Geist Mono 400 |
| Eyebrow — SMALL CAPS | 0.6875 rem | 1.3 | 0.12em | Geist 600, uppercase |

**Rules:**

- Chat messages, flashcard fronts/backs, and study-guide body all use the Body style. Line length is capped at 68ch — enforced by container, not by fiat.
- Eyebrows (SMALL CAPS) mark panel titles ("SOURCES", "CHAT", "STUDY GUIDE") and prep-mode labels. Never more than one eyebrow per visual group.
- Numbers in metadata (`p. 47`, `03:14`, `gpt-4o`) always use Geist Mono, even when sitting in a sentence.
- Italics: Fraunces italics are beautiful — use them for quoted text in study guides and flashcard contexts. Geist italics are utilitarian — use only for emphasis.
- Never mid-sentence bolding. Bold is for labels and headings.

### 3.3 Spacing

8px base. Tailwind's default scale is fine; a few custom additions for the reading-heavy layouts:

```css
--space-reading-gutter: 2rem;   /* margin around body text in study guide */
--space-panel-gutter:   1.5rem; /* inside each of the three panes */
--space-card-gutter:    1rem;   /* inside source cards, flashcards */
```

### 3.4 Borders & radii

We use hairlines, not boxes. Borders are always `1px` (`0.5px` physically on HiDPI via the browser) in `--paper-300`. Radii are restrained:

- `0` — rules, dividers, table cells.
- `4px` — inputs, buttons, flashcards.
- `8px` — panels, dialogs, popovers.
- `9999px` — pills only (citations, prep-mode badge, status indicators).

No drop-shadows on surfaces. Depth is conveyed by background color (`--paper-100` over `--paper-50`) and hairline borders. The one exception: the command palette gets a soft shadow (`0 12px 32px -12px rgb(0 0 0 / 0.18)`) because it floats over the entire app.

### 3.5 Elevation

Four levels only:

1. **Page** — `--paper-50`, no border.
2. **Pane** — `--paper-100`, no border, separated from adjacent panes by a vertical rule in `--paper-300`.
3. **Card / input** — `--paper-200`, 1px border `--paper-300`, 4px radius.
4. **Floating** — command palette, dropdown, toast. `--paper-100`, 1px border, 8px radius, soft shadow.

## 4. Layout

### 4.1 The workspace

The core screen is a three-pane workspace with draggable dividers. All three panes scroll independently.

```
┌──────────────────────────────────────────────────────────────────────────┐
│  Title bar (macOS: traffic lights + breadcrumb · Win/Linux: custom chrome)│
├──────────┬──────────────────────────────────┬────────────────────────────┤
│          │                                  │                            │
│ SOURCES  │  WORKSPACE                       │  CHAT / OUTPUT             │
│          │                                  │                            │
│  [card]  │  (PDF viewer, study guide,       │  [message]                 │
│  [card]  │   or flashcard review —          │  [message]                 │
│  [card]  │   whichever is active)           │  [message]                 │
│          │                                  │                            │
│          │                                  │  ───────────────           │
│          │                                  │  [  input            ]    │
│ + add    │                                  │  [  send  |  mode ▾ ]    │
└──────────┴──────────────────────────────────┴────────────────────────────┘
```

- **Left pane (Sources):** 240–320 px, draggable. Source cards stacked vertically. Empty state at bottom says "Drop a PDF here" and accepts native drag-and-drop from Finder/Explorer.
- **Middle pane (Workspace):** flex-1. A tab bar at top of the pane switches between PDF, Study Guide, Flashcards, Podcast. Each tab is an independent scroll container.
- **Right pane (Chat):** 360–560 px, draggable. Stays visible across all workspace tabs so the student can ask about whatever they're looking at.

Collapsed states: left pane collapses to a 48 px strip showing document favicons / initials; right pane collapses to a 48 px strip with a single "Chat" toggle that pops the pane back out.

### 4.2 Non-workspace screens

- **Notebook list** (home) — centered column, 720 px max width, ample top margin. Each notebook a horizontal card with prep-mode metadata as an eyebrow line above the title.
- **Settings** — two-column: nav rail on left (System, Providers, Appearance, Shortcuts, About), content on right.
- **Prep-mode wizard** — centered modal, 560 px wide. One field per visual row. Submit is `⌘+Return`.
- **First-launch provider setup** — full-window, no chrome. Single-column, centered, 480 px, big Fraunces heading, short copy, provider tiles.

### 4.3 Grid & rhythm

All layouts snap to an 8 px grid. Reading content (chat messages, study guide, flashcards) sits on a vertical baseline rhythm of 24 px — any paragraph, heading, or divider adds a multiple of 24.

## 5. Motion

Motion is subtle, fast, and purposeful. Desktop apps that animate too much feel laggy.

**Timings:** `100 ms` instant, `160 ms` snappy (default for most transitions), `240 ms` deliberate (dialog open, pane resize).
**Easing:** `cubic-bezier(0.22, 1, 0.36, 1)` for entrances (quick start, gentle settle), `cubic-bezier(0.55, 0, 1, 0.45)` for exits (gentle start, quick finish). Never bounce.

**Where motion lives:**

- **Route changes** — 4 px y-translate + opacity fade, 160 ms.
- **Dialog / popover open** — 8 px y-translate + opacity + subtle scale (0.98 → 1), 160 ms.
- **Toast** — slide up from bottom, 240 ms in, 160 ms out.
- **Streaming chat** — tokens render as they arrive; a thin vertical caret (`--ink-500`, 1 px × 1.2em) blinks at 1 Hz at the cursor until the stream ends.
- **Source card parse state** — a slow horizontal sweep of `--ink-500` at 20% opacity across the card while status is `parsing` or `embedding`. No spinners.
- **Flashcard flip** — 240 ms, 3D rotateY with `perspective: 800px`. One of the few places a satisfying motion is warranted.
- **Citation click** — middle pane scrolls to page; briefly highlight the cited region with a `--warning-500` underline that fades over 1200 ms.

**Accessibility:** `prefers-reduced-motion: reduce` disables the flashcard flip, scroll-to-page animation, and route fade. Essential feedback (streaming caret, parse sweep) is replaced with a static indicator.

## 6. Components

### 6.1 Buttons

Three variants, rectangular (not pill), 4 px radius, 8 px + 14 px padding, 32 px tall. UI font, weight 500.

- **Primary** — `--ink-500` background, `--paper-50` text. Hover `--ink-600`, active `--ink-700`.
- **Secondary** — `--paper-200` background, `--paper-900` text, 1 px border `--paper-300`. Hover `--paper-300`.
- **Ghost** — transparent, `--paper-700` text. Hover `--paper-100` background.

Icon-only buttons are 32 × 32 px squares with an 18 px Lucide icon. Destructive actions never use primary — use secondary with `--danger-500` text, and require confirmation for anything that destroys data.

### 6.2 Inputs

32 px tall (matches buttons), 1 px border `--paper-300`, 4 px radius, `--paper-200` background. Focus: 2 px ring in `--focus`, border becomes `--ink-500`. No "floating label" patterns — use a static label sitting 4 px above the input.

Textarea in chat: auto-growing, min 2 rows, max 8 rows, `--paper-200` background.

### 6.3 Source card

The most-looked-at card in the app. Needs to convey: filename, parse status, page count, document color stripe, a subtle drag handle.

```
┌────────────────────────────────────────┐
│ ▎ Thermodynamics — Chapter 4.pdf       │
│ ▎ 47 pp · parsed · ⌘1 to open          │
└────────────────────────────────────────┘
```

- 1 px border `--paper-300`, 4 px radius, `--paper-100` background.
- Left-side color stripe 3 px wide in a document-assigned color (hash of filename → one of six warm tones).
- Title: Body, weight 500, `--paper-900`, single line with ellipsis.
- Meta row: Geist Mono 12px, `--paper-500`.
- Selected state: 1 px border `--ink-500`, background `--paper-200`, stripe 4 px wide.
- Hover: background `--paper-200`.
- Parse status: `pending`, `parsing`, `embedding`, `ready`, `failed` — shown in the meta row, with the parse sweep animation while non-terminal.

### 6.4 Message bubble (chat)

Messages are not bubbles. They are editorial blocks, separated by whitespace, with an eyebrow indicating role.

```
YOU · 19:04
What's the difference between Gibbs and Helmholtz free energy?

CLAUDE · 19:04 · sonnet-4-5
Gibbs free energy (G) measures... [1]
```

- Role eyebrow: SMALL CAPS, Geist 600, 11px, tracking 0.12em, `--paper-500`. Assistant role includes provider + model badge.
- Body: Body type, `--paper-700` on `--paper-50` for user messages, `--paper-900` for assistant.
- No left-right alignment split (ChatGPT/iMessage pattern). Both roles left-align. The eyebrow does the work of distinguishing speakers.

### 6.5 Dialog / popover

Dialogs are centered modal rectangles. 8 px radius, 1 px border, backdrop 40% `--paper-900` with 8 px blur. Close on Escape, click-outside, and ⌘W. Title in Fraunces 500 at H2 size; body in UI font. Action row at bottom, right-aligned, primary action rightmost.

Popovers use the same surface treatment, no backdrop.

### 6.6 Command palette

Opens on `⌘K`. 640 px wide, centered at 20% from viewport top. Single input, flat list of results grouped by section (Notebooks, Actions, Settings, Help). Keyboard-driven; arrow keys navigate, Return executes, Escape dismisses. Lucide icons at 14 px, `--paper-500`.

### 6.7 Empty states

Every empty state gets a one-line Fraunces heading, a single sentence of human-voiced copy, and one primary action. No illustrations. If we need a visual, use a 32 px Lucide icon in `--paper-400`.

Sample copy:

- Notebook list: **"Nothing here yet."** — *Create a notebook and drop in your study material to get started.*
- Sources pane: **"No sources in this notebook."** — *Drag a PDF here, or click below to browse.*
- Chat, before first message: **"Ask anything about this notebook."** — *Answers cite the exact page they come from.*

## 7. Signature elements

These are the small, specific pieces of UI that make StudyLM visually memorable. Every one of them is intentional — do not generalize or soften them in the name of consistency.

### 7.1 Prep-mode badge

Every generated artifact — chat transcript, study guide, flashcard set, podcast — carries a prep-mode badge at its top-left corner. It's the visual promise that the output is tailored.

```
┌───────────────────────────┐
│ VIVA · CONCEPTUAL · 2H    │
└───────────────────────────┘
```

- Pill shape, 1 px border `--ink-500`, transparent background, 2px × 8px padding.
- Text: Geist Mono, 11 px, tracking 0.08em, `--ink-600`, uppercase.
- Composed from the prep-mode enum values separated by `·`. Three elements max; longer summaries tooltip on hover.

### 7.2 Citation pill

Inline in chat answers and study guide body. Clicking opens the PDF viewer at the cited page, with the region briefly highlighted.

```
...this is the second law [p. 47 · ch4.pdf]
```

- `[` and `]` are rendered as thin 0.5 px rules, not characters.
- Interior uses Geist Mono 12 px, `--ink-600` on `--paper-200`, 2 px radius, 1 px × 4 px padding.
- Hover: background becomes `--ink-500` with `--paper-50` text.
- Numbered if the same source is cited twice in a single answer: `[1]`, `[2]` with a footer listing sources. Otherwise inline.

### 7.3 Flashcard

The star of the flashcard review screen. Feels like a real card. 560 × 360 px, centered.

- `--paper-200` background, 1 px border `--paper-300`, 4 px radius.
- Front: Fraunces H1 centered vertically, generous margins. Top-left: prep-mode badge. Top-right: topic label (eyebrow style) and difficulty dot (one of three sizes: 4/6/8 px, `--paper-500`).
- Back: same canvas, the answer in Body type, left-aligned, up to 6 lines. Follow-up prompts (viva mode) appear as a small indented list below the main answer.
- Flip on `Space` (or click). 240 ms 3D flip.
- Rating buttons below the card: `Skip`, `Review again`, `Got it`. Keyboard: `1`, `2`, `3`.

### 7.4 Provider badge

Shown next to the assistant role eyebrow in chat, and in the app title bar.

```
CLAUDE · sonnet-4-5
```

- Provider name in Geist 500, uppercase, tracking 0.1em.
- Model name in Geist Mono 11 px, `--paper-500`.
- Clicking opens provider settings.

### 7.5 Prep-mode wizard

The create-notebook flow. Not a series of screens; one modal, one tall form, all six fields visible, with thoughtful defaults.

- Each field labeled with an eyebrow (SMALL CAPS) to the left and the input to the right.
- The `Exam type` and `Format` fields use radio pills laid out in two rows.
- `Time remaining` is a toggle between "hours" and "days" plus a number input.
- Submit is `⌘+Return`.
- A small preview panel on the right updates live, showing the prep-mode badge that will appear on outputs.

## 8. Screens

Brief walkthrough of each key screen. Detailed specs live next to their components.

### 8.1 First launch

Full-window, no chrome. Three steps, one per screen:

1. **Welcome** — Fraunces display heading "Welcome to StudyLM.", one paragraph of copy, a single `Continue` button.
2. **Choose your LLM** — five provider tiles in a grid: Claude, OpenAI, Google, OpenRouter, Ollama. Selecting a tile reveals either an API-key input or an Ollama-detected model list.
3. **You're set** — "Your first notebook is waiting." → jumps to the prep-mode wizard with a pre-filled demo notebook containing one sample PDF and a prepared prompt.

### 8.2 Notebook list

Centered 720 px column. Header: "Notebooks" in Fraunces display, "+ New notebook" button on the right. Sort toggle: recent / alphabetical. Each list item:

```
──────────────────────────────────────────────
THERMODYNAMICS · END-SEM · 3 DAYS
Thermodynamics — End Sem Revision
4 sources · 2 flashcard sets · last opened 2h ago
──────────────────────────────────────────────
```

- Eyebrow in Geist Mono 11 px, `--paper-500`.
- Title in Fraunces 500, H2 size.
- Meta line in Geist Mono 12 px, `--paper-500`.
- Full-row click opens the notebook; right-click shows a context menu.
- Hairlines between rows, not cards with shadows.

### 8.3 Notebook workspace

Described in §4.1. Defaults:

- When opened, workspace middle shows the first source's PDF.
- Chat pane is non-empty only if chats exist; otherwise shows the empty state.
- If no study guide or flashcards have been generated, a thin banner at the top of the middle pane offers "Generate study guide · Generate flashcards · Generate podcast".

### 8.4 PDF viewer

Middle-pane tab. Renders via pdfjs-dist. Toolbar above: page indicator (Geist Mono), zoom, fit-width, download. No annotations in MVP — this is a read-only viewer. Citations from chat land on the correct page and briefly underline the cited region.

### 8.5 Study guide viewer

Middle-pane tab. Rendered Markdown. Uses the full type system: Fraunces H1/H2 for structure, Body for paragraphs, small caps eyebrows for section labels. Max line length 68ch. Prep-mode badge pinned to the top-left at `position: sticky`.

### 8.6 Flashcard review

Middle-pane tab. Displays one card at a time per §7.3. Progress bar above the card (hairline, fills with `--ink-500`). Keyboard shortcuts printed faintly below the card: `Space flip  ·  1 skip  ·  2 again  ·  3 got it`.

### 8.7 Podcast player

Middle-pane tab. Two-column: waveform + transport controls on the top, transcript on the bottom. Transcript highlights the current line; clicking a line seeks. Host names appear as eyebrows above their lines, alternating left/right via indentation only (no bubble split).

### 8.8 Settings

Two-column. Left rail: System, Providers, Appearance, Shortcuts, About. Right content pane. The Providers screen lists all five providers with their status (`Not configured` / `Connected` / `Detected`), the currently selected one highlighted. Each row expands to show key entry or Ollama model picker.

## 9. Accessibility

Every design decision is gated on the following. Don't merge a component that fails any of these.

- **Contrast:** every body-on-background pair meets WCAG AA (4.5:1). Interactive states (hover, focus, pressed) maintain it. Tested via a contrast-check script in CI.
- **Keyboard:** every interactive element is reachable and operable by keyboard. Focus ring is always visible — `--focus`, 2 px, offset 2 px. Command palette is a first-class nav path.
- **Screen reader:** role attributes set on panes (`complementary`, `main`), eyebrow labels tied to their sections with `aria-labelledby`.
- **Reduced motion:** per §5, `prefers-reduced-motion: reduce` disables all non-essential animation. Parse progress falls back to a static "parsing…" label.
- **Font scaling:** the app respects OS-level font scale up to 150% without horizontal scroll. Beyond that, graceful reflow.
- **Color independence:** no information conveyed by color alone. Parse status uses color + icon + text label. Difficulty dot has a text tooltip.

## 10. Anti-patterns

If you catch yourself reaching for any of these, stop and read this section.

- **Purple-to-blue gradient on anything.** The current universal signifier of "generic AI product". Never.
- **Drop shadows on cards.** We use background color and hairlines for depth.
- **Inter on anything.** It's a fine font; it's also in every SaaS app ever made. Geist is our UI font.
- **Emoji as UI.** Use Lucide icons. Emoji appear only if a student types them in chat.
- **Pill-shaped primary buttons.** Pills carry semantic weight (status, prep mode, citation). A pill-shaped Submit button waters that signal down.
- **Spinner loading states.** Use the parse sweep, skeleton text, or a streaming caret — something that communicates what's happening, not just that something is.
- **Glassmorphism / backdrop-blur on panels.** The one exception is the dialog backdrop. Anywhere else, no.
- **Generic AI stock illustrations.** Flat 3D people, duotone blobs, abstract "neural network" patterns. None of it.
- **Google Material-style FAB.** We don't need a round floating "+" in the bottom-right. Use the app's toolbar.
- **Modal on top of modal.** If a flow requires two modals stacked, redesign the flow. One layer of dialog, always.
- **Dark mode as an inverted light mode.** Dark mode has its own tuned tokens (§3.1). Never simply flip colors.

## 11. References (mood, not to copy)

- **Readwise Reader** — the reading-as-primary-affordance feel.
- **Linear** — the restraint, keyboard-first operation, command palette.
- **Are.na** — editorial typography, warm neutrals.
- **iA Writer** — typographic hierarchy in a reading-heavy context.
- **Campsite** — quiet, human-voiced product surface.

Look at these for direction; never lift UI directly. The point is to internalize the *feel* — substantive, quiet, bookish — not to reproduce someone else's buttons.
