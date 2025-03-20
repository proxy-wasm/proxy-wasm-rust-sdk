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
    fn get_current_time(&self) -> Result<SystemTime, Status> {
        hostcalls::get_current_time()
    }

    fn get_property(&self, path: Vec<&str>) -> Result<Option<Bytes>, Status> {
        hostcalls::get_property(path)
    }

    fn set_property(&self, path: Vec<&str>, value: Option<&[u8]>) -> Result<(), Status> {
        hostcalls::set_property(path, value)
    }

    fn get_shared_data(&self, key: &str) -> Result<(Option<Bytes>, Option<u32>), Status> {
        hostcalls::get_shared_data(key)
    }

    fn set_shared_data(
        &self,
        key: &str,
        value: Option<&[u8]>,
        cas: Option<u32>,
    ) -> Result<(), Status> {
        hostcalls::set_shared_data(key, value, cas)
    }

    fn register_shared_queue(&self, name: &str) -> Result<u32, Status> {
        hostcalls::register_shared_queue(name)
    }

    fn resolve_shared_queue(&self, vm_id: &str, name: &str) -> Result<Option<u32>, Status> {
        hostcalls::resolve_shared_queue(vm_id, name)
    }

    fn dequeue_shared_queue(&self, queue_id: u32) -> Result<Option<Bytes>, Status> {
        hostcalls::dequeue_shared_queue(queue_id)
    }

    fn enqueue_shared_queue(&self, queue_id: u32, value: Option<&[u8]>) -> Result<(), Status> {
        hostcalls::enqueue_shared_queue(queue_id, value)
    }

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

    fn on_http_call_response(
        &mut self,
        _token_id: u32,
        _num_headers: usize,
        _body_size: usize,
        _num_trailers: usize,
    ) {
    }

    fn get_http_call_response_headers(&self) -> Result<Vec<(String, String)>, Status> {
        hostcalls::get_map(MapType::HttpCallResponseHeaders)
    }

    fn get_http_call_response_headers_bytes(&self) -> Result<Vec<(String, Bytes)>, Status> {
        hostcalls::get_map_bytes(MapType::HttpCallResponseHeaders)
    }

    fn get_http_call_response_header(&self, name: &str) -> Result<Option<String>, Status> {
        hostcalls::get_map_value(MapType::HttpCallResponseHeaders, name)
    }

    fn get_http_call_response_header_bytes(&self, name: &str) -> Result<Option<Bytes>, Status> {
        hostcalls::get_map_value_bytes(MapType::HttpCallResponseHeaders, name)
    }

    fn get_http_call_response_body(
        &self,
        start: usize,
        max_size: usize,
    ) -> Result<Option<Bytes>, Status> {
        hostcalls::get_buffer(BufferType::HttpCallResponseBody, start, max_size)
    }

    fn get_http_call_response_trailers(&self) -> Result<Vec<(String, String)>, Status> {
        hostcalls::get_map(MapType::HttpCallResponseTrailers)
    }

    fn get_http_call_response_trailers_bytes(&self) -> Result<Vec<(String, Bytes)>, Status> {
        hostcalls::get_map_bytes(MapType::HttpCallResponseTrailers)
    }

    fn get_http_call_response_trailer(&self, name: &str) -> Result<Option<String>, Status> {
        hostcalls::get_map_value(MapType::HttpCallResponseTrailers, name)
    }

    fn get_http_call_response_trailer_bytes(&self, name: &str) -> Result<Option<Bytes>, Status> {
        hostcalls::get_map_value_bytes(MapType::HttpCallResponseTrailers, name)
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

    fn get_grpc_call_response_body(
        &self,
        start: usize,
        max_size: usize,
    ) -> Result<Option<Bytes>, Status> {
        hostcalls::get_buffer(BufferType::GrpcReceiveBuffer, start, max_size)
    }

    fn cancel_grpc_call(&self, token_id: u32) -> Result<(), Status> {
        hostcalls::cancel_grpc_call(token_id)
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

    fn get_grpc_stream_initial_metadata(&self) -> Result<Vec<(String, Bytes)>, Status> {
        hostcalls::get_map_bytes(MapType::GrpcReceiveInitialMetadata)
    }

    fn get_grpc_stream_initial_metadata_value(&self, name: &str) -> Result<Option<Bytes>, Status> {
        hostcalls::get_map_value_bytes(MapType::GrpcReceiveInitialMetadata, name)
    }

    fn send_grpc_stream_message(
        &self,
        token_id: u32,
        message: Option<&[u8]>,
        end_stream: bool,
    ) -> Result<(), Status> {
        hostcalls::send_grpc_stream_message(token_id, message, end_stream)
    }

    fn on_grpc_stream_message(&mut self, _token_id: u32, _message_size: usize) {}

    fn get_grpc_stream_message(
        &mut self,
        start: usize,
        max_size: usize,
    ) -> Result<Option<Bytes>, Status> {
        hostcalls::get_buffer(BufferType::GrpcReceiveBuffer, start, max_size)
    }

    fn on_grpc_stream_trailing_metadata(&mut self, _token_id: u32, _num_elements: u32) {}

    fn get_grpc_stream_trailing_metadata(&self) -> Result<Vec<(String, Bytes)>, Status> {
        hostcalls::get_map_bytes(MapType::GrpcReceiveTrailingMetadata)
    }

    fn get_grpc_stream_trailing_metadata_value(&self, name: &str) -> Result<Option<Bytes>, Status> {
        hostcalls::get_map_value_bytes(MapType::GrpcReceiveTrailingMetadata, name)
    }

    fn cancel_grpc_stream(&self, token_id: u32) -> Result<(), Status> {
        hostcalls::cancel_grpc_stream(token_id)
    }

    fn close_grpc_stream(&self, token_id: u32) -> Result<(), Status> {
        hostcalls::close_grpc_stream(token_id)
    }

    fn on_grpc_stream_close(&mut self, _token_id: u32, _status_code: u32) {}

    fn get_grpc_status(&self) -> Result<(u32, Option<String>), Status> {
        hostcalls::get_grpc_status()
    }

    fn on_foreign_function(&mut self, _function_id: u32, _arguments_size: usize) {}

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

    fn done(&self) -> Result<(), Status> {
        hostcalls::done()
    }
}

pub trait RootContext: Context {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        true
    }

    fn get_vm_configuration(&self) -> Result<Option<Bytes>, Status> {
        hostcalls::get_buffer(BufferType::VmConfiguration, 0, usize::MAX)
    }

    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        true
    }

    fn get_plugin_configuration(&self) -> Result<Option<Bytes>, Status> {
        hostcalls::get_buffer(BufferType::PluginConfiguration, 0, usize::MAX)
    }

    fn set_tick_period(&self, period: Duration) -> Result<(), Status> {
        hostcalls::set_tick_period(period)
    }

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

    fn get_downstream_data(&self, start: usize, max_size: usize) -> Result<Option<Bytes>, Status> {
        hostcalls::get_buffer(BufferType::DownstreamData, start, max_size)
    }

    fn set_downstream_data(&self, start: usize, size: usize, value: &[u8]) -> Result<(), Status> {
        hostcalls::set_buffer(BufferType::DownstreamData, start, size, value)
    }

    fn resume_downstream(&self) -> Result<(), Status> {
        hostcalls::resume_downstream()
    }

    fn close_downstream(&self) -> Result<(), Status> {
        hostcalls::close_downstream()
    }

    fn on_downstream_close(&mut self, _peer_type: PeerType) {}

    fn on_upstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        Action::Continue
    }

    fn get_upstream_data(&self, start: usize, max_size: usize) -> Result<Option<Bytes>, Status> {
        hostcalls::get_buffer(BufferType::UpstreamData, start, max_size)
    }

    fn set_upstream_data(&self, start: usize, size: usize, value: &[u8]) -> Result<(), Status> {
        hostcalls::set_buffer(BufferType::UpstreamData, start, size, value)
    }

    fn resume_upstream(&self) -> Result<(), Status> {
        hostcalls::resume_upstream()
    }

    fn close_upstream(&self) -> Result<(), Status> {
        hostcalls::close_upstream()
    }

    fn on_upstream_close(&mut self, _peer_type: PeerType) {}

    fn on_log(&mut self) {}
}

pub trait HttpContext: Context {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        Action::Continue
    }

    fn get_http_request_headers(&self) -> Result<Vec<(String, String)>, Status> {
        hostcalls::get_map(MapType::HttpRequestHeaders)
    }

    fn get_http_request_headers_bytes(&self) -> Result<Vec<(String, Bytes)>, Status> {
        hostcalls::get_map_bytes(MapType::HttpRequestHeaders)
    }

    fn set_http_request_headers(&self, headers: Vec<(&str, &str)>) -> Result<(), Status> {
        hostcalls::set_map(MapType::HttpRequestHeaders, headers)
    }

    fn set_http_request_headers_bytes(&self, headers: Vec<(&str, &[u8])>) -> Result<(), Status> {
        hostcalls::set_map_bytes(MapType::HttpRequestHeaders, headers)
    }

    fn get_http_request_header(&self, name: &str) -> Result<Option<String>, Status> {
        hostcalls::get_map_value(MapType::HttpRequestHeaders, name)
    }

    fn get_http_request_header_bytes(&self, name: &str) -> Result<Option<Bytes>, Status> {
        hostcalls::get_map_value_bytes(MapType::HttpRequestHeaders, name)
    }

    fn set_http_request_header(&self, name: &str, value: Option<&str>) -> Result<(), Status> {
        hostcalls::set_map_value(MapType::HttpRequestHeaders, name, value)
    }

    fn set_http_request_header_bytes(
        &self,
        name: &str,
        value: Option<&[u8]>,
    ) -> Result<(), Status> {
        hostcalls::set_map_value_bytes(MapType::HttpRequestHeaders, name, value)
    }

    fn add_http_request_header(&self, name: &str, value: &str) -> Result<(), Status> {
        hostcalls::add_map_value(MapType::HttpRequestHeaders, name, value)
    }

    fn add_http_request_header_bytes(&self, name: &str, value: &[u8]) -> Result<(), Status> {
        hostcalls::add_map_value_bytes(MapType::HttpRequestHeaders, name, value)
    }

    fn remove_http_request_header(&self, name: &str) -> Result<(), Status> {
        hostcalls::remove_map_value(MapType::HttpRequestHeaders, name)
    }

    fn on_http_request_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        Action::Continue
    }

    fn get_http_request_body(
        &self,
        start: usize,
        max_size: usize,
    ) -> Result<Option<Bytes>, Status> {
        hostcalls::get_buffer(BufferType::HttpRequestBody, start, max_size)
    }

    fn set_http_request_body(&self, start: usize, size: usize, value: &[u8]) -> Result<(), Status> {
        hostcalls::set_buffer(BufferType::HttpRequestBody, start, size, value)
    }

    fn on_http_request_trailers(&mut self, _num_trailers: usize) -> Action {
        Action::Continue
    }

    fn get_http_request_trailers(&self) -> Result<Vec<(String, String)>, Status> {
        hostcalls::get_map(MapType::HttpRequestTrailers)
    }

    fn get_http_request_trailers_bytes(&self) -> Result<Vec<(String, Bytes)>, Status> {
        hostcalls::get_map_bytes(MapType::HttpRequestTrailers)
    }

    fn set_http_request_trailers(&self, trailers: Vec<(&str, &str)>) -> Result<(), Status> {
        hostcalls::set_map(MapType::HttpRequestTrailers, trailers)
    }

    fn set_http_request_trailers_bytes(&self, trailers: Vec<(&str, &[u8])>) -> Result<(), Status> {
        hostcalls::set_map_bytes(MapType::HttpRequestTrailers, trailers)
    }

    fn get_http_request_trailer(&self, name: &str) -> Result<Option<String>, Status> {
        hostcalls::get_map_value(MapType::HttpRequestTrailers, name)
    }

    fn get_http_request_trailer_bytes(&self, name: &str) -> Result<Option<Bytes>, Status> {
        hostcalls::get_map_value_bytes(MapType::HttpRequestTrailers, name)
    }

    fn set_http_request_trailer(&self, name: &str, value: Option<&str>) -> Result<(), Status> {
        hostcalls::set_map_value(MapType::HttpRequestTrailers, name, value)
    }

    fn set_http_request_trailer_bytes(
        &self,
        name: &str,
        value: Option<&[u8]>,
    ) -> Result<(), Status> {
        hostcalls::set_map_value_bytes(MapType::HttpRequestTrailers, name, value)
    }

    fn add_http_request_trailer(&self, name: &str, value: &str) -> Result<(), Status> {
        hostcalls::add_map_value(MapType::HttpRequestTrailers, name, value)
    }

    fn add_http_request_trailer_bytes(&self, name: &str, value: &[u8]) -> Result<(), Status> {
        hostcalls::add_map_value_bytes(MapType::HttpRequestTrailers, name, value)
    }

    fn remove_http_request_trailer(&self, name: &str) -> Result<(), Status> {
        hostcalls::remove_map_value(MapType::HttpRequestTrailers, name)
    }

    fn resume_http_request(&self) -> Result<(), Status> {
        hostcalls::resume_http_request()
    }

    fn reset_http_request(&self) -> Result<(), Status> {
        hostcalls::reset_http_request()
    }

    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        Action::Continue
    }

    fn get_http_response_headers(&self) -> Result<Vec<(String, String)>, Status> {
        hostcalls::get_map(MapType::HttpResponseHeaders)
    }

    fn get_http_response_headers_bytes(&self) -> Result<Vec<(String, Bytes)>, Status> {
        hostcalls::get_map_bytes(MapType::HttpResponseHeaders)
    }

    fn set_http_response_headers(&self, headers: Vec<(&str, &str)>) -> Result<(), Status> {
        hostcalls::set_map(MapType::HttpResponseHeaders, headers)
    }

    fn set_http_response_headers_bytes(&self, headers: Vec<(&str, &[u8])>) -> Result<(), Status> {
        hostcalls::set_map_bytes(MapType::HttpResponseHeaders, headers)
    }

    fn get_http_response_header(&self, name: &str) -> Result<Option<String>, Status> {
        hostcalls::get_map_value(MapType::HttpResponseHeaders, name)
    }

    fn get_http_response_header_bytes(&self, name: &str) -> Result<Option<Bytes>, Status> {
        hostcalls::get_map_value_bytes(MapType::HttpResponseHeaders, name)
    }

    fn set_http_response_header(&self, name: &str, value: Option<&str>) -> Result<(), Status> {
        hostcalls::set_map_value(MapType::HttpResponseHeaders, name, value)
    }

    fn set_http_response_header_bytes(
        &self,
        name: &str,
        value: Option<&[u8]>,
    ) -> Result<(), Status> {
        hostcalls::set_map_value_bytes(MapType::HttpResponseHeaders, name, value)
    }

    fn add_http_response_header(&self, name: &str, value: &str) -> Result<(), Status> {
        hostcalls::add_map_value(MapType::HttpResponseHeaders, name, value)
    }

    fn add_http_response_header_bytes(&self, name: &str, value: &[u8]) -> Result<(), Status> {
        hostcalls::add_map_value_bytes(MapType::HttpResponseHeaders, name, value)
    }

    fn remove_http_response_header(&self, name: &str) -> Result<(), Status> {
        hostcalls::remove_map_value(MapType::HttpResponseHeaders, name)
    }

    fn on_http_response_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        Action::Continue
    }

    fn get_http_response_body(
        &self,
        start: usize,
        max_size: usize,
    ) -> Result<Option<Bytes>, Status> {
        hostcalls::get_buffer(BufferType::HttpResponseBody, start, max_size)
    }

    fn set_http_response_body(
        &self,
        start: usize,
        size: usize,
        value: &[u8],
    ) -> Result<(), Status> {
        hostcalls::set_buffer(BufferType::HttpResponseBody, start, size, value)
    }

    fn on_http_response_trailers(&mut self, _num_trailers: usize) -> Action {
        Action::Continue
    }

    fn get_http_response_trailers(&self) -> Result<Vec<(String, String)>, Status> {
        hostcalls::get_map(MapType::HttpResponseTrailers)
    }

    fn get_http_response_trailers_bytes(&self) -> Result<Vec<(String, Bytes)>, Status> {
        hostcalls::get_map_bytes(MapType::HttpResponseTrailers)
    }

    fn set_http_response_trailers(&self, trailers: Vec<(&str, &str)>) -> Result<(), Status> {
        hostcalls::set_map(MapType::HttpResponseTrailers, trailers)
    }

    fn set_http_response_trailers_bytes(&self, trailers: Vec<(&str, &[u8])>) -> Result<(), Status> {
        hostcalls::set_map_bytes(MapType::HttpResponseTrailers, trailers)
    }

    fn get_http_response_trailer(&self, name: &str) -> Result<Option<String>, Status> {
        hostcalls::get_map_value(MapType::HttpResponseTrailers, name)
    }

    fn get_http_response_trailer_bytes(&self, name: &str) -> Result<Option<Bytes>, Status> {
        hostcalls::get_map_value_bytes(MapType::HttpResponseTrailers, name)
    }

    fn set_http_response_trailer(&self, name: &str, value: Option<&str>) -> Result<(), Status> {
        hostcalls::set_map_value(MapType::HttpResponseTrailers, name, value)
    }

    fn set_http_response_trailer_bytes(
        &self,
        name: &str,
        value: Option<&[u8]>,
    ) -> Result<(), Status> {
        hostcalls::set_map_value_bytes(MapType::HttpResponseTrailers, name, value)
    }

    fn add_http_response_trailer(&self, name: &str, value: &str) -> Result<(), Status> {
        hostcalls::add_map_value(MapType::HttpResponseTrailers, name, value)
    }

    fn add_http_response_trailer_bytes(&self, name: &str, value: &[u8]) -> Result<(), Status> {
        hostcalls::add_map_value_bytes(MapType::HttpResponseTrailers, name, value)
    }

    fn remove_http_response_trailer(&self, name: &str) -> Result<(), Status> {
        hostcalls::remove_map_value(MapType::HttpResponseTrailers, name)
    }

    fn resume_http_response(&self) -> Result<(), Status> {
        hostcalls::resume_http_response()
    }

    fn reset_http_response(&self) -> Result<(), Status> {
        hostcalls::reset_http_response()
    }

    fn send_http_response(
        &self,
        status_code: u32,
        headers: Vec<(&str, &str)>,
        body: Option<&[u8]>,
    ) -> Result<(), Status> {
        hostcalls::send_http_response(status_code, headers, body)
    }

    fn send_grpc_response(
        &self,
        grpc_status: GrpcStatusCode,
        grpc_status_message: Option<&str>,
        custom_metadata: Vec<(&str, &[u8])>,
    ) -> Result<(), Status> {
        hostcalls::send_grpc_response(grpc_status, grpc_status_message, custom_metadata)
    }

    fn on_log(&mut self) {}
}
