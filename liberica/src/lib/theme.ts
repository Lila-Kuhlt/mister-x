import THEMES_JSON from "assets/themes.json"

export const RAW_THEMES: Record<string, Theme> = THEMES_JSON;

export type Theme = {
    'text-bright': string,
    'text-dark': string,
    'surface': string,
    'primary': string,
    'secondary': string,
    'accent': string
}

export function applyTheme(theme: Theme) {
    const style = window.getComputedStyle(document.documentElement);
    for (const key of Object.keys(theme)) {
        style.setProperty(`--${key}`, theme[key as keyof Theme]);
    }
}
