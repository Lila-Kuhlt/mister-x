const BUTTON_BASE =
    "select-none transition-all font-sans disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none ";

export const BUTTON_VARIANTS = {
    filled: BUTTON_BASE + "bg-secondary text-contrast rounded-xl font-bold",
    tonal:
        BUTTON_BASE +
        "bg-muted/10 text-contrast rounded-xl font-bold hover:bg-muted/20 animate-entry text-text",
    text: BUTTON_BASE + " py-0 px-0 font-bold uppercase text-primary",
};

export const BUTTON_SIZES = {
    sm: "py-1 px-4 text-xs",
    md: "py-2 px-5 text-xs",
    lg: "py-2.5 px-7 text-sm",
    wide: "w-full py-1 text-xs",
};

export type ButtonVariant = keyof typeof BUTTON_VARIANTS;
export type ButtonSize = keyof typeof BUTTON_SIZES;

export interface ButtonPropsExt {
    variant: ButtonVariant;
    size?: ButtonSize;
}

export type ButtonProps = Omit<React.ComponentProps<"button">, "className"> &
    ButtonPropsExt;

export function BaseButton(props: ButtonProps) {
    return (
        <button
            className={
                BUTTON_SIZES[props.size ?? "md"] +
                " " +
                BUTTON_VARIANTS[props.variant]
            }
            {...props}
        >
            {props.children}
        </button>
    );
}
