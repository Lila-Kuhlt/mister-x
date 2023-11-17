/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {},
  },
  plugins: [],
  safelist: [
    "bg-red-500",
    "bg-pink-500",
    "bg-lime-500",
    "bg-cyan-500",
    "bg-purple-500",
  ],
};
