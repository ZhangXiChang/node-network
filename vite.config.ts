import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import unocssPlugin from "unocss/vite";
import tsconfigPathsPlugin from "vite-tsconfig-paths";

export default defineConfig({
    plugins: [
        solidPlugin(),
        unocssPlugin(),
        tsconfigPathsPlugin({ root: "./" }),
    ],
    clearScreen: false,
});
