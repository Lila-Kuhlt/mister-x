import { PropsWithChildren } from "react";
import { FaHome } from "react-icons/fa";
import { useNavigate } from "react-router-dom";
import { Button } from "components/lila/button";

export function Navbar(props: PropsWithChildren) {
    return (
        <div className="fixed bottom-0 z-auto flex w-dvw items-center justify-between gap-3 bg-base p-2">
            {props.children}
        </div>
    );
}

export function HomeButton() {
    const navigate = useNavigate();

    return (
        <Button onClick={() => navigate("/")} variant="primary" size="lg">
            <FaHome />
        </Button>
    );
}
