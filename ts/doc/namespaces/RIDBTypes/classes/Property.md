[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../globals.md) / [RIDBTypes](../README.md) / Property

# Class: Property

Represents a property within a schema, including various constraints and nested properties.

## Constructors

### new Property()

> **new Property**(): [`Property`](Property.md)

#### Returns

[`Property`](Property.md)

## Properties

### items?

> `optional` `readonly` **items**: [`Property`](Property.md)[]

An optional array of nested properties for array-type properties.

#### Source

pkg/ridb\_rust.d.ts:419

***

### maxItems?

> `optional` `readonly` **maxItems**: `number`

The maximum number of items for array-type properties, if applicable.

#### Source

pkg/ridb\_rust.d.ts:424

***

### maxLength?

> `optional` `readonly` **maxLength**: `number`

The maximum length for string-type properties, if applicable.

#### Source

pkg/ridb\_rust.d.ts:434

***

### minItems?

> `optional` `readonly` **minItems**: `number`

The minimum number of items for array-type properties, if applicable.

#### Source

pkg/ridb\_rust.d.ts:429

***

### minLength?

> `optional` `readonly` **minLength**: `number`

The minimum length for string-type properties, if applicable.

#### Source

pkg/ridb\_rust.d.ts:439

***

### primaryKey?

> `optional` `readonly` **primaryKey**: `string`

The primary key of the property, if applicable.

#### Source

pkg/ridb\_rust.d.ts:414

***

### properties?

> `optional` `readonly` **properties**: `object`

An optional map of nested properties for object-type properties.

#### Index signature

 \[`name`: `string`\]: [`Property`](Property.md)

#### Source

pkg/ridb\_rust.d.ts:449

***

### required?

> `optional` `readonly` **required**: `string`[]

An optional array of required fields for object-type properties.

#### Source

pkg/ridb\_rust.d.ts:444

***

### type

> `readonly` **type**: `string`

The type of the property.

#### Source

pkg/ridb\_rust.d.ts:404

***

### version?

> `optional` `readonly` **version**: `number`

The version of the property, if applicable.

#### Source

pkg/ridb\_rust.d.ts:409
