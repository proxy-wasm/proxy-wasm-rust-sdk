load("@rules_rust//rust:defs.bzl", "rust_binary")

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
