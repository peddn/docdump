import { fileURLToPath, URL } from 'node:url'

import tailwindcss from '@tailwindcss/vite'
import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'

// Adresse des Rust-Servers im Entwicklungsbetrieb. Standard wie `listen` in der
// Beispielkonfiguration; abweichend über die Umgebungsvariable BACKEND_URL.
const backend = process.env.BACKEND_URL ?? 'http://127.0.0.1:8080'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue(), tailwindcss()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  server: {
    // API und rohe Dateien liefert der Rust-Server, alles andere Vite (Konzept 10.1).
    proxy: {
      '/api': backend,
      '/raw': backend,
    },
  },
})
