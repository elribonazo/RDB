[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / Internals

# Class: Internals\<T\>

Represents the internals of a storage system, including the base storage and schema.

## Type parameters

• **T** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

## Constructors

### new Internals()

> **new Internals**\<`T`\>(`internal`): [`Internals`](Internals.md)\<`T`\>

Creates a new `Internals` instance with the provided base storage.

#### Parameters

• **internal**: [`BaseStorage`](BaseStorage.md)\<`T`\>

The base storage instance.

#### Returns

[`Internals`](Internals.md)\<`T`\>

#### Source

../../pkg/ridb\_rust.d.ts:386

## Properties

### internal

> `readonly` **internal**: [`BaseStorage`](BaseStorage.md)\<`T`\>

The base storage instance.

#### Source

../../pkg/ridb\_rust.d.ts:379

***

### schema

> `readonly` **schema**: `T`

The schema associated with the storage.

#### Source

../../pkg/ridb\_rust.d.ts:391
