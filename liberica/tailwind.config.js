/** @type {import('tailwindcss').Config} */
export default {
    content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
    theme: {
        colors: {
            base: "hsl(var(--color-base) / <alpha-value>)",
            surface: "hsl(var(--color-surface) / <alpha-value>)",
            muted: "hsl(var(--color-muted) / <alpha-value>)",
            primary: "hsl(var(--color-primary) / <alpha-value>)",
            secondary: "hsl(var(--color-secondary) / <alpha-value>)",
            on: {
                base: "hsl(var(--color-on-base) / <alpha-value>)",
                surface: "hsl(var(--color-on-surface) / <alpha-value>)",
                muted: "hsl(var(--color-on-muted) / <alpha-value>)",
                primary: "hsl(var(--color-on-primary) / <alpha-value>)",
                secondary: "hsl(var(--color-on-secondary) / <alpha-value>)",
            }
        },
    },
    plugins: [],
};
