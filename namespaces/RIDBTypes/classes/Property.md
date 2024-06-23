[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / Property

# Class: Property

Represents a property within a schema, including various constraints and nested properties.

## Constructors

### new Property()

> **new Property**(): [`Property`](Property.md)

#### Returns

[`Property`](Property.md)

## Properties

### items?

> `readonly` `optional` **items**: [`Property`](Property.md)[]

An optional array of nested properties for array-type properties.

#### Defined in

pkg/ridb\_rust.d.ts:346

***

### maxItems?

> `readonly` `optional` **maxItems**: `number`

The maximum number of items for array-type properties, if applicable.

#### Defined in

pkg/ridb\_rust.d.ts:351

***

### maxLength?

> `readonly` `optional` **maxLength**: `number`

The maximum length for string-type properties, if applicable.

#### Defined in

pkg/ridb\_rust.d.ts:361

***

### minItems?

> `readonly` `optional` **minItems**: `number`

The minimum number of items for array-type properties, if applicable.

#### Defined in

pkg/ridb\_rust.d.ts:356

***

### minLength?

> `readonly` `optional` **minLength**: `number`

The minimum length for string-type properties, if applicable.

#### Defined in

pkg/ridb\_rust.d.ts:366

***

### primaryKey?

> `readonly` `optional` **primaryKey**: `string`

The primary key of the property, if applicable.

#### Defined in

pkg/ridb\_rust.d.ts:341

***

### properties?

> `readonly` `optional` **properties**: `object`

An optional map of nested properties for object-type properties.

#### Index Signature

 \[`name`: `string`\]: [`Property`](Property.md)

#### Defined in

pkg/ridb\_rust.d.ts:376

***

### required?

> `readonly` `optional` **required**: `string`[]

An optional array of required fields for object-type properties.

#### Defined in

pkg/ridb\_rust.d.ts:371

***

### type

> `readonly` **type**: `string`

The type of the property.

#### Defined in

pkg/ridb\_rust.d.ts:331

***

### version?

> `readonly` `optional` **version**: `number`

The version of the property, if applicable.

#### Defined in

pkg/ridb\_rust.d.ts:336
