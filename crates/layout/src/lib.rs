pub mod error;

pub use error::*;

use graphitepdf_font::FontDescriptor;
use graphitepdf_primitives::{Bounds, Size};
use graphitepdf_textkit::TextBlock;

#[derive(Clone, Debug, Default)]
pub struct LayoutEngine;

impl LayoutEngine {
    pub fn layout_text_block(&self, page_size: Size, block: TextBlock) -> Result<LayoutDocument> {
        validate_page_size(page_size)?;

        let node = LayoutNode {
            frame: Bounds::from_origin_size(0.0, 0.0, page_size.width, page_size.height),
            content: LayoutContent::Text(block),
        };

        Ok(LayoutDocument {
            pages: vec![LayoutPage {
                size: page_size,
                nodes: vec![node],
            }],
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayoutDocument {
    pub pages: Vec<LayoutPage>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LayoutPage {
    pub size: Size,
    pub nodes: Vec<LayoutNode>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LayoutNode {
    pub frame: Bounds,
    pub content: LayoutContent,
}

impl LayoutNode {
    pub fn font_descriptor(&self) -> Option<&FontDescriptor> {
        match &self.content {
            LayoutContent::Text(block) => block.spans().iter().find_map(|span| span.font()),
            LayoutContent::Box => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LayoutContent {
    Text(TextBlock),
    Box,
}

fn validate_page_size(size: Size) -> Result<()> {
    if size.width <= 0.0 || size.height <= 0.0 {
        Err(Error::InvalidPageSize {
            width: size.width,
            height: size.height,
        })
    } else {
        Ok(())
    }
}
