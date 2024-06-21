import { describe, it, expect } from 'vitest';
import {
    RIDB,
    SchemaFieldType,
} from "../src";
import {InMemory} from "./fixtures/InMemory";
import {RIDBTypes} from "../cjs";
const schemaType = {
    version: 0,
    primaryKey: 'id',
    type:SchemaFieldType.object,
    properties: {
        id: {
            type:SchemaFieldType.string,
            maxLength: 60
        }
    }
}

const storages: (typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>|undefined)[] = [
    InMemory,
    undefined
]

export default (platform: string) =>
describe(`Testing ${platform}`, () => {

    storages.forEach(storage => {
        describe(`Test ${storage ? 'Typescript' : 'Wasm'} Storage`, () => {

            it("Should be able to create a default database with a valid schema", async () => {
                const db =  new RIDB(
                    {
                        demo: {
                            version: 0,
                            primaryKey: 'id',
                            type:SchemaFieldType.object,
                            properties: {
                                id: {
                                    type:SchemaFieldType.string,
                                    maxLength: 60
                                }
                            }
                        }
                    }
                )
                await db.start(storage)
                expect(db).to.not.be.undefined;
            });
            it("Should be able to create a database with a schema with nested fields", async () => {
                const db =  new RIDB(
                    {
                        demo: {
                            version: 0,
                            primaryKey: 'id',
                            type:SchemaFieldType.object,
                            properties: {
                                id: {
                                    type:SchemaFieldType.string,
                                    maxLength: 60
                                },
                                nested: {
                                    type: SchemaFieldType.object,
                                    properties: {
                                        name: {
                                            type:SchemaFieldType.string,
                                            maxLength: 60
                                        },
                                        email: {
                                            type:SchemaFieldType.string,
                                            maxLength: 60
                                        }
                                    }
                                }
                            }
                        }
                    }
                )
                await db.start(storage);
                expect(db).to.not.be.undefined;
            });
            it("Should be able to create a database with a schema and array fields", async () => {
                const db =  new RIDB(
                    {
                        demo: {
                            version: 0,
                            primaryKey: 'id',
                            type:SchemaFieldType.object,
                            properties: {
                                id: {
                                    type:SchemaFieldType.string,
                                    maxLength: 60
                                },
                                names: {
                                    type: SchemaFieldType.array,
                                    items: [
                                        {
                                            type: SchemaFieldType.string
                                        }
                                    ]
                                }
                            }
                        }
                    }
                )
                await db.start(storage);
                expect(db).to.not.be.undefined;
            })
            it("Should throw an error when schemaType is invalid");
            it("Should contain 1 collection per schema/model specified in the database create fn")
            it("Should validate the require fields are sent when calling db create")
            it('It should be able to create a new document from JSON schema', async () => {
                const db =  new RIDB(
                    {
                        demo: schemaType
                    }
                )
                await db.start(storage);
                expect(db).to.not.be.undefined;
                expect(db.collections).to.not.be.undefined;
                expect(db.collections).to.haveOwnProperty("demo");
                expect(db.collections.demo).to.not.be.undefined;
                expect(db.collections.demo.find).to.not.be.undefined;
                const created = await db.collections.demo.create({} as any)
                expect(created).to.not.be.undefined;
                expect(created).to.haveOwnProperty("id");
                expect(created.id).to.eq("12345")
            })
        })
    })

})