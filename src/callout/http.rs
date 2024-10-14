use crate::callout::promise::Promise;
use crate::hostcalls;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

type OnHttpResponseArgs = (u32, usize, usize, usize);

#[derive(Default)]
pub struct HttpClient {
    m: HashMap<u32, Rc<Promise<OnHttpResponseArgs>>>,
}

impl HttpClient {
    pub fn dispatch(
        &mut self,
        upstream: &str,
        headers: Vec<(&str, &str)>,
        body: Option<&[u8]>,
        trailers: Vec<(&str, &str)>,
        timeout: Duration,
    ) -> Rc<Promise<(u32, usize, usize, usize)>> {
        let token =
            hostcalls::dispatch_http_call(upstream, headers, body, trailers, timeout).unwrap();
        let promise = Promise::new();
        self.m.insert(token, promise.clone());
        promise
    }

    pub fn callback(
        &mut self,
        token_id: u32,
        num_headers: usize,
        body_size: usize,
        num_trailers: usize,
    ) {
        let promise = self.m.remove(&token_id);
        promise
            .unwrap()
            .fulfill((token_id, num_headers, body_size, num_trailers))
    }
}
