# WASI Cryptography Examples

The purpose of this repo is to show how to use different elements of the wasi-crypto interface using the Rust binding implementation from [wasi-crypto](https://github.com/WebAssembly/wasi-crypto). It aims to build on some of the examples shown in the test code in the wasi-crypto repo by showing some additional use cases, for example, using the symmetric crypto interface for performing session based cryptography.

## Prerequisites

- Rust (nightly)
- [cargo wasi](https://github.com/bytecodealliance/cargo-wasi): `cargo install cargo-wasi`
- Wasi target support: `rustup target add wasm32-wasi`
- [Wasmtime](https://github.com/bytecodealliance/wasmtime)
- Compile Wasmtime from source: `cargo build --release --features wasi-crypto` to include the optional wasi-crypto feature.

## Compiling

`cargo wasi build`

## Running

`CARGO_TARGET_WASM32_WASI_RUNNER="wasmtime run --wasi-modules experimental-wasi-crypto" cargo wasi run`
