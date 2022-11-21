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

load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "hello_world",
    srcs = ["hello_world.rs"],
    crate_type = "cdylib",
    edition = "2018",
    out_binary = True,
    deps = [
        "//:proxy_wasm",
        "//bazel/cargo:cfg_if",
        "//bazel/cargo:chrono",
        "//bazel/cargo:log",
    ] + select({
        "@rules_rust//rust/platform:wasm32-unknown-unknown": [],
        "//conditions:default": ["//bazel/cargo:getrandom"],
    }),
)

rust_binary(
    name = "http_auth_random",
    srcs = ["http_auth_random.rs"],
    crate_type = "cdylib",
    edition = "2018",
    out_binary = True,
    deps = [
        "//:proxy_wasm",
        "//bazel/cargo:log",
    ],
)

rust_binary(
    name = "http_headers",
    srcs = ["http_headers.rs"],
    crate_type = "cdylib",
    edition = "2018",
    out_binary = True,
    deps = [
        "//:proxy_wasm",
        "//bazel/cargo:log",
    ],
)

rust_binary(
    name = "http_body",
    srcs = ["http_body.rs"],
    crate_type = "cdylib",
    edition = "2018",
    out_binary = True,
    deps = [
        "//:proxy_wasm",
        "//bazel/cargo:log",
    ],
)

rust_binary(
    name = "http_config",
    srcs = ["http_config.rs"],
    crate_type = "cdylib",
    edition = "2018",
    out_binary = True,
    deps = [
        "//:proxy_wasm",
        "//bazel/cargo:log",
    ],
)
