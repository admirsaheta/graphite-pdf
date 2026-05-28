mod support;

use graphitepdf_kit::*;
use support::output_path;

fn main() {
    println!("Generating complex PDF...");

    // Text content
    let text = TextBuilder::new()
        .font("F1", 16.0)
        .position(100.0, 750.0)
        .text("Hello from GraphitePDF!")
        .text("A pure Rust PDF library with tons of features!")
        .finish();

    // Graphics content (rectangles)
    let graphics = Canvas::new()
        .fill_color(Color::rgb(0.2, 0.5, 0.9))
        .rect(100.0, 600.0, 200.0, 50.0)
        .fill()
        .stroke_color(Color::rgb(1.0, 0.0, 0.0))
        .line_width(3.0)
        .rect(320.0, 600.0, 200.0, 50.0)
        .stroke()
        .finish();

    // Combine everything
    let content = [text, graphics].concat();

    // Build and save the document!
    let doc = DocumentBuilder::new()
        .metadata(
            Metadata::new()
                .title("Complex Example")
                .author("GraphitePDF")
                .subject("PDF generation with Rust")
                .keywords(["rust", "pdf", "generation"]),
        )
        .with_page(PageSize::A4, content);

    let path = output_path("complex").expect("Failed to prepare output path");
    doc.save(&path).expect("failed to write PDF!");
    println!("Saved {} successfully!", path.display());
}
