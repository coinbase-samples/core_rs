# core_rs

A Rust library for building clients and utilities for the Coinbase Prime REST APIs.

## Overview

The `core_rs` crate provides foundational HTTP, authentication, and error handling utilities for interacting with Coinbase Prime APIs. It is designed to be used as a building block for higher-level SDKs and applications.

## Features

- HTTP client abstraction (async, based on `reqwest`)
- Typed request/response handling
- Credential management
- Error handling with `thiserror`
- Utilities for HTTP methods, headers, and status codes

## Setup

### 1. Clone the Repository

```sh
git clone git@github.com:coinbase-samples/core-rs.git
cd core_rs
```

### 2. Build the Library

To build the library, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed (Rust 1.61+ recommended):

```sh
cargo build
```

## License

This project is licensed under the [Apache-2.0 license](LICENSE). 