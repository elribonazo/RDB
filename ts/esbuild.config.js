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




esbuild.build({
    entryPoints: ['src/index.ts'],
    outfile: 'cjs/index.js',
    bundle: true,
    platform: 'neutral',
    target: ['node14'],
    sourcemap: true,
    format: 'cjs',
    plugins: [
        wasmPlugin,
        ...plugins
    ],
    resolveExtensions: ['.ts', '.js', '.wasm'],
    inject:['ridb-rust'],
    mainFields: ['module', 'main'],
}).catch((err) => {

    console.log(err)
    process.exit(1)
});



// Build ES module
esbuild.build({
    entryPoints: ['src/index.ts'],
    outfile: 'esm/index.js',
    bundle: true,
    platform: 'neutral',
    target: ['esnext'],
    sourcemap: true,
    format: 'esm',
    plugins: [
        wasmLoader(),
        ...plugins
    ],

    resolveExtensions: ['.ts', '.js', '.wasm'],
    mainFields: ['module', 'main'],
    inject:['ridb-rust']
}).catch((err) => {
    console.log(err)
    process.exit(1)
});



// Build browser version
esbuild.build({
    entryPoints: ['src/index.ts'],
    outfile: 'umd/index.js',
    bundle: true,
    platform: 'browser',
    target: ['es2020'],
    sourcemap: true,
    format: 'iife',
    globalName:"RIDB",
    plugins: [
        wasmPlugin,
        ...plugins
    ],
    resolveExtensions: ['.ts', '.js', '.wasm'],
    mainFields: ['browser', 'module', 'main'],
    inject:['ridb-rust'],

}).catch((err) => {

    console.log(err)
    process.exit(1)
});

