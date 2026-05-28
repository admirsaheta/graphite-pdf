use crate::document::{ImageSource, PdfMetadata};
use crate::error::Result;
use crate::layout::{LayoutContent, LayoutDocument, LayoutNode, LayoutPage};
use crate::style::Style;
use graphitepdf_primitives::Bounds;

pub trait RenderBackend {
    fn begin_document(&mut self, _metadata: &PdfMetadata) -> Result<()> {
        Ok(())
    }

    fn begin_page(&mut self, _page: &LayoutPage) -> Result<()> {
        Ok(())
    }

    fn fill_rect(&mut self, _frame: Bounds) -> Result<()> {
        Ok(())
    }

    fn draw_text(&mut self, _frame: Bounds, _style: &Style, _text: &str) -> Result<()> {
        Ok(())
    }

    fn draw_image(&mut self, _frame: Bounds, _source: &ImageSource) -> Result<()> {
        Ok(())
    }

    fn end_page(&mut self, _page: &LayoutPage) -> Result<()> {
        Ok(())
    }

    fn end_document(&mut self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct NoopRenderBackend;

impl RenderBackend for NoopRenderBackend {}

pub struct Renderer<B: RenderBackend> {
    backend: B,
}

impl<B: RenderBackend> Renderer<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn render_document(
        &mut self,
        metadata: &PdfMetadata,
        layout: &LayoutDocument,
    ) -> Result<()> {
        let _ = &layout.metadata;

        self.backend.begin_document(metadata)?;

        for page in &layout.pages {
            self.backend.begin_page(page)?;
            for node in &page.nodes {
                self.render_node(node)?;
            }
            self.backend.end_page(page)?;
        }

        self.backend.end_document()
    }

    fn render_node(&mut self, node: &LayoutNode) -> Result<()> {
        if node.style.background_color.is_some() {
            self.backend.fill_rect(node.frame)?;
        }

        match &node.content {
            LayoutContent::View { children } => {
                for child in children {
                    self.render_node(child)?;
                }
            }
            LayoutContent::Text { text } => {
                self.backend.draw_text(node.frame, &node.style, text)?;
            }
            LayoutContent::Image { source } => {
                self.backend.draw_image(node.frame, source)?;
            }
        }

        Ok(())
    }
}
