import { PropsWithChildren } from "react";
import { Button } from "./InputElements";
import { FaHome } from "react-icons/fa";
import { useNavigate } from "react-router-dom";

export function Navbar(props: PropsWithChildren) {
    return (
        <div
            className="absolute bottom-0 flex w-max gap-3 bg-surface p-2"
            style={{
                position: "fixed",
                justifyContent: "space-between",
                alignItems: "center",
                zIndex: 1000,
            }}
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
    );
}
