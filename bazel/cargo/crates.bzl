"""
cargo-raze crate workspace functions

DO NOT EDIT! Replaced on runs of cargo-raze
"""
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")

def _new_http_archive(name, **kwargs):
    if not native.existing_rule(name):
        http_archive(name=name, **kwargs)

def _new_git_repository(name, **kwargs):
    if not native.existing_rule(name):
        new_git_repository(name=name, **kwargs)

def raze_fetch_remote_crates():

    _new_http_archive(
        name = "raze__ahash__0_3_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ahash/ahash-0.3.8.crate",
        type = "tar.gz",
        sha256 = "e8fd72866655d1904d6b0997d0b07ba561047d070fbe29de039031c641b61217",
        strip_prefix = "ahash-0.3.8",
        build_file = Label("//bazel/cargo/remote:ahash-0.3.8.BUILD"),
    )

    _new_http_archive(
        name = "raze__autocfg__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/autocfg/autocfg-1.0.0.crate",
        type = "tar.gz",
        sha256 = "f8aac770f1885fd7e387acedd76065302551364496e46b3dd00860b2f8359b9d",
        strip_prefix = "autocfg-1.0.0",
        build_file = Label("//bazel/cargo/remote:autocfg-1.0.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__cfg_if__0_1_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.10.crate",
        type = "tar.gz",
        sha256 = "4785bdd1c96b2a846b2bd7cc02e86b6b3dbf14e7e53446c4f54c92a361040822",
        strip_prefix = "cfg-if-0.1.10",
        build_file = Label("//bazel/cargo/remote:cfg-if-0.1.10.BUILD"),
    )

    _new_http_archive(
        name = "raze__chrono__0_4_13",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/chrono/chrono-0.4.13.crate",
        type = "tar.gz",
        sha256 = "c74d84029116787153e02106bf53e66828452a4b325cc8652b788b5967c0a0b6",
        strip_prefix = "chrono-0.4.13",
        build_file = Label("//bazel/cargo/remote:chrono-0.4.13.BUILD"),
    )

    _new_http_archive(
        name = "raze__hashbrown__0_8_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hashbrown/hashbrown-0.8.2.crate",
        type = "tar.gz",
        sha256 = "e91b62f79061a0bc2e046024cb7ba44b08419ed238ecbd9adbd787434b9e8c25",
        strip_prefix = "hashbrown-0.8.2",
        build_file = Label("//bazel/cargo/remote:hashbrown-0.8.2.BUILD"),
    )

    _new_http_archive(
        name = "raze__libc__0_2_74",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.74.crate",
        type = "tar.gz",
        sha256 = "a2f02823cf78b754822df5f7f268fb59822e7296276d3e069d8e8cb26a14bd10",
        strip_prefix = "libc-0.2.74",
        build_file = Label("//bazel/cargo/remote:libc-0.2.74.BUILD"),
    )

    _new_http_archive(
        name = "raze__log__0_4_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.4.11.crate",
        type = "tar.gz",
        sha256 = "4fabed175da42fed1fa0746b0ea71f412aa9d35e76e95e59b192c64b9dc2bf8b",
        strip_prefix = "log-0.4.11",
        build_file = Label("//bazel/cargo/remote:log-0.4.11.BUILD"),
    )

    _new_http_archive(
        name = "raze__memory_units__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memory_units/memory_units-0.4.0.crate",
        type = "tar.gz",
        sha256 = "8452105ba047068f40ff7093dd1d9da90898e63dd61736462e9cdda6a90ad3c3",
        strip_prefix = "memory_units-0.4.0",
        build_file = Label("//bazel/cargo/remote:memory_units-0.4.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__num_integer__0_1_43",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-integer/num-integer-0.1.43.crate",
        type = "tar.gz",
        sha256 = "8d59457e662d541ba17869cf51cf177c0b5f0cbf476c66bdc90bf1edac4f875b",
        strip_prefix = "num-integer-0.1.43",
        build_file = Label("//bazel/cargo/remote:num-integer-0.1.43.BUILD"),
    )

    _new_http_archive(
        name = "raze__num_traits__0_2_12",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-traits/num-traits-0.2.12.crate",
        type = "tar.gz",
        sha256 = "ac267bcc07f48ee5f8935ab0d24f316fb722d7a1292e2913f0cc196b29ffd611",
        strip_prefix = "num-traits-0.2.12",
        build_file = Label("//bazel/cargo/remote:num-traits-0.2.12.BUILD"),
    )

    _new_http_archive(
        name = "raze__time__0_1_43",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/time/time-0.1.43.crate",
        type = "tar.gz",
        sha256 = "ca8a50ef2360fbd1eeb0ecd46795a87a19024eb4b53c5dc916ca1fd95fe62438",
        strip_prefix = "time-0.1.43",
        build_file = Label("//bazel/cargo/remote:time-0.1.43.BUILD"),
    )

    _new_http_archive(
        name = "raze__wee_alloc__0_4_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/wee_alloc/wee_alloc-0.4.5.crate",
        type = "tar.gz",
        sha256 = "dbb3b5a6b2bb17cb6ad44a2e68a43e8d2722c997da10e928665c72ec6c0a0b8e",
        strip_prefix = "wee_alloc-0.4.5",
        build_file = Label("//bazel/cargo/remote:wee_alloc-0.4.5.BUILD"),
    )

    _new_http_archive(
        name = "raze__winapi__0_3_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.3.9.crate",
        type = "tar.gz",
        sha256 = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419",
        strip_prefix = "winapi-0.3.9",
        build_file = Label("//bazel/cargo/remote:winapi-0.3.9.BUILD"),
    )

    _new_http_archive(
        name = "raze__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-i686-pc-windows-gnu/winapi-i686-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = Label("//bazel/cargo/remote:winapi-i686-pc-windows-gnu-0.4.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-x86_64-pc-windows-gnu/winapi-x86_64-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = Label("//bazel/cargo/remote:winapi-x86_64-pc-windows-gnu-0.4.0.BUILD"),
    )

