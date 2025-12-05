# Copyright 2025 Google LLC
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

load("@proxy_wasm_rust_sdk//bazel/cargo/remote:crates.bzl", "crate_repositories")

def _crates_deps_impl(module_ctx):
    deps = []
    for repo in crate_repositories():
        if not repo.is_dev_dep:
            deps.append(repo.repo)

    return module_ctx.extension_metadata(
        root_module_direct_deps = deps,
        root_module_direct_dev_deps = [],
    )

crates_deps = module_extension(
    doc = "Dependencies for the Proxy-Wasm Rust SDK.",
    implementation = _crates_deps_impl,
)
