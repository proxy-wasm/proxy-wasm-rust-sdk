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
        sha256 = "6501960c3e4da32495d1e1007ded0769a534cb195c30dea36aa54f9d8a3f0361",
        url = "https://github.com/bazelbuild/rules_rust/releases/download/0.38.0/rules_rust-v0.38.0.tar.gz",
        patches = ["//bazel:rules_rust.patch"],
        patch_args = ["-p1"],
    )

    maybe(
        http_archive,
        name = "bazel_skylib",
        sha256 = "cd55a062e763b9349921f0f5db8c3933288dc8ba4f76dd9416aac68acee3cb94",
        url = "https://github.com/bazelbuild/bazel-skylib/releases/download/1.5.0/bazel-skylib-1.5.0.tar.gz",
    )
