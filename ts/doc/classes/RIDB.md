[**@elribonazo/ridb**](../README.md) • **Docs**

***

[@elribonazo/ridb](../globals.md) / RIDB

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

[ts/src/index.ts:14](https://github.com/elribonazo/RIDB/blob/e6efbc418b319fd72a86705f8540745273bdd697/ts/src/index.ts#L14)

## Accessors

### collections

> `get` **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

#### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

#### Source

[ts/src/index.ts:32](https://github.com/elribonazo/RIDB/blob/e6efbc418b319fd72a86705f8540745273bdd697/ts/src/index.ts#L32)

***

### db

> `get` **db**(): [`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>

#### Returns

[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>

#### Source

[ts/src/index.ts:18](https://github.com/elribonazo/RIDB/blob/e6efbc418b319fd72a86705f8540745273bdd697/ts/src/index.ts#L18)

***

### internal

> `get` **internal**(): [`RIDBTypes`](../namespaces/RIDBTypes/README.md)

#### Returns

[`RIDBTypes`](../namespaces/RIDBTypes/README.md)

#### Source

[ts/src/index.ts:25](https://github.com/elribonazo/RIDB/blob/e6efbc418b319fd72a86705f8540745273bdd697/ts/src/index.ts#L25)

## Methods

### load()

> **load**(): `Promise`\<[`RIDBTypes`](../namespaces/RIDBTypes/README.md)\>

#### Returns

`Promise`\<[`RIDBTypes`](../namespaces/RIDBTypes/README.md)\>

#### Source

[ts/src/index.ts:36](https://github.com/elribonazo/RIDB/blob/e6efbc418b319fd72a86705f8540745273bdd697/ts/src/index.ts#L36)

***

### start()

> **start**(`Storage`?): `Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>\>

#### Parameters

• **Storage?**: *typeof* [`BaseStorage`](../namespaces/RIDBTypes/classes/BaseStorage.md)

#### Returns

`Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>\>

#### Source

[ts/src/index.ts:43](https://github.com/elribonazo/RIDB/blob/e6efbc418b319fd72a86705f8540745273bdd697/ts/src/index.ts#L43)
