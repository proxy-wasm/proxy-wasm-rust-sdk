# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.3] - 2020-12-04

### Fixed

- Fixed support for nested HTTP callouts.
  Thanks [@SvetlinZarev](https://github.com/SvetlinZarev)!

### Changed

- Changed `wee-alloc` to an optional feature.
  Thanks [@yuval-k](https://github.com/yuval-k)!

### Added

- Added support for building for `wasm32-wasi` target.
- Added support for metrics.
- Added support for `RootContext` to create child contexts for streams.
  Thanks [@dgn](https://github.com/dgn)!
- Added support for setting network buffers.

## [0.1.2] - 2020-08-05

### Changed

- Updated `MapType` values to match updated Proxy-Wasm ABI v0.1.0.
  Thanks [@yskopets](https://github.com/yskopets)!

## [0.1.1] - 2020-08-05

### Added

- Added support for building with Bazel.
- Added support for setting HTTP bodies.
  Thanks [@gbrail](https://github.com/gbrail)!

## [0.1.0] - 2020-02-29

### Added

- Initial release.


[0.1.3]: https://github.com/proxy-wasm/proxy-wasm-rust-sdk/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/proxy-wasm/proxy-wasm-rust-sdk/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/proxy-wasm/proxy-wasm-rust-sdk/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/proxy-wasm/proxy-wasm-rust-sdk/releases/tag/v0.1.0
