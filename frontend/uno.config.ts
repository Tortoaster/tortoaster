import {defineConfig, presetIcons, presetUno} from 'unocss'
import { presetTortoaster } from './preset-tortoaster.ts'
import presetTagify from "@unocss/preset-tagify";
import extractorSvelte from "@unocss/extractor-svelte";

export default defineConfig({
    extractors: [
        extractorSvelte(),
    ],
    presets: [
        presetUno({
            dark: 'media',
            preflight: 'on-demand',
        }),
        presetIcons(),
        presetTagify(),
        presetTortoaster,
    ],
    theme: {
        colors: {
            background: '#282c34',
            black: {
                darkest: '#181a1f',
                darker: '#21252b',
                DEFAULT: '#3f4451',
                bright: '#4f5666',
            },
            gray: {
                DEFAULT: '#545862',
                bright: '#9196a1',
            },
            foreground: '#abb2bf',
            white: {
                DEFAULT: '#e6e6e6',
                bright: '#ffffff',
            },
            red: {
                DEFAULT: '#e05561',
                bright: '#ff616e',
            },
            orange: '#d18f52',
            yellow: {
                DEFAULT: '#e6b965',
                bright: '#f0a45d',
            },
            green: {
                DEFAULT: '#8cc265',
                bright: '#a5e075',
            },
            cyan: {
                DEFAULT: '#42b3c2',
                bright: '#4cd1e0',
            },
            blue: {
                DEFAULT: '#4aa5f0',
                bright: '#4dc4ff',
            },
            purple: {
                DEFAULT: '#c162de',
                bright: '#de73ff',
            },
            brown: '#bf4034',
        },
    },
})
