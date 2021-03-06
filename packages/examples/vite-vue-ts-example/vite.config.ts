import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import importMetaEnv from "@import-meta-env/unplugin";
import createSharedViteConfig from "../shared-vite-config.mjs";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), importMetaEnv.vite({ example: ".env.example.public" })],
  ...createSharedViteConfig(),
});
