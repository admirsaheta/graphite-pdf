# graphitepdf-textkit

The text engine for GraphitePDF: attributed text, bidi, script detection, hyphenation, font substitution, line breaking, and fragment layout.

```toml
[dependencies]
graphitepdf-textkit = "0.2"
```

## Authoring types

### `TextSpan` — a run of text with optional styling

```rust
use graphitepdf_textkit::TextSpan;
use graphitepdf_font::{FontDescriptor, FontWeight};
use graphitepdf_primitives::Pt;

let span = TextSpan::new("Hello, world")?
    .with_font(FontDescriptor::new("Helvetica").with_weight(FontWeight::BOLD))
    .with_font_size(Pt::new(14.0))?;
```

### `TextBlock` — ordered collection of spans

```rust
use graphitepdf_textkit::{TextBlock, TextSpan};

let block = TextBlock::new([
    TextSpan::new("The quick ")?,
    TextSpan::new("brown fox")?.with_font(bold_font),
    TextSpan::new(" jumps")?,
]);
```

### `TextAttributes` — per-character styling

Beyond `TextSpan`, `TextAttributes` provides fine-grained control including letter spacing, direction override, language tag, and text decorations.

## Engine configuration

```rust
use graphitepdf_textkit::{TextEngine, TextEngineConfig, FontSubstitution,
                           WordHyphenation, BidiMode, Justification};

let engine = TextEngine::new(TextEngineConfig {
    default_font: FontDescriptor::new("Helvetica"),
    font_substitution: FontSubstitution::BestEffort,
    hyphenation: WordHyphenation::Auto,
    bidi: BidiMode::Auto,
    justification: Justification::Left,
    ..Default::default()
});
```

## Layout output

After layout the engine produces `TextLayout` — a set of `LineFragment`s each with an absolute `TextRect`, baseline, direction, and list of `TextFragment`s (individual glyphs/runs).

## Bidi and script support

- Unicode bidi algorithm applied per-paragraph
- Script detection covers Latin, Arabic, Hebrew, Devanagari, CJK, and Common characters
- Direction changes produce separate runs so each run can have its own font metrics

## Font substitution modes

| Mode | Behaviour |
| --- | --- |
| `Disabled` | use only the requested font; fail if missing |
| `FallbackFamilies(Vec<FontDescriptor>)` | try each fallback in order |
| `BestEffort` | try fallbacks, then Helvetica; never fail |

## Dependencies

`graphitepdf-font`, `graphitepdf-primitives`, `graphitepdf-errors`
