# ReDeFi Relay Parachain

## Project Description

Custom parachain for ReDeFi network [relay](https://github.com/ReDeFi-Blockchain/redefi-relay-runtime) with EVM compatibility support based on the Unqiue Network Frontier [fork](https://github.com/UniqueNetwork/unique-frontier).

## Rust compiler versions

This release was built and tested against the following versions of rustc.

```
Rust Nightly: rustc 1.79.0-nightly (805813650 2024-03-31)
```

Other versions may work.
Note: add targets:

```bash
rustup target add wasm32-unknown-unknown 
rustup target add x86_64-unknown-linux-musl
```

## Build

Build the runtime by cloning this repository and running the following commands from the root directory of the repo:

```bash
 cargo build --profile=production
```
