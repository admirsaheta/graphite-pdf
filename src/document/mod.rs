use crate::style::{Style, Stylesheet, StylesheetContainer};
use graphitepdf_image::{
    DataImageSource, DataUriImageSource, Image as ImageAsset, ImageFormat,
    ImageSource as AssetImageSource, LocalImageSource, RemoteImageSource,
};
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

impl TextNode {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImageNode {
    pub source: ImageSource,
}

impl ImageNode {
    pub fn new(source: impl Into<ImageSource>) -> Self {
        Self {
            source: source.into(),
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
    use crate::style::StyleValue;

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
    }
}
