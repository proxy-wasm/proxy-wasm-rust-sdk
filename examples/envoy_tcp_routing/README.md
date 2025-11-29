## Proxy-Wasm plugin example: Envoy TCP Routing

Proxy-Wasm TCP filter that dynamically routes connections to different upstream clusters based on the source IP address.

This example operates at the [TCP stream context](https://github.com/proxy-wasm/spec/tree/main/abi-versions/v0.2.1#tcp-streams) (L4) rather than the [HTTP application layer](https://github.com/proxy-wasm/spec/tree/main/abi-versions/v0.2.1#http-streams), making it useful for use cases where application-layer processing should be avoided for performance or protocol-agnostic routing decisions.

This example is inspired by the [wasmerang](https://github.com/SiiiTschiii/wasmerang) project, which demonstrates advanced TCP routing patterns in Envoy/Istio/K8s using WASM filters.

### Overview

This example demonstrates how to build a TCP filter that:

- Intercepts incoming TCP connections
- Extracts the source IP address
- Routes traffic to different upstream clusters based on whether the last octet is even or odd
  - **Even last octet** → routes to `egress-router1`
  - **Odd last octet** → routes to `egress-router2`

The filter uses Envoy's `set_envoy_filter_state` foreign function to dynamically override the TCP proxy cluster at runtime, requiring proper protobuf encoding via the included `set_envoy_filter_state.proto` file.

### Building

Build the WASM plugin from the example directory:

```sh
$ cargo build --target wasm32-wasi --release
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
docker run --rm -it --network envoy_tcp_routing_envoymesh --ip 172.22.0.10 curlimages/curl curl http://proxy:10000/ip -H "Host: httpbin.org"

# Odd IP (last octet 11) → routes to egress-router2
docker run --rm -it --network envoy_tcp_routing_envoymesh --ip 172.22.0.11 curlimages/curl curl http://proxy:10000/ip -H "Host: httpbin.org"
```

### Expected Output

Check the Docker Compose logs to see the WASM filter in action:

```console
$ docker compose logs -f
```

**For even IP (last octet 10) → routes via egress-router1:**

```
proxy-1  | [TCP WASM] Source address: 172.22.0.10:39484
proxy-1  | [TCP WASM] Source IP last octet: 10, intercepting ALL traffic
proxy-1  | [TCP WASM] Routing to egress-router1
proxy-1  | [TCP WASM] set_envoy_filter_state status (envoy.tcp_proxy.cluster): Ok(None)
proxy-1  | [TCP WASM] Rerouting to egress-router1 via filter state
proxy-1  | [2025-11-20T03:08:18.423Z] cluster=egress-router1 src=172.22.0.10:39484 dst=172.22.0.2:10000 -> 35.170.145.70:80
```

**For odd IP (last octet 11) → routes via egress-router2:**

```
proxy-1  | [TCP WASM] Source address: 172.22.0.11:55320
proxy-1  | [TCP WASM] Source IP last octet: 11, intercepting ALL traffic
proxy-1  | [TCP WASM] Routing to egress-router2
proxy-1  | [TCP WASM] set_envoy_filter_state status (envoy.tcp_proxy.cluster): Ok(None)
proxy-1  | [TCP WASM] Rerouting to egress-router2 via filter state
proxy-1  | [2025-11-20T03:08:39.974Z] cluster=egress-router2 src=172.22.0.11:55320 dst=172.22.0.2:10000 -> 52.44.182.178:80
```

The `Ok(None)` status confirms that the filter state was successfully set, and you can see in the access logs that traffic is being routed to the correct clusters (`egress-router1` for even IPs, `egress-router2` for odd IPs).

### Note on Destination Information

In this example, both Envoy clusters (`egress-router1` and `egress-router2`) have `httpbin.org` hardcoded as their load balancer endpoints.

In real world scenarios where destination information needs to be preserved across TCP hops, the [PROXY protocol](https://www.haproxy.org/download/1.8/doc/proxy-protocol.txt) could be used to forward metadata like the original source and destination addresses between proxies. This could be implemented using advanced WASM stream context capabilities combined with Envoy's PROXY protocol configuration.

**Future Enhancement:** With Istio (Envoy-based) in a Kubernetes setup and PROXY protocol support, this example could be extended to serve as a source-based egress router. One practical use case would be routing user's web traffic via different external IP addresses based on source-based routing decisions (e.g., even vs. odd source IP like in the example). In conventional networks, this would be achieved with routing tables and IP rules to select egress interfaces based on source addresses. However, this approach is close to impossible with managed Kubernetes services (without additional network plugins like [Multus CNI](https://github.com/k8snetworkplumbingwg/multus-cni), as Kubernetes largely abstracts away the network layer where such layer 3 routing (different outgoing IPs) would normally be configured.
