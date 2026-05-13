## Proxy-Wasm plugin example: Envoy TCP Routing

Proxy-Wasm TCP filter that dynamically routes connections to different upstream clusters based on the source IP address.

This example operates at the [TCP stream context](https://github.com/proxy-wasm/spec/tree/main/abi-versions/v0.2.1#tcp-streams) (L4) rather than the [HTTP application layer](https://github.com/proxy-wasm/spec/tree/main/abi-versions/v0.2.1#http-streams), making it useful for use cases where application-layer processing should be avoided for performance or protocol-agnostic routing decisions.

This example is inspired by the [wasmerang](https://github.com/SiiiTschiii/wasmerang) project, which demonstrates advanced TCP routing patterns in Envoy/Istio/K8s using WASM filters.

### Overview

This example demonstrates how to build a TCP filter that:

- Inspects incoming TCP connections
- Extracts the source IP address
- Routes traffic to different upstream clusters based on whether the last octet is even or odd
  - **Even last octet** → routes to `egress-router1`
  - **Odd last octet** → routes to `egress-router2`

The filter uses Envoy's `set_envoy_filter_state` foreign function to dynamically override the TCP proxy cluster at runtime, requiring proper protobuf encoding via the included `set_envoy_filter_state.proto` file.

### Building

Build the WASM plugin from the example directory:

```sh
$ cargo build --target wasm32-wasip1 --release
```

### Running with Docker Compose

This example can be run with [`docker compose`](https://docs.docker.com/compose/install/) and has a matching Envoy configuration.

From the example directory:

```sh
$ docker compose up
```

### Test the Routing

In a separate terminal test the routing behavior with different source IP addresses:

```bash
# Even IP (last octet 10) → routes to egress-router1
docker run --rm -it --network envoy_tcp_routing_envoy-tcp-routing --ip 172.22.0.10 curlimages/curl curl http://envoy:10000/ip -H "Host: httpbin.org"

# Odd IP (last octet 11) → routes to egress-router2
docker run --rm -it --network envoy_tcp_routing_envoy-tcp-routing --ip 172.22.0.11 curlimages/curl curl http://envoy:10000/ip -H "Host: httpbin.org"
```

### Expected Output

With `docker compose up` running in the foreground, Envoy logs show the selected upstream cluster:

**For even IP (last octet 10) → routes via egress-router1:**

```
envoy-1   | [TCP WASM] Routed source 172.22.0.10:39484 to egress-router1
envoy-1   | [2026-05-13T03:08:18.423Z] cluster=egress-router1 src=172.22.0.10:39484 dst=172.22.0.2:10000 -> 172.22.0.3:80
```

**For odd IP (last octet 11) → routes via egress-router2:**

```
envoy-1   | [TCP WASM] Routed source 172.22.0.11:55320 to egress-router2
envoy-1   | [2026-05-13T03:08:39.974Z] cluster=egress-router2 src=172.22.0.11:55320 dst=172.22.0.2:10000 -> 172.22.0.3:80
```

The access log confirms that traffic is routed to the expected cluster (`egress-router1` for even IPs, `egress-router2` for odd IPs).

### Note on Destination Information

In this example, both Envoy clusters (`egress-router1` and `egress-router2`) target the local `httpbin` service. In deployments where original destination metadata must be preserved across TCP hops, the [PROXY protocol](https://www.haproxy.org/download/1.8/doc/proxy-protocol.txt) is one practical option.
