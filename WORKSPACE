workspace(name = "proxy_wasm_rust_sdk")

load("@proxy_wasm_rust_sdk//bazel:repositories.bzl", "proxy_wasm_rust_sdk_repositories")

proxy_wasm_rust_sdk_repositories()

load("@proxy_wasm_rust_sdk//bazel:dependencies.bzl", "proxy_wasm_rust_sdk_dependencies")

proxy_wasm_rust_sdk_dependencies()

# Needed only when using @cargo_raze//:raze to generate BUILD files in //bazel/cargo.

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "cargo_raze",
    sha256 = "75192fdcb2777527c70e1053a318a9aa0beac3c093401921c1e7c4d53084caa8",
    strip_prefix = "cargo-raze-0.14.1",
    url = "https://github.com/google/cargo-raze/archive/v0.14.1.tar.gz",
)

load("@cargo_raze//:repositories.bzl", "cargo_raze_repositories")

cargo_raze_repositories()

load("@cargo_raze//:transitive_deps.bzl", "cargo_raze_transitive_deps")

cargo_raze_transitive_deps()
