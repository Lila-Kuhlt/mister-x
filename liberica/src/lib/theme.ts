import THEMES_JSON from "assets/themes.json";
import { HexToHSL } from "lib/colors";

export const THEMES: Record<string, Theme> = THEMES_JSON;

export interface Theme {
    base: string;
    surface: string;
    text: string;
    primary: string;
    secondary: string;
    accent: string;
}

export function applyTheme(theme: Theme) {
    const style = document.documentElement.style;
    for (const [name, color] of Object.entries(theme)) {
        const { h, s, l } = HexToHSL(color as string);
        style.setProperty(
            `--color-${name}`,
            `${h.toString()} ${s.toString()} ${l.toString()}`,
        );
    }
}
