pub use graphitepdf_kit::PageSize;
pub use graphitepdf_layout::{LayoutMetadata, LayoutStyle};
pub use graphitepdf_template_macros::{pdf, styles, stylesheet};

use graphitepdf_layout::{Document, Node, Page};

#[derive(Clone, Debug, PartialEq)]
pub enum PdfNode {
    Document(Document),
    Page(Page),
    Node(Node),
    Fragment(Vec<PdfNode>),
    Empty,
}

impl PdfNode {
    pub fn empty() -> Self {
        Self::Empty
    }

    pub fn fragment(children: impl IntoIterator<Item = PdfNode>) -> Self {
        Self::Fragment(children.into_iter().collect())
    }
}

impl From<Document> for PdfNode {
    fn from(value: Document) -> Self {
        Self::Document(value)
    }
}

impl From<Page> for PdfNode {
    fn from(value: Page) -> Self {
        Self::Page(value)
    }
}

impl From<Node> for PdfNode {
    fn from(value: Node) -> Self {
        Self::Node(value)
    }
}

pub mod __private {
    pub use graphitepdf_image::ImageSource as AssetImageSource;
    pub use graphitepdf_kit::PageSize;
    pub use graphitepdf_layout::{
        Document as LayoutDocument, LayoutMetadata, LayoutStyle, Node as LayoutNode,
        Page as LayoutPage,
    };
    pub use graphitepdf_primitives::Size;
    pub use graphitepdf_textkit::{TextBlock, TextSpan};

    use graphitepdf_layout::{Node, Page};

    pub trait IntoPdfPageSize {
        fn into_pdf_size(self) -> Size;
    }

    impl IntoPdfPageSize for Size {
        fn into_pdf_size(self) -> Size {
            self
        }
    }

    impl IntoPdfPageSize for (f32, f32) {
        fn into_pdf_size(self) -> Size {
            Size::new(self.0, self.1)
        }
    }

    impl IntoPdfPageSize for (f64, f64) {
        fn into_pdf_size(self) -> Size {
            Size::new(self.0 as f32, self.1 as f32)
        }
    }

    impl IntoPdfPageSize for PageSize {
        fn into_pdf_size(self) -> Size {
            Size::new(self.width as f32, self.height as f32)
        }
    }

    pub fn into_pdf_size<T>(value: T) -> Size
    where
        T: IntoPdfPageSize,
    {
        value.into_pdf_size()
    }

    pub trait IntoLayoutNodeChildren {
        fn into_layout_nodes(self) -> Vec<Node>;
    }

    impl IntoLayoutNodeChildren for Node {
        fn into_layout_nodes(self) -> Vec<Node> {
            vec![self]
        }
    }

    impl IntoLayoutNodeChildren for Vec<Node> {
        fn into_layout_nodes(self) -> Vec<Node> {
            self
        }
    }

    impl<const N: usize> IntoLayoutNodeChildren for [Node; N] {
        fn into_layout_nodes(self) -> Vec<Node> {
            self.into_iter().collect()
        }
    }

    impl IntoLayoutNodeChildren for Option<Node> {
        fn into_layout_nodes(self) -> Vec<Node> {
            self.into_iter().collect()
        }
    }

    impl IntoLayoutNodeChildren for Option<Vec<Node>> {
        fn into_layout_nodes(self) -> Vec<Node> {
            self.unwrap_or_default()
        }
    }

    pub fn into_layout_nodes<T>(value: T) -> Vec<Node>
    where
        T: IntoLayoutNodeChildren,
    {
        value.into_layout_nodes()
    }

    pub trait IntoLayoutPageChildren {
        fn into_layout_pages(self) -> Vec<Page>;
    }

    impl IntoLayoutPageChildren for Page {
        fn into_layout_pages(self) -> Vec<Page> {
            vec![self]
        }
    }

    impl IntoLayoutPageChildren for Vec<Page> {
        fn into_layout_pages(self) -> Vec<Page> {
            self
        }
    }

    impl<const N: usize> IntoLayoutPageChildren for [Page; N] {
        fn into_layout_pages(self) -> Vec<Page> {
            self.into_iter().collect()
        }
    }

    impl IntoLayoutPageChildren for Option<Page> {
        fn into_layout_pages(self) -> Vec<Page> {
            self.into_iter().collect()
        }
    }

    impl IntoLayoutPageChildren for Option<Vec<Page>> {
        fn into_layout_pages(self) -> Vec<Page> {
            self.unwrap_or_default()
        }
    }

    pub fn into_layout_pages<T>(value: T) -> Vec<Page>
    where
        T: IntoLayoutPageChildren,
    {
        value.into_layout_pages()
    }

    pub fn text_node_from_str(value: &str) -> Node {
        let span = TextSpan::new(value).expect("pdf! produced invalid text content");
        Node::text(TextBlock::from(span))
    }

    pub fn text_node_from_string(value: String) -> Node {
        let span = TextSpan::new(value).expect("pdf! produced invalid text content");
        Node::text(TextBlock::from(span))
    }
}
