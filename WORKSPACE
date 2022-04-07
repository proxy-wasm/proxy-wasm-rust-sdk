workspace(name = "proxy_wasm_rust_sdk")

load("@proxy_wasm_rust_sdk//bazel:repositories.bzl", "proxy_wasm_rust_sdk_repositories")

proxy_wasm_rust_sdk_repositories()

load("@proxy_wasm_rust_sdk//bazel:dependencies.bzl", "proxy_wasm_rust_sdk_dependencies")

proxy_wasm_rust_sdk_dependencies()

# Needed only when using @cargo_raze//:raze to generate BUILD files in //bazel/cargo.

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "cargo_raze",
    sha256 = "58ecdbae2680b71edc19a0f563cdb73e66c8914689b6edab258c8b90a93b13c7",
    strip_prefix = "cargo-raze-0.15.0",
    url = "https://github.com/google/cargo-raze/archive/v0.15.0.tar.gz",
)

load("@cargo_raze//:repositories.bzl", "cargo_raze_repositories")

cargo_raze_repositories()

load("@cargo_raze//:transitive_deps.bzl", "cargo_raze_transitive_deps")

cargo_raze_transitive_deps()
