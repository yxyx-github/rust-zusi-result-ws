import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'
import path from 'path'
import tailwindcss from '@tailwindcss/vite'
import svgLoader from 'vite-svg-loader'

// https://vite.dev/config/
export default defineConfig({
    plugins: [
        vue(),
        vueDevTools(),
        svgLoader(),
        tailwindcss(),
    ],
    resolve: {
        alias: {
            '@': fileURLToPath(new URL('./src', import.meta.url))
        },
    },
    base: './',
    build: {
        manifest: true,
        rollupOptions: {
            external: id => {
                const srcPath = path.resolve(__dirname, 'src')
                return path.normalize(id).startsWith(path.normalize(srcPath)) && /src\/assets\/icons\//.test(id)
            },
        },
    },
})
