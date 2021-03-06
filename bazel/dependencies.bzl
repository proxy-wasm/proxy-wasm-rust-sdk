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

load("@cargo_raze//:repositories.bzl", "cargo_raze_repositories")
load("@cargo_raze//:transitive_deps.bzl", "cargo_raze_transitive_deps")
load("@proxy_wasm_rust_sdk//bazel/cargo:crates.bzl", "raze_fetch_remote_crates")
load("@rules_foreign_cc//:workspace_definitions.bzl", "rules_foreign_cc_dependencies")
load("@rules_rust//rust:repositories.bzl", "rust_repositories")

def proxy_wasm_rust_sdk_dependencies():
    rust_repositories()
    raze_fetch_remote_crates()

def proxy_wasm_rust_sdk_cargo_raze_dependencies():
    rules_foreign_cc_dependencies()
    cargo_raze_repositories()
    cargo_raze_transitive_deps()
