## Proxy-Wasm plugin example: HTTP retry

Proxy-Wasm plugin that logs HTTP request/response headers.

### Building and Testing

```sh
$ rustup toolchain install nightly
$ rustup target add wasm32-wasip1
$ cargo build --target wasm32-wasip1 --release
$ cp target/wasm32-wasip1/release/proxy_wasm_example_http_headers.wasm /tmp/

$ # assuming envoy installed locally, Also tested from company fork of envoy.
$ envoy -c envoy-retry.yaml --log-level info

$ start cluster a and b
$ alias docker=podman
$ docker run -p 8081:80 kennethreitz/httpbin
$ docker run -p 8082:80 kennethreitz/httpbin

$ # make a 302 call which will trigger the retry flow.
$ curl http://localhost:10000/status/302 --headers "host=httpbin"

$ # make a 302 status call with some body
$ curl http://localhost:10000/status/302 --data '{"data": "original_data"}' -H "content-type: application/json"

```

### Using in Envoy

This example can be run with [`docker compose`](https://docs.docker.com/compose/install/)
and has a matching Envoy configuration.

```sh
$ docker compose up
```

Send HTTP request to `localhost:10000/hello`:

```sh
$ curl localhost:10000/hello
Hello, World!
```

Expected Envoy logs:

```console
[...] wasm log http_headers: #2 -> :authority: localhost:10000
[...] wasm log http_headers: #2 -> :path: /hello
[...] wasm log http_headers: #2 -> :method: GET
[...] wasm log http_headers: #2 -> :scheme: http
[...] wasm log http_headers: #2 -> user-agent: curl/7.81.0
[...] wasm log http_headers: #2 -> accept: */*
[...] wasm log http_headers: #2 -> x-forwarded-proto: http
[...] wasm log http_headers: #2 -> x-request-id: 3ed6eb3b-ddce-4fdc-8862-ddb8f168d406
[...] wasm log http_headers: #2 <- :status: 200
[...] wasm log http_headers: #2 <- hello: World
[...] wasm log http_headers: #2 <- powered-by: proxy-wasm
[...] wasm log http_headers: #2 <- content-length: 14
[...] wasm log http_headers: #2 <- content-type: text/plain
[...] wasm log http_headers: #2 completed.
```
