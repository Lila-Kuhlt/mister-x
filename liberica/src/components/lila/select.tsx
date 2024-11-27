import { classes } from "components/lila";

export type SelectProps = Omit<
    React.ComponentProps<"select">,
    "className" | "size"
> &
    SelectPropsExt;
export interface SelectPropsExt {
    size?: keyof typeof INPUT_SIZES;
    noLogo?: boolean;
}

const BASE =
    "text-on-muted rounded-xl bg-muted/20 outline-none ring-muted focus:ring-2";

const INPUT_SIZES = {
    lg: "px-6 py-3 text-md",
};

export function Select(props: SelectProps) {
    return (
        <select className={classes(BASE, INPUT_SIZES[props.size ?? "lg"])}>
            {props.children}
        </select>
    );
}
