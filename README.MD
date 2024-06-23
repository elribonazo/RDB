**@elribonazo/ridb** • **Docs**

***

# @elribonazo/ridb

## What is RIDB
RIDB is a package that can be used in browsers and nodejs in order to build databases.
The project started after years of experience working with web projects in both browser and nodejs platforms, the project was born with some rules / objectives:
1. Strong types + proper validation 
2. Declarative schemas & documents
3. Configurable storages, inMemory, monogoDB, sqlite, indexdb
4. Secure encryption
5. Work seamlessly in browsers or nodejs applications.

## Install

npm: 
``` 
npm i @elribonazo/ridb --save
```

yarn:

``` 
yarn add @elribonazo/ridb
```

## Usage

### Database

In CommonJS Modules:  

```javascript
const {
    RIDB,
    SchemaFieldType
} = require('@elribonazo/ridb');

(async () => {
    const db =  new RIDB({
        demo: {
            version: 0,
            primaryKey: 'id',
            type: SchemaFieldType.object,
            properties: {
                id: {
                    type: SchemaFieldType.string,
                    maxLength: 60
                }
            }
        }
    });
    console.log("Starting the database");
    await db.start();
    console.log("Ok :)");
})()
```

In ES Modules, TypeScript, etc:

```javascript
import {
    RIDB,
    SchemaFieldType
} from '@elribonazo/ridb';

(async () => {
    const db =  new RIDB({
        demo: {
            version: 0,
            primaryKey: 'id',
            type: SchemaFieldType.object,
            properties: {
                id: {
                    type: SchemaFieldType.string,
                    maxLength: 60
                }
            }
        }
    });
    console.log("Starting the database");
    await db.start();
    console.log("Ok :)");
})()
```

## How to run tests

```bash
cd ts 
npm i
npm run test
```

## How to build this project
Build requirements:
* Bash
* Have Rust ([cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)) installed.
* Node JS Version (20/LTS Recommended)

```bash
cd ts 
npm i
npm run build
```

## How to test the project
For now, we have enabled the implementation of the whole wasm + javascript integration.
In order to run it, write the following:

```bash
cd ts 
npm i
npm run test
```

## Namespaces

- [RIDBTypes](namespaces/RIDBTypes/README.md)

## Classes

- [RIDB](classes/RIDB.md)
