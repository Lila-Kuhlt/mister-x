import { Select } from "./lila/select";

export const TextInput = ({
    className,
    onTextChange,
    trim,
    ...props
}: React.InputHTMLAttributes<HTMLInputElement> & {
    onTextChange: (text: string) => void;
    trim?: "start" | "end" | "all";
}) => {
    return (
        <input
            type="text"
            className={`border-slate-300 bg-white placeholder-slate-400 focus:border-purple-500 focus:ring-purple-500 block w-full rounded-md border px-3 py-2 shadow-sm focus:outline-none focus:ring-1 sm:text-sm ${className ?? ""}`}
            placeholder="Lila Pause"
            onChange={(e) => {
                const fn = {
                    end: (s: string) => s.trimEnd(),
                    start: (s: string) => s.trimStart(),
                    all: (s: string) => s.trim(),
                };

                const value = trim && fn[trim](e.target.value);
                onTextChange(value || e.target.value);
            }}
            {...props}
        />
    );
};

export function DropDown<T extends string>({
    items,
    onItemChange,
    ...props
}: React.ComponentProps<"select"> & {
    items: T[];
    onItemChange?: (item: T) => void;
}) {
    return (
        <Select
            onChange={(item) =>
                onItemChange?.(items[item.currentTarget.selectedIndex])
            }
            {...props}
        >
            {items.map((item) => (
                <option key={item}>{item}</option>
            ))}
        </Select>
    );
}
