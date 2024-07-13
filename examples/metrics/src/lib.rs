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

use proxy_wasm::stats;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

use std::convert::TryInto;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(
        MetricsRootContext {
            metrics: WasmMetrics {
                counter: stats::Counter::new(String::from("wasm_counter")),
                gauge: stats::Gauge::new(String::from("wasm_gauge")),
                histogram: stats::Histogram::new(String::from("wasm_histogram")),
            }
        }
    )});
}}

#[derive(Copy, Clone)]
struct WasmMetrics {
    counter: stats::Counter,
    gauge: stats::Gauge,
    histogram: stats::Histogram,
}

struct MetricsRootContext {
    metrics: WasmMetrics,
}

impl Context for MetricsRootContext {}

impl RootContext for MetricsRootContext {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(StreamContext {
            metrics: self.metrics,
        }))
    }
}

struct StreamContext {
    metrics: WasmMetrics,
}

impl Context for StreamContext {}

impl HttpContext for StreamContext {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        let value = match self.get_http_request_header("x-envoy-wasm-metric-value") {
            Some(value) => value.parse::<i64>().unwrap(),
            _ => 0,
        };

        let metric_type = match self.get_http_request_header("x-envoy-wasm-metric") {
            Some(metric_type) => metric_type,
            _ => return Action::Continue,
        };

        match metric_type.as_str() {
            "counter" => self.metrics.counter.increment(value),
            "gauge" => self.metrics.gauge.record(value.try_into().unwrap()),
            "histogram" => self.metrics.histogram.record(value.try_into().unwrap()),
            _ => return Action::Continue,
        }

        Action::Continue
    }
}
