/**
 * @packageDocumentation
 * @description
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
 * ```typescript
 * import { createLevelDBStorage } from "@pluto-encrypted/leveldb";
 * import { Database } from "@pluto-encrypted/database";
 * //default password must be 32 bytes long
 * const defaultPassword = new Uint8Array(32).fill(1);
 * const database = db = await Database.createEncrypted({
 *          name: `my-db`,
 *          encryptionKey: defaultPassword,
 *          storage: createLevelDBStorage({
 *          dbName: "demo",
 *          dbPath: "/tmp/demo"
 *       })
 * });
 * ```
 * 
 */
import {
    SchemaTypeRecord,
    Database,
    InternalsRecord,
    BaseStorage,
    SchemaType,
} from "../../pkg/ridb_rust";
export type * as RIDBTypes from "../../pkg/ridb_rust";

export * from './schema/types';


let internal: typeof import("../../pkg/ridb_rust") | undefined;
let db:  Database<SchemaTypeRecord> | undefined;

export class RIDB<T extends SchemaTypeRecord> {
    constructor(
        private schemas: T
    ) {}

    get db() {
        if (!db) {
            throw new Error("Start the database first")
        }
        return db;
    }

    get collections() {
        return this.db.collections;
    }

    async load() {
        if (!internal) {
            internal = await import("../../pkg/ridb_rust");
        }
        return internal;
    }

     async start(Storage?: typeof BaseStorage<SchemaType>) {
        const DefaultStorage= Storage ?? (await this.load()).InMemory;
        if (!db) {
            const {Database} = await this.load();
            db = await Database.create(
                this.schemas,
                {
                    createStorage: async (schemas) => Object.keys(schemas)
                        .reduce<InternalsRecord>((storages, name) => ({
                            ...storages,
                            [name]: new DefaultStorage(name, schemas[name])
                        }), {})
                }
            )
        }
        return db;
    }
}