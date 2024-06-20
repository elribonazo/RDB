import { SchemaTypeRecord, StorageModule, Database } from "ridb-rust";
export type * as RIDBTypes from "ridb-rust";
export * from './schema/types';
export declare class RIDB<T extends SchemaTypeRecord> {
    private schemas;
    private storage;
    private _db;
    constructor(schemas: T, storage: StorageModule);
    get db(): Database<T>;
    get collections(): { [name in keyof T]: import("ridb-rust").Collection<import("ridb-rust").Schema<T[name]>>; };
    create(): Promise<void>;
}
