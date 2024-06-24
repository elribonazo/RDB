import { describe, it, expect } from 'vitest';
import type { RIDBTypes } from "../build/esm/index";

import {
    SchemaFieldType,
    RIDB,
} from "../build/esm/index";


const storages: (typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>|undefined)[] = [
    undefined
]

export default (platform: string) =>


describe(`[${platform}] Testing`, () => {

    storages.forEach(storage => {
        describe(`[${platform}][${storage ? 'Typescript' : 'Wasm'}] Testing Storage`, () => {
            it('It should be able to create a new document from JSON schema', async () => {

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
            it("Should throw an error with a schema with invalid type", async () => {
                const db =  new RIDB(
                    {
                        demo: {
                            version: 0,
                            primaryKey: 'id',
                            type:"wrong",
                            properties: { }
                        }
                    }
                )
                await expect(async () => db.start(storage)).to.rejects.toThrowError("Validation Error: Schema type is invalid (\"wrong\")")
            })
            it("Should throw an error when schema properties type is invalid", async () => {
                const db =  new RIDB(
                    {
                        demo: {
                            version: 0,
                            primaryKey: 'id',
                            type:"obiect",
                            properties: {
                                id: {
                                    type:"....",
                                    minLength:-1
                                }
                            }
                        }
                    }
                )
                await expect(async () => db.start(storage)).to.rejects.toThrowError("Serialization Error: invalid value: string \"....\", expected an PropertyType (String, Number, Boolean, Object or Array)")
            })
            it("Should throw an error when the minLength is lower than 0", async () => {
                const db =  new RIDB(
                    {
                        demo: {
                            version: 0,
                            primaryKey: 'id',
                            type:"object",
                            properties: {
                                id: {
                                    type:"string",
                                    minLength:-1
                                }
                            }
                        }
                    }
                )
                await expect(async () => db.start(storage)).to.rejects.toThrowError("Validation Error: Min property not valid")

            })
            it("Should throw an error when schemaType with a property that has min higher than max", async () => {
                const db =  new RIDB(
                    {
                        demo: {
                            version: 0,
                            primaryKey: 'id',
                            type:"object",
                            properties: {
                                id: {
                                    type:"string",
                                    maxLength: 2,
                                    minLength:3
                                }
                            }
                        }
                    }
                )
                await expect(async () => db.start(storage)).to.rejects.toThrowError("Validation Error: Min higher than max")
            });
        })
    })
})