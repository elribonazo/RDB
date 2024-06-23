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

[ts/src/index.ts:126](https://github.com/elribonazo/RIDB/blob/b0761106a4c812dd04966adb4057fb388e13a9d3/ts/src/index.ts#L126)

## Properties

### schemas

> `private` **schemas**: `T`

#### Defined in

[ts/src/index.ts:125](https://github.com/elribonazo/RIDB/blob/b0761106a4c812dd04966adb4057fb388e13a9d3/ts/src/index.ts#L125)

## Accessors

### collections

> `get` **collections**(): `object`

#### Returns

`object`

#### Defined in

[ts/src/index.ts:137](https://github.com/elribonazo/RIDB/blob/b0761106a4c812dd04966adb4057fb388e13a9d3/ts/src/index.ts#L137)

***

### db

> `get` `private` **db**(): [`Database`](../namespaces/RIDBTypes/classes/Database.md)\<[`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)\>

#### Returns

[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<[`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)\>

#### Defined in

[ts/src/index.ts:130](https://github.com/elribonazo/RIDB/blob/b0761106a4c812dd04966adb4057fb388e13a9d3/ts/src/index.ts#L130)

## Methods

### createStorage()

> `private` **createStorage**\<`J`\>(`schemas`, `storageConstructor`?): `Promise`\<[`InternalsRecord`](../namespaces/RIDBTypes/type-aliases/InternalsRecord.md)\>

#### Type Parameters

• **J** *extends* [`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)

#### Parameters

• **schemas**: `J`

• **storageConstructor?**: *typeof* [`BaseStorage`](../namespaces/RIDBTypes/classes/BaseStorage.md)

#### Returns

`Promise`\<[`InternalsRecord`](../namespaces/RIDBTypes/type-aliases/InternalsRecord.md)\>

#### Defined in

[ts/src/index.ts:158](https://github.com/elribonazo/RIDB/blob/b0761106a4c812dd04966adb4057fb388e13a9d3/ts/src/index.ts#L158)

***

### load()

> `private` **load**(): `Promise`\<[`RIDBTypes`](../namespaces/RIDBTypes/README.md)\>

#### Returns

`Promise`\<[`RIDBTypes`](../namespaces/RIDBTypes/README.md)\>

#### Defined in

[ts/src/index.ts:141](https://github.com/elribonazo/RIDB/blob/b0761106a4c812dd04966adb4057fb388e13a9d3/ts/src/index.ts#L141)

***

### start()

> **start**(`storageType`?): `Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<[`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)\>\>

#### Parameters

• **storageType?**: *typeof* [`BaseStorage`](../namespaces/RIDBTypes/classes/BaseStorage.md)

#### Returns

`Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<[`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)\>\>

#### Defined in

[ts/src/index.ts:150](https://github.com/elribonazo/RIDB/blob/b0761106a4c812dd04966adb4057fb388e13a9d3/ts/src/index.ts#L150)
