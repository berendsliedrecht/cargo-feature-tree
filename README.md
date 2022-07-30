# Cargo Feature Tree

List your features in a expanded and tree-like format

```console
λ cargo feature-tree
┣━━ base58_rust_base58
┃   ┗━ rust-base58
┣━━ default
┃   ┣━ base58_rust_base58
┃   ┃   ┗━ rust-base58
┃   ┣━ pair_amcl
┃   ┃   ┗━ ursa
┃   ┣━ local_nodes_pool
┃   ┗━ revocation_tests
┣━━ fatal_warnings
┣━━ force_full_interaction_tests
┣━━ local_nodes_pool
┣━━ only_high_cases
┣━━ pair_amcl
┃   ┗━ ursa
┣━━ revocation_tests
┗━━ sodium_static
```

## Installation

```console
cargo install cargo-feature-tree
```

## Usage

```
cargo feature-tree

cargo feature-tree path-to-folder-containing-cargo.toml

cargo feature-tree path-to-explicit-cargo.toml
```
