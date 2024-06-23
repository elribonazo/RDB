[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / Schema

# Class: Schema\<T\>

Represents a schema, including its definition and related methods.

## Type Parameters

• **T** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

## Constructors

### new Schema()

> **new Schema**\<`T`\>(): [`Schema`](Schema.md)\<`T`\>

#### Returns

[`Schema`](Schema.md)\<`T`\>

## Properties

### indexes?

> `readonly` `optional` **indexes**: `string`[]

An optional array of indexes.

#### Defined in

pkg/ridb\_rust.d.ts:104

***

### primaryKey

> `readonly` **primaryKey**: `string`

The primary key of the schema.

#### Defined in

pkg/ridb\_rust.d.ts:89

***

### properties

> `readonly` **properties**: \{ \[name in string \| number \| symbol\]: T\["properties"\]\[name\] \}

The properties defined in the schema.

#### Defined in

pkg/ridb\_rust.d.ts:109

***

### required?

> `readonly` `optional` **required**: `string`[]

An optional array of required fields.

#### Defined in

pkg/ridb\_rust.d.ts:99

***

### schema

> **schema**: [`Schema`](Schema.md)\<`T`\>

The schema definition.

#### Defined in

pkg/ridb\_rust.d.ts:70

***

### type

> `readonly` **type**: `string`

The type of the schema.

#### Defined in

pkg/ridb\_rust.d.ts:94

***

### version

> `readonly` **version**: `number`

The version of the schema.

#### Defined in

pkg/ridb\_rust.d.ts:84

## Methods

### toJSON()

> **toJSON**(): [`SchemaType`](../type-aliases/SchemaType.md)

Converts the schema to a JSON representation.

#### Returns

[`SchemaType`](../type-aliases/SchemaType.md)

The JSON representation of the schema.

#### Defined in

pkg/ridb\_rust.d.ts:118

***

### create()

> `static` **create**\<`TS`\>(`definition`): [`Schema`](Schema.md)\<`TS`\>

Creates a new `Schema` instance from the provided definition.

#### Type Parameters

• **TS** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

#### Parameters

• **definition**: `TS`

The schema definition.

#### Returns

[`Schema`](Schema.md)\<`TS`\>

The created `Schema` instance.

#### Defined in

pkg/ridb\_rust.d.ts:79
