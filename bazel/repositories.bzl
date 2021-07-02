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
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

def proxy_wasm_rust_sdk_repositories():
    maybe(
        http_archive,
        name = "rules_rust",
        sha256 = "d54b379559f3fe6ff0cd251be216a5e35acf241451eec8144455482e8f4748f8",
        strip_prefix = "rules_rust-7e7246f6c48a5d4e69744cd79b9ccb8886966ee2",
        url = "https://github.com/bazelbuild/rules_rust/archive/7e7246f6c48a5d4e69744cd79b9ccb8886966ee2.tar.gz",
    )
