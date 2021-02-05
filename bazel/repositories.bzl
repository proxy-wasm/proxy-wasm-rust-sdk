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
        name = "rules_rust",
        sha256 = "f2d9f804e1a8042a41ad41e1aeeca55ad0fc2d294ecd52e34ef8c63f7ce350fd",
        strip_prefix = "rules_rust-3b02397bde43b1eeee1528227ceb3da6c6bdadd6",
        url = "https://github.com/bazelbuild/rules_rust/archive/3b02397bde43b1eeee1528227ceb3da6c6bdadd6.tar.gz",
    )
