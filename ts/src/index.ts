/**
 * @packageDocumentation
 * 
 * ## What is RIDB
 * RIDB is a package that can be used in browsers and nodejs in order to build databases.
 * The project started after years of experience working with web projects in both browser and nodejs platforms, the project was born with some rules / objectives:
 * 1. Strong types + proper validation 
 * 2. Declarative schemas & documents
 * 3. Configurable storages, inMemory, monogoDB, sqlite, indexdb
 * 4. Secure encryption
 * 5. Work seamlessly in browsers or nodejs applications.
 *
 * ## Supported features for InMemory Storage
 * The inMemory storage is used by default and is currently supporting the following features:
 * 1. Create (OK)
 * 2. Fetch (WIP)
 * 3. FetchOne (WIP)
 * 4. Update (WIP)
 * 5. Remove (WIP)
 * 6. Clean (WIP)
 *
 * ## Install
 * 
 * npm: 
 * ``` 
 * npm i @elribonazo/ridb --save
 * ```
 * 
 * yarn:
 * 
 * ``` 
 * yarn add @elribonazo/ridb
 * ```
 * 
 * ## Usage
 * 
 * ### Database
 * 
 * In CommonJS Modules:  
 * 
 * ```javascript
 * const {
 *     RIDB,
 *     SchemaFieldType
 * } = require('@elribonazo/ridb');
 * 
 * (async () => {
 *     const db =  new RIDB({
 *         demo: {
 *             version: 0,
 *             primaryKey: 'id',
 *             type: SchemaFieldType.object,
 *             properties: {
 *                 id: {
 *                     type: SchemaFieldType.string,
 *                     maxLength: 60
 *                 }
 *             }
 *         }
 *     });
 *     console.log("Starting the database");
 *     await db.start();
 *     console.log("Ok :)");
 * })()
 * ```
 * 
 * In ES Modules, TypeScript, etc:
 * 
 * ```javascript
 * import {
 *     RIDB,
 *     SchemaFieldType
 * } from '@elribonazo/ridb';
 * 
 * (async () => {
 *     const db =  new RIDB({
 *         demo: {
 *             version: 0,
 *             primaryKey: 'id',
 *             type: SchemaFieldType.object,
 *             properties: {
 *                 id: {
 *                     type: SchemaFieldType.string,
 *                     maxLength: 60
 *                 }
 *             }
 *         }
 *     });
 *     console.log("Starting the database");
 *     await db.start();
 *     console.log("Ok :)");
 * })()
 * ```
 * 
 * ## How to run tests
 * 
 * ```bash
 * cd ts 
 * npm i
 * npm run test
 * ```
 * 
 * ## How to build this project
 * Build requirements:
 * * Bash
 * * Have Rust ([cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)) installed.
 * * Node JS Version (20/LTS Recommended)
 * 
 * ```bash
 * cd ts 
 * npm i
 * npm run build
 * ```
 * 
 * ## How to test the project
 * For now, we have enabled the implementation of the whole wasm + javascript integration.
 * In order to run it, write the following:
 * 
 * ```bash
 * cd ts 
 * npm i
 * npm run test
 * ```
 */
import wasmBuffer from "../../pkg/ridb_rust_bg.wasm"
import type * as RIDBTypes from "ridb-rust";
export type * as RIDBTypes from "ridb-rust";

export { OpType, BaseStorage } from 'ridb-rust';

export class RIDB<T extends RIDBTypes.SchemaTypeRecord> {
    private schemas: T;

    private _internal: typeof import("ridb-rust") | undefined;
    private _db: RIDBTypes.Database<T> | undefined;

    constructor(schemas: T) {
        this.schemas = schemas;
    }

    private get db() {
        if (!this._db) {
            throw new Error("Start the database first");
        }
        return this._db;
    }

    get collections() {
        return this.db.collections;
    }

    private async load(): Promise<typeof import("ridb-rust")> {
        this._internal ??= await import("ridb-rust").then(async module => {
            const wasmInstance = module.initSync(wasmBuffer);
            await module.default(wasmInstance);
            return module;
        });
        return this._internal!;
    }

    async start(storageType?: typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>) {
        const db = await this.load();
        this._db ??= await db.Database.create(this.schemas, {
            createStorage: (schemas) => this.createStorage(schemas, storageType)
        });
        return this.db;
    }

    private createStorage<J extends RIDBTypes.SchemaTypeRecord>(schemas: J, storageConstructor?: typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>) {
        if (!this._internal) {
            throw new Error("Start the database first");
        }
        const Storage = storageConstructor ?? this._internal.InMemory;
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
