mod support;

use graphitepdf_kit::{Canvas, Color, DocumentBuilder, Metadata, PageSize, TextBuilder};
use support::output_path;

fn main() {
    let text_content = TextBuilder::new()
        .font("F1", 24.0)
        .position(100.0, 700.0)
        .text("Hello from graphitepdf-kit!")
        .finish();

    let canvas_content = Canvas::new()
        .fill_color(Color::BLUE)
        .rect(100.0, 650.0, 200.0, 20.0)
        .fill()
        .stroke_color(Color::RED)
        .line_width(2.0)
        .rect(320.0, 650.0, 200.0, 20.0)
        .stroke()
        .finish();

    let combined_content = [text_content, canvas_content].concat();

    let doc = DocumentBuilder::new()
        .metadata(
            Metadata::new()
                .title("Sample PDF from graphitepdf-kit")
                .author("graphitepdf")
                .subject("Rust PDF generation")
                .keywords(["Rust", "PDF", "graphitepdf"]),
        )
        .with_page(PageSize::A4, combined_content);

    let path = output_path("simple").expect("Failed to prepare output path");
    doc.save(&path).expect("Failed to write PDF");

    println!("Generated {}!", path.display());
}
