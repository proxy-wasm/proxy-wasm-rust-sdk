workspace(name = "proxy_wasm_rust_sdk")

load("@proxy_wasm_rust_sdk//bazel:repositories.bzl", "proxy_wasm_rust_sdk_repositories")

proxy_wasm_rust_sdk_repositories()

load("@proxy_wasm_rust_sdk//bazel:dependencies.bzl", "proxy_wasm_rust_sdk_cargo_raze_dependencies", "proxy_wasm_rust_sdk_dependencies")

proxy_wasm_rust_sdk_dependencies()

# Needed only when using @cargo_raze//:raze to generate BUILD files in //bazel/cargo.

proxy_wasm_rust_sdk_cargo_raze_dependencies()
