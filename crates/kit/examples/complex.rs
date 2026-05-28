use graphitepdf_kit::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn default_output_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../.artifacts/examples/kit")
}

fn output_path(example_name: &str) -> std::io::Result<PathBuf> {
    if let Ok(path) = std::env::var("GRAPHITEPDF_OUTPUT") {
        let path = PathBuf::from(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        return Ok(path);
    }

    let dir = std::env::var_os("GRAPHITEPDF_OUTPUT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(default_output_dir);
    fs::create_dir_all(&dir)?;

    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let pid = std::process::id();

    Ok(dir.join(format!("{example_name}-{stamp}-{pid}.pdf")))
}

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
                .keywords(["rust", "pdf", "generation"])
        )
        .with_page(PageSize::A4, content);

    let path = output_path("complex").expect("Failed to prepare output path");
    doc.save(&path).expect("failed to write PDF!");
    println!("Saved {} successfully!", path.display());
}
