<p align="center">
  <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-logo-oss.png" alt="GraphitePDF logo" width="560" />
</p>

<p align="center">
  <strong>Text blocks, attributes, line layout, and text-engine behavior for GraphitePDF.</strong>
</p>

<p align="center">
  <img alt="Crate" src="https://img.shields.io/badge/crate-graphitepdf--textkit-1E1E1C?style=for-the-badge&labelColor=060604&color=D4581A" />
  <img alt="Focus" src="https://img.shields.io/badge/focus-text_%7C_layout_%7C_typography-C4C4C0?style=for-the-badge&labelColor=2E2E2C&color=3A3A38" />
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-F58040?style=for-the-badge&labelColor=2E2E2C&color=7C2806" />
</p>

---

## Overview

`graphitepdf-textkit` is the dedicated text subsystem for GraphitePDF.

---

## Scope

`graphitepdf-textkit` contains:

- authoring types such as `TextSpan`, `TextBlock`, `AttributedString`, and `TextAttributes`
- container and geometry types such as `TextRect` and `TextContainer`
- resolved output such as `TextRun`, `TextFragment`, `LineFragment`, and `TextLayout`
- `TextEngine` and `TextEngineConfig` for line-breaking, bidi, justification, and font substitution behavior

---

## Installation

```bash
cargo add graphitepdf-textkit
```

---

## API Summary

| Category | Items |
| --- | --- |
| Authoring | `TextSpan`, `TextBlock`, `AttributedString`, `AttributeRun`, `TextRange` |
| Attributes | `TextAttributes`, `TextDecoration`, `TextDecorationKind`, `TextDirection`, `Script` |
| Containers | `TextRect`, `TextContainer` |
| Layout output | `TextRun`, `TextFragment`, `LineFragment`, `TextLayout` |
| Engine | `TextEngine`, `TextEngineConfig`, `BidiMode`, `LineBreaking`, `Justification`, `FontSubstitution` |

---

## Example

```rust
use graphitepdf_font::FontStore;
use graphitepdf_textkit::{
    TextBlock, TextContainer, TextEngine, TextEngineConfig, TextRect, TextSpan,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let block = TextBlock::from(TextSpan::new("GraphitePDF text layout")?);
    let container = TextContainer::new(TextRect::from_values(0.0, 0.0, 180.0, 60.0))?;
    let engine = TextEngine::new(TextEngineConfig::default());
    let font_store = FontStore::new();

    let layout = engine.layout_text_block(&block, &container, Some(&font_store))?;
    assert!(!layout.lines().is_empty());
    Ok(())
}
```

---

## Design Principles

- keep text shaping and line layout centralized in one crate
- make font fallback, bidi, and line-breaking behavior configurable
- expose text results as structured data rather than ad hoc strings
- keep the text subsystem reusable for higher-level layout and rendering crates

---

## Role In GraphitePDF

This crate is the canonical text engine that feeds `layout` and ultimately the rendering pipeline.

---

## License

MIT
