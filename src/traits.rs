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

use crate::hostcalls;
use crate::types::*;
use std::time::{Duration, SystemTime};

pub trait Context {
    fn get_current_time(&self) -> SystemTime {
        hostcalls::get_current_time().unwrap()
    }

    fn get_property(&self, path: Vec<&str>) -> Option<Bytes> {
        hostcalls::get_property(path).unwrap()
    }

    fn set_property(&self, path: Vec<&str>, value: Option<&[u8]>) {
        hostcalls::set_property(path, value).unwrap()
    }

    fn get_shared_data(&self, key: &str) -> (Option<Bytes>, Option<u32>) {
        hostcalls::get_shared_data(key).unwrap()
    }

    fn set_shared_data(
        &self,
        key: &str,
        value: Option<&[u8]>,
        cas: Option<u32>,
    ) -> Result<(), Status> {
        hostcalls::set_shared_data(key, value, cas)
    }

    fn register_shared_queue(&self, name: &str) -> u32 {
        hostcalls::register_shared_queue(name).unwrap()
    }

    fn resolve_shared_queue(&self, vm_id: &str, name: &str) -> Option<u32> {
        hostcalls::resolve_shared_queue(vm_id, name).unwrap()
    }

    fn dequeue_shared_queue(&self, queue_id: u32) -> Result<Option<Bytes>, Status> {
        hostcalls::dequeue_shared_queue(queue_id)
    }

    fn enqueue_shared_queue(&self, queue_id: u32, value: Option<&[u8]>) -> Result<(), Status> {
        hostcalls::enqueue_shared_queue(queue_id, value)
    }

    /// Sends HTTP request with serialized `headers`, `body`, and serialized trailers to upstream.
    ///
    /// `on_http_call_response()` will be called with `token_id` when the response is received by the host, or after the timeout.
    ///
    /// # Arguments
    ///
    /// * `upstream` - The name of the upstream to send the request to (in your envoy configuration)
    /// * `headers` - The headers to send with the request (in the form of `Vec<(&str, &str)>`)
    /// * `body` - The body of the request (in the form of `Option<&[u8]>` - `None` if no body)
    /// * `trailers` - The trailers to send with the request (in the form of `Vec<(&str, &str)>`)
    /// * `timeout` - The timeout for the request
    ///
    /// # Returns
    ///
    /// * `OK` on success.
    /// * `BAD_ARGUMENT` for unknown upstream, or when headers are missing required `:authority`, `:method` and/or `:path` values.
    /// * `INTERNAL_FAILURE' when the host failed to send requested HTTP call.
    /// * `INVALID_MEMORY_ACCESS` when `upstream_data`, `upstream_size`, `headers_data`, `headers_size`, `body_data`, `body_size`, `trailers_data`, `trailers_size` and/or `return_call_id` point to invalid memory address.
    ///
    /// # Example
    ///
    /// ```rust
    /// use proxy_wasm::traits::*;
    /// use proxy_wasm::types::*;
    /// use std::time::Duration;
    ///
    /// struct MyContext;
    ///
    /// impl HttpContext for MyContext {
    ///
    ///    fn on_http_request_headers(&mut self, _num_headers: usize) -> Action {
    ///
    ///       match self.dispatch_http_call(
    ///        "upstream",
    ///        vec![(":method", "GET"), (":path", "/"), (":authority", "www.example.com")],
    ///        None,
    ///        vec![]
    ///       Duration::from_secs(5),
    ///      ) {
    ///        Ok(_) => Action::Pause,
    ///       Err(_) => Action::Continue,
    ///     }
    ///   }
    /// }
    ///
    /// impl Context for MyContext {
    ///
    ///    fn on_http_call_response(&mut self, token_id: u32, _: usize, body_size: usize, _: usize) {
    ///
    ///     let headers = self.get_http_call_response_headers();
    ///     let body = self.get_http_call_response_body(0, body_size);
    ///
    ///    // Do something with the response
    ///
    ///   }
    /// }
    fn dispatch_http_call(
        &self,
        upstream: &str,
        headers: Vec<(&str, &str)>,
        body: Option<&[u8]>,
        trailers: Vec<(&str, &str)>,
        timeout: Duration,
    ) -> Result<u32, Status> {
        hostcalls::dispatch_http_call(upstream, headers, body, trailers, timeout)
    }

    /// Called when HTTP response for call_id sent using proxy_http_call is received.
    ///
    /// If `num_headers` is 0, then the HTTP call failed.
    ///
    /// All `num_headers` headers can be retrieved using `self.get_http_response_headers()` or individually `self.get_http_response_header()`.
    ///
    /// All `num_trailers` trailers can be retrieved using `self.get_http_response_trailers()` or individually `self.get_http_response_trailer()`.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The token id of the call
    /// * `num_headers` - The number of headers in the response
    /// * `body_size` - The size of the body in the response
    /// * `num_trailers` - The number of trailers in the response
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use proxy_wasm::traits::*;
    /// use proxy_wasm::types::*;
    ///
    /// struct MyContext;
    ///
    /// impl HttpContext for MyContext {
    ///
    ///    fn on_http_request_headers(&mut self, _num_headers: usize) -> Action {
    ///
    ///       match self.dispatch_http_call(
    ///        "upstream",
    ///        vec![(":method", "GET"), (":path", "/"), (":authority", "www.example.com")],
    ///        None,
    ///        vec![]
    ///       Duration::from_secs(5),
    ///      ) {
    ///        Ok(_) => Action::Pause,
    ///       Err(_) => Action::Continue,
    ///     }
    ///   }
    /// }
    ///
    /// impl Context for MyContext {
    ///
    ///    fn on_http_call_response(&mut self, token_id: u32, _: usize, body_size: usize, _: usize) {
    ///
    ///     let headers = self.get_http_call_response_headers();
    ///     let body = self.get_http_call_response_body(0, body_size);
    ///
    ///    // Do something with the response
    ///
    ///   }
    /// }
    /// ```
    fn on_http_call_response(
        &mut self,
        _token_id: u32,
        _num_headers: usize,
        _body_size: usize,
        _num_trailers: usize,
    ) {
    }

    fn get_http_call_response_headers(&self) -> Vec<(String, String)> {
        hostcalls::get_map(MapType::HttpCallResponseHeaders).unwrap()
    }

    fn get_http_call_response_headers_bytes(&self) -> Vec<(String, Bytes)> {
        hostcalls::get_map_bytes(MapType::HttpCallResponseHeaders).unwrap()
    }

    fn get_http_call_response_header(&self, name: &str) -> Option<String> {
        hostcalls::get_map_value(MapType::HttpCallResponseHeaders, name).unwrap()
    }

    fn get_http_call_response_header_bytes(&self, name: &str) -> Option<Bytes> {
        hostcalls::get_map_value_bytes(MapType::HttpCallResponseHeaders, name).unwrap()
    }

    fn get_http_call_response_body(&self, start: usize, max_size: usize) -> Option<Bytes> {
        hostcalls::get_buffer(BufferType::HttpCallResponseBody, start, max_size).unwrap()
    }

    fn get_http_call_response_trailers(&self) -> Vec<(String, String)> {
        hostcalls::get_map(MapType::HttpCallResponseTrailers).unwrap()
    }

    fn get_http_call_response_trailers_bytes(&self) -> Vec<(String, Bytes)> {
        hostcalls::get_map_bytes(MapType::HttpCallResponseTrailers).unwrap()
    }

    fn get_http_call_response_trailer(&self, name: &str) -> Option<String> {
        hostcalls::get_map_value(MapType::HttpCallResponseTrailers, name).unwrap()
    }

    fn get_http_call_response_trailer_bytes(&self, name: &str) -> Option<Bytes> {
        hostcalls::get_map_value_bytes(MapType::HttpCallResponseTrailers, name).unwrap()
    }

    fn dispatch_grpc_call(
        &self,
        upstream_name: &str,
        service_name: &str,
        method_name: &str,
        initial_metadata: Vec<(&str, &[u8])>,
        message: Option<&[u8]>,
        timeout: Duration,
    ) -> Result<u32, Status> {
        hostcalls::dispatch_grpc_call(
            upstream_name,
            service_name,
            method_name,
            initial_metadata,
            message,
            timeout,
        )
    }

    fn on_grpc_call_response(&mut self, _token_id: u32, _status_code: u32, _response_size: usize) {}

    fn get_grpc_call_response_body(&self, start: usize, max_size: usize) -> Option<Bytes> {
        hostcalls::get_buffer(BufferType::GrpcReceiveBuffer, start, max_size).unwrap()
    }

    fn cancel_grpc_call(&self, token_id: u32) {
        hostcalls::cancel_grpc_call(token_id).unwrap()
    }

    fn open_grpc_stream(
        &self,
        cluster_name: &str,
        service_name: &str,
        method_name: &str,
        initial_metadata: Vec<(&str, &[u8])>,
    ) -> Result<u32, Status> {
        hostcalls::open_grpc_stream(cluster_name, service_name, method_name, initial_metadata)
    }

    fn on_grpc_stream_initial_metadata(&mut self, _token_id: u32, _num_elements: u32) {}

    fn get_grpc_stream_initial_metadata(&self) -> Vec<(String, Bytes)> {
        hostcalls::get_map_bytes(MapType::GrpcReceiveInitialMetadata).unwrap()
    }

    fn get_grpc_stream_initial_metadata_value(&self, name: &str) -> Option<Bytes> {
        hostcalls::get_map_value_bytes(MapType::GrpcReceiveInitialMetadata, name).unwrap()
    }

    fn send_grpc_stream_message(&self, token_id: u32, message: Option<&[u8]>, end_stream: bool) {
        hostcalls::send_grpc_stream_message(token_id, message, end_stream).unwrap()
    }

    fn on_grpc_stream_message(&mut self, _token_id: u32, _message_size: usize) {}

    fn get_grpc_stream_message(&mut self, start: usize, max_size: usize) -> Option<Bytes> {
        hostcalls::get_buffer(BufferType::GrpcReceiveBuffer, start, max_size).unwrap()
    }

    fn on_grpc_stream_trailing_metadata(&mut self, _token_id: u32, _num_elements: u32) {}

    fn get_grpc_stream_trailing_metadata(&self) -> Vec<(String, Bytes)> {
        hostcalls::get_map_bytes(MapType::GrpcReceiveTrailingMetadata).unwrap()
    }

    fn get_grpc_stream_trailing_metadata_value(&self, name: &str) -> Option<Bytes> {
        hostcalls::get_map_value_bytes(MapType::GrpcReceiveTrailingMetadata, name).unwrap()
    }

    fn cancel_grpc_stream(&self, token_id: u32) {
        hostcalls::cancel_grpc_stream(token_id).unwrap()
    }

    fn close_grpc_stream(&self, token_id: u32) {
        hostcalls::close_grpc_stream(token_id).unwrap()
    }

    fn on_grpc_stream_close(&mut self, _token_id: u32, _status_code: u32) {}

    fn get_grpc_status(&self) -> (u32, Option<String>) {
        hostcalls::get_grpc_status().unwrap()
    }

    fn call_foreign_function(
        &self,
        function_name: &str,
        arguments: Option<&[u8]>,
    ) -> Result<Option<Bytes>, Status> {
        hostcalls::call_foreign_function(function_name, arguments)
    }

    fn on_done(&mut self) -> bool {
        true
    }

    fn done(&self) {
        hostcalls::done().unwrap()
    }
}

pub trait RootContext: Context {
    /// Called when the host starts the WebAssembly Virtual Machine.
    ///
    /// Its configuration (of `_vm_configuration_size`) can be retrieved using `self.get_vm_configuration()`.
    ///
    /// # Arguments
    ///
    /// * `vm_configuration_size` - the size of the VM configuration
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the configuration was processed successfully, `false` otherwise
    ///
    /// /// # Example
    ///
    /// ```rust
    ///
    /// use proxy_wasm::traits::RootContext;
    ///
    /// struct MyRootContext;
    ///
    /// struct MyVmConfiguration {
    ///     /// Some key
    ///     pub key: String,
    /// }
    ///
    /// impl RootContext for MyRootContext {
    ///
    ///    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
    ///
    ///     let vm_confuguration = self.get_vm_configuration().unwrap();
    ///
    ///     let parsed_vm_configuration: MyVmConfiguration = serde_json::from_slice::<MyVmConfiguration>(&vm_confuguration).unwrap();
    ///
    ///     // Do something with the parsed vm configuration
    ///
    ///    true
    ///   }
    /// }
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        true
    }

    /// Get the VM configuration.
    ///
    /// # Returns
    ///
    /// * `Option<Bytes>` - the VM configuration
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use proxy_wasm::traits::RootContext;
    ///
    /// struct MyRootContext;
    ///
    /// struct MyVmConfiguration {
    ///     /// Some key
    ///     pub key: String,
    /// }
    ///
    /// impl RootContext for MyRootContext {
    ///
    ///    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
    ///
    ///     let vm_confuguration = self.get_vm_configuration().unwrap();
    ///
    ///     let parsed_vm_configuration: MyVmConfiguration = serde_json::from_slice::<MyVmConfiguration>(&vm_confuguration).unwrap();
    ///
    ///     // Do something with the parsed vm configuration
    ///
    ///    true
    ///   }
    /// }
    fn get_vm_configuration(&self) -> Option<Bytes> {
        hostcalls::get_buffer(BufferType::VmConfiguration, 0, usize::MAX).unwrap()
    }

    /// Called when the host starts the Proxy-Wasm plugin.
    ///
    /// Its configuration (of `_plugin_configuration_size`) can be retrieved using `self.get_plugin_configuration()`.
    ///
    /// # Returns
    ///
    /// Plugin must return one of the following values:
    ///
    /// * `true` - to indicate that the configuration was processed successfully.
    /// * `false` - to indicate that the configuration processing failed.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use proxy_wasm::traits::RootContext;
    ///
    /// struct MyRootContext;
    ///
    /// struct MyPluginConfiguration {
    ///     /// Some key
    ///     pub key: String,
    /// }
    ///
    /// impl RootContext for MyRootContext {
    ///
    ///    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
    ///
    ///     let plugin_configuration = self.get_plugin_configuration().unwrap();
    ///
    ///     let parsed_plugin_configuration: MyPluginConfiguration = serde_json::from_slice::<MyPluginConfiguration>(&plugin_configuration).unwrap();
    ///
    ///     // Do something with the parsed plugin configuration
    ///
    ///    true
    ///   }
    /// }
    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        true
    }

    /// Get the plugin configuration.
    ///
    /// # Returns
    ///
    /// * `Option<Bytes>` - the plugin configuration
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use proxy_wasm::traits::RootContext;
    ///
    /// struct MyRootContext;
    ///
    /// struct MyPluginConfiguration {
    ///     /// Some key
    ///     pub key: String,
    /// }
    ///
    /// impl RootContext for MyRootContext {
    ///
    ///    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
    ///
    ///     let plugin_configuration = self.get_plugin_configuration().unwrap();
    ///
    ///     let parsed_plugin_configuration: MyPluginConfiguration = serde_json::from_slice::<MyPluginConfiguration>(&plugin_configuration).unwrap();
    ///
    ///     // Do something with the parsed plugin configuration
    ///
    ///    true
    ///   }
    /// }
    fn get_plugin_configuration(&self) -> Option<Bytes> {
        hostcalls::get_buffer(BufferType::PluginConfiguration, 0, usize::MAX).unwrap()
    }

    /// Sets timer period. When set, `on_tick` will be called every `tick_period`. This is useful for making periodic updates to cached data, etc.
    ///
    /// # Arguments
    ///
    /// * `period` - the period of the timer
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use proxy_wasm::traits::RootContext;
    /// use std::time::Duration;
    /// use log::info;
    ///
    /// struct MyRootContext;
    ///
    /// impl RootContext for MyRootContext {
    ///
    ///   fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
    ///
    ///    self.set_tick_period(Duration::from_millis(5000));
    ///
    ///   true
    ///  }
    ///
    ///  fn on_tick(&mut self) {
    ///
    ///   // Do something every 5 seconds
    ///   info!("tick!")
    ///  }
    /// }
    fn set_tick_period(&self, period: Duration) {
        hostcalls::set_tick_period(period).unwrap()
    }

    /// Called on a timer every tick period.
    ///
    /// The tick period can be configured using `proxy_set_tick_period_milliseconds`.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use proxy_wasm::traits::RootContext;
    /// use std::time::Duration;
    /// use log::info;
    ///
    /// struct MyRootContext;
    ///
    /// impl RootContext for MyRootContext {
    ///
    ///   fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
    ///
    ///    self.set_tick_period(Duration::from_millis(5000));
    ///
    ///   true
    ///  }
    ///
    ///  fn on_tick(&mut self) {
    ///
    ///   // Do something every 5 seconds
    ///   info!("tick!")
    ///  }
    /// }
    fn on_tick(&mut self) {}

    fn on_queue_ready(&mut self, _queue_id: u32) {}

    fn on_log(&mut self) {}

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        None
    }

    fn create_stream_context(&self, _context_id: u32) -> Option<Box<dyn StreamContext>> {
        None
    }

    fn get_type(&self) -> Option<ContextType> {
        None
    }
}

pub trait StreamContext: Context {
    fn on_new_connection(&mut self) -> Action {
        Action::Continue
    }

    fn on_downstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        Action::Continue
    }

    fn get_downstream_data(&self, start: usize, max_size: usize) -> Option<Bytes> {
        hostcalls::get_buffer(BufferType::DownstreamData, start, max_size).unwrap()
    }

    fn set_downstream_data(&self, start: usize, size: usize, value: &[u8]) {
        hostcalls::set_buffer(BufferType::DownstreamData, start, size, value).unwrap()
    }

    fn resume_downstream(&self) {
        hostcalls::resume_downstream().unwrap()
    }

    fn close_downstream(&self) {
        hostcalls::close_downstream().unwrap()
    }

    fn on_downstream_close(&mut self, _peer_type: PeerType) {}

    fn on_upstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        Action::Continue
    }

    fn get_upstream_data(&self, start: usize, max_size: usize) -> Option<Bytes> {
        hostcalls::get_buffer(BufferType::UpstreamData, start, max_size).unwrap()
    }

    fn set_upstream_data(&self, start: usize, size: usize, value: &[u8]) {
        hostcalls::set_buffer(BufferType::UpstreamData, start, size, value).unwrap()
    }

    fn resume_upstream(&self) {
        hostcalls::resume_upstream().unwrap()
    }

    fn close_upstream(&self) {
        hostcalls::close_upstream().unwrap()
    }

    fn on_upstream_close(&mut self, _peer_type: PeerType) {}

    fn on_log(&mut self) {}
}

pub trait HttpContext: Context {
    
    /// Called when HTTP request headers are received from downstream.
    ///
    /// All `num_headers` headers can be retrieved and/or modified using `self.get_http_request_headers()`.
    ///
    /// Individual HTTP request headers can be retrieved and/or modified using `self.get_http_request_header()`, `self.add_http_request_header()`.
    ///
    /// Paused request can be resumed using `self.resume_http_request()` or closed using `self.reset_http_request()`.
    ///
    /// Additionally, instead of forwarding request upstream, a HTTP response can be sent using `self.send_http_response()`.
    ///
    /// # Arguments
    ///
    /// * `num_headers` - the number of HTTP request headers
    /// * `end_of_stream` - indicates if this is the last call for the request headers
    ///
    /// # Returns
    ///
    /// Plugin must return one of the following values:
    ///
    /// * `CONTINUE` to forward `HTTP_REQUEST_HEADERS` fields downstream.
    /// * `PAUSE` to pause processing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use proxy_wasm::traits::*;
    /// use proxy_wasm::types::*;
    ///
    /// use log::info;
    ///
    /// impl HttpContext for MyPlugin {
    ///
    ///    fn on_http_request_headers(&mut self, num_headers: usize, end_of_stream: bool) -> Action {
    ///
    ///     let headers = self.get_http_request_headers();
    ///
    ///     // Process the request
    ///
    ///    Action::Continue
    ///    }
    /// }
    /// ```
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        Action::Continue
    }

    /// Get all HTTP request headers.
    ///
    /// # Returns
    ///
    /// * `Vec<(String, String)>` - a list of HTTP request headers
    ///
    /// # Example
    ///
    /// ```rust
    /// use log::info;
    /// use proxy_wasm::traits::HttpContext;
    ///
    /// impl HttpContext for MyPlugin {
    ///     fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
    ///
    ///         let headers = self.get_http_request_headers();
    ///
    ///         for (name, value) in headers {
    ///            info!("{}: {}", name, value);
    ///         }
    ///     Action::Continue
    ///     }
    /// }
    ///
    fn get_http_request_headers(&self) -> Vec<(String, String)> {
        hostcalls::get_map(MapType::HttpRequestHeaders).unwrap()
    }

    fn get_http_request_headers_bytes(&self) -> Vec<(String, Bytes)> {
        hostcalls::get_map_bytes(MapType::HttpRequestHeaders).unwrap()
    }

    fn set_http_request_headers(&self, headers: Vec<(&str, &str)>) {
        hostcalls::set_map(MapType::HttpRequestHeaders, headers).unwrap()
    }

    fn set_http_request_headers_bytes(&self, headers: Vec<(&str, &[u8])>) {
        hostcalls::set_map_bytes(MapType::HttpRequestHeaders, headers).unwrap()
    }

    /// Get a specific HTTP request header.
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the header
    ///
    /// # Returns
    ///
    /// * `Option<String>` - the value of the header (wrapped in an Option) or `None` if the header does not exist
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use log::info;
    /// use proxy_wasm::traits:*;
    /// use proxy_wasm::types::Action;
    ///
    /// impl HttpContext for MyPlugin {
    ///    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
    ///
    ///      let header = self.get_http_request_header(":path");
    ///
    ///      match header {
    ///         Some(value) => info!("The path is: {}", value),
    ///         None => info!("The path is missing")
    ///      }
    ///      Action::Continue
    ///    }
    /// }
    fn get_http_request_header(&self, name: &str) -> Option<String> {
        hostcalls::get_map_value(MapType::HttpRequestHeaders, name).unwrap()
    }

    fn get_http_request_header_bytes(&self, name: &str) -> Option<Bytes> {
        hostcalls::get_map_value_bytes(MapType::HttpRequestHeaders, name).unwrap()
    }

    fn set_http_request_header(&self, name: &str, value: Option<&str>) {
        hostcalls::set_map_value(MapType::HttpRequestHeaders, name, value).unwrap()
    }

    fn set_http_request_header_bytes(&self, name: &str, value: Option<&[u8]>) {
        hostcalls::set_map_value_bytes(MapType::HttpRequestHeaders, name, value).unwrap()
    }

    /// Add a new HTTP request header to be sent upstream.
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the header
    /// * `value` - the value of the header
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use proxy_wasm::traits::*;
    /// use proxy_wasm::types::Action;
    ///
    /// impl HttpContext for MyPlugin {
    ///    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
    ///
    ///      self.add_http_request_header("x-my-header", "my-value");
    ///
    ///    Action::Continue
    ///   }
    /// }
    /// ```
    fn add_http_request_header(&self, name: &str, value: &str) {
        hostcalls::add_map_value(MapType::HttpRequestHeaders, name, value).unwrap()
    }

    fn add_http_request_header_bytes(&self, name: &str, value: &[u8]) {
        hostcalls::add_map_value_bytes(MapType::HttpRequestHeaders, name, value).unwrap()
    }

    fn on_http_request_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        Action::Continue
    }

    fn get_http_request_body(&self, start: usize, max_size: usize) -> Option<Bytes> {
        hostcalls::get_buffer(BufferType::HttpRequestBody, start, max_size).unwrap()
    }

    fn set_http_request_body(&self, start: usize, size: usize, value: &[u8]) {
        hostcalls::set_buffer(BufferType::HttpRequestBody, start, size, value).unwrap()
    }

    fn on_http_request_trailers(&mut self, _num_trailers: usize) -> Action {
        Action::Continue
    }

    fn get_http_request_trailers(&self) -> Vec<(String, String)> {
        hostcalls::get_map(MapType::HttpRequestTrailers).unwrap()
    }

    fn get_http_request_trailers_bytes(&self) -> Vec<(String, Bytes)> {
        hostcalls::get_map_bytes(MapType::HttpRequestTrailers).unwrap()
    }

    fn set_http_request_trailers(&self, trailers: Vec<(&str, &str)>) {
        hostcalls::set_map(MapType::HttpRequestTrailers, trailers).unwrap()
    }

    fn set_http_request_trailers_bytes(&self, trailers: Vec<(&str, &[u8])>) {
        hostcalls::set_map_bytes(MapType::HttpRequestTrailers, trailers).unwrap()
    }

    fn get_http_request_trailer(&self, name: &str) -> Option<String> {
        hostcalls::get_map_value(MapType::HttpRequestTrailers, name).unwrap()
    }

    fn get_http_request_trailer_bytes(&self, name: &str) -> Option<Bytes> {
        hostcalls::get_map_value_bytes(MapType::HttpRequestTrailers, name).unwrap()
    }

    fn set_http_request_trailer(&self, name: &str, value: Option<&str>) {
        hostcalls::set_map_value(MapType::HttpRequestTrailers, name, value).unwrap()
    }

    fn set_http_request_trailer_bytes(&self, name: &str, value: Option<&[u8]>) {
        hostcalls::set_map_value_bytes(MapType::HttpRequestTrailers, name, value).unwrap()
    }

    fn add_http_request_trailer(&self, name: &str, value: &str) {
        hostcalls::add_map_value(MapType::HttpRequestTrailers, name, value).unwrap()
    }

    fn add_http_request_trailer_bytes(&self, name: &str, value: &[u8]) {
        hostcalls::add_map_value_bytes(MapType::HttpRequestTrailers, name, value).unwrap()
    }

    /// Resumes processing of paused request.
    fn resume_http_request(&self) {
        hostcalls::resume_http_request().unwrap()
    }

    fn reset_http_request(&self) {
        hostcalls::reset_http_request().unwrap()
    }

    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        Action::Continue
    }

    fn get_http_response_headers(&self) -> Vec<(String, String)> {
        hostcalls::get_map(MapType::HttpResponseHeaders).unwrap()
    }

    fn get_http_response_headers_bytes(&self) -> Vec<(String, Bytes)> {
        hostcalls::get_map_bytes(MapType::HttpResponseHeaders).unwrap()
    }

    fn set_http_response_headers(&self, headers: Vec<(&str, &str)>) {
        hostcalls::set_map(MapType::HttpResponseHeaders, headers).unwrap()
    }

    fn set_http_response_headers_bytes(&self, headers: Vec<(&str, &[u8])>) {
        hostcalls::set_map_bytes(MapType::HttpResponseHeaders, headers).unwrap()
    }

    fn get_http_response_header(&self, name: &str) -> Option<String> {
        hostcalls::get_map_value(MapType::HttpResponseHeaders, name).unwrap()
    }

    fn get_http_response_header_bytes(&self, name: &str) -> Option<Bytes> {
        hostcalls::get_map_value_bytes(MapType::HttpResponseHeaders, name).unwrap()
    }

    fn set_http_response_header(&self, name: &str, value: Option<&str>) {
        hostcalls::set_map_value(MapType::HttpResponseHeaders, name, value).unwrap()
    }

    fn set_http_response_header_bytes(&self, name: &str, value: Option<&[u8]>) {
        hostcalls::set_map_value_bytes(MapType::HttpResponseHeaders, name, value).unwrap()
    }

    fn add_http_response_header(&self, name: &str, value: &str) {
        hostcalls::add_map_value(MapType::HttpResponseHeaders, name, value).unwrap()
    }

    fn add_http_response_header_bytes(&self, name: &str, value: &[u8]) {
        hostcalls::add_map_value_bytes(MapType::HttpResponseHeaders, name, value).unwrap()
    }

    fn on_http_response_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        Action::Continue
    }

    fn get_http_response_body(&self, start: usize, max_size: usize) -> Option<Bytes> {
        hostcalls::get_buffer(BufferType::HttpResponseBody, start, max_size).unwrap()
    }

    fn set_http_response_body(&self, start: usize, size: usize, value: &[u8]) {
        hostcalls::set_buffer(BufferType::HttpResponseBody, start, size, value).unwrap()
    }

    fn on_http_response_trailers(&mut self, _num_trailers: usize) -> Action {
        Action::Continue
    }

    fn get_http_response_trailers(&self) -> Vec<(String, String)> {
        hostcalls::get_map(MapType::HttpResponseTrailers).unwrap()
    }

    fn get_http_response_trailers_bytes(&self) -> Vec<(String, Bytes)> {
        hostcalls::get_map_bytes(MapType::HttpResponseTrailers).unwrap()
    }

    fn set_http_response_trailers(&self, trailers: Vec<(&str, &str)>) {
        hostcalls::set_map(MapType::HttpResponseTrailers, trailers).unwrap()
    }

    fn set_http_response_trailers_bytes(&self, trailers: Vec<(&str, &[u8])>) {
        hostcalls::set_map_bytes(MapType::HttpResponseTrailers, trailers).unwrap()
    }

    fn get_http_response_trailer(&self, name: &str) -> Option<String> {
        hostcalls::get_map_value(MapType::HttpResponseTrailers, name).unwrap()
    }

    fn get_http_response_trailer_bytes(&self, name: &str) -> Option<Bytes> {
        hostcalls::get_map_value_bytes(MapType::HttpResponseTrailers, name).unwrap()
    }

    fn set_http_response_trailer(&self, name: &str, value: Option<&str>) {
        hostcalls::set_map_value(MapType::HttpResponseTrailers, name, value).unwrap()
    }

    fn set_http_response_trailer_bytes(&self, name: &str, value: Option<&[u8]>) {
        hostcalls::set_map_value_bytes(MapType::HttpResponseTrailers, name, value).unwrap()
    }

    fn add_http_response_trailer(&self, name: &str, value: &str) {
        hostcalls::add_map_value(MapType::HttpResponseTrailers, name, value).unwrap()
    }

    fn add_http_response_trailer_bytes(&self, name: &str, value: &[u8]) {
        hostcalls::add_map_value_bytes(MapType::HttpResponseTrailers, name, value).unwrap()
    }

    fn resume_http_response(&self) {
        hostcalls::resume_http_response().unwrap()
    }

    fn reset_http_response(&self) {
        hostcalls::reset_http_response().unwrap()
    }

    /// Sends an HTTP response with the body and serialized headers.
    ///
    /// This can be used as long as HTTP response headers were not sent downstream.
    ///
    /// # Arguments
    ///
    /// * `status_code` - the HTTP status code
    /// * `headers` - the HTTP headers as a `Vec` of `(&str, &str)`
    /// * `body` - the HTTP body as a slice of bytes
    ///
    /// # Example
    ///
    /// ```rust
    /// use log::info;
    /// use proxy_wasm::traits::*;
    /// use proxy_wasm::types::*;
    ///
    /// impl HttpContext for MyHttpContext {
    ///
    ///    fn on_http_request_headers(&mut self, _num_headers: usize) -> Action {
    ///
    ///      let auth = self.get_http_request_header("Authorization").unwrap_or_else(
    ///         || self.send_http_response(401, vec![("WWW-Authenticate", "Basic")], Some(b"Unauthorized"))
    ///      );
    ///
    ///      if auth == "I am authorized!" {
    ///       // Send an HTTP response with a status code of 200 and a body of "Hello, World!"
    ///         self.send_http_response(200, vec![("A header", "Some Value")], Some(b"Hello, World!"));
    ///      } else {
    ///         // Send an HTTP response with a status code of 403 and a body of "Forbidden"
    ///         self.send_http_response(403, vec![("location", "authenticate-here.com")], Some(b"Forbidden"));
    ///      }
    ///
    ///      Action::Pause
    ///   }
    /// }
    fn send_http_response(
        &self,
        status_code: u32,
        headers: Vec<(&str, &str)>,
        body: Option<&[u8]>,
    ) {
        hostcalls::send_http_response(status_code, headers, body).unwrap()
    }

    fn on_log(&mut self) {}
}
