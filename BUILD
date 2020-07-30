load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library", "rust_binary")

rust_library(
    name = "proxy_wasm",
    srcs = glob(["src/*.rs"]),
    edition = "2018",
    visibility = ["//visibility:public"],
    deps = [
        "//cargo:chrono",
        "//cargo:hashbrown",
        "//cargo:log",
        "//cargo:wee_alloc",
    ],
)

rust_binary(
    name = "http_headers",
    srcs = ["examples/http_headers.rs"],
    deps = [
        ":proxy_wasm",
        "//cargo:log",
    ],
    edition = "2018",
    crate_type = "cdylib",
    out_binary = True,
)

rust_binary(
    name = "http_auth_random",
    srcs = ["examples/http_auth_random.rs"],
    deps = [
        ":proxy_wasm",
        "//cargo:log",
    ],
    edition = "2018",
    crate_type = "cdylib",
    out_binary = True,
)

rust_binary(
    name = "hello_world",
    srcs = ["examples/hello_world.rs"],
    deps = [
        ":proxy_wasm",
        "//cargo:log",
        "//cargo:chrono"
    ],
    edition = "2018",
    crate_type = "cdylib",
    out_binary = True,
)