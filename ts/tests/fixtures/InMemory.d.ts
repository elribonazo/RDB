import { BaseStorage, Doc, Operation, SchemaType } from "../../../pkg/ridb_rust";
export declare class InMemory<T extends SchemaType> extends BaseStorage<T> {
    write(operation: Operation<T>): Promise<Doc<T>>;
    query(): Promise<void>;
    findDocumentById(id: string): Promise<null>;
    count(): Promise<number>;
    remove(id: string): Promise<void>;
    close(): Promise<void>;
}
