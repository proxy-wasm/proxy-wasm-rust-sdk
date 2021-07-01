workspace(name = "proxy_wasm_rust_sdk")

load("@proxy_wasm_rust_sdk//bazel:repositories.bzl", "proxy_wasm_rust_sdk_repositories")

proxy_wasm_rust_sdk_repositories()

load("@proxy_wasm_rust_sdk//bazel:dependencies.bzl", "proxy_wasm_rust_sdk_dependencies")

proxy_wasm_rust_sdk_dependencies()

# Needed only when using @cargo_raze//:raze to generate BUILD files in //bazel/cargo.

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "cargo_raze",
    sha256 = "73c5cea8ad3f4ef7788116d491070eeb27819fe0f923dbb6f451f69dd5fa752c",
    # v0.11.0 with a few Bazel fixes.
    strip_prefix = "cargo-raze-7614085d2748e55ad3032c9b1dca78f6011cb40e",
    url = "https://github.com/google/cargo-raze/archive/7614085d2748e55ad3032c9b1dca78f6011cb40e.tar.gz",
)

load("@cargo_raze//:repositories.bzl", "cargo_raze_repositories")

cargo_raze_repositories()

load("@cargo_raze//:transitive_deps.bzl", "cargo_raze_transitive_deps")

cargo_raze_transitive_deps()
