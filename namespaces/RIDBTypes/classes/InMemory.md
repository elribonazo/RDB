[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / InMemory

# Class: InMemory\<T\>

Represents an in-memory storage system extending the base storage functionality.

## Extends

- [`BaseStorage`](BaseStorage.md)\<`T`\>

## Type Parameters

• **T** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

## Constructors

### new InMemory()

> **new InMemory**\<`T`\>(`name`, `schema_type`): [`InMemory`](InMemory.md)\<`T`\>

Creates a new `BaseStorage` instance with the provided name and schema type.

#### Parameters

• **name**: `string`

The name of the storage.

• **schema\_type**: `any`

The schema type of the storage.

#### Returns

[`InMemory`](InMemory.md)\<`T`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`constructor`](BaseStorage.md#constructors)

#### Defined in

pkg/ridb\_rust.d.ts:307

## Properties

### name

> `readonly` **name**: `string`

The name of the storage.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`name`](BaseStorage.md#name)

#### Defined in

pkg/ridb\_rust.d.ts:312

***

### schema

> `readonly` **schema**: [`Schema`](Schema.md)\<`T`\>

The schema associated with the storage.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`schema`](BaseStorage.md#schema)

#### Defined in

pkg/ridb\_rust.d.ts:317

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Closes the storage.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the storage is closed.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`close`](BaseStorage.md#close)

#### Defined in

pkg/ridb\_rust.d.ts:324

***

### count()

> **count**(): `Promise`\<`number`\>

Counts the number of documents in the storage.

#### Returns

`Promise`\<`number`\>

A promise that resolves to the number of documents.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`count`](BaseStorage.md#count)

#### Defined in

pkg/ridb\_rust.d.ts:331

***

### findDocumentById()

> **findDocumentById**(`id`): `Promise`\<`null`\>

Finds a document by its ID.

#### Parameters

• **id**: `string`

The ID of the document to find.

#### Returns

`Promise`\<`null`\>

A promise that resolves to the found document or null.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`findDocumentById`](BaseStorage.md#finddocumentbyid)

#### Defined in

pkg/ridb\_rust.d.ts:339

***

### free()

> **free**(): `void`

Frees the resources used by the in-memory storage.

#### Returns

`void`

#### Overrides

[`BaseStorage`](BaseStorage.md).[`free`](BaseStorage.md#free)

#### Defined in

pkg/ridb\_rust.d.ts:606

***

### query()

> **query**(): `Promise`\<`void`\>

Queries the storage.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the query is complete.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`query`](BaseStorage.md#query)

#### Defined in

pkg/ridb\_rust.d.ts:346

***

### remove()

> **remove**(`id`): `Promise`\<`void`\>

Removes a document by its ID.

#### Parameters

• **id**: `string`

The ID of the document to remove.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the document is removed.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`remove`](BaseStorage.md#remove)

#### Defined in

pkg/ridb\_rust.d.ts:354

***

### write()

> **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

Writes an operation to the storage.

#### Parameters

• **op**: [`Operation`](../type-aliases/Operation.md)\<`T`\>

The operation to write.

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A promise that resolves to the document written.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`write`](BaseStorage.md#write)

#### Defined in

pkg/ridb\_rust.d.ts:362
