licenses(["notice"])  # Apache 2

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "wee_alloc",
    srcs = glob(["wee_alloc/**/*.rs"]),
    visibility = ["//visibility:public"],
    deps = [
      "@memory_units//:memory_units",
      "@cfg_if//:cfg_if",
    ],
)