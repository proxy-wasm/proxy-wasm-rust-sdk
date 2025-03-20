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
use crate::traits::*;
use crate::types::*;
use hashbrown::HashMap;
use std::cell::{Cell, RefCell};

thread_local! {
static DISPATCHER: Dispatcher = Dispatcher::new();
}

pub(crate) fn set_root_context(callback: NewRootContext) {
    DISPATCHER.with(|dispatcher| dispatcher.set_root_context(callback));
}

pub(crate) fn set_stream_context(callback: NewStreamContext) {
    DISPATCHER.with(|dispatcher| dispatcher.set_stream_context(callback));
}

pub(crate) fn set_http_context(callback: NewHttpContext) {
    DISPATCHER.with(|dispatcher| dispatcher.set_http_context(callback));
}

pub(crate) fn register_callout(token_id: u32) -> Result<(), HostError> {
    DISPATCHER.with(|dispatcher| dispatcher.register_callout(token_id))
}

pub(crate) fn register_grpc_callout(token_id: u32) -> Result<(), HostError> {
    DISPATCHER.with(|dispatcher| dispatcher.register_grpc_callout(token_id))
}

pub(crate) fn register_grpc_stream(token_id: u32) -> Result<(), HostError> {
    DISPATCHER.with(|dispatcher| dispatcher.register_grpc_stream(token_id))
}

struct NoopRoot;

impl Context for NoopRoot {}
impl RootContext for NoopRoot {}

struct Dispatcher {
    new_root: Cell<Option<NewRootContext>>,
    roots: RefCell<HashMap<u32, Box<dyn RootContext>>>,
    new_stream: Cell<Option<NewStreamContext>>,
    streams: RefCell<HashMap<u32, Box<dyn StreamContext>>>,
    new_http_stream: Cell<Option<NewHttpContext>>,
    http_streams: RefCell<HashMap<u32, Box<dyn HttpContext>>>,
    active_id: Cell<u32>,
    callouts: RefCell<HashMap<u32, u32>>,
    grpc_callouts: RefCell<HashMap<u32, u32>>,
    grpc_streams: RefCell<HashMap<u32, u32>>,
}

impl Dispatcher {
    fn new() -> Dispatcher {
        Dispatcher {
            new_root: Cell::new(None),
            roots: RefCell::new(HashMap::new()),
            new_stream: Cell::new(None),
            streams: RefCell::new(HashMap::new()),
            new_http_stream: Cell::new(None),
            http_streams: RefCell::new(HashMap::new()),
            active_id: Cell::new(0),
            callouts: RefCell::new(HashMap::new()),
            grpc_callouts: RefCell::new(HashMap::new()),
            grpc_streams: RefCell::new(HashMap::new()),
        }
    }

    fn set_root_context(&self, callback: NewRootContext) {
        self.new_root.set(Some(callback));
    }

    fn set_stream_context(&self, callback: NewStreamContext) {
        self.new_stream.set(Some(callback));
    }

    fn set_http_context(&self, callback: NewHttpContext) {
        self.new_http_stream.set(Some(callback));
    }

    fn register_callout(&self, token_id: u32) -> Result<(), HostError> {
        if self
            .callouts
            .borrow_mut()
            .insert(token_id, self.active_id.get())
            .is_some()
        {
            Err("duplicate token_id".into())
        } else {
            Ok(())
        }
    }

    fn register_grpc_stream(&self, token_id: u32) -> Result<(), HostError> {
        if self
            .grpc_streams
            .borrow_mut()
            .insert(token_id, self.active_id.get())
            .is_some()
        {
            Err("duplicate token_id".into())
        } else {
            Ok(())
        }
    }

    fn register_grpc_callout(&self, token_id: u32) -> Result<(), HostError> {
        if self
            .grpc_callouts
            .borrow_mut()
            .insert(token_id, self.active_id.get())
            .is_some()
        {
            Err("duplicate token_id".into())
        } else {
            Ok(())
        }
    }

    fn create_root_context(&self, context_id: u32) -> Result<(), HostError> {
        let new_context = match self.new_root.get() {
            Some(f) => f(context_id),
            None => Box::new(NoopRoot),
        };
        if self
            .roots
            .borrow_mut()
            .insert(context_id, new_context)
            .is_some()
        {
            Err("duplicate context_id".into())
        } else {
            Ok(())
        }
    }

    fn create_stream_context(
        &self,
        context_id: u32,
        root_context_id: u32,
    ) -> Result<(), HostError> {
        let new_context = match self.roots.borrow().get(&root_context_id) {
            Some(root_context) => match self.new_stream.get() {
                Some(f) => f(context_id, root_context_id),
                None => match root_context.create_stream_context(context_id) {
                    Some(stream_context) => stream_context,
                    None => return Err("create_stream_context returned None".into()),
                },
            },
            None => return Err("invalid root_context_id".into()),
        };
        if self
            .streams
            .borrow_mut()
            .insert(context_id, new_context)
            .is_some()
        {
            return Err("duplicate context_id".into());
        }
        Ok(())
    }

    fn create_http_context(&self, context_id: u32, root_context_id: u32) -> Result<(), HostError> {
        let new_context = match self.roots.borrow().get(&root_context_id) {
            Some(root_context) => match self.new_http_stream.get() {
                Some(f) => f(context_id, root_context_id),
                None => match root_context.create_http_context(context_id) {
                    Some(stream_context) => stream_context,
                    None => return Err("create_http_context returned None".into()),
                },
            },
            None => return Err("invalid root_context_id".into()),
        };
        if self
            .http_streams
            .borrow_mut()
            .insert(context_id, new_context)
            .is_some()
        {
            return Err("duplicate context_id".into());
        }
        Ok(())
    }

    fn on_create_context(&self, context_id: u32, root_context_id: u32) -> Result<(), HostError> {
        if root_context_id == 0 {
            self.create_root_context(context_id)
        } else if self.new_http_stream.get().is_some() {
            self.create_http_context(context_id, root_context_id)
        } else if self.new_stream.get().is_some() {
            self.create_stream_context(context_id, root_context_id)
        } else if let Some(root_context) = self.roots.borrow().get(&root_context_id) {
            match root_context.get_type() {
                Some(ContextType::HttpContext) => {
                    self.create_http_context(context_id, root_context_id)
                }
                Some(ContextType::StreamContext) => {
                    self.create_stream_context(context_id, root_context_id)
                }
                None => Err("missing ContextType on root_context".into()),
            }
        } else {
            Err("invalid root_context_id and missing constructors".into())
        }
    }

    fn on_done(&self, context_id: u32) -> Result<bool, HostError> {
        if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            Ok(http_stream.on_done())
        } else if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            Ok(stream.on_done())
        } else if let Some(root) = self.roots.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            Ok(root.on_done())
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_log(&self, context_id: u32) -> Result<(), HostError> {
        if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            http_stream.on_log();
            Ok(())
        } else if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            stream.on_log();
            Ok(())
        } else if let Some(root) = self.roots.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            root.on_log();
            Ok(())
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_delete(&self, context_id: u32) -> Result<(), HostError> {
        if !(self.http_streams.borrow_mut().remove(&context_id).is_some()
            || self.streams.borrow_mut().remove(&context_id).is_some()
            || self.roots.borrow_mut().remove(&context_id).is_some())
        {
            Err("invalid context_id".into())
        } else {
            Ok(())
        }
    }

    fn on_vm_start(
        &self,
        context_id: u32,
        vm_configuration_size: usize,
    ) -> Result<bool, HostError> {
        if let Some(root) = self.roots.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            Ok(root.on_vm_start(vm_configuration_size))
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_configure(
        &self,
        context_id: u32,
        plugin_configuration_size: usize,
    ) -> Result<bool, HostError> {
        if let Some(root) = self.roots.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            Ok(root.on_configure(plugin_configuration_size))
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_tick(&self, context_id: u32) -> Result<(), HostError> {
        if let Some(root) = self.roots.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            root.on_tick();
            Ok(())
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_queue_ready(&self, context_id: u32, queue_id: u32) -> Result<(), HostError> {
        if let Some(root) = self.roots.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            root.on_queue_ready(queue_id);
            Ok(())
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_new_connection(&self, context_id: u32) -> Result<Action, HostError> {
        if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            Ok(stream.on_new_connection())
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_downstream_data(
        &self,
        context_id: u32,
        data_size: usize,
        end_of_stream: bool,
    ) -> Result<Action, HostError> {
        if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            Ok(stream.on_downstream_data(data_size, end_of_stream))
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_downstream_close(&self, context_id: u32, peer_type: PeerType) -> Result<(), HostError> {
        if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            stream.on_downstream_close(peer_type);
            Ok(())
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_upstream_data(
        &self,
        context_id: u32,
        data_size: usize,
        end_of_stream: bool,
    ) -> Result<Action, HostError> {
        if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            Ok(stream.on_upstream_data(data_size, end_of_stream))
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_upstream_close(&self, context_id: u32, peer_type: PeerType) -> Result<(), HostError> {
        if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            stream.on_upstream_close(peer_type);
            Ok(())
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_http_request_headers(
        &self,
        context_id: u32,
        num_headers: usize,
        end_of_stream: bool,
    ) -> Result<Action, HostError> {
        if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            Ok(http_stream.on_http_request_headers(num_headers, end_of_stream))
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_http_request_body(
        &self,
        context_id: u32,
        body_size: usize,
        end_of_stream: bool,
    ) -> Result<Action, HostError> {
        if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            Ok(http_stream.on_http_request_body(body_size, end_of_stream))
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_http_request_trailers(
        &self,
        context_id: u32,
        num_trailers: usize,
    ) -> Result<Action, HostError> {
        if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            Ok(http_stream.on_http_request_trailers(num_trailers))
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_http_response_headers(
        &self,
        context_id: u32,
        num_headers: usize,
        end_of_stream: bool,
    ) -> Result<Action, HostError> {
        if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            Ok(http_stream.on_http_response_headers(num_headers, end_of_stream))
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_http_response_body(
        &self,
        context_id: u32,
        body_size: usize,
        end_of_stream: bool,
    ) -> Result<Action, HostError> {
        if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            Ok(http_stream.on_http_response_body(body_size, end_of_stream))
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_http_response_trailers(
        &self,
        context_id: u32,
        num_trailers: usize,
    ) -> Result<Action, HostError> {
        if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            Ok(http_stream.on_http_response_trailers(num_trailers))
        } else {
            Err("invalid context_id".into())
        }
    }

    fn on_http_call_response(
        &self,
        token_id: u32,
        num_headers: usize,
        body_size: usize,
        num_trailers: usize,
    ) -> Result<(), HostError> {
        match self.callouts.borrow_mut().remove(&token_id) {
            None => Err("invalid token_id".into()),
            Some(context_id) => {
                if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    http_stream.on_http_call_response(
                        token_id,
                        num_headers,
                        body_size,
                        num_trailers,
                    );
                    Ok(())
                } else if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    stream.on_http_call_response(token_id, num_headers, body_size, num_trailers);
                    Ok(())
                } else if let Some(root) = self.roots.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    root.on_http_call_response(token_id, num_headers, body_size, num_trailers);
                    Ok(())
                } else {
                    Err("invalid context_id, no match found".into())
                }
            }
        }
    }

    fn on_grpc_receive_initial_metadata(
        &self,
        token_id: u32,
        headers: u32,
    ) -> Result<(), HostError> {
        match self.grpc_streams.borrow_mut().get(&token_id) {
            Some(id) => {
                let context_id = *id;
                if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    http_stream.on_grpc_stream_initial_metadata(token_id, headers);
                    Ok(())
                } else if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    stream.on_grpc_stream_initial_metadata(token_id, headers);
                    Ok(())
                } else if let Some(root) = self.roots.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    root.on_grpc_stream_initial_metadata(token_id, headers);
                    Ok(())
                } else {
                    Err("invalid context_id, no match found".into())
                }
            }
            None => Err("invalid token_id".into()),
        }
    }

    fn on_grpc_receive(&self, token_id: u32, response_size: usize) -> Result<(), HostError> {
        let context_id = self.grpc_callouts.borrow_mut().remove(&token_id);
        if let Some(context_id) = context_id {
            if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
                self.active_id.set(context_id);
                hostcalls::set_effective_context(context_id)?;
                http_stream.on_grpc_call_response(token_id, 0, response_size);
                Ok(())
            } else if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
                self.active_id.set(context_id);
                hostcalls::set_effective_context(context_id)?;
                stream.on_grpc_call_response(token_id, 0, response_size);
                Ok(())
            } else if let Some(root) = self.roots.borrow_mut().get_mut(&context_id) {
                self.active_id.set(context_id);
                hostcalls::set_effective_context(context_id)?;
                root.on_grpc_call_response(token_id, 0, response_size);
                Ok(())
            } else {
                Err("invalid context_id, no match found".into())
            }
        } else {
            let context_id = self.grpc_streams.borrow().get(&token_id).cloned();
            if let Some(context_id) = context_id {
                if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    http_stream.on_grpc_stream_message(token_id, response_size);
                    Ok(())
                } else if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    stream.on_grpc_stream_message(token_id, response_size);
                    Ok(())
                } else if let Some(root) = self.roots.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    root.on_grpc_stream_message(token_id, response_size);
                    Ok(())
                } else {
                    Err("invalid context_id, no match found".into())
                }
            } else {
                Err("invalid token_id".into())
            }
        }
    }

    fn on_grpc_receive_trailing_metadata(
        &self,
        token_id: u32,
        trailers: u32,
    ) -> Result<(), HostError> {
        match self.grpc_streams.borrow_mut().get(&token_id) {
            Some(id) => {
                let context_id = *id;
                if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    http_stream.on_grpc_stream_trailing_metadata(token_id, trailers);
                    Ok(())
                } else if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    stream.on_grpc_stream_trailing_metadata(token_id, trailers);
                    Ok(())
                } else if let Some(root) = self.roots.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    root.on_grpc_stream_trailing_metadata(token_id, trailers);
                    Ok(())
                } else {
                    Err("invalid context_id, no match found".into())
                }
            }
            None => Err("invalid token_id".into()),
        }
    }

    fn on_grpc_close(&self, token_id: u32, status_code: u32) -> Result<(), HostError> {
        let context_id = self.grpc_callouts.borrow_mut().remove(&token_id);
        if let Some(context_id) = context_id {
            if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
                self.active_id.set(context_id);
                hostcalls::set_effective_context(context_id)?;
                http_stream.on_grpc_call_response(token_id, status_code, 0);
                Ok(())
            } else if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
                self.active_id.set(context_id);
                hostcalls::set_effective_context(context_id)?;
                stream.on_grpc_call_response(token_id, status_code, 0);
                Ok(())
            } else if let Some(root) = self.roots.borrow_mut().get_mut(&context_id) {
                self.active_id.set(context_id);
                hostcalls::set_effective_context(context_id)?;
                root.on_grpc_call_response(token_id, status_code, 0);
                Ok(())
            } else {
                Err("invalid context_id, no match found".into())
            }
        } else {
            let context_id = self.grpc_streams.borrow_mut().remove(&token_id);
            if let Some(context_id) = context_id {
                if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    http_stream.on_grpc_stream_close(token_id, status_code);
                    Ok(())
                } else if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    stream.on_grpc_stream_close(token_id, status_code);
                    Ok(())
                } else if let Some(root) = self.roots.borrow_mut().get_mut(&context_id) {
                    self.active_id.set(context_id);
                    hostcalls::set_effective_context(context_id)?;
                    root.on_grpc_stream_close(token_id, status_code);
                    Ok(())
                } else {
                    Err("invalid context_id, no match found".into())
                }
            } else {
                Err("on_grpc_close: invalid token_id, a non-connected stream has closed".into())
            }
        }
    }

    fn on_foreign_function(
        &self,
        context_id: u32,
        function_id: u32,
        arugments_size: usize,
    ) -> Result<(), HostError> {
        if let Some(http_stream) = self.http_streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            hostcalls::set_effective_context(context_id)?;
            http_stream.on_foreign_function(function_id, arugments_size);
            Ok(())
        } else if let Some(stream) = self.streams.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            hostcalls::set_effective_context(context_id)?;
            stream.on_foreign_function(function_id, arugments_size);
            Ok(())
        } else if let Some(root) = self.roots.borrow_mut().get_mut(&context_id) {
            self.active_id.set(context_id);
            hostcalls::set_effective_context(context_id)?;
            root.on_foreign_function(function_id, arugments_size);
            Ok(())
        } else {
            Err("invalid context_id, no match found".into())
        }
    }
}

#[no_mangle]
pub extern "C" fn proxy_on_context_create(context_id: u32, root_context_id: u32) {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_create_context(context_id, root_context_id)
            .inspect_err(|err| log::error!("proxy_on_done {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_done(context_id: u32) -> bool {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_done(context_id)
            .inspect_err(|err| log::error!("proxy_on_done {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_log(context_id: u32) {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_log(context_id)
            .inspect_err(|err| log::error!("proxy_on_log {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_delete(context_id: u32) {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_delete(context_id)
            .inspect_err(|err| log::error!("proxy_on_delete {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_vm_start(context_id: u32, vm_configuration_size: usize) -> bool {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_vm_start(context_id, vm_configuration_size)
            .inspect_err(|err| log::error!("proxy_on_vm_start {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_configure(context_id: u32, plugin_configuration_size: usize) -> bool {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_configure(context_id, plugin_configuration_size)
            .inspect_err(|err| log::error!("proxy_on_configure {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_tick(context_id: u32) {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_tick(context_id)
            .inspect_err(|err| log::error!("proxy_on_tick {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_queue_ready(context_id: u32, queue_id: u32) {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_queue_ready(context_id, queue_id)
            .inspect_err(|err| log::error!("proxy_on_queue_ready {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_new_connection(context_id: u32) -> Action {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_new_connection(context_id)
            .inspect_err(|err| log::error!("proxy_on_new_connection {}", err))
            .unwrap_or(Action::Continue)
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_downstream_data(
    context_id: u32,
    data_size: usize,
    end_of_stream: bool,
) -> Action {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_downstream_data(context_id, data_size, end_of_stream)
            .inspect_err(|err| log::error!("proxy_on_downstream_data {}", err))
            .unwrap_or(Action::Continue)
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_downstream_connection_close(context_id: u32, peer_type: PeerType) {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_downstream_close(context_id, peer_type)
            .inspect_err(|err| log::error!("proxy_on_downstream_connection_close {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_upstream_data(
    context_id: u32,
    data_size: usize,
    end_of_stream: bool,
) -> Action {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_upstream_data(context_id, data_size, end_of_stream)
            .inspect_err(|err| log::error!("proxy_on_upstream_data {}", err))
            .unwrap_or(Action::Continue)
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_upstream_connection_close(context_id: u32, peer_type: PeerType) {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_upstream_close(context_id, peer_type)
            .inspect_err(|err| log::error!("proxy_on_upstream_connection_close {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_request_headers(
    context_id: u32,
    num_headers: usize,
    end_of_stream: bool,
) -> Action {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_http_request_headers(context_id, num_headers, end_of_stream)
            .inspect_err(|err| log::error!("proxy_on_request_headers {}", err))
            .unwrap_or(Action::Continue)
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_request_body(
    context_id: u32,
    body_size: usize,
    end_of_stream: bool,
) -> Action {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_http_request_body(context_id, body_size, end_of_stream)
            .inspect_err(|err| log::error!("proxy_on_request_body {}", err))
            .unwrap_or(Action::Continue)
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_request_trailers(context_id: u32, num_trailers: usize) -> Action {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_http_request_trailers(context_id, num_trailers)
            .inspect_err(|err| log::error!("proxy_on_request_trailers {}", err))
            .unwrap_or(Action::Continue)
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_response_headers(
    context_id: u32,
    num_headers: usize,
    end_of_stream: bool,
) -> Action {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_http_response_headers(context_id, num_headers, end_of_stream)
            .inspect_err(|err| log::error!("proxy_on_response_headers {}", err))
            .unwrap_or(Action::Continue)
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_response_body(
    context_id: u32,
    body_size: usize,
    end_of_stream: bool,
) -> Action {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_http_response_body(context_id, body_size, end_of_stream)
            .inspect_err(|err| log::error!("proxy_on_response_body {}", err))
            .unwrap_or(Action::Continue)
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_response_trailers(context_id: u32, num_trailers: usize) -> Action {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_http_response_trailers(context_id, num_trailers)
            .inspect_err(|err| log::error!("proxy_on_response_trailers {}", err))
            .unwrap_or(Action::Continue)
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_http_call_response(
    _context_id: u32,
    token_id: u32,
    num_headers: usize,
    body_size: usize,
    num_trailers: usize,
) {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_http_call_response(token_id, num_headers, body_size, num_trailers)
            .inspect_err(|err| log::error!("proxy_on_http_call_response {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_grpc_receive_initial_metadata(
    _context_id: u32,
    token_id: u32,
    headers: u32,
) {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_grpc_receive_initial_metadata(token_id, headers)
            .inspect_err(|err| log::error!("proxy_on_grpc_receive_initial_metadata {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_grpc_receive(_context_id: u32, token_id: u32, response_size: usize) {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_grpc_receive(token_id, response_size)
            .inspect_err(|err| log::error!("proxy_on_grpc_receive {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_grpc_receive_trailing_metadata(
    _context_id: u32,
    token_id: u32,
    trailers: u32,
) {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_grpc_receive_trailing_metadata(token_id, trailers)
            .inspect_err(|err| log::error!("proxy_on_grpc_receive_trailing_metadata {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_grpc_close(_context_id: u32, token_id: u32, status_code: u32) {
    DISPATCHER.with(|dispatcher| {
        dispatcher
            .on_grpc_close(token_id, status_code)
            .inspect_err(|err| log::error!("proxy_on_grpc_close {}", err))
            .unwrap_or_default()
    })
}

#[no_mangle]
pub extern "C" fn proxy_on_foreign_function(
    context_id: u32,
    function_id: u32,
    arguments_size: usize,
) {
    DISPATCHER
        .with(|dispatcher| {
            dispatcher
                .on_foreign_function(context_id, function_id, arguments_size)
                .inspect_err(|err| log::error!("proxy_on_foreign_function {}", err))
        })
        .unwrap_or_default()
}
