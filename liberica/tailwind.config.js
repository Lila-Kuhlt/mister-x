function gen(name) {
    const shades = [...new Array(9)]
        .map((_, i) => i)
        .map(i => 100 * i + 100)
        .map(i => ({ [i]: `var(--${name}-${i})` }))
        .reduce((prev, curr) => ({ ...prev, ...curr }));

    return {
        ...shades,
        DEFAULT: `var(--${name}-500)`
    };
}

/** @type {import('tailwindcss').Config} */
export default {
    content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
    theme: {
        extend: {
            colors: {
                'text': {
                    light: gen('text'),
                    dark: gen('text-dark'),
                    DEFAULT: gen('text')
                },
                'surface': gen('surface'),
                'primary': gen('primary'),
                'secondary': gen('secondary'),
                'accent': gen('accent'),
            },
        }
    },
    plugins: [],
};
