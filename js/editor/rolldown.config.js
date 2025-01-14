import { defineConfig } from "rolldown";
import { minify } from "rollup-plugin-esbuild";

export default defineConfig({
  input: "index.ts",
  output: {
    file: "dist/index.js",
  },
  plugins: [/** @type {any} */ (minify())],
});
