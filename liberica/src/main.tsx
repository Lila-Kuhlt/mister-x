import { CreateTeam } from "page/CreateeTeam";
import { Home } from "page/Home";
import React from "react";
import ReactDOM from "react-dom/client";
import { Route, Routes } from "react-router-dom";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <Routes>
      <Route path="/" element={<Home />} />
      <Route path="/create-team" element={<CreateTeam />} />
    </Routes>
  </React.StrictMode>
);
