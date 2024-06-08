export type SelectProps = Omit<React.ComponentProps<"select">, "className">;

export function Select(props: SelectProps) {
    return (
        <select className="text-text rounded-xl bg-muted/20 px-6 py-3">
            {props.children}
        </select>
    );
}
