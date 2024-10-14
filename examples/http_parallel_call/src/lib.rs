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

use proxy_wasm::hostcalls;
use proxy_wasm::promise::Promise;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(HttpParallelCall::default()) });
}}

type OnHttpResponseArgs = (u32, usize, usize, usize);

#[derive(Default)]
struct HttpParallelCall {
    m: HashMap<u32, Rc<Promise<OnHttpResponseArgs>>>,
}

impl HttpContext for HttpParallelCall {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        // "Hello, "
        let token1 = self
            .dispatch_http_call(
                "httpbin",
                vec![
                    (":method", "GET"),
                    (":path", "/base64/SGVsbG8sIAo="),
                    (":authority", "httpbin.org"),
                ],
                None,
                vec![],
                Duration::from_secs(1),
            )
            .unwrap();

        // "World!"
        let token2 = self
            .dispatch_http_call(
                "httpbin",
                vec![
                    (":method", "GET"),
                    (":path", "/base64/V29ybGQhCg=="),
                    (":authority", "httpbin.org"),
                ],
                None,
                vec![],
                Duration::from_secs(1),
            )
            .unwrap();

        let promise1 = Promise::new();
        let promise2 = Promise::new();
        self.m.insert(token1, promise1.clone());
        self.m.insert(token2, promise2.clone());

        Promise::all_of(vec![
            promise1
                .then(|(_, _, _body_size, _)| get_http_call_response_body_string(0, _body_size))
                .then(|body| body.unwrap_or_default()),
            promise2
                .then(|(_, _, _body_size, _)| get_http_call_response_body_string(0, _body_size))
                .then(|body| body.unwrap_or_default()),
        ])
        .then(|results| {
            send_http_response(
                200,
                vec![],
                Some(
                    format!(
                        "{}{}\n",
                        results[0].strip_suffix("\n").unwrap(),
                        results[1].strip_suffix("\n").unwrap()
                    )
                    .as_bytes(),
                ),
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
        _token_id: u32,
        _num_headers: usize,
        _body_size: usize,
        _num_trailers: usize,
    ) {
        let promise = self.m.remove(&_token_id);
        promise
            .unwrap()
            .fulfill((_token_id, _num_headers, _body_size, _num_trailers));
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
