## Proxy-Wasm plugin example: Envoy TCP routing

Proxy-Wasm plugin that dynamically routes connections to different upstream clusters based on the source IP address.

This example operates at the [TCP stream context](https://github.com/proxy-wasm/spec/tree/main/abi-versions/v0.2.1#tcp-streams) (L4)
rather than the [HTTP application layer](https://github.com/proxy-wasm/spec/tree/main/abi-versions/v0.2.1#http-streams), making it
useful for use cases where application-layer processing should be avoided for performance or protocol-agnostic routing decisions.

This example is inspired by the [wasmerang](https://github.com/SiiiTschiii/wasmerang) project, which demonstrates advanced TCP routing
patterns in Envoy/Istio/K8s using Wasm filters.

### Overview

This example demonstrates how to build a Proxy-Wasm plugin that:

- Inspects incoming TCP connections
- Retrieves the source IP address
- Routes traffic to different upstream clusters based on a custom logic:
  - **Even last octet** → routes to `egress-router1`
  - **Odd last octet** → routes to `egress-router2`

This plugin uses Envoy's `set_envoy_filter_state` foreign function to dynamically override the default TCP proxy cluster at runtime,
requiring proper protobuf encoding via the included `set_envoy_filter_state.proto` file.

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

#### Even source IP (last octet 10)

Send HTTP request to `envoy:10000/ip` using client IP 127.22.0.10:

```sh
$ docker run --rm -it --network envoy_tcp_routing_envoy-tcp-routing --ip 172.22.0.10 curlimages/curl curl http://envoy:10000/ip -H "Host: httpbin.org"
{
  "origin": "172.22.0.3"
}
```

Expected Envoy logs:

```console
[...] [TCP WASM] Routed source 172.22.0.10:42148 to egress-router1
[...] src=172.22.0.10:42148 via=172.22.0.3:10000 dst=172.22.0.2:8080 (egress-router1)
```

#### Odd source IP (last octet 11)

Send HTTP request to `envoy:10000/ip` using client IP 127.22.0.11:

```sh
$ docker run --rm -it --network envoy_tcp_routing_envoy-tcp-routing --ip 172.22.0.11 curlimages/curl curl http://envoy:10000/ip -H "Host: httpbin.org"
{
  "origin": "172.22.0.3"
}
```

Expected Envoy logs:

```console
[...] [TCP WASM] Routed source 172.22.0.11:35736 to egress-router2
[...] src=172.22.0.11:35736 via=172.22.0.3:10000 dst=172.22.0.2:8080 (egress-router2)
```

The access log confirms that traffic is routed to the expected cluster (`egress-router1` for even IPs, `egress-router2` for odd IPs).
