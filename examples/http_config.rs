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

use std::str;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(HttpConfigHeaderRootContext{
            header_content: "".to_string(),
        })
    });
}

struct HttpConfigHeader{
    header_content: String
}

impl Context for HttpConfigHeader {}

impl HttpContext for HttpConfigHeader {

    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        self.add_http_response_header("custom-header", self.header_content.as_str());

        Action::Continue
    }
}

struct HttpConfigHeaderRootContext {
    header_content: String
}

impl Context for HttpConfigHeaderRootContext {}

impl RootContext for HttpConfigHeaderRootContext {
    
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        true
    }

    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        if let Some(config_bytes) = self.get_configuration() {
            self.header_content = str::from_utf8(config_bytes.as_ref()).unwrap().to_owned()
        }
        true
    }

    fn create_http_context(&self, _context_id: u32) -> Box<dyn HttpContext> {
        Box::new(HttpConfigHeader{
            header_content: self.header_content.clone(),
        })
    }

    fn get_type(&self) -> ContextType {
        ContextType::HttpContext
    }

}
