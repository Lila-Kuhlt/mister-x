import { PropsWithChildren } from 'react';

export function Navbar(props: PropsWithChildren) {
    return (
        <div
            className="absolute bottom-0 w-max flex bg-white p-2 gap-3"
            style={{ position: 'fixed', justifyContent: 'space-between', alignItems: 'center', zIndex: 1000 }}
        >
            {props.children}
        </div>
    );
}
