import { CreateTeam } from "page/CreateTeam";
import { Game } from "page/Game";
import  Map  from "page/Map";
import { Home } from "page/Home";
import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router-dom";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/create-team" element={<CreateTeam />} />
        <Route path="/game" element={<Game />} />
        <Route path="/map" element={<Map />} />
      </Routes>
    </BrowserRouter>
  </React.StrictMode>
);
