# Copyright 2020 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def proxy_wasm_rust_sdk_repositories():
    http_archive(
        name = "io_bazel_rules_rust",
        sha256 = "5cb2fbcc3debebc7b68f5f66c1b7ef741bdcca87c70594de688d4518538c36c8",
        strip_prefix = "rules_rust-aa7c6938cf1cc2973bc065c7532f89874bf09818",
        url = "https://github.com/bazelbuild/rules_rust/archive/aa7c6938cf1cc2973bc065c7532f89874bf09818.tar.gz",
    )
