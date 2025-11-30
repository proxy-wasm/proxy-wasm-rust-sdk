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
//! The filter intercepts TCP connections and routes them to different upstream
//! clusters based on whether the last octet of the source IP is even or odd:
//! - Even last octet → egress-router1
//! - Odd last octet → egress-router2

use log::{info, warn};
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

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
        if let Some(source_addr_bytes) = self.get_property(vec!["source", "address"]) {
            if let Ok(source_addr) = std::str::from_utf8(&source_addr_bytes) {
                info!("[TCP WASM] Source address: {}", source_addr);

                // Extract the last octet from the source IP address
                if let Some(last_octet) = extract_last_octet(source_addr) {
                    info!(
                        "[TCP WASM] Source IP last octet: {}, intercepting ALL traffic",
                        last_octet
                    );

                    // Determine target cluster based on even/odd last octet
                    let cluster = if last_octet % 2 == 0 {
                        "egress-router1"
                    } else {
                        "egress-router2"
                    };

                    info!("[TCP WASM] Routing to {}", cluster);

                    // Set the cluster via Envoy's filter state mechanism using proper protobuf encoding
                    use set_envoy_filter_state::{LifeSpan, SetEnvoyFilterStateArguments};

                    let args = SetEnvoyFilterStateArguments {
                        path: "envoy.tcp_proxy.cluster".to_string(),
                        value: cluster.to_string(),
                        span: LifeSpan::FilterChain as i32,
                    };

                    let mut buf = Vec::new();
                    if let Err(e) = prost::Message::encode(&args, &mut buf) {
                        warn!("[TCP WASM] Failed to encode filter state: {}", e);
                        return Action::Continue;
                    }

                    // Use the Envoy-specific filter state mechanism
                    // https://github.com/envoyproxy/envoy/issues/28128
                    let status = self.call_foreign_function("set_envoy_filter_state", Some(&buf));

                    info!(
                        "[TCP WASM] set_envoy_filter_state status (envoy.tcp_proxy.cluster): {:?}",
                        status
                    );
                    info!("[TCP WASM] Rerouting to {} via filter state", cluster);
                }
            }
        }
        Action::Continue
    }
}

/// Extracts the last octet from an IP address string.
///
/// Handles both IPv4 addresses with and without port numbers.
/// Examples:
/// - "192.168.1.10" → Some(10)
/// - "192.168.1.10:8080" → Some(10)
/// - "172.21.0.11:58762" → Some(11)
fn extract_last_octet(addr: &str) -> Option<u8> {
    // Remove port if present (format: "ip:port")
    let ip_part = addr.split(':').next()?;

    // Split by '.' and get the last segment
    let last_segment = ip_part.split('.').next_back()?;

    // Parse as u8
    last_segment.parse::<u8>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_last_octet() {
        assert_eq!(extract_last_octet("192.168.1.10"), Some(10));
        assert_eq!(extract_last_octet("192.168.1.10:8080"), Some(10));
        assert_eq!(extract_last_octet("172.21.0.11:58762"), Some(11));
        assert_eq!(extract_last_octet("10.0.0.2"), Some(2));
        assert_eq!(extract_last_octet("invalid"), None);
        assert_eq!(extract_last_octet(""), None);
    }

    #[test]
    fn test_routing_logic() {
        // Even last octet should route to egress-router1
        let last_octet = 10;
        let cluster = if last_octet % 2 == 0 {
            "egress-router1"
        } else {
            "egress-router2"
        };
        assert_eq!(cluster, "egress-router1");

        // Odd last octet should route to egress-router2
        let last_octet = 11;
        let cluster = if last_octet % 2 == 0 {
            "egress-router1"
        } else {
            "egress-router2"
        };
        assert_eq!(cluster, "egress-router2");
    }
}
