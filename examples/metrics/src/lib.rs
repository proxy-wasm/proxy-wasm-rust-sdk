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

use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[derive(Copy, Clone)]
struct HttpAuthMetrics {
    counter_example: u32,
    gauge_example: u32,
    histogram_example: u32,
}

struct HttpAuthRoot {
    metrics: HttpAuthMetrics,
    monitored_path: String,
}

struct HttpAuth {
    metrics: HttpAuthMetrics,
    monitored_path: String,
}

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(HttpAuthRoot {
            metrics: HttpAuthMetrics {
                counter_example: 0,
                gauge_example: 0,
                histogram_example: 0,
            },
            monitored_path: "".to_string(),
        })
    });
}}

impl Context for HttpAuth {}
impl HttpContext for HttpAuth {
    fn on_http_request_headers(&mut self, num_headers: usize, _: bool) -> Action {
        self.record_metric(self.metrics.histogram_example, num_headers as u64)
            .unwrap();

        match self.get_http_request_header(":path") {
            Some(path) if path == self.monitored_path => {
                info!("Monitored path {} accessed.", self.monitored_path);

                self.increment_metric(self.metrics.counter_example, 1)
                    .unwrap();

                Action::Continue
            }
            _ => Action::Continue,
        }
    }

    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        let counter_value = self.get_metric(self.metrics.counter_example).unwrap();
        let gauge_value = self.get_metric(self.metrics.gauge_example).unwrap();
        // histogram retrieval isn't supported

        self.set_http_response_header("Powered-By", Some("proxy-wasm"));
        self.set_http_response_header("My-Counter", Some(format!("{}", counter_value).as_str()));
        self.set_http_response_header("My-Gauge", Some(format!("{}", gauge_value).as_str()));

        Action::Continue
    }
}

impl Context for HttpAuthRoot {}

impl RootContext for HttpAuthRoot {
    fn on_configure(&mut self, _: usize) -> bool {
        self.metrics.counter_example = self
            .define_metric(MetricType::Counter, "counter_example")
            .expect("failed defining counter_example metric");
        self.metrics.gauge_example = self
            .define_metric(MetricType::Gauge, "gauge_example")
            .expect("failed defining gauge_example metric");
        self.metrics.histogram_example = self
            .define_metric(MetricType::Histogram, "histogram_example")
            .expect("failed defining histogram_example metric");

        self.record_metric(self.metrics.gauge_example, 10).unwrap();

        self.monitored_path = "/headers".to_string();

        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpAuth {
            metrics: self.metrics,
            monitored_path: self.monitored_path.clone(),
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}
