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
        sha256 = "73580f341f251f2fc633b73cdf74910f4da64d06a44c063cbf5c01b1de753ec1",
        url = "https://github.com/bazelbuild/rules_rust/releases/download/0.5.0/rules_rust-v0.5.0.tar.gz",
        patches = ["//bazel:rules_rust.patch"],
        patch_args = ["-p1"],
    )
