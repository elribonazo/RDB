[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../globals.md) / [RIDBTypes](../README.md) / Schema

# Class: Schema\<T\>

Represents a schema, including its definition and related methods.

## Type parameters

• **T** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

## Constructors

### new Schema()

> **new Schema**\<`T`\>(): [`Schema`](Schema.md)\<`T`\>

#### Returns

[`Schema`](Schema.md)\<`T`\>

## Properties

### indexes?

> `optional` `readonly` **indexes**: `string`[]

An optional array of indexes.

#### Source

pkg/ridb\_rust.d.ts:536

***

### primaryKey

> `readonly` **primaryKey**: `string`

The primary key of the schema.

#### Source

pkg/ridb\_rust.d.ts:521

***

### properties

> `readonly` **properties**: \{ \[name in string \| number \| symbol\]: T\["properties"\]\[name\] \}

The properties defined in the schema.

#### Source

pkg/ridb\_rust.d.ts:541

***

### required?

> `optional` `readonly` **required**: `string`[]

An optional array of required fields.

#### Source

pkg/ridb\_rust.d.ts:531

***

### schema

> **schema**: [`Schema`](Schema.md)\<`T`\>

The schema definition.

#### Source

pkg/ridb\_rust.d.ts:502

***

### type

> `readonly` **type**: `string`

The type of the schema.

#### Source

pkg/ridb\_rust.d.ts:526

***

### version

> `readonly` **version**: `number`

The version of the schema.

#### Source

pkg/ridb\_rust.d.ts:516

## Methods

### toJSON()

> **toJSON**(): [`SchemaType`](../type-aliases/SchemaType.md)

Converts the schema to a JSON representation.

#### Returns

[`SchemaType`](../type-aliases/SchemaType.md)

The JSON representation of the schema.

#### Source

pkg/ridb\_rust.d.ts:550

***

### create()

> `static` **create**\<`TS`\>(`definition`): [`Schema`](Schema.md)\<`TS`\>

Creates a new `Schema` instance from the provided definition.

#### Type parameters

• **TS** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

#### Parameters

• **definition**: `TS`

The schema definition.

#### Returns

[`Schema`](Schema.md)\<`TS`\>

The created `Schema` instance.

#### Source

pkg/ridb\_rust.d.ts:511
