# crame

A build tool for c projects, with a small testing framework.

[![Crates.io](https://img.shields.io/crates/v/crame.svg)](https://crates.io/crates/crame)
[![tests](https://github.com/sonro/crame/actions/workflows/tests.yml/badge.svg?branch=main)](https://github.com/sonro/crame/actions/workflows/tests.yml)
[![license](https://img.shields.io/crates/l/crame.svg)](#license)

## Installation

### Prerequisites

`crame` projects currently use [Just](https://github.com/casey/just) as their
main build tool and test runner. The [justfile](template/justfile) is also
dependent on [fd](https://github.com/sharkdp/fd) and requires
[watchexec](https://github.com/watchexec/watchexec) for file watching.

### Cargo

If you're a **Rust programmer**, crame can be installed with `cargo`.

```sh
cargo install crame
```

### Building

`crame` is written in Rust, so you'll need to grab a
[Rust installation](https://www.rust-lang.org/) in order to compile it.

To build crame:

```sh
git clone https://github.com/sonro/crame
cd crame
cargo build --release
./target/release/crame
```

## License

`crame` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
