import { type Config } from "tailwindcss";

import generated from "@tailwindcss/typography";

export default {
  content: [
    "{routes,islands,components}/**/*.{ts,tsx,js,jsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        "comic": ["Comic Relief", "cursive"],
      },
      boxShadow: {
        "comic": "4px 4px #1c1917",
      },
    },
  },
  plugins: [
    generated,
  ],
} satisfies Config;
