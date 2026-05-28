use crate::document::{Document, ImageSource, Node, NodeKind, PdfMetadata, TextNode};
use crate::error::{GraphitePdfError, Result};
use crate::style::{FontDescriptor, Style};
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
            .map(|page| {
                let size = Size::new(612.0, 792.0);
                self.layout_page(
                    size,
                    page.style(),
                    match page.kind() {
                        NodeKind::View { children } => children,
                        _ => &[],
                    },
                )
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(LayoutDocument {
            metadata: document.get_metadata().clone(),
            pages,
        })
    }

    fn layout_page(&self, size: Size, page_style: &Style, children: &[Node]) -> Result<LayoutPage> {
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
        let width = style.width.map(|value| value.value()).unwrap_or_else(|| {
            (available_width - style.margin.left.value() - style.margin.right.value()).max(0.0)
        });
        let default_height = match node.kind() {
            NodeKind::Text(TextNode { .. }) => 20.0,
            NodeKind::Image(_) => 120.0,
            NodeKind::View { .. } => 0.0,
        };
        let mut height = style
            .height
            .map(|value| value.value())
            .unwrap_or(default_height);

        let content = match node.kind() {
            NodeKind::View { children } => {
                let mut child_cursor_y = y + style.padding.top.value();
                let children = children
                    .iter()
                    .map(|child| {
                        let child_layout = self.layout_node(
                            child,
                            x + style.padding.left.value(),
                            child_cursor_y,
                            width,
                        )?;
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

impl LayoutNode {
    pub fn font_descriptor(&self) -> Option<FontDescriptor> {
        self.style.font_descriptor()
    }
}

#[derive(Clone, Debug)]
pub enum LayoutContent {
    View { children: Vec<LayoutNode> },
    Text { text: String },
    Image { source: ImageSource },
}

impl LayoutContent {
    pub fn image_source(&self) -> Option<&ImageSource> {
        match self {
            Self::Image { source } => Some(source),
            Self::View { .. } | Self::Text { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::ImageNode;
    use crate::style::{StyleValue, Stylesheet, StylesheetContainer};
    use graphitepdf_font::{FontStyle, FontWeight as FontVariantWeight};
    use graphitepdf_image::RemoteImageSource;

    #[test]
    fn layout_preserves_stylesheet_driven_font_metadata() {
        let engine = LayoutEngine;
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
        let page = Node::new(
            NodeKind::View {
                children: vec![Node::from_stylesheet(
                    NodeKind::Text(TextNode::new("Styled")),
                    &container,
                    &stylesheet,
                )],
            },
            Style::default(),
        );
        let document = Document::new().add_page(page);

        let layout = engine
            .layout_document(&document)
            .expect("document should layout");
        let descriptor = layout.pages[0].nodes[0]
            .font_descriptor()
            .expect("text node should expose font descriptor");

        assert_eq!(descriptor.family(), "Inter");
        assert_eq!(descriptor.font_style(), FontStyle::Italic);
        assert_eq!(descriptor.font_weight(), FontVariantWeight::BOLD);
    }

    #[test]
    fn layout_retains_asset_backed_image_sources() {
        let engine = LayoutEngine;
        let page = Node::new(
            NodeKind::View {
                children: vec![Node::new(
                    NodeKind::Image(ImageNode::new(RemoteImageSource::new(
                        "https://example.com/image.png",
                    ))),
                    Style::default(),
                )],
            },
            Style::default(),
        );
        let document = Document::new().add_page(page);

        let layout = engine
            .layout_document(&document)
            .expect("document should layout");

        let source = layout.pages[0].nodes[0]
            .content
            .image_source()
            .expect("image content should expose its source");

        assert_eq!(
            source.as_asset_source(),
            graphitepdf_image::ImageSource::from(RemoteImageSource::new(
                "https://example.com/image.png",
            ))
        );
    }
}
