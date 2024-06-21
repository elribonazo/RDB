[**@elribonazo/ridb**](../README.md) • **Docs**

***

[@elribonazo/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

## Type parameters

• **T** *extends* [`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`schemas`): [`RIDB`](RIDB.md)\<`T`\>

#### Parameters

• **schemas**: `T`

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

#### Source

[ts/src/index.ts:65](https://github.com/elribonazo/RIDB/blob/3d5d079eabaff0493648f6889b99823c5605ae57/ts/src/index.ts#L65)

## Properties

### \_db

> `private` **\_db**: `undefined` \| [`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>

#### Source

[ts/src/index.ts:64](https://github.com/elribonazo/RIDB/blob/3d5d079eabaff0493648f6889b99823c5605ae57/ts/src/index.ts#L64)

***

### \_internal

> `private` **\_internal**: `undefined` \| [`RIDBTypes`](../namespaces/RIDBTypes/README.md)

#### Source

[ts/src/index.ts:63](https://github.com/elribonazo/RIDB/blob/3d5d079eabaff0493648f6889b99823c5605ae57/ts/src/index.ts#L63)

***

### schemas

> `private` **schemas**: `T`

#### Source

[ts/src/index.ts:66](https://github.com/elribonazo/RIDB/blob/3d5d079eabaff0493648f6889b99823c5605ae57/ts/src/index.ts#L66)

## Accessors

### collections

> `get` **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

#### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

#### Source

[ts/src/index.ts:83](https://github.com/elribonazo/RIDB/blob/3d5d079eabaff0493648f6889b99823c5605ae57/ts/src/index.ts#L83)

***

### db

> `get` **db**(): [`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>

#### Returns

[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>

#### Source

[ts/src/index.ts:69](https://github.com/elribonazo/RIDB/blob/3d5d079eabaff0493648f6889b99823c5605ae57/ts/src/index.ts#L69)

***

### internal

> `get` **internal**(): [`RIDBTypes`](../namespaces/RIDBTypes/README.md)

#### Returns

[`RIDBTypes`](../namespaces/RIDBTypes/README.md)

#### Source

[ts/src/index.ts:76](https://github.com/elribonazo/RIDB/blob/3d5d079eabaff0493648f6889b99823c5605ae57/ts/src/index.ts#L76)

## Methods

### load()

> **load**(): `Promise`\<[`RIDBTypes`](../namespaces/RIDBTypes/README.md)\>

#### Returns

`Promise`\<[`RIDBTypes`](../namespaces/RIDBTypes/README.md)\>

#### Source

[ts/src/index.ts:87](https://github.com/elribonazo/RIDB/blob/3d5d079eabaff0493648f6889b99823c5605ae57/ts/src/index.ts#L87)

***

### start()

> **start**(`Storage`?): `Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>\>

#### Parameters

• **Storage?**: *typeof* [`BaseStorage`](../namespaces/RIDBTypes/classes/BaseStorage.md)

#### Returns

`Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>\>

#### Source

[ts/src/index.ts:94](https://github.com/elribonazo/RIDB/blob/3d5d079eabaff0493648f6889b99823c5605ae57/ts/src/index.ts#L94)
