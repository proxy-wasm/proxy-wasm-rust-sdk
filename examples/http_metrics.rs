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

use log::error;
use proxy_wasm::hostcalls;
use proxy_wasm::traits::*;
use proxy_wasm::types::{Action, LogLevel, MetricType, Status};
use std::cmp::min;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(HttpMetrics) });
}

struct HttpMetrics;

impl Context for HttpMetrics {}

impl HttpContext for HttpMetrics {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        match counter("proxy_wasm_rust.request_counter") {
            Ok(metric) => {
                // Ignore errors
                metric.inc().ok();
            }
            Err(err) => {
                error!("Cannot get metric: {:?}", err);
            }
        }
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        match counter("proxy_wasm_rust.response_counter") {
            Ok(metric) => {
                // Ignore errors
                metric.inc().ok();
            }
            Err(err) => {
                error!("Cannot get metric: {:?}", err);
            }
        }
        Action::Continue
    }
}

struct Counter(u32);

impl Counter {
    fn add(&self, offset: u64) -> Result<(), Status> {
        if offset > 0 {
            let delta = min(offset, std::i64::MAX as u64) as i64;
            if let Err(err) = hostcalls::increment_metric(self.0, delta) {
                return Err(err);
            }
        }
        Ok(())
    }

    fn inc(&self) -> Result<(), Status> {
        self.add(1)
    }

    // Dead code just to show how it works
    #[allow(dead_code)]
    fn value(&self) -> Result<u64, Status> {
        hostcalls::get_metric(self.0)
    }
}

fn counter(name: &str) -> Result<Box<Counter>, Status> {
    hostcalls::define_metric(MetricType::Counter, name)
        .map(|metric_id| Box::new(Counter(metric_id)))
}
