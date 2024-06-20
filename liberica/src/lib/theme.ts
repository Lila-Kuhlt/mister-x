import THEMES_JSON from "assets/themes.json";
import { hexToHSL } from "lib/colors";
import { camelToKebabCase } from "lib/util";

const LOCAL_STORAGE_THEME_KEY = "theme";
const BROADCAST_CHANNEL_NAME = "theme";

export type ThemeName = keyof typeof THEMES_JSON;

export const THEMES: Record<ThemeName, Theme> = THEMES_JSON;
export const THEME_NAMES = Object.keys(THEMES) as ThemeName[];

export const BROADCAST_CHANNEL = new BroadcastChannel(BROADCAST_CHANNEL_NAME);

BROADCAST_CHANNEL.onmessage = (msg: { data: ThemeName }) => {
    console.debug(
        `Received Boradcast Message on channel ${BROADCAST_CHANNEL_NAME}: `,
        msg.data,
    );
    if (!THEME_NAMES.includes(msg.data)) return;
    applyTheme(msg.data, { persistent: false, broadcast: false });
};

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

export function saveTheme(themeName?: ThemeName) {
    if (!themeName) {
        localStorage.removeItem(LOCAL_STORAGE_THEME_KEY);
        return;
    }

    localStorage.setItem(LOCAL_STORAGE_THEME_KEY, themeName);
}

export function loadTheme(): ThemeName | null {
    return localStorage.getItem(LOCAL_STORAGE_THEME_KEY) as ThemeName | null;
}

export interface ApplyThemeOptions {
    persistent?: boolean;
    broadcast?: boolean;
}

const applyThemeOptionsDefaults: Required<ApplyThemeOptions> = {
    persistent: false,
    broadcast: false,
};

export function applyTheme(
    themeName: ThemeName,
    options: ApplyThemeOptions = {},
) {
    console.assert(THEME_NAMES.includes(themeName), "Set Theme does not exist");
    const { persistent, broadcast } = {
        ...applyThemeOptionsDefaults,
        ...options,
    };

    const style = document.documentElement.style;
    const theme = THEMES[themeName];

    type ThemeEntry = [keyof Theme, string];

    for (const [name, val] of Object.entries(theme) as ThemeEntry[]) {
        let color = val;

        if (val.startsWith("@")) {
            const link = val.substring(1) as keyof Theme;
            color = theme[link];
        }

        const { h, s, l } = hexToHSL(color);
        const hsl = `${h} ${s}% ${l}%`;

        style.setProperty("--color-" + camelToKebabCase(name), hsl);
    }

    if (persistent) saveTheme(themeName);
    if (broadcast) BROADCAST_CHANNEL.postMessage(themeName);
}
