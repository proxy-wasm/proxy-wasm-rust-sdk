# BCR Publishing Templates

This directory contains templates used by the [Publish to BCR](https://github.com/bazel-contrib/publish-to-bcr) GitHub Action to automatically publish new versions of proxy-wasm-rust-sdk to the [Bazel Central Registry (BCR)](https://github.com/bazelbuild/bazel-central-registry).

## Files

- **metadata.template.json**: Contains repository metadata including homepage, maintainers, and repository location
- **source.template.json**: Template for generating the source.json file that tells BCR where to download release archives
- **presubmit.yml**: Defines build and test tasks that BCR will run to verify each published version

## How it works

When a new tag matching the pattern `v*.*.*` is created:
1. The GitHub Actions workflow triggers the Publish to BCR action
2. The workflow uses these templates to generate a BCR entry
3. A pull request is automatically created against the Bazel Central Registry
4. Once merged, the new version becomes available to Bazel users via bzlmod

## Template Variables

The following variables are automatically substituted:
- `{OWNER}`: Repository owner (proxy-wasm)
- `{REPO}`: Repository name (proxy-wasm-rust-sdk)
- `{VERSION}`: Version number extracted from the tag (e.g., `0.2.5` from `v0.2.5`)
- `{TAG}`: Full tag name (e.g., `v0.2.5`)

## Setup Requirements

Before publishing to BCR can work, the following setup is required:

### 1. Fork the Bazel Central Registry

Create a fork of [bazelbuild/bazel-central-registry](https://github.com/bazelbuild/bazel-central-registry) in your GitHub account or organization. This fork is used to create pull requests against the upstream BCR.

### 2. Create a Personal Access Token (PAT)

Create a "Classic" Personal Access Token with the following permissions:
- **repo**: Full control of private repositories (required to push to the BCR fork)
- **workflow**: Update GitHub Action workflows (required to trigger workflows)

To create the token:
1. Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Click "Generate new token (classic)"
3. Give it a descriptive name (e.g., "BCR Publish Token")
4. Select the `repo` and `workflow` scopes
5. Click "Generate token" and copy the token value

> **Note**: Fine-grained PATs are not fully supported yet because they cannot open pull requests against public repositories.

### 3. Configure the Repository Secret

Add the PAT as a repository secret named `BCR_PUBLISH_TOKEN`:
1. Go to your repository Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Name: `BCR_PUBLISH_TOKEN`
4. Value: Paste the PAT created in step 2
5. Click "Add secret"

### 4. Configure the Registry Fork (Optional)

If your BCR fork is not in the same organization as this repository, you may need to update the workflow in `.github/workflows/publish-to-bcr.yml` to specify the `registry_fork` parameter.

## More Information

- [Publish to BCR documentation](https://github.com/bazel-contrib/publish-to-bcr)
- [BCR documentation](https://bazel.build/external/registry)
