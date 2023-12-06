import { PropsWithChildren } from "react";

export function Navbar(props: PropsWithChildren) {
    return (
      <div
        className="position-absolute bottom-0 w-max d-flex bg-white p-2 justify-content-between align-items-center gap-3"
        style={{ zIndex: 10000 }}
      >
        {props.children}
      </div>
    );
  }
