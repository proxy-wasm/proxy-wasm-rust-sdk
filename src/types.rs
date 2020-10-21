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

use crate::traits::*;

pub type NewRootContext = fn(context_id: u32) -> Box<dyn RootContext>;
pub type NewStreamContext = fn(context_id: u32, root_context_id: u32) -> Box<dyn StreamContext>;
pub type NewHttpContext = fn(context_id: u32, root_context_id: u32) -> Box<dyn HttpContext>;

#[repr(u32)]
#[derive(Debug)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Critical = 5,
}

#[repr(u32)]
#[derive(Debug)]
pub enum FilterStatus {
    Continue = 0,
    StopIteration = 1,
}

#[repr(u32)]
#[derive(Debug)]
pub enum FilterHeadersStatus {
    Continue = 0,
    StopIteration = 1,
    ContinueAndEndStream = 2,
    StopAllIterationAndBuffer = 3,
    StopAllIterationAndWatermark = 4,
}

#[repr(u32)]
#[derive(Debug)]
pub enum FilterMetadataStatus {
    Continue = 0,
}

#[repr(u32)]
#[derive(Debug)]
pub enum FilterTrailersStatus {
    Continue = 0,
    StopIteration = 1,
}

#[repr(u32)]
#[derive(Debug)]
pub enum FilterDataStatus {
    Continue = 0,
    StopIterationAndBuffer = 1,
    StopIterationAndWatermark = 2,
    StopIterationNoBuffer = 3,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum Status {
    Ok = 0,
    NotFound = 1,
    BadArgument = 2,
    SerializationFailure = 3,
    ParseFailure = 4,
    BadExpression = 5,
    InvalidMemoryAccess = 6,
    Empty = 7,
    CasMismatch = 8,
    ResultMismatch = 9,
    InternalFailure = 10,
    BrokenConnection = 11,
    Unimplemented = 12,
}

#[repr(u32)]
#[derive(Debug)]
pub enum BufferType {
    HttpRequestBody = 0,
    HttpResponseBody = 1,
    DownstreamData = 2,
    UpstreamData = 3,
    HttpCallResponseBody = 4,
    GrpcReceiveBuffer = 5,
    VmConfiguration = 6,
    PluginConfiguration = 7,
    CallData = 8,
}

#[repr(u32)]
#[derive(Debug)]
pub enum MapType {
    HttpRequestHeaders = 0,
    HttpRequestTrailers = 1,
    HttpResponseHeaders = 2,
    HttpResponseTrailers = 3,
    GrpcReceiveInitialMetadata = 4,
    GrpcReceiveTrailingMetadata = 5,
    HttpCallResponseHeaders = 6,
    HttpCallResponseTrailers = 7,
}

#[repr(u32)]
#[derive(Debug)]
pub enum PeerType {
    Unknown = 0,
    Local = 1,
    Remote = 2,
}

#[repr(u32)]
#[derive(Debug)]
pub enum MetricType {
    Counter = 0,
    Gauge = 1,
    Histogram = 2,
}

pub type Bytes = Vec<u8>;
