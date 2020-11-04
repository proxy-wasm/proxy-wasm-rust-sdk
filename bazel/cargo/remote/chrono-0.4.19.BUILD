"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//bazel/cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "notice", # MIT from expression "MIT OR Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "chrono" with type "bench" omitted

rust_library(
    name = "chrono",
    crate_type = "lib",
    deps = [
        "@raze__libc__0_2_80//:libc",
        "@raze__num_integer__0_1_44//:num_integer",
        "@raze__num_traits__0_2_14//:num_traits",
        "@raze__time__0_1_44//:time",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.4.19",
    tags = ["cargo-raze"],
    crate_features = [
        "clock",
        "default",
        "libc",
        "oldtime",
        "std",
        "time",
        "winapi",
    ],
)

# Unsupported target "serde" with type "bench" omitted
# Unsupported target "wasm" with type "test" omitted
