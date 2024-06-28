import { defineConfig } from "vite";
import solid from "vite-plugin-solid";

export default defineConfig(async () => ({
    plugins: [solid()],
    clearScreen: false,
    server: {
        host: "127.0.0.1",
        port: 10279,
    },
}));
