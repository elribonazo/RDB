[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../globals.md) / [RIDBTypes](../README.md) / Storage

# Class: Storage\<T\>

Represents a storage system containing a map of internal storages.

## Type parameters

• **T** *extends* [`InternalsRecord`](../type-aliases/InternalsRecord.md)

The record of internals.

## Constructors

### new Storage()

> **new Storage**\<`T`\>(): [`Storage`](Storage.md)\<`T`\>

#### Returns

[`Storage`](Storage.md)\<`T`\>

## Properties

### internals

> `readonly` **internals**: \{ \[name in string \| number \| symbol\]: T\[name\] \}

The internals in the storage.

This is a read-only property where the key is the name of the internal and the value is a `BaseStorage` instance.

#### Source

pkg/ridb\_rust.d.ts:363

## Methods

### create()

> `static` **create**\<`TS`\>(`internals`): [`Storage`](Storage.md)\<`TS`\>

Creates a new `Storage` instance with the provided internals.

#### Type parameters

• **TS** *extends* [`InternalsRecord`](../type-aliases/InternalsRecord.md) = [`InternalsRecord`](../type-aliases/InternalsRecord.md)

The record of internals.

#### Parameters

• **internals**: `TS`

The internals to use for the storage.

#### Returns

[`Storage`](Storage.md)\<`TS`\>

The created `Storage` instance.

#### Source

pkg/ridb\_rust.d.ts:354
