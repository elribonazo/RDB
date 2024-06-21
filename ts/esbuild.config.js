const esbuild = require('esbuild');
const fs = require('fs');
const path = require('path');
const { wasmLoader } = require('esbuild-plugin-wasm')
const {NodeResolvePlugin} = require('@esbuild-plugins/node-resolve');
const wasmPlugin = {
    name: 'wasm',
    setup(build) {
        build.onResolve({ filter: /\.wasm$/ }, args => {
            return { path: path.resolve(args.resolveDir, args.path), namespace: 'wasm' };
        });
        build.onLoad({ filter: /.*/, namespace: 'wasm' }, async (args) => {
            const buffer = await fs.promises.readFile(args.path);
            const base64 = buffer.toString('base64');
            return {
                contents: `export default Buffer.from("${base64}", "base64");`,
                loader: 'binary',
            };
        });
    },
};
const plugins = [
    NodeResolvePlugin({
        extensions: ['.ts', '.js', '.wasm'],
        onResolved: (resolved) => {
            if (resolved.includes('node_modules')) {
                return {
                    external: true,
                }
            }
            return resolved
        },
    }),
]

const generic = {
    chunkNames: "[name]",
    assetNames: "[name]",
    entryPoints: ['src/index.ts'],
    bundle: true,
    sourcemap: true,
    resolveExtensions: ['.ts', '.js', '.wasm'],
    inject:['ridb-rust'],
    mainFields: ['module', 'main'],
}

esbuild.build({
    ...generic,
    outdir: 'cjs',
    platform: 'neutral',
    target: ['node14'],
    format: 'cjs',
    plugins: [
        wasmPlugin,
        ...plugins
    ],
}).catch((err) => {
    console.log(err)
    process.exit(1)
});
// Build ES module
esbuild.build({
    ...generic,
    outdir: 'esm',
    splitting: true,
    platform: 'neutral',
    target: ['esnext'],
    format: 'esm',
    plugins: [
        wasmLoader(),
        ...plugins
    ],
}).catch((err) => {
    console.log(err)
    process.exit(1)
});
// Build browser version
esbuild.build({
    ...generic,
    outdir: 'umd',
    platform: 'browser',
    target: ['es2020'],
    format: 'iife',
    globalName:"RIDB",
    plugins: [
        wasmPlugin,
        ...plugins
    ],
    mainFields: ['browser', 'module', 'main'],


}).catch((err) => {
    console.log(err)
    process.exit(1)
});

