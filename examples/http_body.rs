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

use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(HttpBody::new()) });
}

#[derive(Default)]
struct HttpBody {
    total_body_size: usize,
}

impl HttpBody {
    fn new() -> HttpBody {
        Default::default()
    }
}

impl Context for HttpBody {}

impl HttpContext for HttpBody {
    fn on_http_response_headers(&mut self, _: usize) -> Action {
        // If there is a Content-Length header and we change the length of
        // the body later, then clients will break. So remove it.
        // We must do this here, because once we exit this function we
        // can no longer modify the response headers.
        self.set_http_response_header("content-length", None);
        // Don't continue to the next callout in the chain because we might
        // modify the body.
        Action::Pause
    }

    fn on_http_response_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        self.total_body_size += body_size;
        if !end_of_stream {
            // Wait -- we'll be called again when the complete body is buffered
            // at the host side.
            return Action::Pause;
        }

        // Replace the message body if it contains the text "secret".
        // Since we returned "Pause" previuously, this will return the whole body.
        // However, we have to calculate the size ourselves.
        if let Some(body_bytes) = self.get_http_response_body(0, self.total_body_size) {
            let body_str = String::from_utf8(body_bytes).unwrap();
            if body_str.find("secret").is_some() {
                let new_body = format!(
                    "Original message body ({} bytes) redacted.",
                    self.total_body_size
                );
                self.set_http_response_body(0, self.total_body_size, &new_body.into_bytes());
            }
        }
        Action::Continue
    }
}
