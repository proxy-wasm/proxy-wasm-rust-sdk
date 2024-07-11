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
use crate::traits;
use crate::types;

pub struct Counter {
    id: u32,
    name: String,
}

impl Counter {
    pub fn counter(name: String) -> Counter {
        let returned_id = hostcalls::define_metric(types::MetricType::Counter, &name).unwrap();
        Counter {
            id: returned_id,
            name,
        }
    }
}

impl traits::Metric for Counter {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn record(&self, _: u64) {
        // A Counter can only be incremented.
        return;
    }
}

pub struct Gauge {
    id: u32,
    name: String,
}

impl Gauge {
    pub fn gauge(name: String) -> Gauge {
        let returned_id = hostcalls::define_metric(types::MetricType::Gauge, &name).unwrap();
        Gauge {
            id: returned_id,
            name,
        }
    }
}

impl traits::Metric for Gauge {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn increment(&self, _: i64) {
        // A gauge can only be recorded.
        return;
    }
}

pub struct Histogram {
    id: u32,
    name: String,
}

impl Histogram {
    pub fn histogram(name: String) -> Histogram {
        let returned_id = hostcalls::define_metric(types::MetricType::Histogram, &name).unwrap();
        Histogram {
            id: returned_id,
            name,
        }
    }
}

impl traits::Metric for Histogram {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn increment(&self, _: i64) {
        // A Histogram can only be recorded.
        return;
    }
}
