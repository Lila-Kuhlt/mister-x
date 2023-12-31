import L from "leaflet";

export const extractPos = (item: {
  lat: number;
  long: number;
}): [number, number] => {
  return [item.lat, item.long];
};

export const VIEW_BOUNDS: L.LatLngBounds = new L.LatLngBounds(
  [49.0129685, 8.3782551],
  [48.9906205, 8.4203851]
);

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
