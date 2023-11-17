import { HTMLAttributes } from "react";

export function ColorSwatch(
  props: { color: string; size?: string } & HTMLAttributes<HTMLDivElement>
) {
  const color =
    (props.color.startsWith("#") && `[${props.color}]`) || props.color;
  return <div className={`w-10 h-10 bg-${color} rounded ` + props.className} />;
}
