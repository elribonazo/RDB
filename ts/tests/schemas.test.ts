import { describe, it, expect } from 'vitest';
import {
    Database,
    SchemaFieldType,
} from "../src";

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

import * as InMemory from "../src/storage/InMemory";

describe('RIDB', () => {
    it("Should be able to create a default database with a valid schema");
    it("Should be able to create a database with a schema with nested fields");
    it("Should be able to create a database with a schema and array fields")
    it("Should throw an error when schemaType is invalid");
    it("Should contain 1 collection per schema/model specified in the database create fn")
    it("Should validate the require fields are sent when calling db create")

    it('It should be able to create a new document from JSON schema', async () => {
        const db = await Database.create(
            {
                demo: schemaType
            },
            InMemory
        )
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