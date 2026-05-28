pub mod document;
pub mod error;
pub mod layout;
pub mod render;
pub mod style;

pub use document::{Document, ImageSource, Node, NodeKind, PdfMetadata, TextNode};
pub use error::{GraphitePdfError, Result};
pub use graphitepdf_errors as errors;
pub use graphitepdf_kit as kit;
pub use graphitepdf_primitives as primitives;
pub use primitives::{Bounds, Color, Point, Pt, Size};
pub use kit::{
    DocumentBuilder, Page, PageMargins, PageSize, Metadata,
    TextBuilder, Canvas, LineCap, LineJoin, TextAlignment, TextRenderingMode, FontWeight,
    Object,
};

#[cfg(test)]
mod tests {
    use super::*;
    use kit::DocumentBuilder;

    #[test]
    fn smoke_test_pipeline() {
        let text = kit::TextBuilder::new()
            .font("F1", 24.0)
            .position(100.0, 700.0)
            .text("Hello GraphitePDF")
            .finish();
        
        let doc = DocumentBuilder::new()
            .with_page(PageSize::A4, text);

        let mut buf = Vec::new();
        doc.write(&mut buf).expect("pipeline should render");
    }
}
