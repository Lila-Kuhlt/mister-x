import { CreateTeam } from "page/CreateTeam";
import { Game } from "page/Game";
import { Home } from "page/Home";
import { Replay } from "page/Replay";
import { Admin } from "page/Admin";
import { Gadgets } from "page/Gadgets";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import "style/main.css";
import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import LanguageDetector from "i18next-browser-languagedetector";

import en_translation from "i18n/en.json";
import de_translation from "i18n/de.json";

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

const rootElement = document.getElementById("root") as HTMLElement;
ReactDOM.createRoot(rootElement).render(
    <BrowserRouter>
        <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/create" element={<CreateTeam />} />
            <Route path="/game" element={<Game />} />
            <Route path="/replay" element={<Replay />} />
            <Route path="/admin" element={<Admin />} />
            <Route path="/gadgets" element={<Gadgets />} />
        </Routes>
    </BrowserRouter>,
);
