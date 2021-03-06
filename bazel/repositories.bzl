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
        name = "cargo_raze",
        sha256 = "c664e258ea79e7e4ec2f2b57bca8b1c37f11c8d5748e02b8224810da969eb681",
        strip_prefix = "cargo-raze-0.11.0",
        url = "https://github.com/google/cargo-raze/archive/v0.11.0.tar.gz",
    )

    maybe(
        http_archive,
        name = "rules_foreign_cc",
        sha256 = "c2cdcf55ffaf49366725639e45dedd449b8c3fe22b54e31625eb80ce3a240f1e",
        strip_prefix = "rules_foreign_cc-0.1.0",
        url = "https://github.com/bazelbuild/rules_foreign_cc/archive/0.1.0.zip",
    )

    maybe(
        http_archive,
        name = "rules_rust",
        sha256 = "f2d9f804e1a8042a41ad41e1aeeca55ad0fc2d294ecd52e34ef8c63f7ce350fd",
        strip_prefix = "rules_rust-3b02397bde43b1eeee1528227ceb3da6c6bdadd6",
        url = "https://github.com/bazelbuild/rules_rust/archive/3b02397bde43b1eeee1528227ceb3da6c6bdadd6.tar.gz",
    )
