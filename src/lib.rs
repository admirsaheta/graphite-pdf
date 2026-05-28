pub mod document;
pub mod error;
pub mod layout;
pub mod render;
pub mod style;

use document::Document;
use layout::{LayoutDocument, LayoutEngine};
use render::{RenderBackend, Renderer};

pub use document::{
    Document as PdfDocument, ImageFit, ImageNode, ImageSource, Node, NodeKind, Page, PdfMetadata,
    TextNode,
};
pub use error::{GraphitePdfError, Result};
pub use graphitepdf_primitives as primitives;
pub use layout::{LayoutContent, LayoutNode, LayoutPage};
pub use primitives::{Bounds, Color, Point, Pt, Size};
pub use render::{NoopRenderBackend, RenderBackend as Backend, Renderer as RenderPipeline};
pub use style::{AlignItems, EdgeInsets, FlexDirection, JustifyContent, Style};

pub struct GraphitePdf<B: RenderBackend> {
    layout_engine: LayoutEngine,
    renderer: Renderer<B>,
}

impl<B: RenderBackend> GraphitePdf<B> {
    pub fn new(backend: B) -> Self {
        Self {
            layout_engine: LayoutEngine::default(),
            renderer: Renderer::new(backend),
        }
    }

    pub fn with_layout_engine(layout_engine: LayoutEngine, backend: B) -> Self {
        Self {
            layout_engine,
            renderer: Renderer::new(backend),
        }
    }

    pub fn layout(&self, document: &Document) -> Result<LayoutDocument> {
        self.layout_engine.layout_document(document)
    }

    pub fn render(&mut self, document: &Document) -> Result<()> {
        let layout = self.layout(document)?;
        self.renderer.render_document(document.metadata(), &layout)
    }

    pub fn backend(&self) -> &B {
        self.renderer.backend()
    }

    pub fn backend_mut(&mut self) -> &mut B {
        self.renderer.backend_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{Node, Page};
    use crate::render::NoopRenderBackend;
    use graphitepdf_primitives::Size;

    #[test]
    fn smoke_test_pipeline() {
        let page = Page::new(Size::new(595.0, 842.0)).with_child(Node::text("Hello GraphitePDF"));
        let document = Document::new().with_page(page);

        let mut engine = GraphitePdf::new(NoopRenderBackend::default());

        engine.render(&document).expect("pipeline should render");
    }
}
