import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Tauri の開発サーバ用に固定ポートを使う
export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: { port: 1420, strictPort: true },
});
