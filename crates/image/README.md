<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>Image source modeling, decoding, caching, and asset resolution for GraphitePDF.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--image-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-images_%7C_sources_%7C_cache-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-image` provides typed image sources, decoded image assets, and async resolution helpers for raster and SVG image inputs.

---

## Scope

`graphitepdf-image` contains:

- `ImageSource` with data, local, remote, and data-URI variants
- `ImageFormat`, `RasterImage`, `SvgImage`, and `Image`
- `ImageCache` and `ResolveImageOptions`
- async asset resolution helpers such as `resolve_image()` and `resolve_image_with_options()`

---

## Installation

```bash
cargo add graphitepdf-image
```

---

## API Summary

| Category | Items |
| --- | --- |
| Image sources | `ImageSource`, `DataImageSource`, `LocalImageSource`, `RemoteImageSource`, `DataUriImageSource` |
| Source options | `RemoteMethod`, `RemoteCredentials`, `ResolveImageOptions` |
| Decoded assets | `Image`, `RasterImage`, `SvgImage`, `ImageAsset` |
| Runtime helpers | `ImageCache`, `resolve_image()`, `resolve_image_with_options()` |

---

## Example

```rust
use graphitepdf_image::{
    DataImageSource, ImageFormat, ImageSource, LocalImageSource, RemoteImageSource,
};

fn main() {
    let inline: ImageSource = DataImageSource::new(vec![137, 80, 78, 71], ImageFormat::Png).into();
    let local: ImageSource = LocalImageSource::new("assets/logo.svg")
        .with_format(ImageFormat::Svg)
        .into();
    let remote: ImageSource = RemoteImageSource::new("https://example.com/logo.png").into();

    let _sources = [inline, local, remote];
}
```

---

## Design Principles

- keep image inputs typed and explicit
- separate source descriptions from decoded assets
- support caching and async loading without forcing rendering policy
- make SVG images first-class alongside raster images

---

## Role In GraphitePDF

This crate is a shared asset service for `layout`, `render`, `document`, and `kit`. It is where raw image inputs become reusable `Image` assets.

---

## License

MIT
