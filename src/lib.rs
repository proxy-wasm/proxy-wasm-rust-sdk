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

pub mod hostcalls;
pub mod traits;
pub mod types;

mod allocator;
mod dispatcher;
mod logger;

pub fn set_log_level(level: types::LogLevel) {
    logger::set_log_level(level);
}

pub fn set_root_context(callback: types::NewRootContext) {
    dispatcher::set_root_context(callback);
}

pub fn set_stream_context(callback: types::NewStreamContext) {
    dispatcher::set_stream_context(callback);
}

pub fn set_http_context(callback: types::NewHttpContext) {
    dispatcher::set_http_context(callback);
}

#[no_mangle]
pub extern "C" fn proxy_abi_version_0_2_1() {}
