import { PropsWithChildren } from "react";
import { Button } from "./InputElements";
import { FaHome } from "react-icons/fa";
import { useNavigate } from "react-router-dom";

export function Navbar(props: PropsWithChildren) {
  return (
    <div
      className="position-absolute bottom-0 w-max flex bg-white p-2 gap-3"
      style={{ justifyContent: "space-between", alignItems: "center" }}
    >
      {props.children}
    </div>
  );
}

export function HomeButton() {
  const navigate = useNavigate();

  return (
    <Button onClick={() => navigate("/")}>
      <FaHome />
    </Button>
  )
}
