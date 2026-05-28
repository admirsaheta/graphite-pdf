pub mod error {
    pub use graphitepdf_layout::error::*;
}

pub use error::*;
pub use graphitepdf_layout::{
    Document, EdgeInsets, LayoutContent, LayoutDocument, LayoutMetadata, LayoutNode, LayoutPage,
    LayoutPipelineStep, MathFragment, Node, NodeKind, ORDERED_PIPELINE, Page, SafeFont,
    SafeLayoutDocument, SafeLayoutNode, SafeLayoutPage, SafeLayoutStyle, SafeNodeKind,
};

use crate::document;
use graphitepdf_font::FontStore;
use graphitepdf_primitives::Size;
use graphitepdf_textkit::{TextBlock, TextEngine, TextEngineConfig};

pub trait LayoutDocumentSource {
    fn to_layout_document(&self) -> Document;
}

impl LayoutDocumentSource for document::Document {
    fn to_layout_document(&self) -> Document {
        Document::from(self)
    }
}

impl LayoutDocumentSource for Document {
    fn to_layout_document(&self) -> Document {
        self.clone()
    }
}

#[derive(Default)]
pub struct LayoutEngine {
    inner: graphitepdf_layout::LayoutEngine,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            inner: graphitepdf_layout::LayoutEngine::new(),
        }
    }

    pub fn with_text_engine_config(mut self, config: TextEngineConfig) -> Self {
        self.inner = self.inner.with_text_engine_config(config);
        self
    }

    pub fn with_default_page_size(mut self, size: Size) -> Result<Self> {
        self.inner = self.inner.with_default_page_size(size)?;
        Ok(self)
    }

    pub fn text_engine(&self) -> &TextEngine {
        self.inner.text_engine()
    }

    pub fn font_store(&self) -> &FontStore {
        self.inner.font_store()
    }

    pub fn font_store_mut(&mut self) -> &mut FontStore {
        self.inner.font_store_mut()
    }

    pub fn layout_text_block(&self, page_size: Size, block: TextBlock) -> Result<LayoutDocument> {
        self.inner.layout_text_block(page_size, block)
    }

    pub fn layout_document<T>(&self, document: &T) -> Result<SafeLayoutDocument>
    where
        T: LayoutDocumentSource + ?Sized,
    {
        self.inner.layout_document(&document.to_layout_document())
    }

    pub fn as_core(&self) -> &graphitepdf_layout::LayoutEngine {
        &self.inner
    }

    pub fn into_core(self) -> graphitepdf_layout::LayoutEngine {
        self.inner
    }
}

impl From<graphitepdf_layout::LayoutEngine> for LayoutEngine {
    fn from(value: graphitepdf_layout::LayoutEngine) -> Self {
        Self { inner: value }
    }
}

impl From<LayoutEngine> for graphitepdf_layout::LayoutEngine {
    fn from(value: LayoutEngine) -> Self {
        value.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{
        Document as CompatDocument, ImageNode, Node as CompatNode, NodeKind, TextNode,
    };
    use crate::style::{Style, StyleValue, Stylesheet, StylesheetContainer};
    use graphitepdf_font::{FontStyle, FontWeight as FontVariantWeight};
    use graphitepdf_image::RemoteImageSource;

    #[test]
    fn layouts_compat_documents_through_split_layout_engine() {
        let container = StylesheetContainer::new(612.0, 792.0);
        let stylesheet = Stylesheet::new(StyleValue::Object(
            [
                ("fontFamily".to_string(), "Inter".into()),
                ("fontStyle".to_string(), "italic".into()),
                ("fontWeight".to_string(), 700.into()),
            ]
            .into_iter()
            .collect(),
        ));
        let page = CompatNode::new(
            NodeKind::View {
                children: vec![CompatNode::from_stylesheet(
                    NodeKind::Text(TextNode::new("Styled")),
                    &container,
                    &stylesheet,
                )],
            },
            Style::default(),
        );
        let document = CompatDocument::new().add_page(page);

        let layout = LayoutEngine::new()
            .layout_document(&document)
            .expect("document should layout");
        let descriptor = layout.pages()[0].nodes()[0]
            .font_descriptor()
            .expect("text node should expose font descriptor");

        assert_eq!(descriptor.family(), "Inter");
        assert_eq!(descriptor.font_style(), FontStyle::Italic);
        assert_eq!(descriptor.font_weight(), FontVariantWeight::BOLD);
    }

    #[test]
    fn keeps_image_sources_when_facading_the_split_layout_engine() {
        let page = CompatNode::new(
            NodeKind::View {
                children: vec![CompatNode::new(
                    NodeKind::Image(ImageNode::new(RemoteImageSource::new(
                        "https://example.com/image.png",
                    ))),
                    Style::default(),
                )],
            },
            Style::default(),
        );
        let document = CompatDocument::new().add_page(page);

        let layout = LayoutEngine::new()
            .layout_document(&document)
            .expect("document should layout");

        let node = &layout.pages()[0].nodes()[0];
        assert!(matches!(
            node.kind,
            SafeNodeKind::ImageSource { ref source }
                if *source == graphitepdf_image::ImageSource::from(RemoteImageSource::new(
                    "https://example.com/image.png",
                ))
        ));
    }
}
