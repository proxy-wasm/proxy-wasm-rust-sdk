load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "proxy_wasm",
    srcs = glob(["src/*.rs"]),
    edition = "2018",
    visibility = ["//visibility:public"],
    deps = [
        "//bazel/cargo:hashbrown",
        "//bazel/cargo:log",
    ],
)
