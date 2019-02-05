# tic-tac-toe in rust/wasm


[![Demo](https://img.shields.io/badge/demo-online-blue.svg)](https://sepiropht.github.io/tic-tac-toe-wasm/)

A Rust/Wasm implementation of Tic Tac Toe.

The ui is stil in plain javascript.

There is also js implementation of the ai for comparaison with wasm.

It use a Monte Carlo simulation to power the Ai


## Prerequisites

Rust / Node / npm

Currently, only the nightly toolchain of Rust is supporting WebAssembly:

```shell
rustup default nightly
rustup target add wasm32-unknown-unknown
```

## Install

Make sure you have `cargo install wasm-pack`, then:

```shell
npm install
```

## Run

```shell
npm start
```

## Test

Run the rust tests

```shell
npm run test:rust
```
