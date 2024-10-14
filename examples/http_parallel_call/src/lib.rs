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

use proxy_wasm::callout::http::HttpClient;
use proxy_wasm::callout::promise::Promise;
use proxy_wasm::hostcalls;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(HttpParallelCall::default()) });
}}

#[derive(Default)]
struct HttpParallelCall {
    client: HttpClient,
}

impl HttpContext for HttpParallelCall {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        // "Hello, "
        let promise1 = self
            .client
            .dispatch(
                "httpbin",
                vec![
                    (":method", "GET"),
                    (":path", "/base64/SGVsbG8sIA=="),
                    (":authority", "httpbin.org"),
                ],
                None,
                vec![],
                Duration::from_secs(1),
            )
            .then(|(_, _, body_size, _)| get_http_call_response_body_string(0, body_size))
            .then(|body| body.unwrap_or_default());

        // "World!"
        let promise2 = self
            .client
            .dispatch(
                "httpbin",
                vec![
                    (":method", "GET"),
                    (":path", "/base64/V29ybGQh"),
                    (":authority", "httpbin.org"),
                ],
                None,
                vec![],
                Duration::from_secs(1),
            )
            .then(|(_, _, body_size, _)| get_http_call_response_body_string(0, body_size))
            .then(|body| body.unwrap_or_default());

        Promise::all_of(vec![promise1, promise2]).then(|results| {
            send_http_response(
                200,
                vec![],
                Some(format!("{}{}\n", results[0], results[1]).as_bytes()),
            );
        });

        Action::Pause
    }

    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        self.set_http_response_header("Powered-By", Some("proxy-wasm"));
        Action::Continue
    }
}

impl Context for HttpParallelCall {
    fn on_http_call_response(
        &mut self,
        token_id: u32,
        num_headers: usize,
        body_size: usize,
        num_trailers: usize,
    ) {
        self.client
            .callback(token_id, num_headers, body_size, num_trailers)
    }
}

fn get_http_call_response_body_string(start: usize, max_size: usize) -> Option<String> {
    match hostcalls::get_buffer(BufferType::HttpCallResponseBody, start, max_size).unwrap() {
        None => None,
        Some(bytes) => {
            let body_string = String::from_utf8(bytes.to_vec()).unwrap();
            Some(body_string)
        }
    }
}

fn send_http_response(status_code: u32, headers: Vec<(&str, &str)>, body: Option<&[u8]>) {
    hostcalls::send_http_response(status_code, headers, body).unwrap()
}
