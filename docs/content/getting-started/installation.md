# Installation

GraphitePDF requires **Rust 1.85** or later and uses the 2024 edition.

## Adding the facade crate

Add `graphitepdf` to your `Cargo.toml` for one-import access to the full stack:

```toml
[dependencies]
graphitepdf = "0.1"
```

## Using individual crates

If you only need part of the stack, depend on individual crates directly:

```toml
[dependencies]
graphitepdf-layout    = "0.1"
graphitepdf-textkit   = "0.1"
graphitepdf-renderer  = "0.1"
```

## Workspace setup

For a workspace that uses GraphitePDF across multiple crates, define it once in `[workspace.dependencies]`:

```toml
[workspace.dependencies]
graphitepdf = "0.1"
```

Then in each member:

```toml
[dependencies]
graphitepdf = { workspace = true }
```

## Feature flags

The top-level crate exposes feature flags for optional components. Check the [Cargo.toml](https://github.com/admirsaheta/graphite-pdf/blob/main/Cargo.toml) for the current list.

## Minimum Supported Rust Version

| MSRV | Rust edition |
| --- | --- |
| 1.85 | 2024 |

The MSRV is tested in CI and only bumped with a minor version bump.
