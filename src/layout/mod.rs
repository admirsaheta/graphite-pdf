use crate::document::{Document, ImageSource, Node, NodeKind, PdfMetadata, TextNode};
use crate::error::{GraphitePdfError, Result};
use crate::style::Style;
use graphitepdf_primitives::{Bounds, Size};

#[derive(Clone, Debug, Default)]
pub struct LayoutEngine;

impl LayoutEngine {
    pub fn layout_document(&self, document: &Document) -> Result<LayoutDocument> {
        if document.pages().is_empty() {
            return Err(GraphitePdfError::InvalidDocument(
                "document must contain at least one page".to_string(),
            ));
        }

        let pages = document
            .pages()
            .iter()
            .map(|page| self.layout_page(page.size(), page.style(), page.children()))
            .collect::<Result<Vec<_>>>()?;

        Ok(LayoutDocument {
            metadata: document.metadata().clone(),
            pages,
        })
    }

    fn layout_page(
        &self,
        size: Size,
        page_style: &Style,
        children: &[Node],
    ) -> Result<LayoutPage> {
        let mut cursor_y = page_style.padding.top.value();
        let nodes = children
            .iter()
            .map(|node| {
                let layout = self.layout_node(node, 0.0, cursor_y, size.width)?;
                cursor_y += layout.frame.size.height + node.style().margin.bottom.value();
                Ok(layout)
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(LayoutPage { size, nodes })
    }

    fn layout_node(
        &self,
        node: &Node,
        parent_x: f32,
        parent_y: f32,
        available_width: f32,
    ) -> Result<LayoutNode> {
        let style = node.style().clone();
        let x = parent_x + style.margin.left.value();
        let y = parent_y + style.margin.top.value();
        let width = style
            .width
            .map(|value| value.value())
            .unwrap_or_else(|| (available_width - style.margin.left.value() - style.margin.right.value()).max(0.0));
        let default_height = match node.kind() {
            NodeKind::Text(TextNode { .. }) => 20.0,
            NodeKind::Image(_) => 120.0,
            NodeKind::View { .. } => 0.0,
        };
        let mut height = style.height.map(|value| value.value()).unwrap_or(default_height);

        let content = match node.kind() {
            NodeKind::View { children } => {
                let mut child_cursor_y = y + style.padding.top.value();
                let children = children
                    .iter()
                    .map(|child| {
                        let child_layout =
                            self.layout_node(child, x + style.padding.left.value(), child_cursor_y, width)?;
                        child_cursor_y +=
                            child_layout.frame.size.height + child.style().margin.bottom.value();
                        Ok(child_layout)
                    })
                    .collect::<Result<Vec<_>>>()?;

                let content_height = children
                    .last()
                    .map(|child| (child.frame.origin.y + child.frame.size.height) - y)
                    .unwrap_or(0.0);
                if height == 0.0 {
                    height = content_height + style.padding.bottom.value();
                }

                LayoutContent::View { children }
            }
            NodeKind::Text(TextNode { content }) => LayoutContent::Text {
                text: content.clone(),
            },
            NodeKind::Image(image) => LayoutContent::Image {
                source: image.source.clone(),
            },
        };

        Ok(LayoutNode {
            frame: Bounds::from_origin_size(x, y, width, height),
            style,
            content,
        })
    }
}

#[derive(Clone, Debug)]
pub struct LayoutDocument {
    pub metadata: PdfMetadata,
    pub pages: Vec<LayoutPage>,
}

#[derive(Clone, Debug)]
pub struct LayoutPage {
    pub size: Size,
    pub nodes: Vec<LayoutNode>,
}

#[derive(Clone, Debug)]
pub struct LayoutNode {
    pub frame: Bounds,
    pub style: Style,
    pub content: LayoutContent,
}

#[derive(Clone, Debug)]
pub enum LayoutContent {
    View { children: Vec<LayoutNode> },
    Text { text: String },
    Image { source: ImageSource },
}
