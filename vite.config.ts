import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import unocssPlugin from "unocss/vite"

export default defineConfig(async () => ({
    plugins: [solidPlugin(), unocssPlugin()],
    clearScreen: false,
    server: {
        host: "127.0.0.1",
        port: 10279,
    },
}));
