load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "hello_world",
    srcs = ["hello_world/src/lib.rs"],
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
    srcs = ["http_auth_random/src/lib.rs"],
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
    srcs = ["http_headers/src/lib.rs"],
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
    srcs = ["http_body/src/lib.rs"],
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
    srcs = ["http_config/src/lib.rs"],
    crate_type = "cdylib",
    edition = "2018",
    out_binary = True,
    deps = [
        "//:proxy_wasm",
        "//bazel/cargo:log",
    ],
)
