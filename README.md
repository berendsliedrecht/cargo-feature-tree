# Cargo Feature Tree

List your features in a expanded and tree-like format

```console
λ cargo feature-tree
zmix
├── PS_Signature_G1
├── PS_Signature_G2
├── asm
│   └── std
│       ├── ver_enc
│       └── PS_Signature_G2
├── default
│   └── std
│       ├── ver_enc
│       └── PS_Signature_G2
├── portable
│   └── std
│       ├── ver_enc
│       └── PS_Signature_G2
├── std
│   ├── ver_enc
│   └── PS_Signature_G2
└── ver_enc
```

```console
λ cargo feature-tree default
zmix
└── default
    └── std
        ├── ver_enc
        └── PS_Signature_G2
```

## Installation

```console
cargo install cargo-feature-tree
```

## Usage

```
cargo feature-tree <FEATURE>

cargo feature-tree path-to-folder-containing-cargo.toml

cargo feature-tree path-to-explicit-cargo.toml
```
