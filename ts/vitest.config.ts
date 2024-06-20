/// <reference types="vitest" />
import { defineConfig } from 'vite'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

const isCI = process.env.CI === "true";

export default defineConfig({
    plugins:[
        wasm(),
        topLevelAwait()
    ],

    build: {
        minify: 'terser',
        terserOptions: { format: { comments: 'all' } },
    },
    test: {
        reporters: ['verbose'],
        environment: "node",
        coverage: {
            provider: 'istanbul',
            reporter: isCI ? ['json-summary'] : ['json-summary', "html"],
            thresholds: {
                branches: 50,
                functions: 50,
                lines: 50,
                statements: 50
            },
            include: [
                'src/**/*',
                "pkg/**"
            ],
        },

    }
})
