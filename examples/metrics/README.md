## Proxy-Wasm plugin example: Metrics

Proxy-Wasm plugin that demonstrates how to define and update metrics.

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

Send HTTP requests to `localhost:10000/headers`:


Retrieve metrics from `localhost:9901/stats`:

```console
$ curl -s localhost:9901/stats | grep 'example'
wasmcustom.conter_example: 1
wasmcustom.gauge_example: 10
wasmcustom.histogram_example: P0(nan,8) P25(nan,8.075) P50(nan,9.05) P75(nan,10.25) P90(nan,10.7) P95(nan,10.85) P99(nan,10.969999999999999) P99.5(nan,10.985) P99.9(nan,10.997) P100(nan,11)
```
