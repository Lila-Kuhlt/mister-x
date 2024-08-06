import globals from "globals";
import eslintJs from "@eslint/js";
import tseslint from "typescript-eslint";

import eslintPluginReactHooks from "eslint-plugin-react-hooks";
import eslintPluginReact from "eslint-plugin-react";
import eslintPlugini18next from "eslint-plugin-i18next";

// Prettier
import eslintPluginPrettier from "eslint-plugin-prettier/recommended";
import eslintConfigPrettier from "eslint-config-prettier";

const reactPlugin = {
    plugins: {
        react: eslintPluginReact,
    },
    rules: eslintPluginReact.configs.recommended.rules,
    settings: {
        react: { version: "detect" },
    },
};

const reactHooksPlugin = {
    plugins: {
        "react-hooks": eslintPluginReactHooks,
    },
    rules: eslintPluginReactHooks.configs.recommended.rules,
};

const i18Plugin = {
    plugins: {
        i18next: eslintPlugini18next,
    },
    rules: eslintPlugini18next.configs.recommended.rules,
};

/** @type {import("eslint").Config} */
export default [
    { ignores: ["src/lib/bindings.ts", "node_modules", "dist"] },
    {
        languageOptions: {
            globals: globals.browser,
            parserOptions: {
                project: true,
                tsconfigRootDir: import.meta.dirname,
            },
        },
    },

    reactPlugin,
    reactHooksPlugin,
    i18Plugin,

    eslintPluginPrettier,
    eslintConfigPrettier, // disables some rules that cause conflicts

    ...tseslint.configs.strictTypeChecked, // strict is a superset of recommended
    ...tseslint.configs.stylistic,

    {
        rules: {
            "@typescript-eslint/no-confusing-void-expression": "off",
        },
    },
];
