# @aware-aeco/cli

The AWARE CLI as an npm package. Wraps the underlying Rust + C# binaries.

## Install

```bash
npm install -g @aware-aeco/cli
aware --version
```

On install, `postinstall` downloads the right binary for your platform (Windows x64, Linux x64, or macOS arm64) from the [GitHub Releases](https://github.com/aware-aeco/aware/releases) for the matching version.

For other platforms, build from source: https://github.com/aware-aeco/aware

## Docs

See the main repo: https://github.com/aware-aeco/aware

## License

Apache-2.0 — same as the upstream project.

## Behind the curtain

This package is a thin shim. The actual implementation is:

- The Rust CLI binary (`aware`) — open source at [aware-aeco/aware](https://github.com/aware-aeco/aware), under `cli/`
- The C# NativeAOT sidecar (`aware-sidecar`) — same repo, under `cli-sidecar/`

The package's `postinstall` hook fetches both binaries from the GitHub Release matching the package version. The npm tarball itself is ~10 KB.

To skip the download (corp networks, etc.):

```bash
AWARE_NPM_SKIP_DOWNLOAD=1 npm install -g @aware-aeco/cli
```

Then install the binaries manually from the GitHub Release.
