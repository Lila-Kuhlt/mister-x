import { PropsWithChildren } from "react";

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
