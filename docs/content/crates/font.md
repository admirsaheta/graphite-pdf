# graphitepdf-font

Font descriptors, sources, registration, loading, metrics, and fallback support.

```toml
[dependencies]
graphitepdf-font = "0.2"
```

## Key types

### `FontDescriptor` — describes a font by family and variant

```rust
use graphitepdf_font::{FontDescriptor, FontStyle, FontWeight};

let helvetica = FontDescriptor::new("Helvetica");
let bold_italic = FontDescriptor::new("Inter")
    .with_style(FontStyle::Italic)
    .with_weight(FontWeight::BOLD);
```

### `FontWeight`

```rust
pub struct FontWeight(u16);

// named constants
FontWeight::THIN        // 100
FontWeight::LIGHT       // 300
FontWeight::NORMAL      // 400
FontWeight::MEDIUM      // 500
FontWeight::SEMI_BOLD   // 600
FontWeight::BOLD        // 700
FontWeight::EXTRA_BOLD  // 800
FontWeight::BLACK       // 900
```

### `FontSource` — where to load the font data from

```rust
use graphitepdf_font::FontSource;

// file on disk
let local  = FontSource::local("/usr/share/fonts/Inter.ttf");

// async HTTP fetch
let remote = FontSource::remote("https://example.com/fonts/Inter.ttf");

// base64 data URI
let uri    = FontSource::data_uri("data:font/truetype;base64,...");

// standard PDF built-in (no embedding needed)
use graphitepdf_font::StandardFont;
let std    = FontSource::standard(StandardFont::HelveticaBold);
```

### `StandardFont` — the 14 built-in PDF fonts

```rust
pub enum StandardFont {
    Courier, CourierBold, CourierOblique, CourierBoldOblique,
    Helvetica, HelveticaBold, HelveticaOblique, HelveticaBoldOblique,
    TimesRoman, TimesBold, TimesItalic, TimesBoldItalic,
    Symbol, ZapfDingbats,
}
```

Standard fonts require no embedding — they are always available in PDF viewers.

### `FontRegistration` — a loadable font entry

```rust
use graphitepdf_font::{FontRegistration, FontSource, FontStyle, FontWeight};

let reg = FontRegistration::new("Inter", FontSource::local("/fonts/Inter-Regular.ttf"))
    .with_style(FontStyle::Normal)
    .with_weight(FontWeight::NORMAL);
```

## Async loading

Font data is fetched asynchronously when the source is `Local` or `Remote`. The `tokio` runtime is required. Use `FontRegistration` to pre-register fonts before layout begins.

## Dependencies

`graphitepdf-errors`, `reqwest` (rustls-tls), `tokio`, `base64`
