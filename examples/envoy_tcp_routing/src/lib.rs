// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! TCP Rerouting Example
//!
//! This example demonstrates dynamic TCP routing based on source IP address.
//! Inspired by https://github.com/SiiiTschiii/wasmerang
//!
//! The filter inspects TCP connections and routes them to different upstream
//! clusters based on whether the last byte of the source IP is even or odd:
//! - Even last octet → egress-router1
//! - Odd last octet → egress-router2

use std::net::{IpAddr, SocketAddr};

use log::{info, warn};
use prost::Message;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use set_envoy_filter_state::{LifeSpan, SetEnvoyFilterStateArguments};

// Include the generated protobuf code
pub mod set_envoy_filter_state {
    include!("generated/envoy.source.extensions.common.wasm.rs");
}

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(TcpReroutingRoot)
    });
}}

struct TcpReroutingRoot;

impl Context for TcpReroutingRoot {}

impl RootContext for TcpReroutingRoot {
    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            info!(
                "[TCP WASM] Configuration: {:?}",
                std::str::from_utf8(&config_bytes).unwrap_or("invalid UTF-8")
            );
        }
        true
    }

    fn create_stream_context(&self, _context_id: u32) -> Option<Box<dyn StreamContext>> {
        Some(Box::new(TcpReroutingStream))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::StreamContext)
    }
}

struct TcpReroutingStream;

impl Context for TcpReroutingStream {}

impl StreamContext for TcpReroutingStream {
    fn on_new_connection(&mut self) -> Action {
        let Some(source_addr_bytes) = self.get_property(vec!["source", "address"]) else {
            warn!("[TCP WASM] Missing source address property");
            return Action::Continue;
        };

        let Ok(source_addr) = std::str::from_utf8(&source_addr_bytes) else {
            warn!("[TCP WASM] Source address is not valid UTF-8");
            return Action::Continue;
        };

        let Some(last_byte) = extract_last_ip_byte(source_addr) else {
            warn!(
                "[TCP WASM] Failed to parse source address for routing: {}",
                source_addr
            );
            return Action::Continue;
        };

        let cluster = select_cluster(last_byte);
        let args = SetEnvoyFilterStateArguments {
            path: "envoy.tcp_proxy.cluster".to_string(),
            value: cluster.to_string(),
            span: LifeSpan::FilterChain as i32,
        };

        let mut buf = Vec::new();
        if let Err(err) = args.encode(&mut buf) {
            warn!("[TCP WASM] Failed to encode filter state: {}", err);
            return Action::Continue;
        }

        if let Err(err) = self.call_foreign_function("set_envoy_filter_state", Some(&buf)) {
            warn!("[TCP WASM] Failed to set Envoy filter state: {:?}", err);
            return Action::Continue;
        }

        info!("[TCP WASM] Routed source {} to {}", source_addr, cluster);
        Action::Continue
    }
}

fn select_cluster(last_byte: u8) -> &'static str {
    if last_byte.is_multiple_of(2) {
        "egress-router1"
    } else {
        "egress-router2"
    }
}

/// Extracts the last byte from a source address.
///
/// Handles IPv4/IPv6 addresses with and without port numbers.
/// Examples:
/// - "192.168.1.10" → Some(10)
/// - "192.168.1.10:8080" → Some(10)
/// - "172.21.0.11:58762" → Some(11)
/// - "[2001:db8::1]:8080" → Some(1)
fn extract_last_ip_byte(addr: &str) -> Option<u8> {
    let ip = if let Ok(socket_addr) = addr.parse::<SocketAddr>() {
        socket_addr.ip()
    } else {
        addr.parse::<IpAddr>().ok()?
    };

    Some(match ip {
        IpAddr::V4(ipv4) => ipv4.octets()[3],
        IpAddr::V6(ipv6) => ipv6.octets()[15],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_last_ip_byte() {
        assert_eq!(extract_last_ip_byte("192.168.1.10"), Some(10));
        assert_eq!(extract_last_ip_byte("192.168.1.10:8080"), Some(10));
        assert_eq!(extract_last_ip_byte("172.21.0.11:58762"), Some(11));
        assert_eq!(extract_last_ip_byte("10.0.0.2"), Some(2));
        assert_eq!(extract_last_ip_byte("2001:db8::2"), Some(2));
        assert_eq!(extract_last_ip_byte("[2001:db8::3]:8080"), Some(3));
        assert_eq!(extract_last_ip_byte("invalid"), None);
        assert_eq!(extract_last_ip_byte(""), None);
    }

    #[test]
    fn test_routing_logic() {
        assert_eq!(select_cluster(10), "egress-router1");
        assert_eq!(select_cluster(11), "egress-router2");
    }
}
