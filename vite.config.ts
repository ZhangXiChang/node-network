import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import unocss from "unocss/vite"

export default defineConfig(async () => ({
    plugins: [unocss(), solid()],
    clearScreen: false,
    server: {
        host: "127.0.0.1",
        port: 10279,
    },
}));
