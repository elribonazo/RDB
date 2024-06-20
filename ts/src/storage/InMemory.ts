import {CreateStorage} from "ridb-rust";
import {OpType, BaseStorage, RIDBTypes} from "../index";

class InMemory<T extends RIDBTypes.SchemaType> extends BaseStorage<T>   {
    async write(operation:RIDBTypes.Operation<T>): Promise<RIDBTypes.Doc<T>> {
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
    .reduce<RIDBTypes.InternalsRecord>((storages, name) => ({
        ...storages,
        [name]: new InMemory(name, records[name])
    }), {})
