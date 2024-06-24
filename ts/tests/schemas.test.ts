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
            it("Should throw an error when schemaType is invalid", async () => {
                const db =  new RIDB(
                    {
                        demo: {
                            version: 0,
                            primaryKey: 'id',
                            type:"wrong",
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
            });
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
        })
    })

})