[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / BaseStorage

# Class: BaseStorage\<T\>

Represents the base storage implementation, extending `StorageInternal`.

## Extends

- [`StorageInternal`](StorageInternal.md)\<`T`\>

## Extended by

- [`InMemory`](InMemory.md)

## Type parameters

• **T** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

## Constructors

### new BaseStorage()

> **new BaseStorage**\<`T`\>(`name`, `schema_type`): [`BaseStorage`](BaseStorage.md)\<`T`\>

Creates a new `BaseStorage` instance with the provided name and schema type.

#### Parameters

• **name**: `string`

The name of the storage.

• **schema\_type**: `any`

The schema type of the storage.

#### Returns

[`BaseStorage`](BaseStorage.md)\<`T`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`constructor`](StorageInternal.md#constructors)

#### Source

pkg/ridb\_rust.d.ts:274

## Properties

### name

> `readonly` **name**: `string`

The name of the storage.

#### Source

pkg/ridb\_rust.d.ts:279

***

### schema

> `readonly` **schema**: [`Schema`](Schema.md)\<`T`\>

The schema associated with the storage.

#### Source

pkg/ridb\_rust.d.ts:284

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Closes the storage.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the storage is closed.

#### Overrides

[`StorageInternal`](StorageInternal.md).[`close`](StorageInternal.md#close)

#### Source

pkg/ridb\_rust.d.ts:291

***

### count()

> **count**(): `Promise`\<`number`\>

Counts the number of documents in the storage.

#### Returns

`Promise`\<`number`\>

A promise that resolves to the number of documents.

#### Overrides

[`StorageInternal`](StorageInternal.md).[`count`](StorageInternal.md#count)

#### Source

pkg/ridb\_rust.d.ts:298

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

#### Overrides

[`StorageInternal`](StorageInternal.md).[`findDocumentById`](StorageInternal.md#finddocumentbyid)

#### Source

pkg/ridb\_rust.d.ts:306

***

### free()

> **free**(): `void`

Frees the resources used by the base storage.

#### Returns

`void`

#### Source

pkg/ridb\_rust.d.ts:266

***

### query()

> **query**(): `Promise`\<`void`\>

Queries the storage.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the query is complete.

#### Overrides

[`StorageInternal`](StorageInternal.md).[`query`](StorageInternal.md#query)

#### Source

pkg/ridb\_rust.d.ts:313

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

#### Overrides

[`StorageInternal`](StorageInternal.md).[`remove`](StorageInternal.md#remove)

#### Source

pkg/ridb\_rust.d.ts:321

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

#### Overrides

[`StorageInternal`](StorageInternal.md).[`write`](StorageInternal.md#write)

#### Source

pkg/ridb\_rust.d.ts:329
