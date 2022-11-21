# Copyright 2022 Google LLC
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

load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load("@rules_rust//rust:defs.bzl", "rust_library")

cargo_build_script(
    name = "proxy_wasm_build_script",
    srcs = ["build.rs"],
    edition = "2018",
    tags = ["manual"],
    visibility = ["//visibility:private"],
)

rust_library(
    name = "proxy_wasm",
    srcs = glob(["src/*.rs"]),
    edition = "2018",
    visibility = ["//visibility:public"],
    deps = [
        ":proxy_wasm_build_script",
        "//bazel/cargo:hashbrown",
        "//bazel/cargo:log",
    ],
)
