import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import suidPlugin from "@suid/vite-plugin"

export default defineConfig(async () => ({
    plugins: [solidPlugin(), suidPlugin()],
    clearScreen: false,
    server: {
        host: "127.0.0.1",
        port: 10279,
    },
}));
