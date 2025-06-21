import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  // Tauri expects a fixed port and no HMR overlay for best compatibility
  server: {
    port: 1420,
    strictPort: true,
    hmr: {
      overlay: false,
    },
  },
  // Prevent Vite from obscuring Rust errors
  clearScreen: false,
  // Tauri uses environment variables differently than typical web apps
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Tauri needs this for proper production builds
    target: 'esnext',
    // Don't minify for better debug information in development
    minify: !process.env.TAURI_DEBUG,
    // Produce sourcemaps for development builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
})
