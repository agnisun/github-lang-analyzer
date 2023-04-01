import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from 'vite-plugin-wasm';
import * as path from 'path'

export default defineConfig( {
    plugins: [react(), wasm()],
    server: {
        port: 3000,
        strictPort: true,
    },
    build: {
      sourcemap: false,
    },
    esbuild: {
        supported: {
            'top-level-await': true
        },
    },
    resolve: {
        alias: {
            '@shared': path.resolve(__dirname, 'src/shared/'),
            '@entities': path.resolve(__dirname, 'src/entities/'),
            '@features': path.resolve(__dirname, 'src/features/'),
            '@app': path.resolve(__dirname, 'src/app/'),
        }
    }
})