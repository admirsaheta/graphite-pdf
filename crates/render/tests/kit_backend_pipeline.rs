use std::io::Read as _;

use flate2::read::ZlibDecoder;
use graphitepdf_font::{FontSource, StandardFont};
use graphitepdf_image::{DataImageSource, Image, ImageFormat, RasterImage};
use graphitepdf_layout::{Document, EdgeInsets, LayoutEngine, LayoutStyle, Node, Page};
use graphitepdf_primitives::{Pt, Size};
use graphitepdf_render::{RenderCommand, RenderEngine, RendererSession, render_to_bytes};
use graphitepdf_svg::try_parse_svg;
use graphitepdf_textkit::{TextBlock, TextSpan};

#[test]
fn layout_documents_render_to_real_pdf_bytes_via_kit_backend() {
    let document = sample_document();
    let layout = LayoutEngine::new()
        .layout_document(&document)
        .expect("layout should build");
    let rendered = RenderEngine::new()
        .build(&layout)
        .expect("render document should build");
    let text_command = rendered.pages[0]
        .commands
        .iter()
        .find_map(|command| match command {
            RenderCommand::DrawText(operation) => Some(operation),
            _ => None,
        })
        .expect("rendered document should contain text");

    assert!(text_command.layout.is_some());
    assert_eq!(
        text_command.font_source.as_ref(),
        Some(&FontSource::standard(StandardFont::Courier))
    );

    let pdf = render_to_bytes(&document).expect("document should render to PDF bytes");
    let decoded_streams = decode_pdf_streams(&pdf);

    assert!(pdf.starts_with(b"%PDF-1.7"));
    assert!(contains_bytes(&pdf, b"/Type /Font"));
    assert!(contains_bytes(&pdf, b"/Type /Page"));
    assert!(decoded_streams.contains("BT\n"));
    assert!(decoded_streams.contains(" Tf\n"));
    assert!(decoded_streams.contains(" Tj\n"));
    assert!(decoded_streams.contains("BI\n/Width 1\n/Height 1"));
    assert!(decoded_streams.matches("% svg ").count() >= 2);
}

#[test]
fn renderer_session_emits_real_pdf_output_for_layout_documents() {
    let mut session = RendererSession::new(sample_document());
    let pdf = session
        .to_bytes()
        .expect("renderer session should emit PDF bytes");
    let decoded_streams = decode_pdf_streams(&pdf);

    assert!(pdf.starts_with(b"%PDF-1.7"));
    assert!(decoded_streams.contains("BT\n"));
    assert!(decoded_streams.contains("BI\n/Width 1\n/Height 1"));
}

#[test]
fn unresolved_image_sources_are_resolved_before_pdf_encoding() {
    let document = Document::new().with_page(
        Page::new([
            Node::image_source(DataImageSource::new(tiny_png(), ImageFormat::Png)).with_style(
                LayoutStyle::new()
                    .with_width(Pt::new(18.0))
                    .with_height(Pt::new(18.0)),
            ),
        ])
        .with_size(Size::new(120.0, 120.0))
        .with_style(LayoutStyle::new().with_padding(EdgeInsets::all(Pt::new(12.0)))),
    );

    let pdf = render_to_bytes(&document).expect("image source should resolve during PDF encoding");
    let decoded_streams = decode_pdf_streams(&pdf);

    assert!(pdf.starts_with(b"%PDF-1.7"));
    assert!(decoded_streams.contains("BI\n/Width 1\n/Height 1"));
}

fn sample_document() -> Document {
    let text = TextBlock::from(TextSpan::new("Task 6 text").expect("text span should build"));
    let image = Image::Raster(RasterImage {
        width: 1,
        height: 1,
        data: tiny_png(),
        format: ImageFormat::Png,
        key: Some(String::from("pixel")),
    });
    let svg = try_parse_svg(
        r##"<svg width="16" height="12" viewBox="0 0 16 12">
<rect width="16" height="12" fill="#ff5500" />
</svg>"##,
    )
    .expect("SVG should parse");

    Document::new().with_page(
        Page::new([
            Node::text(text).with_style(
                LayoutStyle::new()
                    .with_font_family("Courier")
                    .with_font_source(FontSource::standard(StandardFont::Courier))
                    .with_font_size(Pt::new(14.0)),
            ),
            Node::image_asset(image).with_style(
                LayoutStyle::new()
                    .with_width(Pt::new(18.0))
                    .with_height(Pt::new(18.0)),
            ),
            Node::svg(svg).with_style(LayoutStyle::new().with_width(Pt::new(24.0))),
            Node::math("x^2 + y^2").with_style(LayoutStyle::new().with_width(Pt::new(32.0))),
        ])
        .with_size(Size::new(240.0, 240.0))
        .with_style(LayoutStyle::new().with_padding(EdgeInsets::all(Pt::new(12.0)))),
    )
}

fn decode_pdf_streams(pdf: &[u8]) -> String {
    let mut decoded = String::new();
    let mut offset = 0;

    while let Some(stream_start) = find_bytes(&pdf[offset..], b"stream\n") {
        let start = offset + stream_start + "stream\n".len();
        let Some(stream_end) = find_bytes(&pdf[start..], b"\nendstream") else {
            break;
        };
        let stream = &pdf[start..start + stream_end];
        let mut inflater = ZlibDecoder::new(stream);
        let mut bytes = Vec::new();
        if inflater.read_to_end(&mut bytes).is_ok() {
            decoded.push_str(&String::from_utf8_lossy(&bytes));
            decoded.push('\n');
        }
        offset = start + stream_end + "\nendstream".len();
    }

    decoded
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn tiny_png() -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut encoder = png::Encoder::new(&mut bytes, 1, 1);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().expect("PNG header should encode");
    writer
        .write_image_data(&[0x22, 0x66, 0xCC])
        .expect("PNG pixels should encode");
    drop(writer);
    bytes
}
