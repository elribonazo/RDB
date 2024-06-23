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

[ts/src/index.ts:46](https://github.com/elribonazo/RIDB/blob/bef263115c187d0dea0f3a38e0d0a0410d7a024f/ts/src/index.ts#L46)

## Properties

### schemas

> `private` **schemas**: `T`

#### Defined in

[ts/src/index.ts:45](https://github.com/elribonazo/RIDB/blob/bef263115c187d0dea0f3a38e0d0a0410d7a024f/ts/src/index.ts#L45)

## Accessors

### collections

> `get` **collections**(): `object`

#### Returns

`object`

#### Defined in

[ts/src/index.ts:57](https://github.com/elribonazo/RIDB/blob/bef263115c187d0dea0f3a38e0d0a0410d7a024f/ts/src/index.ts#L57)

***

### db

> `get` `private` **db**(): [`Database`](../namespaces/RIDBTypes/classes/Database.md)\<[`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)\>

#### Returns

[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<[`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)\>

#### Defined in

[ts/src/index.ts:50](https://github.com/elribonazo/RIDB/blob/bef263115c187d0dea0f3a38e0d0a0410d7a024f/ts/src/index.ts#L50)

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

[ts/src/index.ts:78](https://github.com/elribonazo/RIDB/blob/bef263115c187d0dea0f3a38e0d0a0410d7a024f/ts/src/index.ts#L78)

***

### load()

> `private` **load**(): `Promise`\<[`RIDBTypes`](../namespaces/RIDBTypes/README.md)\>

#### Returns

`Promise`\<[`RIDBTypes`](../namespaces/RIDBTypes/README.md)\>

#### Defined in

[ts/src/index.ts:61](https://github.com/elribonazo/RIDB/blob/bef263115c187d0dea0f3a38e0d0a0410d7a024f/ts/src/index.ts#L61)

***

### start()

> **start**(`storageType`?): `Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<[`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)\>\>

#### Parameters

• **storageType?**: *typeof* [`BaseStorage`](../namespaces/RIDBTypes/classes/BaseStorage.md)

#### Returns

`Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<[`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)\>\>

#### Defined in

[ts/src/index.ts:70](https://github.com/elribonazo/RIDB/blob/bef263115c187d0dea0f3a38e0d0a0410d7a024f/ts/src/index.ts#L70)
