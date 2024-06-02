/** @type {import('tailwindcss').Config} */
export default {
    content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
    theme: {
        colors: {
            "surface": "hsl(var(--color-surface) / <alpha-value>)",
            "overlay": "hsl(var(--color-overlay) / <alpha-value>)",
            "muted": "hsl(var(--color-muted) / <alpha-value>)",
            "text": "hsl(var(--color-text) / <alpha-value>)",
            "contrast": "hsl(var(--color-surface) / <alpha-value>)",
            "primary": "hsl(var(--color-primary) / <alpha-value>)",
            "secondary": "hsl(var(--color-secondary) / <alpha-value>)",
            "accent": "hsl(var(--color-accent) / <alpha-value>)"
        },
    },
    plugins: [],
};
