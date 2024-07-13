## Proxy-Wasm plugin example: HTTP headers

Proxy-Wasm plugin that logs HTTP request/response headers.

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

Send HTTP request to `localhost:10000/`:

```sh
$ curl localhost:10000/ -H "x-envoy-wasm-metric-value: 100" -H "x-envoy-wasm-metric: gauge"
```

For instance that request will set the example gauge to 100. Which you can see using the stats endpoint

```sh
& curl -s localhost:9001/stats | grep wasmcustom.wasm_gauge

100
```
