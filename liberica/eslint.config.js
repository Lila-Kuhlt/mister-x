import globals from "globals";
import eslintJs from "@eslint/js";
import tseslint from "typescript-eslint";

import eslintPluginReactHooks from "eslint-plugin-react-hooks";
import eslintPluginReact from "eslint-plugin-react";
import eslintPlugini18next from "eslint-plugin-i18next";
import eslintPluginReactRefesh from "eslint-plugin-react-refresh";

// Prettier
import eslintPluginPrettier from "eslint-plugin-prettier/recommended";
import eslintConfigPrettier from "eslint-config-prettier";

function flattenConfig(plugin, name, overrides = {}) {
    return {
        plugins: { [name]: plugin },
        rules: {
            ...plugin.configs?.recommended?.rules,
            ...overrides,
        },
    };
}

const reactPlugin = {
    ...flattenConfig(eslintPluginReact, "react", {
        "react/react-in-jsx-scope": "off",
    }),
    settings: {
        react: { version: "detect" },
    },
};

const reactRefreshPlugin = flattenConfig(
    eslintPluginReactRefesh,
    "react-refresh",
    {
        "react-refresh/only-export-components": "warn",
    },
);

const reactHooksPlugin = flattenConfig(eslintPluginReactHooks, "react-hooks");
const i18Plugin = flattenConfig(eslintPlugini18next, "i18next");

/** @type {import("eslint").Config} */
export default [
    {
        ignores: [
            "src/lib/bindings.ts",
            "node_modules",
            "dist",
            "*.config.js",
            "*.config.ts",
        ],
    },
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
    reactRefreshPlugin,

    eslintPluginPrettier,
    eslintConfigPrettier, // disables some rules that cause conflicts

    ...tseslint.configs.strictTypeChecked, // strict is a superset of recommended
    ...tseslint.configs.stylistic,

    {
        rules: {
            "@typescript-eslint/no-confusing-void-expression": "off",
            "@typescript-eslint/restrict-template-expressions": ["warn", { "allowNumber": true }]
        }
    }
];
