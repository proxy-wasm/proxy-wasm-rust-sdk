load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load("@rules_rust//rust:defs.bzl", "rust_library")

cargo_build_script(
    name = "proxy_wasm_build_script",
    srcs = ["build.rs"],
    edition = "2018",
    tags = ["manual"],
    visibility = ["//visibility:private"],
)

rust_library(
    name = "proxy_wasm",
    srcs = glob(["src/*.rs"]),
    edition = "2018",
    visibility = ["//visibility:public"],
    deps = [
        ":proxy_wasm_build_script",
        "//bazel/cargo:hashbrown",
        "//bazel/cargo:log",
    ],
)
