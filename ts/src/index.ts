import {SchemaTypeRecord, StorageModule, Database} from "ridb-rust";

export type * as RIDBTypes from "ridb-rust";
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

    async create() {
        const internal = await import("ridb-rust")
        const {Database} = internal;
        this._db = await Database.create(
            this.schemas,
            this.storage
        )
    }

}