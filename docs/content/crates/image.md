# graphitepdf-image

Image sources, decoded assets, async resolution, format detection, and LRU cache.

```toml
[dependencies]
graphitepdf-image = "0.2"
```

## Image sources

```rust
use graphitepdf_image::{
    ImageSource, DataImageSource, LocalImageSource,
    RemoteImageSource, DataUriImageSource, ImageFormat,
};

// raw bytes with known format
let data = DataImageSource::new(png_bytes, ImageFormat::Png);

// local file (format inferred from extension)
let local = LocalImageSource::new("/images/logo.png");

// remote URL (async fetch)
let remote = RemoteImageSource::new("https://example.com/photo.jpg");

// data URI
let uri = DataUriImageSource::new("data:image/png;base64,iVBORw0...");

// untyped — wrap any of the above
let source: ImageSource = local.into();
```

## Resolving images

Resolution is async and cached by default:

```rust
use graphitepdf_image::{resolve_image, resolve_image_with_options, ResolveImageOptions};

// resolves, decodes, and caches
let image = resolve_image(source).await?;

// bypass the cache
let image = resolve_image_with_options(
    source,
    ResolveImageOptions { cache: false },
).await?;
```

## Decoded image types

```rust
pub enum Image {
    Raster(RasterImage),  // PNG or JPEG
    Svg(SvgImage),        // parsed SVG scene
}

pub struct RasterImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub format: ImageFormat,
    pub key: Option<String>,
}

pub struct SvgImage {
    pub width: f32,
    pub height: f32,
    pub data: SvgNode,    // from graphitepdf-svg
    pub raw_data: Vec<u8>,
    pub key: Option<String>,
}
```

## Global cache

A process-wide LRU cache stores resolved images keyed by source URL or base64 hash. Default capacity: 30 entries.

```rust
use graphitepdf_image::{global_image_cache, ImageCache};

let cache = global_image_cache();
println!("cached images: {}", cache.len());
cache.clear();
```

## EXIF orientation

JPEG images with EXIF orientation tags are automatically rotated/flipped on decode so the image appears correctly without any manual handling.

## Dependencies

`graphitepdf-svg`, `graphitepdf-errors`, `reqwest` (rustls-tls), `tokio`, `base64`
