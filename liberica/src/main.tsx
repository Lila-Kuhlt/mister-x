import { CreateTeam } from "page/CreateTeam";
import { Game } from "page/Game";
import { Home } from "page/Home";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import "style/main.css";
import { Admin } from "page/Admin";
import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import LanguageDetector from "i18next-browser-languagedetector";

import en_translation from "i18n/en.json";
import de_translation from "i18n/de.json";

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    debug: true,
    interpolation: {
      escapeValue: false, // not needed for react!!
    },
    fallbackLng: "en",
    resources: {
      en: {
        translation: en_translation,
      },
      de: {
        translation: de_translation,
      },
    },
  });

ReactDOM.createRoot(document.getElementById("root")!).render(
  <BrowserRouter>
    <Routes>
      <Route path="/" element={<Home />} />
      <Route path="/create" element={<CreateTeam />} />
      <Route path="/game" element={<Game />} />
      <Route path="/admin" element={<Admin />} />
    </Routes>
  </BrowserRouter>
);
