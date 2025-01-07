import { defineConfig } from "@pandacss/dev";
import { createPreset } from "@park-ui/panda-preset";
import neutral from "@park-ui/panda-preset/colors/neutral";

export default defineConfig({
    include: ["./ui/**/*.{js,jsx,ts,tsx}"],
    outdir: "styled-system",
    preflight: true,
    jsxFramework: "solid",
    presets: [
        createPreset({
            accentColor: neutral,
            grayColor: neutral,
            radius: "sm",
        }),
    ],
});
