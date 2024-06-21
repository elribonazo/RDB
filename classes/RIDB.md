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

[../src/index.ts:70](https://github.com/elribonazo/RIDB/blob/e7a589ed88e5f8a1cf6a9f63844d534653394964/ts/src/index.ts#L70)

## Accessors

### collections

> `get` **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

#### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

#### Source

[../src/index.ts:88](https://github.com/elribonazo/RIDB/blob/e7a589ed88e5f8a1cf6a9f63844d534653394964/ts/src/index.ts#L88)

***

### db

> `get` **db**(): [`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>

#### Returns

[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>

#### Source

[../src/index.ts:74](https://github.com/elribonazo/RIDB/blob/e7a589ed88e5f8a1cf6a9f63844d534653394964/ts/src/index.ts#L74)

***

### internal

> `get` **internal**(): [`RIDBTypes`](../namespaces/RIDBTypes/README.md)

#### Returns

[`RIDBTypes`](../namespaces/RIDBTypes/README.md)

#### Source

[../src/index.ts:81](https://github.com/elribonazo/RIDB/blob/e7a589ed88e5f8a1cf6a9f63844d534653394964/ts/src/index.ts#L81)

## Methods

### load()

> **load**(): `Promise`\<[`RIDBTypes`](../namespaces/RIDBTypes/README.md)\>

#### Returns

`Promise`\<[`RIDBTypes`](../namespaces/RIDBTypes/README.md)\>

#### Source

[../src/index.ts:92](https://github.com/elribonazo/RIDB/blob/e7a589ed88e5f8a1cf6a9f63844d534653394964/ts/src/index.ts#L92)

***

### start()

> **start**(`Storage`?): `Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>\>

#### Parameters

• **Storage?**: *typeof* [`BaseStorage`](../namespaces/RIDBTypes/classes/BaseStorage.md)

#### Returns

`Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>\>

#### Source

[../src/index.ts:99](https://github.com/elribonazo/RIDB/blob/e7a589ed88e5f8a1cf6a9f63844d534653394964/ts/src/index.ts#L99)
