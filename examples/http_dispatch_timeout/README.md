## Proxy-Wasm plugin example: HTTP dispatch timeout

Proxy-Wasm plugin that logs HTTP request/response headers.

### Building and Testing

```sh
$ rustup toolchain install nightly
$ rustup target add wasm32-wasip1
$ cargo build --target wasm32-wasip1 --release
$ cp target/wasm32-wasip1/release/proxy_wasm_http_dispatch_timeout.wasm /tmp/

$ # assuming envoy installed locally, Also tested from company fork of envoy.
$ envoy -c envoy-timeout.yaml --log-level info

$ # start cluster a
$ alias docker=podman
$ docker run -p 8081:80 kennethreitz/httpbin

$ # curl request to make a sidecall with delay of 5s but timeout of 1s.
$ curl -v http://localhost:10000/timeout

```

### Using in Envoy

Send HTTP request to `localhost:10000/timeout`:

Envoy timeout logs in on_http_response
```log
wasm log: #2 http call response :status: 504
wasm log: #2 http call response content-length: 24
wasm log: #2 http call response content-type: text/plain
wasm log: Response body: "upstream request timeout"
```
