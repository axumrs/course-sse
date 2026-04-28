import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  server: {
    proxy: {
      "/sse": "http://172.29.101.146:9527",
      "/api": "http://172.29.101.146:9527",
    },
  },
});
