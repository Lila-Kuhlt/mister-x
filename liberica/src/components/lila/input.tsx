export type TextInputProps = Omit<
    React.ComponentProps<"input">,
    "className" | "type"
>;

export function TextInput(props: TextInputProps) {
    return (
        <input
            {...props}
            type="text"
            className="text-text rounded-xl bg-muted/20 px-6 py-3 outline-none ring-muted focus:ring-2"
        >
            {props.children}
        </input>
    );
}
