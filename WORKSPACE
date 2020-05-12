workspace(name = "proxy_wasm_rust_sdk")

load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository", "new_git_repository")
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

git_repository(
  name = "io_bazel_rules_rust",
  commit = "c056d676c8bc67c1e63d0496776cfcc43e7110d7",
  remote = "https://github.com/Shikugawa/rules_rust",
)

new_git_repository(
  name = "wee_alloc",
  commit = "f26c431df6fb6c7df0d6f8e0675471b9c56d8787",
  remote = "https://github.com/rustwasm/wee_alloc",
  build_file = "//external:wee_alloc.BUILD",
)

new_git_repository(
  name = "memory_units",
  commit = "d4e90ecd0efd81adb9feea093e1427077f5b29ff",
  remote = "https://github.com/pepyakin/memory_units",
  build_file = "//external:memory_units.BUILD",
)

new_git_repository(
  name = "cfg_if",
  commit = "f71bf60f212312faddee7da525fcf47daac66499",
  remote = "https://github.com/alexcrichton/cfg-if",
  build_file = "//external:cfg_if.BUILD",
)

http_archive(
  name = "log",
  build_file = "//external:log.BUILD",
  strip_prefix = "log-0.4.0",
  urls = [
    "https://github.com/rust-lang/log/archive/0.4.0.zip",
  ],
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")

bazel_version(name = "bazel_version")