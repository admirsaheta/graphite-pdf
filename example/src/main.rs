use graphitepdf_font::{FontDescriptor, FontStyle, FontWeight, StandardFont};
use graphitepdf_image::{DataUriImageSource, ImageSource};
use graphitepdf_layout::{Document, EdgeInsets, LayoutMetadata, LayoutStyle, Node, Page};
use graphitepdf_math::MathOptions;
use graphitepdf_primitives::{Color, Pt, Size};
use graphitepdf_render::render_to_file;
use graphitepdf_svg::parse_svg;
use graphitepdf_template_macros::{styles, stylesheet};
use graphitepdf_textkit::{TextBlock, TextSpan};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const A4_W: f32 = 595.0;
const A4_H: f32 = 842.0;
const PAD: f32 = 40.0;
const CW: f32 = 515.0;

const ICON_DATA_URI: &str = "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI2NCIgaGVpZ2h0PSI2NCIgdmlld0JveD0iMCAwIDY0IDY0Ij48cmVjdCB3aWR0aD0iNjQiIGhlaWdodD0iNjQiIHJ4PSIxMiIgZmlsbD0iI0RCRUFGRSIvPjxjaXJjbGUgY3g9IjMyIiBjeT0iMzIiIHI9IjE4IiBmaWxsPSIjMUQ0RUQ4Ii8+PC9zdmc+";

fn main() -> Result<(), Box<dyn Error>> {
    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("output");
    fs::create_dir_all(&output_dir)?;
    let output_path = output_dir.join("template-showcase.pdf");
    render_to_file(&build_document()?, &output_path)?;
    println!("Wrote {}", output_path.display());
    Ok(())
}

fn build_document() -> Result<Document, Box<dyn Error>> {
    Ok(Document::new()
        .with_metadata(LayoutMetadata {
            title: Some("GraphitePDF Showcase".into()),
            author: Some("GraphitePDF Example".into()),
            subject: Some("Template, layout, and rendering pipeline demonstration".into()),
            keywords: vec![
                "rust".into(),
                "pdf".into(),
                "graphitepdf".into(),
                "layout".into(),
            ],
            creator: Some("graphitepdf-example".into()),
            producer: Some("GraphitePDF v0.2.0".into()),
        })
        .with_page(cover_page())
        .with_page(typography_page()?)
        .with_page(math_page()?)
        .with_page(architecture_page()))
}

// ─── Helpers ───────────────────────────────────────────────────────────────

fn txt(s: &str) -> TextBlock {
    TextBlock::from(TextSpan::new(s).expect("text span"))
}

fn text_node(s: &str, style: LayoutStyle) -> Node {
    Node::text(txt(s)).with_style(style)
}

fn spacer(h: f32) -> Node {
    Node::box_node().with_style(LayoutStyle::new().with_height(Pt::new(h)))
}

fn hex_color(s: &str) -> Color {
    let s = s.trim_start_matches('#');
    Color::rgb(
        u8::from_str_radix(&s[0..2], 16).unwrap_or(0),
        u8::from_str_radix(&s[2..4], 16).unwrap_or(0),
        u8::from_str_radix(&s[4..6], 16).unwrap_or(0),
    )
}

fn section_header(page_num: u8, label: &str, title: &str) -> Node {
    let svg = parse_svg(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="515" height="80" viewBox="0 0 515 80">
  <rect width="515" height="80" fill="#EFF6FF" rx="6"/>
  <rect width="5" height="80" fill="#1D4ED8"/>
  <text x="18" y="24" font-family="Helvetica" font-size="10" fill="#64748B">{label}</text>
  <text x="18" y="60" font-family="Helvetica" font-size="24" fill="#0F172A">{title}</text>
  <text x="490" y="24" font-family="Helvetica" font-size="10" fill="#94A3B8">{page_num}/4</text>
</svg>"##
    ));
    Node::svg(svg).with_style(
        LayoutStyle::new()
            .with_width(Pt::new(CW))
            .with_height(Pt::new(80.0))
            .with_margin(EdgeInsets {
                top: Pt::new(0.0),
                right: Pt::new(0.0),
                bottom: Pt::new(20.0),
                left: Pt::new(0.0),
            }),
    )
}

fn labeled_panel(title: &str, title_color: Color, lines: &[&str], bg: Color) -> Node {
    let mut children: Vec<Node> = vec![
        Node::text(txt(title)).with_style(
            LayoutStyle::new()
                .with_font_size(Pt::new(13.0))
                .with_line_height(Pt::new(18.0))
                .with_font_weight(FontWeight::BOLD)
                .with_color(title_color),
        ),
        spacer(6.0),
    ];
    for line in lines {
        children.push(text_node(
            line,
            styles! { font_size: 11.5, line_height: 17.5, color: "#334155" },
        ));
    }
    Node::view(children).with_style(
        LayoutStyle::new()
            .with_background_color(bg)
            .with_padding(EdgeInsets::all(Pt::new(16.0)))
            .with_margin(EdgeInsets {
                top: Pt::new(0.0),
                right: Pt::new(0.0),
                bottom: Pt::new(10.0),
                left: Pt::new(0.0),
            }),
    )
}

fn content_page_style() -> LayoutStyle {
    LayoutStyle::new()
        .with_background_color(hex_color("F8FAFC"))
        .with_padding(EdgeInsets {
            top: Pt::new(36.0),
            right: Pt::new(PAD),
            bottom: Pt::new(36.0),
            left: Pt::new(PAD),
        })
}

// ─── Page 1: Cover ─────────────────────────────────────────────────────────

fn cover_page() -> Page {
    let hero = parse_svg(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="595" height="340" viewBox="0 0 595 340">
  <rect width="595" height="340" fill="#0F172A"/>
  <circle cx="520" cy="95" r="88" fill="#1A2D4A"/>
  <circle cx="562" cy="46" r="52" fill="#1E3354"/>
  <circle cx="492" cy="158" r="36" fill="#162540"/>
  <rect width="8" height="340" fill="#0EA5E9"/>
  <g transform="translate(24, 130)">
    <polygon points="40,0 80,40 40,80 0,40" fill="none" stroke="#0EA5E9" stroke-width="2.5"/>
    <polygon points="40,22 58,40 40,58 22,40" fill="#0EA5E9"/>
    <circle cx="40" cy="40" r="4" fill="#E0F2FE"/>
  </g>
  <text x="118" y="148" font-family="Helvetica" font-size="54" fill="#F1F5F9">GraphitePDF</text>
  <rect x="118" y="158" width="76" height="26" rx="6" fill="#1D4ED8"/>
  <text x="128" y="176" font-family="Helvetica" font-size="13" fill="#BFDBFE">v 0.2.0</text>
  <text x="118" y="214" font-family="Helvetica" font-size="19" fill="#7DD3FC">A pure Rust PDF rendering engine</text>
  <text x="118" y="240" font-family="Helvetica" font-size="13" fill="#4B5563">Layout  -  Composition  -  Rendering Pipelines</text>
  <circle cx="430" cy="228" r="5" fill="#F59E0B"/>
  <circle cx="452" cy="206" r="3" fill="#0EA5E9"/>
  <circle cx="468" cy="240" r="6" fill="#F59E0B"/>
  <circle cx="416" cy="246" r="2.5" fill="#94A3B8"/>
  <rect x="0" y="308" width="595" height="26" fill="#0C1524"/>
  <text x="118" y="326" font-family="Helvetica" font-size="11" fill="#475569">github.com/admirsaheta/graphite-pdf</text>
  <text x="480" y="326" font-family="Helvetica" font-size="11" fill="#475569">2026</text>
  <rect x="0" y="334" width="595" height="6" fill="#0EA5E9"/>
</svg>"##,
    );

    let intro = Node::view(vec![
        text_node(
            "FOUR-PAGE SHOWCASE",
            styles! { font_size: 10.0, line_height: 14.0, font_weight: bold, color: "#0EA5E9" },
        ),
        spacer(6.0),
        text_node(
            "Template  -  Layout  -  Render",
            styles! { font_size: 28.0, line_height: 36.0, font_weight: bold, color: "#0F172A" },
        ),
        spacer(10.0),
        text_node(
            "This document is produced entirely in Rust by the GraphitePDF engine \
             and serves as a live demonstration of its four-layer pipeline.",
            styles! { font_size: 13.0, line_height: 20.0, color: "#475569" },
        ),
        spacer(12.0),
        text_node(
            "->  pdf!, styles!, and stylesheet! macros compile JSX-like markup to typed builder calls.",
            styles! { font_size: 11.5, line_height: 17.5, color: "#334155" },
        ),
        text_node(
            "->  The layout engine resolves text flow, SVG, images, and math equations.",
            styles! { font_size: 11.5, line_height: 17.5, color: "#334155" },
        ),
        text_node(
            "->  The render engine serialises the safe layout document into a PDF file.",
            styles! { font_size: 11.5, line_height: 17.5, color: "#334155" },
        ),
        text_node(
            "->  Each crate is independently versioned and usable at any abstraction level.",
            styles! { font_size: 11.5, line_height: 17.5, color: "#334155" },
        ),
    ])
    .with_style(LayoutStyle::new().with_padding(EdgeInsets {
        top: Pt::new(26.0),
        right: Pt::new(PAD),
        bottom: Pt::new(18.0),
        left: Pt::new(PAD),
    }));

    let footer_svg = parse_svg(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="515" height="38" viewBox="0 0 515 38">
  <line x1="0" y1="0" x2="515" y2="0" stroke="#E2E8F0" stroke-width="1"/>
  <rect y="10" width="24" height="4" rx="2" fill="#0EA5E9"/>
  <rect x="30" y="10" width="16" height="4" rx="2" fill="#1D4ED8"/>
  <rect x="52" y="10" width="12" height="4" rx="2" fill="#F59E0B"/>
  <text x="0" y="32" font-family="Helvetica" font-size="9" fill="#94A3B8">GraphitePDF v0.2.0  -  MIT License  -  Rust 1.88+</text>
  <text x="450" y="32" font-family="Helvetica" font-size="9" fill="#94A3B8">Page 1 of 4</text>
</svg>"##,
    );

    Page::new([
        Node::svg(hero).with_style(
            LayoutStyle::new()
                .with_width(Pt::new(A4_W))
                .with_height(Pt::new(340.0)),
        ),
        intro,
        Node::svg(footer_svg).with_style(
            LayoutStyle::new()
                .with_width(Pt::new(CW))
                .with_height(Pt::new(38.0))
                .with_margin(EdgeInsets {
                    top: Pt::new(0.0),
                    right: Pt::new(0.0),
                    bottom: Pt::new(0.0),
                    left: Pt::new(PAD),
                }),
        ),
    ])
    .with_size(Size::new(A4_W, A4_H))
    .with_style(styles! { background_color: "#FFFFFF" })
}

// ─── Page 2: Typography ─────────────────────────────────────────────────────

fn typography_page() -> Result<Page, Box<dyn Error>> {
    let ds = stylesheet! {
        .subheading => {
            font_size: 13.0, line_height: 18.0,
            font_weight: bold, color: "#1D4ED8",
        },
        .body => {
            font_size: 12.0, line_height: 18.0, color: "#334155",
        },
    };

    let helvetica_panel = Node::view(vec![
        text_node("Helvetica  (sans-serif)", ds.subheading.clone()),
        spacer(6.0),
        text_node(
            "The default font family for headings and UI text. Clean, geometric, and highly legible at small sizes.",
            ds.body.clone(),
        ),
        spacer(6.0),
        text_node(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            styles! { font_size: 12.0, line_height: 18.0, font_weight: bold, color: "#0F172A" },
        ),
    ])
    .with_style(
        LayoutStyle::new()
            .with_background_color(hex_color("EFF6FF"))
            .with_padding(EdgeInsets::all(Pt::new(16.0)))
            .with_margin(EdgeInsets {
                top: Pt::new(0.0),
                right: Pt::new(0.0),
                bottom: Pt::new(10.0),
                left: Pt::new(0.0),
            }),
    );

    let times_panel = Node::view(vec![
        text_node("Times Roman  (serif)", ds.subheading.clone()),
        spacer(6.0),
        text_node(
            "The classic serif typeface. Ideal for long-form body text, citations, and academic typesetting.",
            styles! { font_size: 12.0, line_height: 18.0, font_family: "Times-Roman", color: "#334155" },
        ),
        spacer(6.0),
        text_node(
            "Timeless elegance - serifs that guide the reader through long paragraphs.",
            styles! { font_size: 11.0, line_height: 16.0, font_family: "Times-Italic", color: "#475569" },
        ),
    ])
    .with_style(
        LayoutStyle::new()
            .with_background_color(hex_color("F0FDF4"))
            .with_padding(EdgeInsets::all(Pt::new(16.0)))
            .with_margin(EdgeInsets {
                top: Pt::new(0.0),
                right: Pt::new(0.0),
                bottom: Pt::new(10.0),
                left: Pt::new(0.0),
            }),
    );

    let courier_panel = Node::view(vec![
        text_node("Courier  (monospace)", ds.subheading.clone()),
        spacer(6.0),
        text_node(
            "Fixed-width for code listings, terminal output, and technical identifiers.",
            ds.body.clone(),
        ),
        spacer(6.0),
        text_node(
            "fn render() -> Result<(), Box<dyn Error>> { render_to_file(&doc, &path) }",
            styles! { font_size: 11.0, line_height: 17.0, font_family: "Courier", color: "#1E293B" },
        ),
    ])
    .with_style(
        LayoutStyle::new()
            .with_background_color(hex_color("FFFBEB"))
            .with_padding(EdgeInsets::all(Pt::new(16.0)))
            .with_margin(EdgeInsets {
                top: Pt::new(0.0),
                right: Pt::new(0.0),
                bottom: Pt::new(10.0),
                left: Pt::new(0.0),
            }),
    );

    let stylesheet_panel = labeled_panel(
        "stylesheet! macro  -  compile-time design tokens",
        hex_color("065F46"),
        &[
            "The `ds` struct on this page was produced by: stylesheet! { .subheading => { ... }, .body => { ... } }.",
            "Each named field is a plain LayoutStyle, so it integrates seamlessly with the Node builder API.",
            "Used above: ds.subheading and ds.body - both zero-cost at runtime, resolved at compile time.",
        ],
        hex_color("ECFDF5"),
    );

    Ok(Page::new([
        section_header(2, "TYPOGRAPHY", "Named Styles  and  Font Families"),
        helvetica_panel,
        times_panel,
        courier_panel,
        stylesheet_panel,
    ])
    .with_size(Size::new(A4_W, A4_H))
    .with_style(content_page_style()))
}

// ─── Page 3: Math & TextKit ─────────────────────────────────────────────────

fn math_page() -> Result<Page, Box<dyn Error>> {
    let euler = Node::math_with_options(
        r"e^{i\pi} + 1 = 0",
        MathOptions::new()
            .inline(false)
            .color_from_primitives(hex_color("1D4ED8"))
            .debug(false),
    )
    .with_style(
        LayoutStyle::new()
            .with_width(Pt::new(180.0))
            .with_height(Pt::new(64.0))
            .with_background_color(hex_color("EFF6FF"))
            .with_margin(EdgeInsets {
                top: Pt::new(0.0),
                right: Pt::new(0.0),
                bottom: Pt::new(4.0),
                left: Pt::new(0.0),
            }),
    );

    let gaussian = Node::math_with_options(
        r"\int_0^\infty e^{-x^2}\, dx = \frac{\sqrt{\pi}}{2}",
        MathOptions::new()
            .inline(false)
            .color_from_primitives(hex_color("065F46"))
            .debug(false),
    )
    .with_style(
        LayoutStyle::new()
            .with_width(Pt::new(300.0))
            .with_height(Pt::new(68.0))
            .with_background_color(hex_color("ECFDF5"))
            .with_margin(EdgeInsets {
                top: Pt::new(0.0),
                right: Pt::new(0.0),
                bottom: Pt::new(4.0),
                left: Pt::new(0.0),
            }),
    );

    let quadratic = Node::math_with_options(
        r"x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}",
        MathOptions::new()
            .inline(false)
            .color_from_primitives(hex_color("92400E"))
            .debug(false),
    )
    .with_style(
        LayoutStyle::new()
            .with_width(Pt::new(260.0))
            .with_height(Pt::new(68.0))
            .with_background_color(hex_color("FFFBEB"))
            .with_margin(EdgeInsets {
                top: Pt::new(0.0),
                right: Pt::new(0.0),
                bottom: Pt::new(14.0),
                left: Pt::new(0.0),
            }),
    );

    let heading_span = TextSpan::new("TextKit  +  Font Crate")?
        .with_font(
            FontDescriptor::new(StandardFont::Helvetica.family_name())
                .with_weight(FontWeight::BOLD)
                .with_style(FontStyle::Normal),
        )
        .with_font_size(Pt::new(15.0))?;

    let body_span = TextSpan::new(
        "  This node is constructed directly with graphitepdf-textkit spans. Each span carries \
         its own FontDescriptor and size; the layout engine merges them into a single attributed \
         string and runs the text engine to produce a measured TextLayout before page composition.",
    )?
    .with_font(FontDescriptor::new(StandardFont::TimesRoman.family_name()))
    .with_font_size(Pt::new(12.0))?;

    let textkit_node = Node::text(TextBlock::new([heading_span, body_span])).with_style(
        LayoutStyle::new()
            .with_background_color(hex_color("FFF7ED"))
            .with_padding(EdgeInsets::all(Pt::new(16.0)))
            .with_margin(EdgeInsets {
                top: Pt::new(0.0),
                right: Pt::new(0.0),
                bottom: Pt::new(10.0),
                left: Pt::new(0.0),
            }),
    );

    let icon: ImageSource = DataUriImageSource::new(ICON_DATA_URI).into();
    let image_panel = Node::view(vec![
        Node::image_source(icon).with_style(
            LayoutStyle::new()
                .with_width(Pt::new(64.0))
                .with_height(Pt::new(64.0)),
        ),
        text_node(
            "Image assets carry data-uri, local path, or remote URL sources. The image crate \
             resolves them at render time and embeds them in the PDF content stream.",
            styles! { font_size: 11.0, line_height: 16.5, color: "#475569" },
        ),
    ])
    .with_style(
        LayoutStyle::new()
            .with_background_color(hex_color("F8FAFC"))
            .with_padding(EdgeInsets::all(Pt::new(14.0)))
            .with_margin(EdgeInsets {
                top: Pt::new(0.0),
                right: Pt::new(0.0),
                bottom: Pt::new(10.0),
                left: Pt::new(0.0),
            }),
    );

    Ok(Page::new([
        section_header(3, "MATH  AND  TEXTKIT", "Equations  and  Rich Typography"),
        labeled_panel(
            "graphitepdf-math  -  LaTeX to SVG to PDF",
            hex_color("1D4ED8"),
            &[
                "Equations are written in LaTeX and rendered to SVG by the math crate before compositing into the page.",
                "Pass MathOptions to control display vs inline mode, colour, and debug SVG output.",
            ],
            hex_color("EFF6FF"),
        ),
        euler,
        text_node(
            "Euler's identity  -  considered the most beautiful equation in mathematics.",
            styles! { font_size: 10.0, line_height: 14.0, color: "#64748B" },
        ),
        spacer(8.0),
        gaussian,
        text_node(
            "Gaussian integral  -  the area under a Gaussian curve over the entire real line.",
            styles! { font_size: 10.0, line_height: 14.0, color: "#64748B" },
        ),
        spacer(8.0),
        quadratic,
        textkit_node,
        image_panel,
    ])
    .with_size(Size::new(A4_W, A4_H))
    .with_style(content_page_style()))
}

// ─── Page 4: Architecture ───────────────────────────────────────────────────

fn architecture_page() -> Page {
    let pipeline_svg = parse_svg(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="515" height="90" viewBox="0 0 515 90">
  <rect width="515" height="90" fill="#F8FAFC" rx="8"/>
  <rect x="0" y="0" width="100" height="60" rx="6" fill="#DBEAFE" stroke="#1D4ED8" stroke-width="1.5"/>
  <text x="10" y="20" font-family="Helvetica" font-size="8" fill="#1D4ED8">MACRO</text>
  <text x="10" y="38" font-family="Helvetica" font-size="12" fill="#0F172A">pdf!</text>
  <text x="10" y="54" font-family="Helvetica" font-size="9" fill="#475569">styles!</text>
  <line x1="100" y1="30" x2="117" y2="30" stroke="#94A3B8" stroke-width="1.5"/>
  <polygon points="117,26 125,30 117,34" fill="#94A3B8"/>
  <rect x="125" y="0" width="100" height="60" rx="6" fill="#D1FAE5" stroke="#059669" stroke-width="1.5"/>
  <text x="135" y="20" font-family="Helvetica" font-size="8" fill="#059669">LAYOUT</text>
  <text x="135" y="38" font-family="Helvetica" font-size="12" fill="#0F172A">Engine</text>
  <text x="135" y="54" font-family="Helvetica" font-size="9" fill="#475569">text + SVG</text>
  <line x1="225" y1="30" x2="242" y2="30" stroke="#94A3B8" stroke-width="1.5"/>
  <polygon points="242,26 250,30 242,34" fill="#94A3B8"/>
  <rect x="250" y="0" width="100" height="60" rx="6" fill="#FEF3C7" stroke="#D97706" stroke-width="1.5"/>
  <text x="260" y="20" font-family="Helvetica" font-size="8" fill="#D97706">RENDER</text>
  <text x="260" y="38" font-family="Helvetica" font-size="12" fill="#0F172A">Engine</text>
  <text x="260" y="54" font-family="Helvetica" font-size="9" fill="#475569">PDF bytes</text>
  <line x1="350" y1="30" x2="367" y2="30" stroke="#94A3B8" stroke-width="1.5"/>
  <polygon points="367,26 375,30 367,34" fill="#94A3B8"/>
  <rect x="375" y="0" width="100" height="60" rx="6" fill="#FCE7F3" stroke="#BE185D" stroke-width="1.5"/>
  <text x="385" y="20" font-family="Helvetica" font-size="8" fill="#BE185D">OUTPUT</text>
  <text x="385" y="38" font-family="Helvetica" font-size="12" fill="#0F172A">PDF file</text>
  <text x="385" y="54" font-family="Helvetica" font-size="9" fill="#475569">standards</text>
  <text x="0" y="80" font-family="Helvetica" font-size="9" fill="#94A3B8">Template macros expand  ->  layout resolves nodes  ->  render serialises  ->  valid PDF output</text>
</svg>"##,
    );

    let pipeline_node = Node::svg(pipeline_svg).with_style(
        LayoutStyle::new()
            .with_width(Pt::new(CW))
            .with_height(Pt::new(90.0))
            .with_margin(EdgeInsets {
                top: Pt::new(0.0),
                right: Pt::new(0.0),
                bottom: Pt::new(18.0),
                left: Pt::new(0.0),
            }),
    );

    let crates_panel = labeled_panel(
        "Crate Ecosystem",
        hex_color("0F172A"),
        &[
            "graphitepdf              Facade - re-exports the full public API surface",
            "graphitepdf-layout       Layout engine - document model and sizing pipeline",
            "graphitepdf-render       Render engine - safe layout document to PDF bytes",
            "graphitepdf-template     Runtime surface for the pdf! macro DSL",
            "graphitepdf-template-macros  Proc macros - pdf!, styles!, stylesheet!",
            "graphitepdf-svg          SVG parser and node model",
            "graphitepdf-math         Math equation renderer (LaTeX to SVG to PDF)",
            "graphitepdf-textkit      Text layout and attributed string engine",
            "graphitepdf-font         Font descriptors and standard PDF font registry",
            "graphitepdf-image        Raster and data-URI image source resolution",
            "graphitepdf-primitives   Value types: Color, Pt, Size, Bounds",
            "graphitepdf-stylesheet   CSS-style property sheet and resolver",
            "graphitepdf-kit          Low-level PDF byte-stream builder (kit API)",
        ],
        hex_color("F1F5F9"),
    );

    let summary_panel = labeled_panel(
        "Rendering Pipeline",
        hex_color("065F46"),
        &[
            "1. Template macros expand JSX-like markup into typed builder calls at compile time.",
            "2. The layout engine resolves sizes, flows text, and composites SVG and math nodes.",
            "3. The render engine walks the safe layout document and emits PDF content streams.",
            "4. render_to_file() writes the complete, standards-compliant PDF to disk.",
        ],
        hex_color("ECFDF5"),
    );

    Page::new([
        section_header(4, "ARCHITECTURE", "Crate Ecosystem  and  Pipeline"),
        pipeline_node,
        crates_panel,
        summary_panel,
    ])
    .with_size(Size::new(A4_W, A4_H))
    .with_style(content_page_style())
}
