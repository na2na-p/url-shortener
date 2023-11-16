import react from "@vitejs/plugin-react-swc";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  define: {
    "import.meta.vitest": "undefined",
  },
  server: {
    host: "0.0.0.0",
    port: 3000,
  },
  test: {
    globals: true,
    environment: "jsdom",
    setupFiles: "setup-vitest.ts",
    /**
     * for in-source testing.
     * {@link https://vitest.dev/guide/in-source.html}
     */
    includeSource: ["{src,integrationTest,docs}/**/*.{ts,tsx}"],
  },
  resolve: {
    alias: {
      "@/": `${__dirname}/src/`,
      "@root/": `${__dirname}/`,
    },
  },
});
