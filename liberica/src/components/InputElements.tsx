export const TextInput = (
  props:
    | React.InputHTMLAttributes<HTMLInputElement> & {
        onTextChange: (text: string) => void;
        trim?: "start" | "end" | "all";
      }
) => {
  return (
    <input
      {...props}
      type="text"
      className="block w-full px-3 py-2 bg-white border rounded-md shadow-sm border-slate-300 placeholder-slate-400 focus:outline-none focus:border-purple-500 focus:ring-purple-500 sm:text-sm focus:ring-1"
      placeholder="Lila Pause"
      onChange={(e) => {
        const fn = {
          end: String.prototype.trimEnd,
          start: String.prototype.trimStart,
          all: String.prototype.trim,
        };
        const value = props.trim && fn[props.trim].call(e.target.value);
        props.onTextChange(value || e.target.value);
      }}
    />
  );
};

export function DropDown<T extends string>(
  props: React.SelectHTMLAttributes<HTMLSelectElement> & {
    items: T[];
    onItemChange?: (item: T) => void;
    slected?: number | T;
  }
) {
  const isSelcetd = (item: string, index: number) =>
    (typeof props.slected === "string" && item === props.slected) ||
    (typeof props.slected === "number" && index === props.slected) ||
    false;

  return (
    <select
      className="block w-full px-3 py-2 bg-white border rounded-md shadow-sm border-slate-300 placeholder-slate-400 focus:outline-none focus:border-purple-500 focus:ring-purple-500 sm:text-sm focus:ring-1"
      onChange={(item) =>
        props.onItemChange?.(props.items[item.currentTarget.selectedIndex])
      }
    >
      {props.items.map((item, index) => (
        <option key={item} selected={isSelcetd(item, index)}>
          {item}
        </option>
      ))}
    </select>
  );
}

export const Button = (
  { className, ...props }: React.PropsWithChildren<React.ButtonHTMLAttributes<HTMLButtonElement>>
) => {
  return (
    <button
      className={`middle none center rounded-lg bg-purple-500 text-white shadow-md shadow-pink-500/20 py-2 px-6 font-sans text-sm font-bold transition-all hover:shadow-lg hover:shadow-pink-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none flex justify-center ${className}`}
      {...props}
    >
      {props.children}
    </button>
  );
};
