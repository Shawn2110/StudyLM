import type { Config } from "tailwindcss";
import tailwindcssAnimate from "tailwindcss-animate";

/*
 * Tailwind theme — reads design tokens from src/styles/tokens.css.
 * See docs/design.md §3 for the source of truth on colors/spacing/type.
 * Most shadcn semantic names (background, foreground, primary, …) are
 * preserved as aliases over the paper/ink palette so existing primitives
 * keep working.
 */

const config: Config = {
  darkMode: ["class", "[data-theme='dark']"],
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    container: {
      center: true,
      padding: "2rem",
      screens: { "2xl": "1400px" },
    },
    fontFamily: {
      sans: ["var(--font-ui)", "system-ui", "sans-serif"],
      serif: ["var(--font-display)", "serif"],
      mono: ["var(--font-mono)", "ui-monospace", "monospace"],
    },
    borderRadius: {
      none: "var(--radius-none)",
      sm: "var(--radius-sm)",
      DEFAULT: "var(--radius-sm)",
      lg: "var(--radius-lg)",
      full: "var(--radius-pill)",
    },
    extend: {
      colors: {
        paper: {
          50: "var(--paper-50)",
          100: "var(--paper-100)",
          200: "var(--paper-200)",
          300: "var(--paper-300)",
          400: "var(--paper-400)",
          500: "var(--paper-500)",
          700: "var(--paper-700)",
          900: "var(--paper-900)",
        },
        ink: {
          400: "var(--ink-400)",
          500: "var(--ink-500)",
          600: "var(--ink-600)",
          700: "var(--ink-700)",
        },
        success: "var(--success-500)",
        warning: "var(--warning-500)",
        danger: "var(--danger-500)",

        /* Semantic aliases over paper/ink so shadcn primitives keep working. */
        background: "var(--paper-50)",
        foreground: "var(--paper-700)",
        border: "var(--paper-300)",
        input: "var(--paper-300)",
        ring: "var(--focus)",
        primary: {
          DEFAULT: "var(--ink-500)",
          foreground: "var(--paper-50)",
        },
        secondary: {
          DEFAULT: "var(--paper-200)",
          foreground: "var(--paper-900)",
        },
        destructive: {
          DEFAULT: "var(--danger-500)",
          foreground: "var(--paper-50)",
        },
        muted: {
          DEFAULT: "var(--paper-100)",
          foreground: "var(--paper-500)",
        },
        accent: {
          DEFAULT: "var(--paper-200)",
          foreground: "var(--paper-900)",
        },
        popover: {
          DEFAULT: "var(--paper-100)",
          foreground: "var(--paper-700)",
        },
        card: {
          DEFAULT: "var(--paper-100)",
          foreground: "var(--paper-700)",
        },
      },
      spacing: {
        "reading-gutter": "var(--space-reading-gutter)",
        "panel-gutter": "var(--space-panel-gutter)",
        "card-gutter": "var(--space-card-gutter)",
      },
      transitionTimingFunction: {
        enter: "var(--ease-enter)",
        exit: "var(--ease-exit)",
      },
      transitionDuration: {
        instant: "var(--motion-instant)",
        snappy: "var(--motion-snappy)",
        deliberate: "var(--motion-deliberate)",
      },
      keyframes: {
        "accordion-down": {
          from: { height: "0" },
          to: { height: "var(--radix-accordion-content-height)" },
        },
        "accordion-up": {
          from: { height: "var(--radix-accordion-content-height)" },
          to: { height: "0" },
        },
        "parse-sweep": {
          "0%": { transform: "translateX(-100%)" },
          "100%": { transform: "translateX(100%)" },
        },
      },
      animation: {
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
        "parse-sweep": "parse-sweep 1.6s var(--ease-enter) infinite",
      },
    },
  },
  plugins: [tailwindcssAnimate],
};

export default config;
