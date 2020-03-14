# Calling rust code from a svelte app via wasm
## how it works
Uses `wasm-pack` and `wasm-pack-plugin` to compire Rust code to wasm and load it in the svelte project

Had to use webpack instead of rollup due to more mature wasm support

## How to install

```sh
npm install
```

## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm run dev
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

## How to run unit tests

```sh
# Runs tests in Firefox
npm test -- --firefox

# Runs tests in Chrome
npm test -- --chrome

# Runs tests in Safari
npm test -- --safari
```
