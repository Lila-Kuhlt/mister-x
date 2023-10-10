import { CreateTeam } from "page/CreateeTeam";
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
      </Routes>
    </BrowserRouter>
  </React.StrictMode>
);
