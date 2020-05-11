licenses(["notice"])  # Apache 2

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library", "rust_binary")

rust_library(
    name = "proxy_wasm",
    srcs = glob(["src/*.rs"]),
    rustc_flags = [
        "--edition=2018",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "@wee_alloc//:wee_alloc",
        "@log//:log",
    ],
)

rust_binary(
    name = "http_headers",
    srcs = ["examples/http_headers.rs"],
    deps = [
        ":proxy_wasm",
        "@log//:log",
    ],
    rustc_flags = [
        "--edition=2018",
    ],
    crate_type = "cdylib",
    out_binary = True,
)

rust_binary(
    name = "http_auth_random",
    srcs = ["examples/http_auth_random.rs"],
    deps = [
        ":proxy_wasm",
        "@log//:log",
    ],
    rustc_flags = [
        "--edition=2018",
    ],
    crate_type = "cdylib",
    out_binary = True,
) 