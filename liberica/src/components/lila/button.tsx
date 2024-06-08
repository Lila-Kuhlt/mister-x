import { classes } from "components/lila";

const BASE =
    "select-none transition-all font-sans disabled:pointer-events-none active:opacity-[0.85] disabled:opacity-50 disabled:shadow-none ";

export const BUTTON_VARIANTS = {
    primary:
        "bg-primary text-on-primary rounded-xl font-bold hover:bg-primary/90 animate-entry",
    secondary:
        "bg-secondary text-on-secondary rounded-xl font-bold hover:bg-secondary/90 animate-entry",
    muted: "bg-muted/10 text-on-muted rounded-xl font-bold hover:bg-muted/20 animate-entry",
};

export const BUTTON_SIZES = {
    sm: "py-1 px-4 text-xs",
    md: "py-2 px-5 text-xs",
    lg: "py-2.5 px-7 text-sm",
    "sm-wide": "w-full py-1 text-sm",
    "md-wide": "w-full py-2 text-md",
};

export type ButtonVariant = keyof typeof BUTTON_VARIANTS;
export type ButtonSize = keyof typeof BUTTON_SIZES;

export interface ButtonPropsExt {
    variant: ButtonVariant;
    size?: ButtonSize;
}

export type ButtonProps = Omit<React.ComponentProps<"button">, "className"> &
    ButtonPropsExt;

export function Button(props: ButtonProps) {
    return (
        <button
            className={classes(
                BASE,
                BUTTON_VARIANTS[props.variant],
                BUTTON_SIZES[props.size ?? "sm"],
            )}
            {...props}
        >
            {props.children}
        </button>
    );
}
