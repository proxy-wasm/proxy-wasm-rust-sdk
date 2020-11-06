workspace(name = "proxy_wasm_rust_sdk")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "io_bazel_rules_rust",
    sha256 = "17dbf791f4dab0fd4496ce5345af35e9ce2f6d011c1c8423436da517d019a3ea",
    strip_prefix = "rules_rust-2f97db595b05b1ee8cc44bde5bdf03c00bd169fb",
    url = "https://github.com/bazelbuild/rules_rust/archive/2f97db595b05b1ee8cc44bde5bdf03c00bd169fb.tar.gz",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

load("@io_bazel_rules_rust//:workspace.bzl", "rust_workspace")

rust_workspace()

load("//bazel/cargo:crates.bzl", "raze_fetch_remote_crates")

raze_fetch_remote_crates()
