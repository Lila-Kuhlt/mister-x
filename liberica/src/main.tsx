import { CreateTeam } from "page/CreateTeam";
import { Game } from "page/Game";
import { Home } from "page/Home";
import { Replay } from "page/Replay";
import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import "style/main.css";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-icons/font/bootstrap-icons.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/create" element={<CreateTeam />} />
        <Route path="/game" element={<Game />} />
        <Route path="/replay" element={<Replay />} />
      </Routes>
    </BrowserRouter>
  </React.StrictMode>
);
