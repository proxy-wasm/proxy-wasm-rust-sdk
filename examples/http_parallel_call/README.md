## Proxy-Wasm plugin example: HTTP parallel call

Proxy-Wasm plugin that makes multiply HTTP callout and combine responses as final response .

### Building

```sh
$ cargo build --target wasm32-wasi --release
```

### Using in Envoy

This example can be run with [`docker compose`](https://docs.docker.com/compose/install/)
and has a matching Envoy configuration.

```sh
$ docker compose up
```

#### Access granted.

Send HTTP request to `localhost:10000/headers`:

```sh
$ curl localhost:10000/headers
Hello, World!\n
```
