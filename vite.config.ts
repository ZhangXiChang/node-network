import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import unocssPlugin from "unocss/vite";

export default defineConfig({
    plugins: [solidPlugin(), unocssPlugin()],
    clearScreen: false,
});
