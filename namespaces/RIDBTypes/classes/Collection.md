[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / Collection

# Class: Collection\<T\>

Collection is a class that represents a collection of documents in a database.

## Type parameters

• **T** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

A schema type defining the structure of the documents in the collection.

## Constructors

### new Collection()

> **new Collection**\<`T`\>(`name`, `schema`): [`Collection`](Collection.md)\<`T`\>

Constructs a new Collection instance.

#### Parameters

• **name**: `string`

The name of the collection.

• **schema**: `T`

The schema defining the structure of the documents in the collection.

#### Returns

[`Collection`](Collection.md)\<`T`\>

#### Source

pkg/ridb\_rust.d.ts:80

## Properties

### schema

> **schema**: `T`

#### Source

pkg/ridb\_rust.d.ts:72

## Methods

### create()

> **create**(`document`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

Creates a new document in the collection.

#### Parameters

• **document**: [`Doc`](../type-aliases/Doc.md)\<`T`\>

The document to create.

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A promise that resolves to the created document.

#### Source

pkg/ridb\_rust.d.ts:112

***

### delete()

> **delete**(`id`): `Promise`\<`void`\>

Deletes a document in the collection by its ID.

#### Parameters

• **id**: `string`

The ID of the document to delete.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the deletion is complete.

#### Source

pkg/ridb\_rust.d.ts:120

***

### find()

> **find**(): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>[]\>

Finds all documents in the collection.

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>[]\>

A promise that resolves to an array of documents.

#### Source

pkg/ridb\_rust.d.ts:87

***

### findOne()

> **findOne**(`id`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

Finds a single document in the collection by its ID.

#### Parameters

• **id**: `string`

The ID of the document to find.

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A promise that resolves to the found document.

#### Source

pkg/ridb\_rust.d.ts:95

***

### update()

> **update**(`id`, `document`): `Promise`\<`void`\>

Updates a document in the collection by its ID.

#### Parameters

• **id**: `string`

The ID of the document to update.

• **document**: `Partial`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A partial document containing the fields to update.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the update is complete.

#### Source

pkg/ridb\_rust.d.ts:104

***

### from()

> `static` **from**\<`TS`\>(`name`, `schema`): [`Collection`](Collection.md)\<`TS`\>

Creates a new Collection instance from a given schema.

#### Type parameters

• **TS** *extends* [`SchemaType`](../type-aliases/SchemaType.md) = [`SchemaType`](../type-aliases/SchemaType.md)

A schema type. Defaults to SchemaType.

#### Parameters

• **name**: `string`

The name of the collection.

• **schema**: `TS`

The schema defining the structure of the documents in the collection.

#### Returns

[`Collection`](Collection.md)\<`TS`\>

A new Collection instance.

#### Source

pkg/ridb\_rust.d.ts:68
