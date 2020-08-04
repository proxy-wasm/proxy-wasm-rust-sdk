workspace(name = "proxy_wasm_rust_sdk")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "bazel_skylib",
    sha256 = "97e70364e9249702246c0e9444bccdc4b847bed1eb03c5a3ece4f83dfe6abc44",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
    ],
)

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")

bazel_skylib_workspace()

http_archive(
    name = "io_bazel_rules_rust",
    sha256 = "484a2b2b67cd2d1fa1054876de7f8d291c4b203fd256bc8cbea14d749bb864ce",
    # Last commit where "out_binary = True" works.
    # See: https://github.com/bazelbuild/rules_rust/issues/386
    strip_prefix = "rules_rust-fda9a1ce6482973adfda022cadbfa6b300e269c3",
    url = "https://github.com/bazelbuild/rules_rust/archive/fda9a1ce6482973adfda022cadbfa6b300e269c3.tar.gz",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")

bazel_version(name = "bazel_version")

load("//cargo:crates.bzl", "raze_fetch_remote_crates")

raze_fetch_remote_crates()
