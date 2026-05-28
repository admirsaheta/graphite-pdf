mod support;

use anyhow::Result;
use graphitepdf_kit::{
    DocumentBuilder, Metadata, PageSize, SvgRenderOptions, TextBuilder, ToPdfPageContent,
};
use graphitepdf_kit::svg::parse_svg;
use support::output_path;

fn main() -> Result<()> {
    let svg = parse_svg(
        r##"
        <svg xmlns="http://www.w3.org/2000/svg" width="320" height="180" viewBox="0 0 320 180">
          <defs>
            <path id="spark" d="M0 24 L24 0 L48 24 L72 12 L96 48" />
          </defs>
          <rect x="8" y="8" width="304" height="164" rx="18" fill="#f6f8fb" stroke="#1f2937" stroke-width="2"/>
          <g transform="translate(24,26)">
            <text x="0" y="18" font-size="18" fill="#111827">SVG Rendering</text>
            <text x="0" y="42" font-size="10" fill="#4b5563">paths, transforms, text, and reused definitions</text>
          </g>
          <g transform="translate(28,92)">
            <line x1="0" y1="52" x2="252" y2="52" stroke="#cbd5e1" stroke-width="2"/>
            <use href="#spark" transform="translate(0,8)" fill="none" stroke="#2563eb" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <circle cx="96" cy="56" r="7" fill="#0f172a"/>
            <rect x="182" y="0" width="92" height="44" rx="10" fill="#dcfce7" stroke="#15803d" stroke-width="2"/>
            <text x="198" y="27" font-size="13" fill="#166534">Reusable &lt;use&gt;</text>
          </g>
        </svg>
        "##,
    );

    let title = TextBuilder::new()
        .font("F1", 20.0)
        .position(48.0, 788.0)
        .text("GraphitePDF SVG Example")
        .finish();

    let svg_content = svg.to_pdf_page_content_with_options(
        &SvgRenderOptions::new()
            .position(48.0, 500.0)
            .size(480.0, 270.0)
            .font_name("F1")
            .font_size(12.0),
    )?;

    let caption = TextBuilder::new()
        .font("F1", 11.0)
        .position(48.0, 470.0)
        .text("Rendered from an in-memory SVG node into PDF page content.")
        .finish();

    let content = [title, svg_content, caption].concat();
    let path = output_path("svg")?;

    DocumentBuilder::new()
        .metadata(
            Metadata::new()
                .title("GraphitePDF SVG Example")
                .author("graphitepdf")
                .subject("SVG to PDF page content")
                .keywords(["graphitepdf", "svg", "pdf", "example"]),
        )
        .with_page(PageSize::A4, content)
        .save(&path)?;

    println!("Generated {}!", path.display());
    Ok(())
}
