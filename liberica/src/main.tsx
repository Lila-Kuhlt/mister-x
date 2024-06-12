import { CreateTeam } from "page/CreateTeam";
import { Game } from "page/Game";
import { SelectTeam } from "page/SelectTeam";
import { Replay } from "page/Replay";
import { Admin } from "page/Admin";
import { Debug } from "page/Debug";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import LanguageDetector from "i18next-browser-languagedetector";

import "style/main.css";

import en_translation from "i18n/en.json";
import de_translation from "i18n/de.json";
import { THEMES, ThemeName, applyTheme, loadTheme } from "lib/theme";

i18n.use(LanguageDetector)
    .use(initReactI18next)
    .init({
        debug: true,
        interpolation: {
            escapeValue: false, // not needed for react!!
        },
        fallbackLng: "en",
        resources: {
            en: { translation: en_translation },
            de: { translation: de_translation },
        },
    })
    .catch(
        (e: unknown) =>
            e instanceof Error && console.error("i18n init error", e.message),
    );

applyTheme(loadTheme() ?? Object.keys(THEMES)[0] as ThemeName);

const rootElement = document.getElementById("root") as HTMLElement;
ReactDOM.createRoot(rootElement).render(
    <BrowserRouter>
        <Routes>
            <Route path="/" element={<SelectTeam />} />
            <Route path="/create" element={<CreateTeam />} />
            <Route path="/game" element={<Game />} />
            <Route path="/replay" element={<Replay />} />
            <Route path="/admin" element={<Admin />} />
            {import.meta.env.DEV && <Route path="/debug" element={<Debug />} />}
        </Routes>
    </BrowserRouter>,
);
