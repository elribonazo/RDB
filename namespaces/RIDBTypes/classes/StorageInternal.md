[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / StorageInternal

# Class: `abstract` StorageInternal\<T\>

Represents the internal storage interface with abstract methods for various storage operations.

## Extended by

- [`BaseStorage`](BaseStorage.md)

## Type Parameters

• **T** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

## Constructors

### new StorageInternal()

> **new StorageInternal**\<`T`\>(): [`StorageInternal`](StorageInternal.md)\<`T`\>

#### Returns

[`StorageInternal`](StorageInternal.md)\<`T`\>

## Methods

### close()

> `abstract` **close**(): `Promise`\<`void`\>

Closes the storage.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the storage is closed.

#### Defined in

pkg/ridb\_rust.d.ts:432

***

### count()

> `abstract` **count**(): `Promise`\<`number`\>

Counts the number of documents in the storage.

#### Returns

`Promise`\<`number`\>

A promise that resolves to the number of documents.

#### Defined in

pkg/ridb\_rust.d.ts:417

***

### findDocumentById()

> `abstract` **findDocumentById**(`id`): `Promise`\<`null`\>

Finds a document by its ID.

#### Parameters

• **id**: `string`

The ID of the document to find.

#### Returns

`Promise`\<`null`\>

A promise that resolves to the found document or null.

#### Defined in

pkg/ridb\_rust.d.ts:410

***

### query()

> `abstract` **query**(): `Promise`\<`void`\>

Queries the storage.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the query is complete.

#### Defined in

pkg/ridb\_rust.d.ts:402

***

### remove()

> `abstract` **remove**(`id`): `Promise`\<`void`\>

Removes a document by its ID.

#### Parameters

• **id**: `string`

The ID of the document to remove.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the document is removed.

#### Defined in

pkg/ridb\_rust.d.ts:425

***

### write()

> `abstract` **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

Writes an operation to the storage.

#### Parameters

• **op**: [`Operation`](../type-aliases/Operation.md)\<`T`\>

The operation to write.

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A promise that resolves to the document written.

#### Defined in

pkg/ridb\_rust.d.ts:395
