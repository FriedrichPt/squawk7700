import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig, loadEnv } from "vite";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");
  const target = env.VITE_API_PROXY ?? "http://127.0.0.1:8181";
  return {
    plugins: [sveltekit()],
    server: {
      proxy: {
        "/api": { target, changeOrigin: true },
      },
    },
  };
});
