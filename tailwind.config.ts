import type { Config } from "tailwindcss";
import tailwindcssAnimate from "tailwindcss-animate";

/*
 * Tailwind theme — reads design tokens from src/styles/tokens.css.
 * v2 (claude.ai-inspired): cream surfaces, coral accent, soft radii (6/12/16),
 * subtle shadows, single Geist font family. Legacy paper/ink class names
 * still work via CSS variable aliases declared in tokens.css.
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
      sans: ["var(--font-sans)", "system-ui", "sans-serif"],
      serif: ["var(--font-sans)", "system-ui", "sans-serif"], // Fraunces dropped — alias to sans
      mono: ["var(--font-mono)", "ui-monospace", "monospace"],
    },
    borderRadius: {
      none: "var(--radius-none)",
      sm: "var(--radius-sm)",
      DEFAULT: "var(--radius-sm)",
      md: "var(--radius-sm)",
      lg: "var(--radius-lg)",
      xl: "var(--radius-xl)",
      "2xl": "var(--radius-xl)",
      full: "var(--radius-pill)",
    },
    boxShadow: {
      none: "none",
      sm: "var(--shadow-sm)",
      DEFAULT: "var(--shadow-md)",
      md: "var(--shadow-md)",
      lg: "var(--shadow-lg)",
    },
    extend: {
      colors: {
        /* Canonical v2 names */
        bg: "var(--bg)",
        surface: "var(--surface)",
        "surface-alt": "var(--surface-alt)",
        "border-default": "var(--border)",
        "border-strong": "var(--border-strong)",
        muted: { DEFAULT: "var(--surface-alt)", foreground: "var(--muted)" },
        text: "var(--text)",
        "text-strong": "var(--text-strong)",
        accent: {
          DEFAULT: "var(--accent)",
          hover: "var(--accent-hover)",
          active: "var(--accent-active)",
          soft: "var(--accent-soft)",
          on: "var(--accent-on)",
          foreground: "var(--accent-on)",
        },

        /* Semantic */
        success: "var(--success-500)",
        warning: "var(--warning-500)",
        danger: "var(--danger-500)",

        /* Legacy paper/ink — keep so existing components keep rendering. */
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

        /* Shadcn semantic aliases — re-pointed at v2 tokens. */
        background: "var(--bg)",
        foreground: "var(--text)",
        border: "var(--border)",
        input: "var(--border)",
        ring: "var(--focus)",
        primary: {
          DEFAULT: "var(--accent)",
          foreground: "var(--accent-on)",
        },
        secondary: {
          DEFAULT: "var(--surface-alt)",
          foreground: "var(--text)",
        },
        destructive: {
          DEFAULT: "var(--danger-500)",
          foreground: "var(--accent-on)",
        },
        popover: {
          DEFAULT: "var(--surface)",
          foreground: "var(--text)",
        },
        card: {
          DEFAULT: "var(--surface)",
          foreground: "var(--text)",
        },
      },
      spacing: {
        "reading-gutter": "var(--space-reading-gutter)",
        "panel-gutter": "var(--space-panel-gutter)",
        "card-gutter": "var(--space-card-gutter)",
        sidebar: "var(--sidebar-width)",
        topbar: "var(--topbar-height)",
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
