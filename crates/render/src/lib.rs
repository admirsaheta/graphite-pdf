pub mod error;

pub use error::*;

use graphitepdf_layout::{LayoutContent, LayoutDocument, LayoutNode};
use graphitepdf_primitives::Bounds;

#[derive(Clone, Debug, Default)]
pub struct RenderEngine;

impl RenderEngine {
    pub fn build(&self, layout: &LayoutDocument) -> Result<RenderDocument> {
        let pages = layout
            .pages
            .iter()
            .map(|page| RenderPage {
                commands: page
                    .nodes
                    .iter()
                    .flat_map(render_node)
                    .collect(),
            })
            .collect();

        Ok(RenderDocument { pages })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderDocument {
    pub pages: Vec<RenderPage>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderPage {
    pub commands: Vec<RenderCommand>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RenderCommand {
    DrawText { frame: Bounds, text: String },
    DrawBox { frame: Bounds },
}

fn render_node(node: &LayoutNode) -> Vec<RenderCommand> {
    match &node.content {
        LayoutContent::Text(block) => vec![RenderCommand::DrawText {
            frame: node.frame,
            text: block.plain_text(),
        }],
        LayoutContent::Box => vec![RenderCommand::DrawBox { frame: node.frame }],
    }
}
