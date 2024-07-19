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

#[derive(Copy, Clone)]
pub struct Counter {
    id: u32,
}

impl Counter {
    pub fn new(name: String) -> Counter {
        let returned_id = hostcalls::define_metric(types::MetricType::Counter, &name)
            .expect("failed to define counter '{}', name");
        Counter { id: returned_id }
    }
}

impl traits::Metric for Counter {
    fn id(&self) -> u32 {
        self.id
    }
}

impl traits::IncrementingMetric for Counter {}

#[derive(Copy, Clone)]
pub struct Gauge {
    id: u32,
}

impl Gauge {
    pub fn new(name: String) -> Gauge {
        let returned_id = hostcalls::define_metric(types::MetricType::Gauge, &name)
            .expect("failed to define gauge '{}', name");
        Gauge { id: returned_id }
    }
}

impl traits::Metric for Gauge {
    fn id(&self) -> u32 {
        self.id
    }
}

impl traits::RecordingMetric for Gauge {}

#[derive(Copy, Clone)]
pub struct Histogram {
    id: u32,
}

impl Histogram {
    pub fn new(name: String) -> Histogram {
        let returned_id = hostcalls::define_metric(types::MetricType::Histogram, &name)
            .expect("failed to define histogram '{}', name");
        Histogram { id: returned_id }
    }
}

impl traits::Metric for Histogram {
    fn id(&self) -> u32 {
        self.id
    }
}

impl traits::RecordingMetric for Histogram {}
