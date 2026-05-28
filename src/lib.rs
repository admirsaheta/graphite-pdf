pub mod document;
pub mod error;
pub mod layout;
pub mod render;
pub mod renderer;
pub mod style;
pub mod textkit;

pub use crate::layout as layout_crate;
pub use crate::render as render_crate;
pub use crate::renderer as renderer_crate;
pub use crate::textkit as textkit_crate;
pub use document::{Document, ImageNode, ImageSource, Node, NodeKind, PdfMetadata, TextNode};
pub use error::{GraphitePdfError, Result};
pub use graphitepdf_errors as errors;
pub use graphitepdf_font as font;
pub use graphitepdf_image as image;
pub use graphitepdf_image::{
    DataImageSource, DataUriImageSource, ImageAsset, ImageFormat, ImageSource as AssetImageSource,
    LocalImageSource, RemoteCredentials, RemoteImageSource, RemoteMethod, ResolveImageOptions,
    resolve_image, resolve_image_with_options,
};
pub use graphitepdf_kit as kit;
pub use graphitepdf_layout::{
    LayoutContent as CoreLayoutContent, LayoutDocument as CoreLayoutDocument,
    LayoutEngine as CoreLayoutEngine, LayoutNode as CoreLayoutNode, LayoutPage as CoreLayoutPage,
};
pub use graphitepdf_math as math;
pub use graphitepdf_math::{
    MathDimension, MathOptions, MathRender, SvgNode as MathSvgNode, SvgNodeKind as MathSvgNodeKind,
    render_math, render_math_with_options,
};
pub use graphitepdf_primitives as primitives;
pub use graphitepdf_render::{
    RenderCommand, RenderDocument as CoreRenderDocument, RenderEngine as CoreRenderEngine,
    RenderPage as CoreRenderPage,
};
pub use graphitepdf_stylesheet as stylesheet;
pub use graphitepdf_svg as svg;
pub use graphitepdf_svg::{SvgNode, SvgNodeKind, SvgProps, parse_svg, try_parse_svg};
pub use kit::{
    Canvas, DocumentBuilder, FontWeight, LineCap, LineJoin, Metadata, Object, Page, PageMargins,
    PageSize, SvgRenderOptions, TextAlignment, TextBuilder, TextRenderingMode, ToPdfPageContent,
    render_math_to_page_content, render_math_to_page_content_with_options,
    render_svg_node_to_page_content, render_svg_node_to_page_content_with_options,
};
pub use primitives::{Bounds, Color, Point, Pt, Size};
pub use renderer::{NoopRenderBackend, RenderBackend, Renderer};
pub use style::{
    AlignItems, EdgeInsets, FlexDirection, FontDescriptor, FontSource, FontStyle,
    FontVariantWeight, JustifyContent, StandardFont, Style, StyleValue, Stylesheet,
    StylesheetContainer, StylesheetExpandedStyle, StylesheetMap, StylesheetSafeStyle,
};
pub use textkit::{TextBlock, TextSpan};

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

        let doc = DocumentBuilder::new().with_page(PageSize::A4, text);

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

    #[test]
    fn re_exports_stylesheet_and_asset_image_abstractions() {
        let stylesheet = Stylesheet::new(StyleValue::Object(
            [
                ("fontFamily".to_string(), "Inter".into()),
                ("fontWeight".to_string(), 700.into()),
            ]
            .into_iter()
            .collect(),
        ));
        let style = Style::from_stylesheet(&StylesheetContainer::new(100.0, 100.0), &stylesheet);
        let asset_source = AssetImageSource::from(LocalImageSource::new("example.png"));

        assert_eq!(
            style
                .font_descriptor()
                .map(|descriptor| descriptor.family().to_string()),
            Some(String::from("Inter"))
        );
        assert_eq!(
            ImageSource::from(asset_source.clone()).as_asset_source(),
            asset_source
        );
    }
}
