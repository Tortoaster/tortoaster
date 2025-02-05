import { Preset } from 'unocss'

export const presetTortoaster: Preset = {
    name: 'tortoaster-preset',
    rules: [
        [/^m-([.\d]+)$/, ([_, num]) => ({ margin: `${+num * 0.8}rem` })],
        [/^p-([.\d]+)$/, ([_, num]) => ({ padding: `${+num * 0.8}rem` })],
    ],
}
