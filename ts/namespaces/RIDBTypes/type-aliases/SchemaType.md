[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../globals.md) / [RIDBTypes](../README.md) / SchemaType

# Type alias: SchemaType

> **SchemaType**: `object`

Represents the type definition for a schema.

## Type declaration

### indexes?

> `optional` `readonly` **indexes**: `string`[]

An optional array of indexes.

### primaryKey

> `readonly` **primaryKey**: `string`

The primary key of the schema.

### properties

> `readonly` **properties**: `object`

The properties defined in the schema.

#### Index signature

 \[`name`: `string`\]: [`Property`](../classes/Property.md)

### required?

> `optional` `readonly` **required**: `string`[]

An optional array of required fields.

### type

> `readonly` **type**: `string`

The type of the schema.

### version

> `readonly` **version**: `number`

The version of the schema.

## Source

pkg/ridb\_rust.d.ts:459
