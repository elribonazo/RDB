import { SchemaTypeRecord, StorageModule, Database } from "../../pkg/ridb_rust";
export type * as RIDBTypes from "../../pkg/ridb_rust";
export * from './schema/types';
export declare class RIDB<T extends SchemaTypeRecord> {
    private schemas;
    private storage;
    private _db;
    constructor(schemas: T, storage: StorageModule);
    get db(): Database<T>;
    get collections(): { [name in keyof T]: import("../../pkg/ridb_rust").Collection<import("../../pkg/ridb_rust").Schema<T[name]>>; };
    start(): Promise<void>;
}
