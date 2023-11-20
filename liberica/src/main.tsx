import { CreateTeam } from "page/CreateTeam";
import { Game } from "page/Game";
import { Home } from "page/Home";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import "style/main.css";
import { Admin } from "page/Admin";

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
