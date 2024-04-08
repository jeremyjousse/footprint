import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),

  kit: {
    adapter: adapter(),
    alias: {
      $components: "src/components",
      $domain: "src/domain",
      $i18n: "src/i18n",
      $lib: "src/lib",
      $services: "src/services",
      $stores: "src/stores",
    },
  },
};

export default config;
