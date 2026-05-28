mod support;

use std::io::Error as IoError;

use graphitepdf_font::{FontSource, StandardFont};
use graphitepdf_layout::{Document, EdgeInsets, LayoutEngine, LayoutMetadata, LayoutStyle, Node, Page};
use graphitepdf_primitives::{Color, Pt, Size};
use graphitepdf_renderer::{RenderCommand, RenderEngine, RendererSession};
use graphitepdf_textkit::{TextBlock, TextSpan};
use support::output_path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let document = sample_document()?;

    let layout = LayoutEngine::new().layout_document(&document)?;
    let rendered = RenderEngine::new().build(&layout)?;
    let command_count: usize = rendered.pages.iter().map(|page| page.commands.len()).sum();
    let text_count = rendered
        .pages
        .iter()
        .flat_map(|page| page.commands.iter())
        .filter(|command| matches!(command, RenderCommand::DrawText(_)))
        .count();

    let path = output_path("pipeline")?;
    let mut session = RendererSession::new(document);
    let snapshot = session.render_snapshot()?;
    if snapshot.document().pages.len() != rendered.pages.len() {
        return Err(IoError::other("renderer session page count diverged from explicit render build").into());
    }

    session.save(&path)?;

    println!(
        "Rendered layout -> render -> renderer pipeline: {} page(s), {} command(s), {} text command(s).",
        layout.pages().len(),
        command_count,
        text_count
    );
    println!("Saved PDF to {}", path.display());
    println!(
        "Set GRAPHITEPDF_OUTPUT for an exact file path or GRAPHITEPDF_OUTPUT_DIR to override the default .artifacts destination."
    );

    Ok(())
}

fn sample_document() -> Result<Document, Box<dyn std::error::Error>> {
    let title = TextBlock::from(TextSpan::new("GraphitePDF split pipeline")?);
    let body = TextBlock::from(TextSpan::new(
        "This example lays out a document, builds render commands, and writes the final PDF through the renderer session.",
    )?);

    Ok(Document::new()
        .with_metadata(LayoutMetadata {
            title: Some(String::from("GraphitePDF Pipeline Example")),
            author: Some(String::from("graphitepdf")),
            subject: Some(String::from("layout -> render -> renderer example")),
            keywords: vec![
                String::from("graphitepdf"),
                String::from("layout"),
                String::from("render"),
                String::from("renderer"),
                String::from("example"),
            ],
            creator: Some(String::from("graphitepdf-renderer example")),
            producer: Some(String::from("graphitepdf")),
        })
        .with_page(
            Page::new([
                Node::view([
                    Node::box_node().with_style(
                        LayoutStyle::new()
                            .with_width(Pt::new(452.0))
                            .with_height(Pt::new(92.0))
                            .with_background_color(Color::rgb(0xe0, 0xec, 0xff)),
                    ),
                    Node::text(title).with_style(
                        LayoutStyle::new()
                            .with_font_family("Helvetica")
                            .with_font_source(FontSource::standard(StandardFont::HelveticaBold))
                            .with_font_size(Pt::new(20.0))
                            .with_line_height(Pt::new(24.0))
                            .with_color(Color::rgb(0x17, 0x2b, 0x4d)),
                    ),
                    Node::text(body).with_style(
                        LayoutStyle::new()
                            .with_font_family("Helvetica")
                            .with_font_source(FontSource::standard(StandardFont::Helvetica))
                            .with_font_size(Pt::new(12.0))
                            .with_line_height(Pt::new(16.0))
                            .with_color(Color::rgb(0x2d, 0x37, 0x48)),
                    ),
                ])
                .with_style(
                    LayoutStyle::new()
                        .with_padding(EdgeInsets::all(Pt::new(20.0)))
                        .with_background_color(Color::rgb(0xf8, 0xfa, 0xfc)),
                ),
            ])
            .with_size(Size::new(512.0, 256.0))
            .with_style(
                LayoutStyle::new()
                    .with_padding(EdgeInsets::all(Pt::new(28.0)))
                    .with_background_color(Color::rgb(0xff, 0xff, 0xff)),
            ),
        ))
}
