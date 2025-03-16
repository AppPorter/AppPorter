import { PrimeVueResolver } from '@primevue/auto-import-resolver'
import vue from '@vitejs/plugin-vue'
import autoprefixer from 'autoprefixer'
import { fileURLToPath, URL } from 'node:url'
import tailwind from 'tailwindcss'
import Components from 'unplugin-vue-components/vite'
import { defineConfig } from 'vite'
import vueDevTools from 'vite-plugin-vue-devtools'

const host = process.env.TAURI_DEV_HOST

export default defineConfig(async () => ({
  css: {
    postcss: {
      plugins: [tailwind(), autoprefixer()],
    },
  },
  plugins: [
    vue(),
    vueDevTools(),
    Components({
      resolvers: [PrimeVueResolver()],
    }),
  ],
  build: {
    target: 'esnext',
    rollupOptions: {
      output: {
        // Split chunks by node_modules to optimize cache and loading performance
        manualChunks(id: string) {
          if (id.indexOf('node_modules') !== -1) {
            const basic = id.toString().split('node_modules/')[1]
            const sub1 = basic.split('/')[0]
            if (sub1 !== '.pnpm') {
              return sub1.toString()
            }
            const name2 = basic.split('/')[1]
            return name2.split('@')[name2[0] === '@' ? 1 : 0].toString()
          }
        },
      },
    },
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },

  // Configuration specific to Tauri development

  // Disable clearing console to preserve Rust error messages during development
  clearScreen: false,

  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    // Configure Hot Module Replacement (HMR) for development
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // Exclude Rust backend code from file watching
      ignored: ['**/src-tauri/**'],
    },
  },
}))
