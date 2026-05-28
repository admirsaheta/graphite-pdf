use graphitepdf_image::{
    DataImageSource, DataUriImageSource, Image as ImageAsset, ImageFormat,
    ImageSource as AssetImageSource, LocalImageSource, RemoteImageSource,
};
use graphitepdf_layout::{Document as LayoutDocument, Node as LayoutNode, Page as LayoutPage};
use graphitepdf_primitives::Pt;
pub use graphitepdf_style::{
    AlignItems, EdgeInsets, FlexDirection, FontDescriptor, FontSource, FontStyle,
    FontVariantWeight, JustifyContent, StandardFont, Style, StyleValue, Stylesheet,
    StylesheetContainer, StylesheetExpandedStyle, StylesheetMap, StylesheetSafeStyle,
};
use graphitepdf_textkit::{TextBlock, TextSpan};
use std::path::PathBuf;

pub type PdfMetadata = graphitepdf_layout::LayoutMetadata;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Document {
    metadata: PdfMetadata,
    pages: Vec<Node>,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_metadata(mut self, metadata: PdfMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn with_metadata(self, metadata: PdfMetadata) -> Self {
        self.set_metadata(metadata)
    }

    pub fn add_page(mut self, page: Node) -> Self {
        self.pages.push(page);
        self
    }

    pub fn get_metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    pub fn metadata(&self) -> &PdfMetadata {
        self.get_metadata()
    }

    pub fn pages(&self) -> &[Node] {
        &self.pages
    }

    pub fn to_layout_document(&self) -> LayoutDocument {
        let mut document = LayoutDocument::new().with_metadata(self.metadata.clone());
        for page in &self.pages {
            document.add_page(page.to_layout_page());
        }
        document
    }

    pub fn into_layout_document(self) -> LayoutDocument {
        LayoutDocument::from(&self)
    }
}

impl From<&Document> for LayoutDocument {
    fn from(value: &Document) -> Self {
        value.to_layout_document()
    }
}

impl From<Document> for LayoutDocument {
    fn from(value: Document) -> Self {
        LayoutDocument::from(&value)
    }
}

impl graphitepdf_renderer::RendererDocumentSource for Document {
    fn build_render_document(
        &self,
        layout_engine: &graphitepdf_layout::LayoutEngine,
        render_engine: &graphitepdf_render::RenderEngine,
    ) -> graphitepdf_renderer::Result<graphitepdf_render::RenderDocument> {
        let layout = layout_engine.layout_document(&LayoutDocument::from(self))?;
        Ok(render_engine.build(&layout)?)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    kind: NodeKind,
    style: Style,
}

impl Node {
    pub fn new(kind: NodeKind, style: Style) -> Self {
        Self { kind, style }
    }

    pub fn from_stylesheet(
        kind: NodeKind,
        container: &StylesheetContainer,
        stylesheet: &Stylesheet,
    ) -> Self {
        Self::new(kind, Style::from_stylesheet(container, stylesheet))
    }

    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn to_layout_node(&self) -> LayoutNode {
        let mut style = self.style.to_layout_style();

        match &self.kind {
            NodeKind::View { children } => LayoutNode::view(children.iter().map(LayoutNode::from))
                .with_style(style),
            NodeKind::Text(text) => text
                .to_text_block()
                .map(LayoutNode::text)
                .unwrap_or_else(graphitepdf_layout::Node::box_node)
                .with_style(style),
            NodeKind::Image(image) => {
                if image.source.as_asset().is_none() && style.height.is_none() {
                    style.height = Some(Pt::new(120.0));
                }

                image.to_layout_node().with_style(style)
            }
        }
    }

    pub fn to_layout_page(&self) -> LayoutPage {
        let style = self.style.to_layout_style();

        match &self.kind {
            NodeKind::View { children } => {
                LayoutPage::new(children.iter().map(LayoutNode::from)).with_style(style)
            }
            _ => LayoutPage::new([self.to_layout_node()]).with_style(style),
        }
    }
}

impl From<&Node> for LayoutNode {
    fn from(value: &Node) -> Self {
        value.to_layout_node()
    }
}

impl From<Node> for LayoutNode {
    fn from(value: Node) -> Self {
        LayoutNode::from(&value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeKind {
    View { children: Vec<Node> },
    Text(TextNode),
    Image(ImageNode),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextNode {
    pub content: String,
}

impl TextNode {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }

    fn to_text_block(&self) -> Option<TextBlock> {
        let span = TextSpan::new(self.content.clone()).ok()?;
        Some(TextBlock::from(span))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImageNode {
    pub source: ImageSource,
}

impl ImageNode {
    pub fn new(source: impl Into<ImageSource>) -> Self {
        Self {
            source: source.into(),
        }
    }

    fn to_layout_node(&self) -> LayoutNode {
        match &self.source {
            ImageSource::Asset(asset) => LayoutNode::image_asset(asset.clone()),
            ImageSource::Path(_)
            | ImageSource::Bytes(_)
            | ImageSource::AssetSource(_) => LayoutNode::image_source(self.source.as_asset_source()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImageSource {
    Path(PathBuf),
    Bytes(Vec<u8>),
    AssetSource(AssetImageSource),
    Asset(ImageAsset),
}

impl ImageSource {
    pub fn as_asset_source(&self) -> AssetImageSource {
        match self {
            Self::Path(path) => LocalImageSource::new(path.clone()).into(),
            Self::Bytes(bytes) => bytes.clone().into(),
            Self::AssetSource(source) => source.clone(),
            Self::Asset(asset) => asset_image_source(asset),
        }
    }

    pub const fn as_asset(&self) -> Option<&ImageAsset> {
        match self {
            Self::Asset(asset) => Some(asset),
            Self::Path(_) | Self::Bytes(_) | Self::AssetSource(_) => None,
        }
    }
}

impl From<PathBuf> for ImageSource {
    fn from(value: PathBuf) -> Self {
        Self::Path(value)
    }
}

impl From<Vec<u8>> for ImageSource {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl From<&[u8]> for ImageSource {
    fn from(value: &[u8]) -> Self {
        Self::Bytes(value.to_vec())
    }
}

impl From<AssetImageSource> for ImageSource {
    fn from(value: AssetImageSource) -> Self {
        Self::AssetSource(value)
    }
}

impl From<ImageAsset> for ImageSource {
    fn from(value: ImageAsset) -> Self {
        Self::Asset(value)
    }
}

impl From<DataImageSource> for ImageSource {
    fn from(value: DataImageSource) -> Self {
        Self::AssetSource(value.into())
    }
}

impl From<DataUriImageSource> for ImageSource {
    fn from(value: DataUriImageSource) -> Self {
        Self::AssetSource(value.into())
    }
}

impl From<LocalImageSource> for ImageSource {
    fn from(value: LocalImageSource) -> Self {
        Self::AssetSource(value.into())
    }
}

impl From<RemoteImageSource> for ImageSource {
    fn from(value: RemoteImageSource) -> Self {
        Self::AssetSource(value.into())
    }
}

impl From<ImageSource> for AssetImageSource {
    fn from(value: ImageSource) -> Self {
        match value {
            ImageSource::Path(path) => LocalImageSource::new(path).into(),
            ImageSource::Bytes(bytes) => bytes.into(),
            ImageSource::AssetSource(source) => source,
            ImageSource::Asset(asset) => asset_image_source(&asset),
        }
    }
}

fn asset_image_source(asset: &ImageAsset) -> AssetImageSource {
    match asset {
        ImageAsset::Raster(image) => {
            DataImageSource::new(image.data.clone(), image.format).into()
        }
        ImageAsset::Svg(image) => {
            DataImageSource::new(image.raw_data.clone(), ImageFormat::Svg).into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_legacy_and_crate_image_sources_into_asset_sources() {
        let path = PathBuf::from("/tmp/example.png");
        let legacy = ImageSource::Path(path.clone());
        let asset = legacy.as_asset_source();

        assert_eq!(asset, AssetImageSource::from(LocalImageSource::new(path)));

        let remote = RemoteImageSource::new("https://example.com/image.png");
        let wrapped = ImageSource::from(remote.clone());

        assert_eq!(wrapped.as_asset_source(), AssetImageSource::from(remote));
    }

    #[test]
    fn converts_shared_image_assets_into_asset_sources() {
        let asset = ImageAsset::Raster(graphitepdf_image::RasterImage {
            width: 2,
            height: 3,
            data: vec![1, 2, 3, 4],
            format: ImageFormat::Png,
            key: Some(String::from("logo")),
        });
        let wrapped = ImageSource::from(asset.clone());

        assert_eq!(wrapped.as_asset(), Some(&asset));
        assert_eq!(
            wrapped.as_asset_source(),
            AssetImageSource::from(DataImageSource::new(vec![1, 2, 3, 4], ImageFormat::Png))
        );
    }

    #[test]
    fn builds_nodes_from_stylesheets() {
        let container = StylesheetContainer::new(200.0, 300.0);
        let stylesheet = Stylesheet::new(StyleValue::Object(
            [("fontFamily".to_string(), "Inter".into())]
                .into_iter()
                .collect(),
        ));

        let node = Node::from_stylesheet(
            NodeKind::Text(TextNode::new("Hello")),
            &container,
            &stylesheet,
        );

        assert_eq!(node.style().font_family.as_deref(), Some("Inter"));
        assert_eq!(
            LayoutNode::from(&node).style().font_family.as_deref(),
            Some("Inter")
        );
    }

    #[test]
    fn preserves_legacy_default_height_for_unresolved_image_sources() {
        let node = Node::new(
            NodeKind::Image(ImageNode::new(RemoteImageSource::new(
                "https://example.com/image.png",
            ))),
            Style::default(),
        );

        let layout_node = node.to_layout_node();

        assert_eq!(layout_node.style().height, Some(Pt::new(120.0)));
    }

    #[test]
    fn converts_compat_document_into_split_layout_document() {
        let document = Document::new()
            .set_metadata(PdfMetadata {
                title: Some(String::from("Compat")),
                ..PdfMetadata::default()
            })
            .add_page(Node::new(
                NodeKind::View {
                    children: vec![Node::new(
                        NodeKind::Text(TextNode::new("Hello facade")),
                        Style::default(),
                    )],
                },
                Style::default(),
            ));

        let layout_document = document.to_layout_document();

        assert_eq!(layout_document.metadata().title.as_deref(), Some("Compat"));
        assert_eq!(layout_document.pages().len(), 1);
        assert_eq!(layout_document.pages()[0].nodes().len(), 1);
    }
}
