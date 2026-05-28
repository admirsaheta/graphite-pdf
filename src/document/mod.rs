use crate::primitives::Size;
use crate::style::Style;

#[derive(Clone, Debug, Default)]
pub struct Document {
    metadata: PdfMetadata,
    pages: Vec<Page>,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_metadata(mut self, metadata: PdfMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn with_page(mut self, page: Page) -> Self {
        self.pages.push(page);
        self
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }

    pub fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
}

#[derive(Clone, Debug)]
pub struct Page {
    size: Size,
    style: Style,
    children: Vec<Node>,
}

impl Page {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            style: Style::default(),
            children: Vec::new(),
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_child(mut self, child: Node) -> Self {
        self.children.push(child);
        self
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    style: Style,
    kind: NodeKind,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            style: Style::default(),
            kind,
        }
    }

    pub fn view(children: Vec<Node>) -> Self {
        Self::new(NodeKind::View { children })
    }

    pub fn text(content: impl Into<String>) -> Self {
        Self::new(NodeKind::Text(TextNode {
            content: content.into(),
        }))
    }

    pub fn image(source: ImageSource) -> Self {
        Self::new(NodeKind::Image(ImageNode {
            source,
            fit: ImageFit::default(),
        }))
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }
}

#[derive(Clone, Debug)]
pub enum NodeKind {
    View { children: Vec<Node> },
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
    pub fit: ImageFit,
}

#[derive(Clone, Debug)]
pub enum ImageSource {
    Path(String),
    Bytes(Vec<u8>),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ImageFit {
    #[default]
    Contain,
    Cover,
    Fill,
}

#[derive(Clone, Debug, Default)]
pub struct PdfMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Vec<String>,
}
