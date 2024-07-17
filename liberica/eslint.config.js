import globals from "globals";
import eslintJs from "@eslint/js";
import tseslint from "typescript-eslint";

/* import eslintPluginReact from "eslint-plugin-react";
 * import eslintPluginReactHooks from "eslint-plugin-react-hooks";
 * import eslintPluginReactRefresh from "eslint-plugin-react-refresh";
*/ import eslintPlugini18next from "eslint-plugin-i18next";

// Prettier
import eslintPluginPrettier from "eslint-plugin-prettier/recommended";
import eslintConfigPrettier from "eslint-config-prettier";

/* Currently some React related plugins are still not supported:
 *
 * - eslint-plugin-react
 * - eslint-plugin-react-hooks
 * - eslint-plugin-react-refresh
 * - eslint-plutin-i18next
 *
 * For more information see the corresponding PRs:
 * - https://github.com/jsx-eslint/eslint-plugin-react/pull/3743
 * - https://github.com/edvardchen/eslint-plugin-i18next/pull/126
 * - https://github.com/facebook/react/pull/29770
 */

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

    eslintJs.configs.recommended,

    // eslintPluginReact.configs.recommended,
    // eslintPlugini18next.configs.recommended,

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
