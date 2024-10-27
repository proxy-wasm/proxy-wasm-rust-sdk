# Development

## Testing

GitHub Actions can be executed locally using the [`act`] tool.

All tests can be executed using:

    act

Individual tests can be executed using `-j` and `--matrix` parameters, e.g.:

    act -j bazel
    act -j stable
    act -j nightly
    act -j examples --matrix example:http_auth_random

## Updating Bazel dependencies

When adding and/or updating Cargo dependencies, the existing Bazel `BUILD` files
must be regenerated to match the updated `Cargo.toml`:

```sh
bazelisk run //bazel/cargo:crates_vendor -- --repin all
```


[`act`]: https://github.com/nektos/act
[`bazelisk`]: https://github.com/bazelbuild/bazelisk
