import THEMES_JSON from "assets/themes.json";
import { HexToHSL } from "lib/colors";
import { fromCamelToKebabCase } from "lib/util";

export const THEMES: Record<string, Theme> = THEMES_JSON;

export interface Theme {
    base: string;
    surface: string;
    muted: string;
    primary: string;
    secondary: string;

    onBase: string;
    onSurface: string;
    onPrimary: string;
    onSecondary: string;
    onMuted: string;
}

export function applyTheme(theme: Theme) {
    const style = document.documentElement.style;

    for (const [name, val] of Object.entries(theme) as [
        keyof Theme,
        string,
    ][]) {
        let color = val;

        if (val.startsWith("@")) {
            const link = val.substring(1) as keyof Theme;
            color = theme[link];
            console.log(name, "links to " + link + " --> " + color);
        }

        const { h, s, l } = HexToHSL(color);
        const hsl = `${h.toString()} ${s.toString()}% ${l.toString()}%`;

        console.log(name, fromCamelToKebabCase(name));
        style.setProperty("--color-" + fromCamelToKebabCase(name), hsl);
    }
}
