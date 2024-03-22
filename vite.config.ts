import { defineConfig } from "vitest/config";
// import { svelte } from "@sveltejs/vite-plugin-svelte";
import { sveltekit } from "@sveltejs/kit/vite";

export default defineConfig({
  plugins: [
    sveltekit(),
    // svelte({
    //   compilerOptions: {
    //     customElement: true,
    //   },
    // }),
  ],
  test: {
    include: ["src/**/*.{test,spec}.{js,ts}"],
  },
});
