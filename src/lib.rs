pub mod document;
pub mod error;
pub mod layout;
pub mod render;
pub mod style;

pub use document::{Document, ImageSource, Node, NodeKind, PdfMetadata, TextNode};
pub use error::{GraphitePdfError, Result};
pub use graphitepdf_errors as errors;
pub use graphitepdf_kit as kit;
pub use graphitepdf_math as math;
pub use graphitepdf_primitives as primitives;
pub use graphitepdf_svg as svg;
pub use graphitepdf_math::{
    MathDimension, MathOptions, MathRender, SvgNode as MathSvgNode, SvgNodeKind as MathSvgNodeKind,
    render_math, render_math_with_options,
};
pub use graphitepdf_svg::{SvgNode, SvgNodeKind, SvgProps, parse_svg, try_parse_svg};
pub use primitives::{Bounds, Color, Point, Pt, Size};
pub use kit::{
    DocumentBuilder, Page, PageMargins, PageSize, Metadata,
    TextBuilder, Canvas, LineCap, LineJoin, TextAlignment, TextRenderingMode, FontWeight,
    Object, SvgRenderOptions, ToPdfPageContent, render_math_to_page_content,
    render_math_to_page_content_with_options, render_svg_node_to_page_content,
    render_svg_node_to_page_content_with_options,
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

    #[test]
    fn re_exports_svg_and_math_page_content_helpers() {
        let svg = parse_svg(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10">
                <rect x="0" y="0" width="10" height="10" fill="black"/>
            </svg>"#,
        );

        let content = svg
            .to_pdf_page_content_with_options(&SvgRenderOptions::new().position(10.0, 20.0))
            .expect("root crate should expose the rendering trait");

        assert!(!content.is_empty());
    }
}
