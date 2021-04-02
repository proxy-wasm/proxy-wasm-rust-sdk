// Copyright 2021 Google LLC
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

mod helloworld;

use helloworld::HelloRequest;
use log::trace;
use protobuf::Message;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(GrpcCallTest) });
}

struct GrpcCallTest;

impl HttpContext for GrpcCallTest {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        let mut req = HelloRequest::new();
        req.set_name("John Smith".to_string());
        let message = req.write_to_bytes().unwrap();

        match self.dispatch_grpc_call(
            "test",
            "helloworld.Greeter",
            "SayHello",
            "",
            String::from_utf8(message).unwrap().as_mut(),
            Duration::from_secs(5),
        ) {
            Ok(_) => trace!("success"),
            Err(e) => trace!("Failed {:?}", e),
        }

        Action::Pause
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        self.set_http_response_header("Powered-By", Some("proxy-wasm"));
        Action::Continue
    }
}

impl Context for GrpcCallTest {
    fn on_grpc_receive(&mut self, token_id: u32, response_size: usize) {
        trace!("{}", token_id.to_string());
        trace!("{}", response_size.to_string());
        self.resume_http_request()
    }

    fn on_grpc_close(&mut self, _token_id: u32, _status_code: u32) {
        trace!("{}", _token_id.to_string());
        trace!("{}", _status_code.to_string());
        self.resume_http_request()
    }
}
