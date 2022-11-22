"""
@generated
cargo-raze generated Bazel file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")  # buildifier: disable=load

def raze_fetch_remote_crates():
    """This function defines a collection of repos and should be called in a WORKSPACE file"""
    maybe(
        http_archive,
        name = "raze__ahash__0_8_2",
        url = "https://crates.io/api/v1/crates/ahash/0.8.2/download",
        type = "tar.gz",
        sha256 = "bf6ccdb167abbf410dcb915cabd428929d7f6a04980b54a11f26a39f1c7f7107",
        strip_prefix = "ahash-0.8.2",
        build_file = Label("//bazel/cargo/remote:BUILD.ahash-0.8.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cfg_if__1_0_0",
        url = "https://crates.io/api/v1/crates/cfg-if/1.0.0/download",
        type = "tar.gz",
        sha256 = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd",
        strip_prefix = "cfg-if-1.0.0",
        build_file = Label("//bazel/cargo/remote:BUILD.cfg-if-1.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__hashbrown__0_13_1",
        url = "https://crates.io/api/v1/crates/hashbrown/0.13.1/download",
        type = "tar.gz",
        sha256 = "33ff8ae62cd3a9102e5637afc8452c55acf3844001bd5374e0b0bd7b6616c038",
        strip_prefix = "hashbrown-0.13.1",
        build_file = Label("//bazel/cargo/remote:BUILD.hashbrown-0.13.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__log__0_4_16",
        url = "https://crates.io/api/v1/crates/log/0.4.16/download",
        type = "tar.gz",
        sha256 = "6389c490849ff5bc16be905ae24bc913a9c8892e19b2341dbc175e14c341c2b8",
        strip_prefix = "log-0.4.16",
        build_file = Label("//bazel/cargo/remote:BUILD.log-0.4.16.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__once_cell__1_16_0",
        url = "https://crates.io/api/v1/crates/once_cell/1.16.0/download",
        type = "tar.gz",
        sha256 = "86f0b0d4bf799edbc74508c1e8bf170ff5f41238e5f8225603ca7caaae2b7860",
        strip_prefix = "once_cell-1.16.0",
        build_file = Label("//bazel/cargo/remote:BUILD.once_cell-1.16.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__version_check__0_9_4",
        url = "https://crates.io/api/v1/crates/version_check/0.9.4/download",
        type = "tar.gz",
        sha256 = "49874b5167b65d7193b8aba1567f5c7d93d001cafc34600cee003eda787e483f",
        strip_prefix = "version_check-0.9.4",
        build_file = Label("//bazel/cargo/remote:BUILD.version_check-0.9.4.bazel"),
    )
