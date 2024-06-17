import { motion, useSpring } from "framer-motion";
import React, { useRef } from "react";
import { PropsWithChildren } from "react";

export function Navbar(props: PropsWithChildren) {
    return (
        <div className="fixed bottom-0 z-auto flex max-h-14 w-dvw items-center justify-between gap-3 rounded-t-2xl bg-base p-2">
            {props.children}
        </div>
    );
}

export function NavbarHeader(props: PropsWithChildren) {
    return <>{props.children}</>;
}

export function NavbarBody(props: PropsWithChildren) {
    return <>{props.children}</>;
}

export function NavbarDrawer(props: PropsWithChildren) {
    const y = useSpring(0, { bounce: 0, duration: window.innerHeight / 3 }); // 854 / 4 = 285ms
    const headerRef = useRef<HTMLDivElement>(null);
    const bodyRef = useRef<HTMLDivElement>(null);

    const body = React.Children.map(props.children, (child) => {
        if (!React.isValidElement(child)) return;
        if (child.type !== NavbarBody) return;
        return child;
    });

    const header = React.Children.map(props.children, (child) => {
        if (!React.isValidElement(child)) return;
        if (child.type !== NavbarHeader) return;
        return child;
    });

    const bodyHeight = bodyRef.current?.getBoundingClientRect().height ?? 0;
    const headerHeight = headerRef.current?.getBoundingClientRect().height ?? 0;

    return (
        <motion.div
            drag="y"
            style={{ y, maxHeight: headerHeight }}
            className={`fixed bottom-0 z-auto flex w-dvw flex-col rounded-t-2xl bg-base`}
            onDragEnd={(_, info) => {
                y.stop();
                if (info.velocity.y < -10) {
                    y.set(-bodyHeight);
                    return;
                }
                y.set(0);
            }}
        >
            <div ref={headerRef}>
                <div className="flex justify-center pt-2">
                    <hr className="center w-20 rounded-full border-2 border-on-base/20" />
                </div>
                <div className="flex flex-row gap-2 p-2">{header}</div>
            </div>
            <div className="flex flex-col" ref={bodyRef}>
                {body}
            </div>
        </motion.div>
    );
}
