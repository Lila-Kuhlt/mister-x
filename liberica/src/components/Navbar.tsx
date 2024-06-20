import { PropsWithChildren } from "react";

export function Navbar(props: PropsWithChildren) {
    return (
        <div className="fixed bottom-0 z-auto flex max-h-14 w-dvw items-center justify-between gap-3 rounded-t-2xl bg-base p-2">
            {props.children}
        </div>
    );
}
