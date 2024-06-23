[**@elribonazo/ridb**](../README.md) • **Docs**

***

[@elribonazo/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`schemas`): [`RIDB`](RIDB.md)\<`T`\>

#### Parameters

• **schemas**: `T`

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

#### Defined in

[ts/src/index.ts:135](https://github.com/elribonazo/RIDB/blob/40f1b97fccf8055ed95c62368d24ffac744b38c2/ts/src/index.ts#L135)

## Accessors

### collections

> `get` **collections**(): `object`

#### Returns

`object`

#### Defined in

[ts/src/index.ts:146](https://github.com/elribonazo/RIDB/blob/40f1b97fccf8055ed95c62368d24ffac744b38c2/ts/src/index.ts#L146)

## Methods

### start()

> **start**(`storageType`?): `Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<[`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)\>\>

#### Parameters

• **storageType?**: *typeof* [`BaseStorage`](../namespaces/RIDBTypes/classes/BaseStorage.md)

#### Returns

`Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<[`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)\>\>

#### Defined in

[ts/src/index.ts:159](https://github.com/elribonazo/RIDB/blob/40f1b97fccf8055ed95c62368d24ffac744b38c2/ts/src/index.ts#L159)
