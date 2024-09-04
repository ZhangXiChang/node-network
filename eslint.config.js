import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import unocss from "@unocss/eslint-config/flat";

export default [
    { files: ["**/*.{js,mjs,cjs,ts,tsx}"] },
    { languageOptions: { globals: globals.browser } },
    pluginJs.configs.recommended,
    ...tseslint.configs.recommended,
    unocss,
    {
        rules: {
            "semi": ["error", "always"],
            "comma-dangle": [
                "error",
                "always-multiline",
            ],
            "quotes": ["error", "double"],
        },
    },
];
