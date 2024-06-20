import {BaseStorage, CreateStorage, Doc, InternalsRecord, Operation, OpType, SchemaType} from "../../../pkg/ridb_rust";

class InMemory<T extends SchemaType> extends BaseStorage<T>   {
    async write(operation:Operation<T>): Promise<Doc<T>> {
        if (operation.opType === OpType.CREATE) {
            return operation.data;
        }
        throw new Error("Method not implemented.");
    }

    query(): Promise<void> {
        throw new Error("Method not implemented.");
    }

    findDocumentById(id: string): Promise<null> {
        throw new Error("Method not implemented.");
    }

    count(): Promise<number> {
        throw new Error("Method not implemented.");
    }

    remove(id: string): Promise<void> {
        throw new Error("Method not implemented.");
    }

    close(): Promise<void> {
        throw new Error("Method not implemented.");
    }
}

export const createStorage: CreateStorage = async (records) => Object.keys(records)
    .reduce<InternalsRecord>((storages, name) => ({
        ...storages,
        [name]: new InMemory(name, records[name])
    }), {})
