import THEMES_JSON from "assets/themes.json";
import Values from "values.js";

export const THEMES: Record<string, Theme> = THEMES_JSON;

export interface Theme {
    text: string;
    surface: string;
    primary: string;
    secondary: string;
    accent: string;
}

export function applyTheme(theme: Theme) {
    const style = document.documentElement.style;
    for (const key of Object.keys(theme)) {
        const shades = new Values(theme[key as keyof Theme], "base").all(22);
        for (const [i, shade] of shades.entries()) {
            const name = (i * 100 + 100).toString();
            style.setProperty(`--${key}-${name}`, "#" + shade.hex);
        }
    }
}
