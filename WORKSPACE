workspace(name = "proxy_wasm_rust_sdk")

load("@proxy_wasm_rust_sdk//bazel:repositories.bzl", "proxy_wasm_rust_sdk_repositories")

proxy_wasm_rust_sdk_repositories()

load("@proxy_wasm_rust_sdk//bazel:dependencies.bzl", "proxy_wasm_rust_sdk_dependencies")

proxy_wasm_rust_sdk_dependencies()

# Needed only when using @cargo_raze//:raze to generate BUILD files in //bazel/cargo.

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "cargo_raze",
    sha256 = "0a7986b1a8ec965ee7aa317ac61e82ea08568cfdf36b7ccc4dd3d1aff3b36e0b",
    strip_prefix = "cargo-raze-0.12.0",
    url = "https://github.com/google/cargo-raze/archive/v0.12.0.tar.gz",
)

load("@cargo_raze//:repositories.bzl", "cargo_raze_repositories")

cargo_raze_repositories()

load("@cargo_raze//:transitive_deps.bzl", "cargo_raze_transitive_deps")

cargo_raze_transitive_deps()
