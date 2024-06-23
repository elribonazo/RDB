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

[ts/src/index.ts:126](https://github.com/elribonazo/RIDB/blob/16a776a02482a53dea4bcee65ba3886992a6fca5/ts/src/index.ts#L126)

## Accessors

### collections

> `get` **collections**(): `object`

#### Returns

`object`

#### Defined in

[ts/src/index.ts:137](https://github.com/elribonazo/RIDB/blob/16a776a02482a53dea4bcee65ba3886992a6fca5/ts/src/index.ts#L137)

## Methods

### start()

> **start**(`storageType`?): `Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<[`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)\>\>

#### Parameters

• **storageType?**: *typeof* [`BaseStorage`](../namespaces/RIDBTypes/classes/BaseStorage.md)

#### Returns

`Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<[`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)\>\>

#### Defined in

[ts/src/index.ts:150](https://github.com/elribonazo/RIDB/blob/16a776a02482a53dea4bcee65ba3886992a6fca5/ts/src/index.ts#L150)
