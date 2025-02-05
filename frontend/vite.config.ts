import { inlineSvg } from "@svelte-put/inline-svg/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import UnoCSS from 'unocss/vite'
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    inlineSvg([
      { directories: "static/svg" },
    ]),
    UnoCSS(),
    sveltekit(),
  ],
});
