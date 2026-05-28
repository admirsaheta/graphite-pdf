use crate::style::Style;
use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
pub struct PdfMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Vec<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Document {
    metadata: PdfMetadata,
    pages: Vec<Node>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            metadata: PdfMetadata::default(),
            pages: Vec::new(),
        }
    }

    pub fn set_metadata(mut self, metadata: PdfMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn add_page(mut self, page: Node) -> Self {
        self.pages.push(page);
        self
    }

    pub fn get_metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    pub fn pages(&self) -> &[Node] {
        &self.pages
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    kind: NodeKind,
    style: Style,
}

impl Node {
    pub fn new(kind: NodeKind, style: Style) -> Self {
        Self { kind, style }
    }

    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }

    pub fn style(&self) -> &Style {
        &self.style
    }
}

#[derive(Clone, Debug)]
pub enum NodeKind {
    View {
        children: Vec<Node>,
    },
    Text(TextNode),
    Image(ImageNode),
}

#[derive(Clone, Debug)]
pub struct TextNode {
    pub content: String,
}

#[derive(Clone, Debug)]
pub struct ImageNode {
    pub source: ImageSource,
}

#[derive(Clone, Debug)]
pub enum ImageSource {
    Path(PathBuf),
    Bytes(Vec<u8>),
}
