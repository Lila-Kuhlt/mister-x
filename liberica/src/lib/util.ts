export const opt = (bool: boolean | (() => boolean), value: string) =>
  (typeof bool === "function" ? bool() : bool) && value;

export const getContrastingTextColor = (hexBackgroundColor: string) => {
  // Remove the '#' character if present
  hexBackgroundColor = hexBackgroundColor.replace(/^#/, "");

  // Convert the hex color to its RGB components
  const r = parseInt(hexBackgroundColor.slice(0, 2), 16);
  const g = parseInt(hexBackgroundColor.slice(2, 4), 16);
  const b = parseInt(hexBackgroundColor.slice(4, 6), 16);

  // Calculate the relative luminance of the background color
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;

  // Determine the text color based on luminance
  if (luminance > 0.5) {
    // Use black text on light backgrounds
    return "#000000";
  } else {
    // Use white text on dark backgrounds
    return "#FFFFFF";
  }
};

export const clamp = (x: number, min: number, max: number) => {
  return Math.min(Math.max(x, min), max)
}
