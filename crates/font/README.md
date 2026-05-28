<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>Font descriptors, registration, loading, fallback, and standard-font support for GraphitePDF.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--font-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-fonts_%7C_loading_%7C_fallback-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-font` models font families, variants, sources, standard fonts, emoji sources, and font loading for the GraphitePDF workspace.

---

## Scope

`graphitepdf-font` contains:

- `FontDescriptor`, `FontStyle`, and `FontWeight`
- `FontSource` for local, remote, data-URI, and standard-font inputs
- registration types such as `FontRegistration` and `FontFamilyRegistration`
- `FontStore` for lookup, loading, emoji URL resolution, and hyphenation callbacks

---

## Installation

```bash
cargo add graphitepdf-font
```

---

## API Summary

| Category | Items |
| --- | --- |
| Font identity | `FontDescriptor`, `FontStyle`, `FontWeight`, `StandardFont` |
| Font sources | `FontSource` |
| Registration | `FontRegistration`, `FontVariantRegistration`, `FontFamilyRegistration` |
| Runtime store | `FontStore`, `LoadedFont`, `RegisteredFont` |
| Emoji and fallback | `EmojiSource`, `EmojiFormat`, `HyphenationCallback` |

---

## Example

```rust
use graphitepdf_font::{FontDescriptor, FontRegistration, FontSource, FontStore, StandardFont};

#[tokio::main]
async fn main() -> graphitepdf_font::Result<()> {
    let mut store = FontStore::new();
    store.register_font(FontRegistration::new(
        "Helvetica",
        FontSource::standard(StandardFont::Helvetica),
    ))?;

    let descriptor = FontDescriptor::new("Helvetica");
    let _loaded = store.load(&descriptor).await?;

    Ok(())
}
```

---

## Design Principles

- keep font identity and loading separate from text layout policy
- support multiple source types without hiding fallibility
- expose standard-font behavior as first-class data
- make fallback and emoji integration explicit rather than magical

---

## Role In GraphitePDF

This crate is a shared service used by `textkit`, `layout`, `render`, `style`, and `kit`. It is the workspace's central place for describing and resolving fonts.

---

## License

MIT
