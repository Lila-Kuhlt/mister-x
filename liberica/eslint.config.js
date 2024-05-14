import globals from "globals";
import eslintJs from "@eslint/js";
import tseslint from "typescript-eslint";
import stylistic from "@stylistic/eslint-plugin"

/* Currently some React related plugins are still not supported:
 *
 * - eslint-plugin-react
 * - eslint-plugin-react-hooks
 * - eslint-plugin-react-refresh
 *
 * For more information see the corresponding PRs:
 * - https://github.com/jsx-eslint/eslint-plugin-react/pull/3743
 */

export default [
    { ignores: ["src/lib/bindings.ts", "node_modules", "dist"] },
    { languageOptions: { globals: globals.browser } },

    stylistic.configs.customize({
        indent: 4,
        quotes: 'single',
        semi: true,
        jsx: true,
    }),

    eslintJs.configs.recommended,

    ...tseslint.configs.strict, // strict is a superset of recommended
    ...tseslint.configs.stylistic,
];
