import {SchemaTypeRecord, StorageModule, Database} from "../../pkg/ridb_rust";
export type * as RIDBTypes from "../../pkg/ridb_rust";

export * from './schema/types';
export class RIDB<T extends SchemaTypeRecord> {
    private _db: Database<T> | undefined;
    constructor(
        private schemas: T,
        private storage: StorageModule
    ) {}

    get db() {
        if (!this._db) {
            throw new Error("Start the database first")
        }
        return this._db;
    }

    get collections() {
        return this.db.collections;
    }

    async start() {
        const internal = await import("../../pkg/ridb_rust")
        const {Database} = internal;
        this._db = await Database.create(
            this.schemas,
            this.storage
        )
    }

}