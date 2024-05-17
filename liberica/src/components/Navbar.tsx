import { PropsWithChildren } from "react";

export function Navbar(props: PropsWithChildren) {
    return (
        <div
            className="absolute bottom-0 flex w-max gap-3 bg-white p-2"
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
