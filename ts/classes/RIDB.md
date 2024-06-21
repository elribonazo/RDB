[**@elribonazo/ridb**](../README.md) • **Docs**

***

[@elribonazo/ridb](../globals.md) / RIDB

# Class: RIDB\<T\>

Represents a database instance with the ability to manage collections and internal storage.

## Type parameters

• **T** *extends* [`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)

The record of schema types.

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`schemas`): [`RIDB`](RIDB.md)\<`T`\>

Creates a new `RIDB` instance with the provided schemas.

#### Parameters

• **schemas**: `T`

The schemas to use for the database.

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

#### Source

[ts/src/index.ts:26](https://github.com/elribonazo/RIDB/blob/f06d8130ee392b5866dffa2203082ded0a4f3aa0/ts/src/index.ts#L26)

## Accessors

### collections

> `get` **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Gets the collections from the database.

#### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections in the database.

#### Source

[ts/src/index.ts:59](https://github.com/elribonazo/RIDB/blob/f06d8130ee392b5866dffa2203082ded0a4f3aa0/ts/src/index.ts#L59)

***

### db

> `get` **db**(): [`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>

Gets the database instance. Throws an error if the database is not started.

#### Throws

Will throw an error if the database is not started.

#### Returns

[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>

The database instance.

#### Source

[ts/src/index.ts:34](https://github.com/elribonazo/RIDB/blob/f06d8130ee392b5866dffa2203082ded0a4f3aa0/ts/src/index.ts#L34)

***

### internal

> `get` **internal**(): [`RIDBTypes`](../namespaces/RIDBTypes/README.md)

Gets the internal storage module. Throws an error if the database is not started.

#### Throws

Will throw an error if the database is not started.

#### Returns

[`RIDBTypes`](../namespaces/RIDBTypes/README.md)

The internal storage module.

#### Source

[ts/src/index.ts:47](https://github.com/elribonazo/RIDB/blob/f06d8130ee392b5866dffa2203082ded0a4f3aa0/ts/src/index.ts#L47)

## Methods

### load()

> **load**(): `Promise`\<[`RIDBTypes`](../namespaces/RIDBTypes/README.md)\>

Loads the internal storage module if it is not already loaded.

#### Returns

`Promise`\<[`RIDBTypes`](../namespaces/RIDBTypes/README.md)\>

A promise that resolves to the internal storage module.

#### Source

[ts/src/index.ts:68](https://github.com/elribonazo/RIDB/blob/f06d8130ee392b5866dffa2203082ded0a4f3aa0/ts/src/index.ts#L68)

***

### start()

> **start**(`Storage`?): `Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>\>

Starts the database with the provided storage class, or defaults to `InMemory` storage.

#### Parameters

• **Storage?**: *typeof* [`BaseStorage`](../namespaces/RIDBTypes/classes/BaseStorage.md)

The storage class to use.

#### Returns

`Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>\>

A promise that resolves to the started database instance.

#### Source

[ts/src/index.ts:81](https://github.com/elribonazo/RIDB/blob/f06d8130ee392b5866dffa2203082ded0a4f3aa0/ts/src/index.ts#L81)
