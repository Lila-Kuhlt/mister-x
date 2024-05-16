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
            className={`block w-full px-3 py-2 bg-white border rounded-md shadow-sm border-slate-300 placeholder-slate-400 focus:outline-none focus:border-purple-500 focus:ring-purple-500 sm:text-sm focus:ring-1 ${className ?? ''}`}
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
            className={`block w-full px-3 py-2 bg-white border rounded-md shadow-sm border-slate-300 placeholder-slate-400 focus:outline-none focus:border-purple-500 focus:ring-purple-500 sm:text-sm focus:ring-1 ${className ?? ''}`}
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
            className={`middle none center rounded-lg bg-purple-500 text-white shadow-md shadow-pink-500/20 py-2 px-6 font-sans text-sm font-bold transition-all hover:shadow-lg hover:shadow-pink-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none flex justify-center ${className ?? ''}`}
            {...props}>
            {props.children}
        </button>
    );
};
