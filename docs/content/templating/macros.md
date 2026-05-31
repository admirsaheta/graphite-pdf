# Template Macros Reference

## `pdf!`

Assembles a `graphitepdf_layout::Document` from a tree of JSX-like elements.

### Syntax

```
pdf! {
    <Document>
        <Page [size=…] [style=…]>
            (<View | <Text | <Image | { expr })* 
        </Page>
        { expr }*
    </Document>
}
```

### Elements

#### `<Document>`

Root element. Must be present exactly once. Children must be `<Page>` elements or `{ Page }` expressions.

#### `<Page>`

Maps to `Page::new(children).with_size(…)`.

| Attribute | Type | Notes |
| --- | --- | --- |
| `size` | string `"A4"` / `"A5"` / `"LETTER"` / `"LEGAL"` | Standard page size |
| `size` | `{ PageSize::A4 }` | `graphitepdf_kit::PageSize` expression |
| `size` | `{ (595.0_f32, 842.0_f32) }` | `(width, height)` tuple |
| `style` | `{ LayoutStyle }` | Page-level style (background, padding) |

Children can be `<View>`, `<Text>`, `<Image>` elements, or `{ Vec<Node> }` / `{ Option<Node> }` expressions.

#### `<View>`

Maps to `Node::view(children)`. Accepts an optional `style` attribute.

```rust
<View style={styles! { background_color: "#F1F5F9" }}>
    <Text>"child"</Text>
</View>
```

#### `<Text>`

Maps to `Node::text(TextBlock::from(…))`. Text content is either:

- A string literal: `"Hello"` → a single-span `TextBlock`
- A `{ expr }` that produces a `String`, `&str`, `TextBlock`, or `Node`

```rust
<Text style={styles! { font_size: 14.0 }}>"Static text"</Text>
<Text>{"Dynamic: "}{format!("{n} items")}</Text>
```

Multiple `{ expr }` children in a `<Text>` are concatenated into one `TextBlock`.

#### `<Image>`

Maps to `Node::image_source(…)`.

| Attribute | Type | Notes |
| --- | --- | --- |
| `src` | `{ ImageSource }` | Any `graphitepdf_image::ImageSource` |
| `style` | `{ LayoutStyle }` | Must include `width` and `height` |

### Expression children

Any child that is `{ expr }` is spliced in directly. The expression may produce:

| Type | Valid inside |
| --- | --- |
| `Node` | `<Page>`, `<View>` |
| `Vec<Node>` | `<Page>`, `<View>` |
| `Option<Node>` | `<Page>`, `<View>` |
| `Option<Vec<Node>>` | `<Page>`, `<View>` |
| `Page` | `<Document>` |
| `Vec<Page>` | `<Document>` |
| `Option<Page>` | `<Document>` |

---

## `styles!`

Produces a `LayoutStyle` from a comma-separated list of property assignments.

### Syntax

```rust
let style = styles! {
    key: value,
    key: value,
    // …
};
```

### Supported properties

| Key | Rust type | Example values |
| --- | --- | --- |
| `width` | `Pt` | `200.0` |
| `height` | `Pt` | `100.0` |
| `font_size` | `Pt` | `14.0` |
| `line_height` | `Pt` | `20.0` |
| `color` | `Color` | `"#334155"`, `"white"`, `"black"` |
| `background_color` | `Color` | `"#EFF6FF"` |
| `font_family` | `String` | `"Helvetica"`, `"Times-Roman"` |
| `font_style` | `FontStyle` | `normal`, `italic`, `oblique` |
| `font_weight` | `FontWeight` | `thin`, `light`, `normal`, `medium`, `semi_bold`, `bold`, `extra_bold`, `black` |
| `z_index` | `i32` | `1`, `-1` |
| `page_break_before` | `bool` | `true`, `false` |
| `page_break_after` | `bool` | `true`, `false` |

String color values are resolved at compile time — `"#1D4ED8"` becomes `Color::rgb(0x1D, 0x4E, 0xD8)` in the binary.

Properties NOT listed above (`margin`, `padding`) must be set via the `LayoutStyle` builder API:

```rust
let panel = styles! { background_color: "#F8FAFC" }
    .with_padding(EdgeInsets::all(Pt::new(16.0)))
    .with_margin(EdgeInsets { bottom: Pt::new(12.0), ..Default::default() });
```

### Using Rust expressions as values

If a value is not a recognized literal, it is spliced in as-is:

```rust
let size = Pt::new(24.0);
let s = styles! { font_size: size }; // size must be Pt
```

---

## `stylesheet!`

Produces an anonymous struct where each field is a named `LayoutStyle`.

### Syntax

```rust
let ds = stylesheet! {
    .name => {
        key: value,
        // …
    },
    .other_name => {
        // …
    },
};
```

Each entry uses the same property syntax as `styles!`.

### Usage

```rust
let ds = stylesheet! {
    .heading => {
        font_size: 20.0,
        font_weight: bold,
        color: "#0F172A",
    },
    .body => {
        font_size: 12.0,
        line_height: 18.0,
        color: "#334155",
    },
    .caption => {
        font_size: 10.0,
        color: "#64748B",
    },
};

// Fields are plain LayoutStyle — clone or move freely
let doc = pdf! {
    <Document>
        <Page size="A4">
            <Text style={ds.heading.clone()}>"Invoice #1042"</Text>
            <Text style={ds.body.clone()}>"Due: 2026-06-01"</Text>
        </Page>
    </Document>
};
```

The generated struct derives `Clone` and `Debug`. Field names map 1:1 to the identifiers after `.` in the macro input.

### Combining with the builder API

`stylesheet!` fields are plain `LayoutStyle` values. Chain builder methods after the macro to add properties that `styles!` does not support:

```rust
let ds = stylesheet! {
    .card => { background_color: "#FFFFFF", z_index: 1 },
};

let card_style = ds.card.with_padding(EdgeInsets::all(Pt::new(16.0)));
```
