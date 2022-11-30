import { defineConfig } from 'vite';

export default defineConfig({
  server: {
    fs: {
      // allow: ['snake-wasm', 'main.js'],
      strict: false,
    }
  }
})