import THEMES_JSON from "assets/themes.json";
import { hexToHSL } from "lib/colors";
import { camelToKebabCase } from "lib/util";

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

        const { h, s, l } = hexToHSL(color);
        // This has to be like this, to please the linter
        const hsl = `${h.toString()} ${s.toString()}% ${l.toString()}%`;

        style.setProperty("--color-" + camelToKebabCase(name), hsl);
    }
}
