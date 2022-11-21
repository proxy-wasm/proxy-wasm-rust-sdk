# WebAssembly for Proxies (Rust SDK)

[![Build Status][build-badge]][build-link]
[![Crate][crate-badge]][crate-link]
[![Documentation][docs-badge]][docs-link]
[![Apache 2.0 License][license-badge]][license-link]

[build-badge]: https://github.com/proxy-wasm/proxy-wasm-rust-sdk/workflows/Rust/badge.svg?branch=master
[build-link]: https://github.com/proxy-wasm/proxy-wasm-rust-sdk/actions?query=workflow%3ARust+branch%3Amaster
[crate-badge]: https://img.shields.io/crates/v/proxy-wasm.svg
[crate-link]: https://crates.io/crates/proxy-wasm
[docs-badge]: https://docs.rs/proxy-wasm/badge.svg
[docs-link]: https://docs.rs/proxy-wasm
[license-badge]: https://img.shields.io/github/license/proxy-wasm/proxy-wasm-rust-sdk
[license-link]: https://github.com/proxy-wasm/proxy-wasm-rust-sdk/blob/master/LICENSE

## Examples

- [Hello World](./examples/hello_world.rs)
- [HTTP Auth random](./examples/http_auth_random.rs)
- [HTTP Headers](./examples/http_headers.rs)
- [HTTP Response body](./examples/http_body.rs)
- [HTTP Configuration](./examples/http_config.rs)

### Using Proxy-Wasm plugins in Envoy

Each example can be run with `docker compose` and has a matching Envoy configuration. These instruction require that both
[Rust](https://www.rust-lang.org/) and [docker compose](https://docker-docs.netlify.app/compose/install/) are installed.

1. Build the WASM filter, change the environment variable to the folder name of the example:
   ```sh
   export EXAMPLE_DIR=hello_world
   cargo build --target wasm32-wasi --manifest-path=examples/${EXAMPLE_DIR}/Cargo.toml
   ```
2. Run the example:
   ```sh
   docker compose up --file examples/docker-compose.yaml -e EXAMPLE=${EXAMPLE_DIR}
   ```

## Articles & blog posts from the community

- [Extending Envoy with WASM and Rust](https://antweiss.com/blog/extending-envoy-with-wasm-and-rust/)
- [Writing Envoy filters in Rust with WebAssembly](https://content.red-badger.com/resources/extending-istio-with-rust-and-webassembly)

## Updating dependencies

When updating dependencies, you need to regenerate Bazel `BUILD` files to match updated `Cargo.toml`:

```sh
cargo install cargo-raze --version 0.15.0
cargo raze --generate-lockfile
```
