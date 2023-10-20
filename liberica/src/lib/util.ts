export const opt = (bool: boolean | (() => boolean), value: string) =>
  (typeof bool === "function" ? bool() : bool) && value;
