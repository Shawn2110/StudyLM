# Design — StudyLM

**Status:** Draft v2 — modern AI workspace
**Last updated:** 2026-04-25
**Supersedes:** v1 (scholarly-editorial / Fraunces / hairline minimalism). v1
preserved in git history if you ever want to look back.

This document defines StudyLM's visual and interaction design. It is the
source of truth for component styling, typography, color, motion, and
layout. When architecture, behavior, or accessibility decisions touch
visual design, they belong here too.

## 1. Philosophy

StudyLM is a serious tool for a serious moment, but it is **a modern AI
workspace, not a printed book**. The reference point is Anthropic's Claude
product surface: warm cream backgrounds, a single confident coral accent,
clean humanist sans-serif throughout, soft rounded surfaces, generous but
not wasteful whitespace, gentle elevation. Reading and writing in this
app should feel like a calm, contemporary place to spend two hours — not
a library, not a SaaS dashboard.

Three pillars:

1. **Warm, not stark.** Cream surfaces, never harsh white. Coral accent,
   never cold blue. The whole frame should feel like a comfortable desk
   under a reading lamp.
2. **Confident restraint.** One accent color. One sans-serif family. Three
   radii. Three shadows. No gradients, no glass, no gratuitous color, no
   decorative illustration.
3. **Use the window.** Lists and detail surfaces fill the available width
   via a sidebar shell, not a centered editorial column. Reading surfaces
   (chat, study guide) reintroduce a max line length where it matters.

## 2. What we take from Claude.ai — and what we don't

**Borrow:**

- The cream warm-neutral palette and a single warm accent.
- Sidebar app shell with the list of conversations/notebooks always
  visible.
- One humanist sans family for everything (Geist, in our case).
- Soft 12–16 px card radii, full-pill chips and badges.
- Subtle elevation conveyed by background tint + border, with one soft
  shadow reserved for the dialog and command-palette layer.
- Empty states with personality: one short sentence, one icon, one action.

**Deliberately don't borrow:**

- Chat-as-the-only-surface. We have notebooks, sources, study guides,
  flashcards. Chat is one tab among several.
- Brand marks, the Claude orb, voice/dictation surfaces.
- Full-width streaming-only layouts.
- Generic AI gradients (purple-to-blue, "neural" patterns).

## 3. Tokens

All tokens in `src/styles/tokens.css`. Tailwind reads from these via
`tailwind.config.ts`. Dark mode shares the same token names with the
`[data-theme='dark']` selector providing alternates.

### 3.1 Color

Cream warm neutrals + a single coral accent + desaturated semantic colors.

```css
:root {
  /* Cream warm neutrals */
  --bg:            oklch(97.5% 0.006 75);  /* page */
  --surface:       oklch(99.0% 0.003 75);  /* cards, panels */
  --surface-alt:   oklch(95.5% 0.008 75);  /* input bg, hover */
  --border:        oklch(90.0% 0.010 75);  /* default hairline */
  --border-strong: oklch(82.0% 0.012 75);  /* selected hairline */
  --muted:         oklch(56.0% 0.015 75);  /* secondary text */
  --text:          oklch(28.0% 0.018 75);  /* body */
  --text-strong:   oklch(16.0% 0.022 75);  /* headings */

  /* Coral accent */
  --accent:        oklch(64.0% 0.140 35);  /* primary */
  --accent-hover:  oklch(58.0% 0.150 32);  /* hover */
  --accent-active: oklch(52.0% 0.155 30);  /* pressed */
  --accent-soft:   oklch(94.5% 0.030 35);  /* tinted background */
  --accent-on:     oklch(99.0% 0.003 75);  /* text on accent */

  /* Semantic — desaturated to live among the warm neutrals */
  --success: oklch(60.0% 0.090 160);
  --warning: oklch(72.0% 0.130 80);
  --danger:  oklch(58.0% 0.150 25);

  /* Focus ring */
  --focus: oklch(64.0% 0.140 35 / 0.40);

  /* Elevation */
  --shadow-sm: 0 1px 2px oklch(0% 0 0 / 0.05);
  --shadow-md: 0 2px 8px oklch(0% 0 0 / 0.06), 0 1px 2px oklch(0% 0 0 / 0.04);
  --shadow-lg: 0 12px 32px oklch(0% 0 0 / 0.10), 0 2px 6px oklch(0% 0 0 / 0.04);
}

:root[data-theme="dark"] {
  --bg:            oklch(17.0% 0.010 75);
  --surface:       oklch(20.5% 0.012 75);
  --surface-alt:   oklch(24.0% 0.013 75);
  --border:        oklch(30.0% 0.014 75);
  --border-strong: oklch(38.0% 0.014 75);
  --muted:         oklch(62.0% 0.012 75);
  --text:          oklch(86.0% 0.010 75);
  --text-strong:   oklch(96.0% 0.008 75);

  --accent:        oklch(72.0% 0.130 35);
  --accent-hover:  oklch(78.0% 0.135 32);
  --accent-active: oklch(84.0% 0.140 30);
  --accent-soft:   oklch(28.0% 0.045 35);
  --accent-on:     oklch(17.0% 0.010 75);
}
```

**Usage rules:**

- `--bg` is the page background and the only top-level surface.
- `--surface` is for any card, panel, or sidebar that needs to lift off the
  page. `--surface-alt` is for inputs, raised states, hover.
- `--accent` is the only accent color — primary buttons, the active
  navigation pill, focus borders, the citation pill, the prep-mode chip.
  Do not introduce a second accent.
- `--accent-soft` is the very pale tinted background used for "selected"
  list rows and the prep-mode badge.
- Body text is `--text` on `--bg` (~9:1 contrast). Headings use
  `--text-strong`. `--muted` is for metadata and secondary copy only.
- Semantic colors are for status only, never decoration.

### 3.2 Typography

One family — **Geist** — used everywhere. Geist Mono for numbers, file
paths, code, prep-mode chips. There is no display serif in v2; Fraunces
has been removed.

```css
:root {
  --font-sans: "Geist", "Inter", system-ui, sans-serif;
  --font-mono: "Geist Mono", "JetBrains Mono", ui-monospace, monospace;
}
```

Type scale:

| Role | Size | Weight | Tracking | Notes |
|---|---|---|---|---|
| Display (page title) | 1.875 rem (30 px) | 600 | -0.025em | Geist |
| H1 | 1.5 rem (24 px) | 600 | -0.02em | Geist |
| H2 | 1.25 rem (20 px) | 600 | -0.015em | Geist |
| H3 | 1 rem (16 px) | 600 | -0.01em | Geist |
| Body | 0.9375 rem (15 px) | 400 | 0 | Geist |
| UI | 0.875 rem (14 px) | 500 | 0 | buttons, labels |
| Meta | 0.8125 rem (13 px) | 400 | 0 | metadata, captions |

**Rules:**

- No SMALL CAPS section labels in v2. Use plain `text-sm font-medium
  text-muted` for "Sources", "Recent", panel labels.
- Geist Mono for any quantitative/identity text: page numbers, model
  names, file extensions, the prep-mode chip body.
- Headings use `--text-strong`; body uses `--text`; everything secondary
  uses `--muted`.
- Line-height: 1.5 for body, 1.25 for headings.

### 3.3 Spacing

8 px base. Tailwind's defaults are kept; one custom token for the sidebar:

```css
:root {
  --sidebar-width: 17.5rem; /* 280 px */
}
```

### 3.4 Radii

Three radii in v2 (was four).

- **6 px** — inputs, buttons, small interactive surfaces. Friendlier than
  the old 4 px.
- **12 px** — cards, panels, source cards, sidebar pill items.
- **16 px** — dialogs, popovers, command palette.
- **9999 px (pill)** — chips, status badges, prep-mode badge, the active
  sidebar item.

### 3.5 Elevation

Four levels:

1. **Page** — `--bg`, no border, no shadow.
2. **Surface** — `--surface`, 1 px `--border`, no shadow. Sidebar, source
   cards, settings rail.
3. **Hover/raised** — `--surface-alt` background, optional `--shadow-sm`
   for cards that the cursor lifts.
4. **Floating** — dialog, command palette, dropdown. `--surface` with
   `--shadow-lg` and 1 px `--border`.

No backdrop blur. No glassmorphism. Drop shadows are tiny and warm-tinted.

## 4. Layout

### 4.1 The app shell — sidebar + main

```
┌────────────────────────────────────────────────────────────────────┐
│  [STUDYLM]                                              [⚙]        │  ← top bar (slim, 48 px)
├──────────────┬─────────────────────────────────────────────────────┤
│              │                                                     │
│  + New       │   <route content goes here, full width>             │
│              │                                                     │
│  Notebooks   │                                                     │
│  ──────────  │                                                     │
│  ● Cloud …   │                                                     │
│  ○ Algo Wk3  │                                                     │
│  ○ Constit … │                                                     │
│              │                                                     │
│              │                                                     │
│              │                                                     │
│  Settings    │                                                     │
└──────────────┴─────────────────────────────────────────────────────┘
```

- **Left sidebar:** 280 px, `--surface` background, hairline right border.
  Contains: a primary "+ New notebook" button, a "Notebooks" label, the
  list of notebooks (current one highlighted), and a Settings link
  pinned to the bottom.
- **Top bar:** 48 px, hairline bottom border. Wordmark on the left, gear
  icon (settings) on the right.
- **Main pane:** flex-1 of the remaining width. Routes own their own
  internal padding and max-widths.

The home route ("/") shows a centered welcome state in the main pane when
no notebook is selected.

### 4.2 Detail surfaces

The notebook detail at `/notebooks/$id` uses the full main pane, with
internal padding (`px-8 py-6`) and content laid out as a responsive grid
of source cards (3 across at desktop, 2 at medium, 1 at narrow).

Future workspace surfaces (chat / study guide / flashcards / podcast)
become tabbed views inside the same detail pane.

### 4.3 Modal flows

The create-notebook wizard is a modal, **520 px wide**, single column,
plain text labels (no SMALL CAPS), `⌘ + Return` to submit.

## 5. Motion

Motion stays subtle. Same timings as v1: `100 ms` instant, `160 ms`
snappy, `240 ms` deliberate. Easing: `ease-out` style for entrances
(`cubic-bezier(0.22, 1, 0.36, 1)`), `ease-in` for exits.

- Sidebar item hover: 100 ms background change.
- Card hover: 160 ms `translate-y-[-1px]` + `shadow-sm`.
- Dialog open: 160 ms fade + small upward translate.
- Status badge color change: 240 ms fade.
- Streaming caret (later, in chat): 1 Hz blink.

`prefers-reduced-motion: reduce` disables transforms and spinners; status
indicators fall back to plain text.

## 6. Components

### 6.1 Buttons

Three variants. **36 px tall** (was 32 — slightly more tactile and matches
the modern app feel). 6 px radius. Geist 500 at 14 px. Standard padding
12 px × 16 px.

- **Primary** — `--accent` background, `--accent-on` text. Hover
  `--accent-hover`, active `--accent-active`.
- **Secondary** — `--surface-alt` background, `--text` text, 1 px
  `--border`. Hover bumps border to `--border-strong`.
- **Ghost** — transparent, `--muted` text. Hover background `--surface-alt`,
  text becomes `--text`.

Icon-only buttons are 36 × 36 squares with a 18 px Lucide icon.
Destructive variant: secondary shape with `--danger` text. Confirmation
required for destructive actions touching data.

### 6.2 Inputs

36 px tall, 6 px radius, 1 px `--border`, `--surface-alt` background.
Focus: border becomes `--accent`, plus 3 px outer ring in `--focus`.
Plain labels above (no SMALL CAPS, no floating labels).

Textarea: same shape, multi-line, auto-grows.

### 6.3 Source card

12 px radius, `--surface` background, 1 px `--border`. File icon
(Lucide `FileText`) on the left in `--muted`. Filename in body weight 500
right of the icon. Status pill at the bottom with semantic color (warm
neutral while busy, success while ready, danger while failed). Hover:
`translate-y-[-1px]` and `--shadow-sm`. Selected: 1 px `--accent` border
and tinted `--accent-soft` background.

The v1 left-edge color stripe is removed (too NotebookLM-derivative).

### 6.4 Sidebar item

Pill-shaped row inside the sidebar. Left-aligned text. 32 px tall,
horizontal padding 12 px. Inactive: transparent, `--muted` text, hover
`--surface-alt`. Active: `--accent-soft` background, `--text-strong`
text.

### 6.5 Dialog

16 px radius, `--surface` background, 1 px `--border`, `--shadow-lg`.
Backdrop is `oklch(0% 0 0 / 0.30)` — solid translucent black, no blur.
Title in H2 type (Geist 600 at 20 px), description in body type.
Action row at the bottom right; primary action rightmost.

### 6.6 Prep-mode badge

Pill, `--accent-soft` background, `--accent` text, no border. Geist Mono
12 px, tracking 0.04em (was 0.08), uppercase. Three pieces max separated
by `·`.

### 6.7 Empty state

Centered. One Lucide icon (32 px) in `--muted`, one short sentence in
body type, one primary button. No dashed borders. Background stays as
`--bg` so the empty surface doesn't read as "missing card".

## 7. Accessibility

- Contrast: every body-on-background pair meets WCAG AA (4.5:1).
- Focus ring `--focus` is always visible — 3 px outer ring on inputs and
  buttons via `:focus-visible`.
- Keyboard: every interactive element reachable. `⌘K` command palette is
  the primary nav (deferred to a later phase).
- Screen reader: panes labelled (`complementary`, `main`, `navigation`).
- Reduced motion: per §5.
- Font scaling: respects OS scale up to 150% without horizontal scroll.
- Color independence: status badges always pair color with a text label.

## 8. Anti-patterns

If you reach for any of these, stop:

- **Drop shadows on everything.** Use `--shadow-sm` for hover only,
  `--shadow-lg` for floating layer only.
- **Backdrop blur on dialogs.** Solid translucent black backdrop, no blur.
- **A second accent color.** Only `--accent`. Status colors are not
  accents.
- **Fraunces or any other display serif.** Geist throughout.
- **SMALL CAPS section labels.** Plain `text-sm font-medium text-muted`.
- **Spinner loaders.** Use a thin progress bar or a status badge that
  changes; never an indeterminate spinner.
- **Document color stripes.** v1 had them; we do not.
- **Centered max-width column on detail/list views.** That belongs to
  reading content (chat, study guide). Lists and grids fill the pane.
- **Generic AI illustrations / 3D blobs / abstract neural patterns.** No.
