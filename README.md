# crame

A build tool for c projects, with a small testing framework.

[![Crates.io](https://img.shields.io/crates/v/crame.svg)](https://crates.io/crates/crame)
[![tests](https://github.com/sonro/crame/actions/workflows/tests.yml/badge.svg?branch=main)](https://github.com/sonro/crame/actions/workflows/tests.yml)
[![license](https://img.shields.io/crates/l/crame.svg)](#license)

- [Rationale](#rationale)
- [Installation](#installation)
- [How to use](#how-to-use)

## Features

- Create a c project with a simple directory structure, main file, and test
  runner.
- Automatically initializes a git repository.
- Dependency free test runner.
- Build, run, test, watch, and add code modules with [Just].

### Planned features

- Move builder, runner, tester, and watcher out of justfile and into the
  application.
- Configurable build options.
- Add code modules with tests already setup.

## Rationale

This CLI tool is intended to give a
[Cargo](https://github.com/rust-lang/cargo/)-like experience for simple
c projects. It's primary goals are to reduce the tedium of writing a makefile,
adding and running tests, and adding additional source files with proper
include guards.

The framework currently relies on [Just] for most of its functionality. The
basis of this application is the [justfile].

### Why write it in Rust?

As the project is inspired by Cargo and other modern development tools, it
seems natural to use Rust and its plethora of CLI libraries.

## Installation

### Prerequisites

crame projects currently use [Just] as their main build tool and test runner.
The [justfile] is also dependent on [fd](https://github.com/sharkdp/fd) and
requires [watchexec](https://github.com/watchexec/watchexec) for file watching.

A c compiler linked to the `cc` executable is also required.

### Cargo

If you're a **Rust programmer**, crame can be installed with `cargo`.

```sh
cargo install crame
```

### Building

crame is written in Rust, so you'll need to grab a
[Rust installation](https://www.rust-lang.org/) in order to compile it.

To build crame:

```sh
git clone https://github.com/sonro/crame
cd crame
cargo build --release
./target/release/crame
```

## How to use

### Create a project

Using `crame new` creates a project in the specified directory. Automatically
intializes a git repository, unless the directory is already within one, or the
`--vcs` option is set to `none`.

```sh
crame new my-project
```

Resulting directory structure:

```tree
my-project
????????? Crame.toml
????????? justfile
????????? lib
????????? src
???   ????????? main.c
????????? tests
    ????????? run.c
    ????????? test_all.c
    ????????? unit
        ????????? it_works.c
```

### Building and running

Build the program as an executable in the `target/` directory.

```sh
just build
```

Build the program and then run it.

```sh
just run
```

The justfile will add all the `.c` files in `src/` and `lib/` as arguments for
the c compiler. It doesn't need to be kept up to date as with a makefile.

### Adding modules

Use `just add-module` to create `.c` and `.h` files in the `src` directory.

```sh
just add-module my_module
```

Resulting files:

```c
// src/my_module.h
#ifndef MY_PROJECT_MY_MODULE_H
#define MY_PROJECT_MY_MODULE_H

#endif
```

```c
// src/my_module.c
#include "my_module.h"
```

### Testing

Test files must have the follwing layout:

```c
#if defined HEADERS
// include all headers in this section
#elif defined TESTS
// create tests in this section
#endif
```

Add a unit test in `tests/unit/`. Use the `TEST` macro to specify a test name
and function and the `ASSERT` macro to test a Boolean value.

```c
// tests/unit/my_test.c
#if defined HEADERS
#include "../../src/my_module.h"
#elif defined TESTS

TEST("test name") {
    ASSERT(1 + 1 == 2);
}

#endif
```

Include your test files in `tests/test_all.c`.

```c
// tests/test_all.c
#include "unit/my_test.c"
...
```

Build and run all tests with

```ini
just test
```

This builds all `.c` files in the `src/`, `lib/` and `tests/` directories.

## License

crame is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

[Just]: https://github.com/casey/just
[justfile]: template/justfile
