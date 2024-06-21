import {
    SchemaTypeRecord,
    Database,
    InternalsRecord,
    BaseStorage,
    SchemaType
} from "../../pkg/ridb_rust";
export type * as RIDBTypes from "../../pkg/ridb_rust";

export * from './schema/types';
export class RIDB<T extends SchemaTypeRecord> {
    private _internal: typeof import("../../pkg/ridb_rust") | undefined;
    private _db: Database<T> | undefined;
    constructor(
        private schemas: T
    ) {}

    get db() {
        if (!this._db) {
            throw new Error("Start the database first")
        }
        return this._db;
    }

    get internal() {
        if (!this._internal) {
            throw new Error("Start the database first")
        }
        return this._internal;
    }

    get collections() {
        return this.db.collections;
    }

    async load() {
        if (!this._internal) {
            this._internal = await import("../../pkg/ridb_rust");
        }
        return this.internal;
    }

     async start(Storage?: typeof BaseStorage<SchemaType>) {
        const DefaultStorage = Storage ?? (await this.load()).InMemory;
        if (!this._db) {
            const {Database} = await this.load();
            this._db = await Database.create(
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
        return this.db;
    }
}