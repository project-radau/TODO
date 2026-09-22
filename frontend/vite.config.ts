import { defineConfig } from "vite";

export default defineConfig({
  clearScreen: false,
  server: {
    host: true,
    port: 5173,
    strictPort: true,
    hmr: {
      protocol: "ws",
      host: "10.188.153.81",
      port: 5173,
    },
  },
});