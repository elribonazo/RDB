**@elribonazo/ridb** • **Docs**

***

# @elribonazo/ridb

## Description

## What is RIDB
RIDB is a package that can be used in browsers and nodejs in order to build databases.
The project started after years of experience working with web projects in both browser and nodejs platforms, the project was born with some rules / objectives:
1. Strong types + proper validation 
2. Declarative schemas & documents
3. Configurable storages, inMemory, monogoDB, sqlite, indexdb
4. Secure encryption
5. Work seemlessly in browsers or nodejs applications.

## How to build this project
Build requirements:
* Bash
* Have Rust ([cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed.
* Node JS Version (20/LTS Recommended)

```bash
cd ts 
npm i
npm run build
```

## How to test the project
For now have enabled the implementation of the whole wasm + javascript integration.
In order to run it, write the following:

```bash
cd ts 
npm i
npm run test
```

```typescript
import { createLevelDBStorage } from "@pluto-encrypted/leveldb";
import { Database } from "@pluto-encrypted/database";
//default password must be 32 bytes long
const defaultPassword = new Uint8Array(32).fill(1);
const database = db = await Database.createEncrypted({
         name: `my-db`,
         encryptionKey: defaultPassword,
         storage: createLevelDBStorage({
         dbName: "demo",
         dbPath: "/tmp/demo"
      })
});
```

## Namespaces

- [RIDBTypes](namespaces/RIDBTypes/README.md)

## Classes

- [RIDB](classes/RIDB.md)
