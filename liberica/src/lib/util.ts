export const extractPos = (item: {
    lat: number;
    long: number;
}): [number, number] => {
    return [item.lat, item.long];
};

export const opt = (bool: boolean | (() => boolean), value: string) =>
    (typeof bool === "function" ? bool() : bool) && value;

export const getContrastingTextColor = (bg: string) => {
    const hex = bg.replace(/^#/, "");
    const r = parseInt(hex.slice(0, 2), 16);
    const g = parseInt(hex.slice(2, 4), 16);
    const b = parseInt(hex.slice(4, 6), 16);
    const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;

    return luminance > 0.5 ? "#000000" : "#FFFFFF";
};

export const clamp = (x: number, min: number, max: number) => {
  return Math.min(Math.max(x, min), max)
};
