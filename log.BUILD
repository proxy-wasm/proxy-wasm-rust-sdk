licenses(["notice"])  # Apache 2

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "log",
    srcs = glob(["src/**/*.rs"]),
    visibility = ["//visibility:public"],
    deps = [
        "@cfg_if//:cfg_if",
    ],
)