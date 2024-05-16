export const TextInput = ({
    className,
    onTextChange,
    trim,
    ...props
}: React.InputHTMLAttributes<HTMLInputElement> & {
    onTextChange: (text: string) => void;
    trim?: 'start' | 'end' | 'all';
}) => {
    return (
        <input
            type="text"
            className={`block w-full rounded-md border border-slate-300 bg-white px-3 py-2 placeholder-slate-400 shadow-sm focus:border-purple-500 focus:outline-none focus:ring-1 focus:ring-purple-500 sm:text-sm ${className ?? ''}`}
            placeholder="Lila Pause"
            onChange={(e) => {
                const fn = {
                    end: String.prototype.trimEnd,
                    start: String.prototype.trimStart,
                    all: String.prototype.trim,
                };
                const value = trim && fn[trim].call(e.target.value);
                onTextChange(value || e.target.value);
            }}
            {...props}
        />
    );
};

export function DropDown<T extends string>({
    className,
    items,
    onItemChange,
    ...props
}: React.SelectHTMLAttributes<HTMLSelectElement> & {
    items: T[];
    onItemChange?: (item: T) => void;
}) {
    return (
        <select
            className={`block w-full rounded-md border border-slate-300 bg-white px-3 py-2 placeholder-slate-400 shadow-sm focus:border-purple-500 focus:outline-none focus:ring-1 focus:ring-purple-500 sm:text-sm ${className ?? ''}`}
            onChange={(item) =>
                onItemChange?.(items[item.currentTarget.selectedIndex])
            }
            {...props}>
            {items.map((item) => (
                <option key={item}>{item}</option>
            ))}
        </select>
    );
}

export const Button = ({
    className,
    ...props
}: React.PropsWithChildren<React.ButtonHTMLAttributes<HTMLButtonElement>>) => {
    return (
        <button
            className={`middle none center flex justify-center rounded-lg bg-purple-500 px-6 py-2 font-sans text-sm font-bold text-white shadow-md shadow-pink-500/20 transition-all hover:shadow-lg hover:shadow-pink-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none ${className ?? ''}`}
            {...props}>
            {props.children}
        </button>
    );
};
