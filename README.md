# core_rs

> **Deprecated:** This repository (`coinbase-samples/core_rs`) is deprecated. Active development and releases continue at **[coinbase/core_rs](https://github.com/coinbase/core_rs)**. New projects should use **`core_rs` 0.2.0+** from [crates.io](https://crates.io/crates/core_rs) once that version is published.

A Rust library for building clients and utilities for the Coinbase Prime REST APIs.

## Overview

The `core_rs` crate provides foundational HTTP, authentication, and error handling utilities for interacting with Coinbase Prime APIs. It is designed to be used as a building block for higher-level SDKs and applications.

## Features

- HTTP client abstraction (async, based on `reqwest`)
- Typed request/response handling
- Credential management
- Error handling with `thiserror`
- Utilities for HTTP methods, headers, and status codes

## Installation

Add the library from [crates.io](https://crates.io/crates/core_rs):

```sh
cargo add core_rs
```

Or add it to your `Cargo.toml`:

```toml
[dependencies]
core_rs = "0.1"
```

## Setup

### 1. Clone the Repository

```sh
git clone git@github.com:coinbase-samples/core_rs.git
cd core_rs
```

### 2. Build the Library

To build the library, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed (Rust 1.61+ recommended):

```sh
cargo build
```

## License

This project is licensed under the [Apache-2.0 license](LICENSE).
