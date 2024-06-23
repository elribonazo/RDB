/**
 * @packageDocumentation
 * ## What is RIDB
 * RIDB is a package that can be used in browsers and nodejs in order to build databases.
 * The project started after years of experience working with web projects in both browser and nodejs platforms, the project was born with some rules / objectives:
 * 1. Strong types + proper validation 
 * 2. Declarative schemas & documents
 * 3. Configurable storages, inMemory, monogoDB, sqlite, indexdb
 * 4. Secure encryption
 * 5. Work seemlessly in browsers or nodejs applications.
 *
 * ## How to build this project
 * Build requirements:
 * * Bash
 * * Have Rust ([cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed.
 * * Node JS Version (20/LTS Recommended)
 *
 * ```bash
 * cd ts 
 * npm i
 * npm run build
 * ```
 *
 * ## How to test the project
 * For now have enabled the implementation of the whole wasm + javascript integration.
 * In order to run it, write the following:
 *
 * ```bash
 * cd ts 
 * npm i
 * npm run test
 * ```
 * 
 */

import wasmBuffer from "../../pkg/ridb_rust_bg.wasm"
import type * as RIDBTypes from "ridb-rust";
export type * as RIDBTypes from "ridb-rust";

let internal: typeof import("ridb-rust") | undefined;
let db: RIDBTypes.Database<RIDBTypes.SchemaTypeRecord> | undefined;
export { OpType, BaseStorage } from 'ridb-rust';

export class RIDB<T extends RIDBTypes.SchemaTypeRecord> {
    private schemas: T;
    constructor(schemas: T) {
        this.schemas = schemas;
    }

    private get db() {
        if (!db) {
            throw new Error("Start the database first");
        }
        return db;
    }

    get collections() {
        return this.db.collections;
    }

    private async load(): Promise<typeof import("ridb-rust")> {
        internal ??= await import("ridb-rust").then(async module => {
            const wasmInstance = module.initSync(wasmBuffer);
            await module.default(wasmInstance);
            return module;
        });
        return internal!;
    }

    async start(storageType?: typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>) {
        const self = this;
        db ||= await (await this.load()).Database.create(this.schemas, {
            createStorage: (schemas) => self.createStorage(schemas, storageType)
        });
        return db;
    }

    private async createStorage<J extends RIDBTypes.SchemaTypeRecord>(schemas: J, storageConstructor?: typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>) {
        const Storage = storageConstructor ?? ((await this.load()).InMemory as typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>);
        return Object.keys(schemas).reduce((storages, name) => ({
            ...storages,
            [name]: new Storage(name, schemas[name])
        }), {} as RIDBTypes.InternalsRecord);
    }
}

export const SchemaFieldType = {
    "string": 'string' as const,
    "number": 'number' as const,
    "boolean": 'boolean' as const,
    "array": 'array' as const,
    "object": 'object' as const,
};
