import typography from "@tailwindcss/typography";
import { addDynamicIconSelectors } from "@iconify/tailwind";
import type { Config } from "tailwindcss";

export default {
  content: ["./src/**/*.{html,js,svelte,ts}", "./static/svg/*.svg"],

  theme: {
    aspectRatio: {
      "square": "1 / 1",
      "2/1": "2 / 1",
      "3/2": "3 / 2",
    },
    borderColor: {
      DEFAULT: "var(--darkest-black)",
    },
    borderWidth: {
      DEFAULT: "var(--border-thickness)",
    },
    borderRadius: {
      DEFAULT: "var(--border-radius)",
    },
    colors: {
      transparent: "transparent",
      background: "var(--background)",
      black: {
        DEFAULT: "var(--black)",
        "bright": "var(--bright-black)",
        "darker": "var(--darker-black)",
        "darkest": "var(--darkest-black)",
      },
      gray: {
        DEFAULT: "var(--gray)",
        "bright": "var(--bright-gray)",
      },
      foreground: "var(--foreground)",
      white: {
        DEFAULT: "var(--white)",
        "bright": "var(--bright-white)",
      },
      red: {
        DEFAULT: "var(--red)",
        "bright": "var(--bright-red)",
      },
      orange: "var(--orange)",
      yellow: {
        DEFAULT: "var(--yellow)",
        "bright": "var(--bright-yellow)",
      },
      green: {
        DEFAULT: "var(--green)",
        "bright": "var(--bright-green)",
      },
      cyan: {
        DEFAULT: "var(--cyan)",
        "bright": "var(--bright-cyan)",
      },
      blue: {
        DEFAULT: "var(--blue)",
        "bright": "var(--bright-blue)",
      },
      purple: {
        DEFAULT: "var(--purple)",
        "bright": "var(--bright-purple)",
      },
      brown: "var(--brown)",
    },
    margin: {
      "0": "0",
      "auto": "auto",
      "border": "calc(var(--border-thickness) - 1pt)",
      "border-plus": "calc(var(--border-thickness) + 1pt)",
      "half": "1rem",
      DEFAULT: "2rem",
      "double": "4rem",
    },
    spacing: {
      "0": "0",
      "border": "calc(var(--border-thickness) - 1pt)",
      "half": "1rem",
      DEFAULT: "2rem",
      "double": "4rem",
    },
    transitionDuration: {
      DEFAULT: "var(--transition-duration)",
    },
    transitionTimingFunction: {
      DEFAULT: "var(--transition-easing)",
    },
    width: {
      "border": "var(--border-thickness)",
      "spacing-half": "1rem",
      "spacing": "2rem",
      "spacing-double": "4rem",
      "1/3": "33.333333%",
      "2/3": "66.666667%",
      "full": "100%",
    },
    height: {
      "border": "var(--border-thickness)",
      "spacing-half": "1rem",
      "spacing": "2rem",
      "spacing-double": "4rem",
      "1/3": "33.333333%",
      "2/3": "66.666667%",
      "full": "100%",
    },
    extend: {
      typography: ({ theme }: {theme: (key: string) => string}) => ({
        white: {
          css: {
            '--tw-prose-body': theme("colors.white.bright"),
            '--tw-prose-headings': theme("colors.white.bright"),
            '--tw-prose-lead': theme("colors.white.bright"),
            '--tw-prose-links': theme("colors.white.bright"),
            '--tw-prose-bold': theme("colors.white.bright"),
            '--tw-prose-counters': theme("colors.white.bright"),
            '--tw-prose-bullets': theme("colors.white.bright"),
            '--tw-prose-hr': theme("colors.black.darker"),
            '--tw-prose-quotes': theme("colors.white"),
            '--tw-prose-quote-borders': theme("colors.black.darker"),
            '--tw-prose-captions': theme("colors.white"),
            '--tw-prose-kbd': theme("colors.white"),
            '--tw-prose-kbd-shadows': theme("colors.black.darkest"),
            '--tw-prose-code': theme("colors.white"),
            '--tw-prose-pre-code': theme("colors.white"),
            '--tw-prose-pre-bg': theme("colors.white"),
            '--tw-prose-th-borders': theme("colors.black.darkest"),
            '--tw-prose-td-borders': theme("colors.black.darkest"),
          },
        },
      }),
    },
  },

  plugins: [
      typography,
      addDynamicIconSelectors(),
  ],
} satisfies Config;
