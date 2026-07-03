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

load("@rules_rust//cargo:defs.bzl", "cargo_build_script")
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_shared_library")

exports_files([
    "Cargo.toml",
])

cargo_build_script(
    name = "proxy_wasm_build_script",
    srcs = ["build.rs"],
    edition = "2024",
    tags = ["manual"],
    visibility = ["//visibility:private"],
)

rust_library(
    name = "proxy_wasm",
    srcs = glob(["src/*.rs"]),
    edition = "2024",
    visibility = ["//visibility:public"],
    deps = [
        ":proxy_wasm_build_script",
        "//bazel/cargo/remote:hashbrown",
        "//bazel/cargo/remote:log",
    ],
)

rust_shared_library(
    name = "http_auth_random",
    srcs = ["examples/http_auth_random/src/lib.rs"],
    edition = "2024",
    rustc_flags = ["-Cstrip=debuginfo"],
    visibility = ["//visibility:private"],
    deps = [
        ":proxy_wasm",
        "//bazel/cargo/remote:log",
    ],
)
