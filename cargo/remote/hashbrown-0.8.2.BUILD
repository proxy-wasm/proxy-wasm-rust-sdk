"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "notice", # Apache-2.0 from expression "Apache-2.0 OR MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "bench" with type "bench" omitted
# Unsupported target "build-script-build" with type "custom-build" omitted

rust_library(
    name = "hashbrown",
    crate_type = "lib",
    deps = [
        "@raze__ahash__0_3_8//:ahash",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2018",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.8.2",
    tags = ["cargo-raze"],
    crate_features = [
        "ahash",
        "inline-more",
    ],
)

# Unsupported target "hasher" with type "test" omitted
# Unsupported target "rayon" with type "test" omitted
# Unsupported target "serde" with type "test" omitted
# Unsupported target "set" with type "test" omitted
