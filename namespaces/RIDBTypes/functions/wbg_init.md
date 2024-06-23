[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / \_\_wbg\_init

# Function: \_\_wbg\_init()

> **\_\_wbg\_init**(`module_or_path`?): `Promise`\<[`InitOutput`](../interfaces/InitOutput.md)\>

If `module_or_path` is {RequestInfo} or {URL}, makes a request and
for everything else, calls `WebAssembly.instantiate` directly.

## Parameters

• **module\_or\_path?**: [`InitInput`](../type-aliases/InitInput.md) \| `Promise`\<[`InitInput`](../type-aliases/InitInput.md)\>

## Returns

`Promise`\<[`InitOutput`](../interfaces/InitOutput.md)\>

## Defined in

pkg/ridb\_rust.d.ts:683
