import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

const packageVersion = process.env.npm_package_version;

// https://vite.dev/config/
export default defineConfig({
  envDir: "../",
  server: {
    host: "0.0.0.0",
    port: 8788,
  },
  define: {
    "import.meta.env.PACKAGE_VERSION": JSON.stringify(packageVersion),
  },
  plugins: [react()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
});
