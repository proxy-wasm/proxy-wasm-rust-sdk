## Proxy-Wasm plugin example: Metadata

Proxy-Wasm plugin that demonstrates reading metadata set by other filters

### Building

```sh
$ cargo build --target wasm32-wasip1 --release
```

### Using in Envoy

This example can be run with [`docker compose`](https://docs.docker.com/compose/install/)
and has a matching Envoy configuration.

```sh
$ docker compose up
```

Send a HTTP request to `localhost:10000/` will return the configured response. 

```sh
$ curl localhost:10000/
Welcome, set the `x-custom-metadata` header to change the response!
```


Send a HTTP request to `localhost:10000/` with a `x-custom-metadata` header value.

```sh
$ curl localhost:10000/ -H "x-custom-metadata: some-value"
Custom response with `x-custom-metadata` value "some-value"
```
