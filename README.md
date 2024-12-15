# SP1 Keccak Preimage

## Overview

An example of using the Keccak precompile in a [SP1](https://github.com/succinctlabs/sp1) program.

- `/program`: The SP1 program.
- `/script`: Script for generating proofs.

The patch for the `tiny-keccak` crate can be found in the [program/Cargo.toml](./program/Cargo.toml) file.

```toml
[patch.crates-io]
tiny-keccak = { git = "https://github.com/sp1-patches/tiny-keccak", branch = "patch-v2.0.2" }
```

## Generate a proof using the prover network

1. `cd ./script`
2. `SP1_PROVER=network SP1_PRIVATE_KEY=<your_key> RUST_LOG=info cargo run --release`

Requires having a valid `SP1_PRIVATE_KEY` value. For more information, see the [prover
network](https://docs.succinct.xyz/docs/generating-proofs/prover-network/key-setup) docs.

## Generate a proof locally

1. `cd ./script`
2. `RUST_LOG=info cargo run --release`

Depending on if creating a `core`, `compressed`, or `plonk` proof, the hardware requirements will
vary. For more information, see the [prover
options](https://github.com/succinctlabs/sp1/blob/dev/book/generating-proofs/prover-options.md) and
[hardware
requirements](https://github.com/succinctlabs/sp1/blob/dev/book/getting-started/hardware-requirements.md)
docs.
