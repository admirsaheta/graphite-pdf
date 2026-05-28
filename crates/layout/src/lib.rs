pub mod error;

pub use error::*;

use graphitepdf_font::{
    FontDescriptor, FontSource, FontStore, FontStyle, FontWeight, StandardFont,
};
use graphitepdf_image::{Image, ImageSource as AssetImageSource};
use graphitepdf_math::{MathOptions, render_math_with_options};
use graphitepdf_primitives::{Bounds, Color, Pt, Size};
use graphitepdf_stylesheet::{
    Container as StylesheetContainer, Style as StylesheetMap, StyleValue, Stylesheet,
};
use graphitepdf_svg::SvgNode;
use graphitepdf_textkit::{
    TextAttributes, TextBlock, TextContainer, TextEngine, TextEngineConfig, TextLayout, TextRect,
};

const DEFAULT_PAGE_WIDTH: f32 = 612.0;
const DEFAULT_PAGE_HEIGHT: f32 = 792.0;

pub const ORDERED_PIPELINE: [LayoutPipelineStep; 10] = [
    LayoutPipelineStep::PageSizing,
    LayoutPipelineStep::Styles,
    LayoutPipelineStep::Inheritance,
    LayoutPipelineStep::Assets,
    LayoutPipelineStep::TextLayout,
    LayoutPipelineStep::SvgResolution,
    LayoutPipelineStep::Dimensions,
    LayoutPipelineStep::Pagination,
    LayoutPipelineStep::Origins,
    LayoutPipelineStep::ZIndex,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutPipelineStep {
    PageSizing,
    Styles,
    Inheritance,
    Assets,
    TextLayout,
    SvgResolution,
    Dimensions,
    Pagination,
    Origins,
    ZIndex,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayoutMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Vec<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EdgeInsets {
    pub top: Pt,
    pub right: Pt,
    pub bottom: Pt,
    pub left: Pt,
}

impl EdgeInsets {
    pub const fn new(top: Pt, right: Pt, bottom: Pt, left: Pt) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub const fn all(value: Pt) -> Self {
        Self::new(value, value, value, value)
    }

    pub const fn horizontal(self) -> f32 {
        self.left.value() + self.right.value()
    }

    pub const fn vertical(self) -> f32 {
        self.top.value() + self.bottom.value()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayoutStyle {
    pub width: Option<Pt>,
    pub height: Option<Pt>,
    pub margin: Option<EdgeInsets>,
    pub padding: Option<EdgeInsets>,
    pub background_color: Option<Color>,
    pub color: Option<Color>,
    pub font_family: Option<String>,
    pub font_style: Option<FontStyle>,
    pub font_weight: Option<FontWeight>,
    pub font_source: Option<FontSource>,
    pub font_size: Option<Pt>,
    pub line_height: Option<Pt>,
    pub z_index: Option<i32>,
    pub page_break_before: Option<bool>,
    pub page_break_after: Option<bool>,
}

impl LayoutStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_width(mut self, width: Pt) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: Pt) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_margin(mut self, margin: EdgeInsets) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn with_padding(mut self, padding: EdgeInsets) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_font_family(mut self, family: impl Into<String>) -> Self {
        self.font_family = Some(family.into());
        self
    }

    pub fn with_font_style(mut self, style: FontStyle) -> Self {
        self.font_style = Some(style);
        self
    }

    pub fn with_font_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = Some(weight);
        self
    }

    pub fn with_font_source(mut self, source: FontSource) -> Self {
        self.font_source = Some(source);
        self
    }

    pub fn with_font_size(mut self, font_size: Pt) -> Self {
        self.font_size = Some(font_size);
        self
    }

    pub fn with_line_height(mut self, line_height: Pt) -> Self {
        self.line_height = Some(line_height);
        self
    }

    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = Some(z_index);
        self
    }

    pub fn with_page_break_before(mut self, value: bool) -> Self {
        self.page_break_before = Some(value);
        self
    }

    pub fn with_page_break_after(mut self, value: bool) -> Self {
        self.page_break_after = Some(value);
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Document {
    metadata: LayoutMetadata,
    pages: Vec<Page>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            metadata: LayoutMetadata::default(),
            pages: Vec::new(),
        }
    }

    pub fn with_metadata(mut self, metadata: LayoutMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn metadata(&self) -> &LayoutMetadata {
        &self.metadata
    }

    pub fn with_page(mut self, page: Page) -> Self {
        self.pages.push(page);
        self
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Page {
    size: Option<Size>,
    style: LayoutStyle,
    stylesheet: Option<Stylesheet>,
    nodes: Vec<Node>,
}

impl Page {
    pub fn new(nodes: impl IntoIterator<Item = Node>) -> Self {
        Self {
            size: None,
            style: LayoutStyle::default(),
            stylesheet: None,
            nodes: nodes.into_iter().collect(),
        }
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_style(mut self, style: LayoutStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_stylesheet(mut self, stylesheet: Stylesheet) -> Self {
        self.stylesheet = Some(stylesheet);
        self
    }

    pub fn with_node(mut self, node: Node) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn size(&self) -> Option<Size> {
        self.size
    }

    pub fn style(&self) -> &LayoutStyle {
        &self.style
    }

    pub fn stylesheet(&self) -> Option<&Stylesheet> {
        self.stylesheet.as_ref()
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MathFragment {
    pub source: String,
    pub options: MathOptions,
}

impl MathFragment {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            options: MathOptions::default(),
        }
    }

    pub fn with_options(mut self, options: MathOptions) -> Self {
        self.options = options;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeKind {
    View,
    Box,
    Text(TextBlock),
    ImageAsset(Image),
    ImageSource(AssetImageSource),
    Svg(SvgNode),
    Math(MathFragment),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    kind: NodeKind,
    style: LayoutStyle,
    stylesheet: Option<Stylesheet>,
    children: Vec<Node>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind,
            style: LayoutStyle::default(),
            stylesheet: None,
            children: Vec::new(),
        }
    }

    pub fn view(children: impl IntoIterator<Item = Node>) -> Self {
        Self {
            kind: NodeKind::View,
            style: LayoutStyle::default(),
            stylesheet: None,
            children: children.into_iter().collect(),
        }
    }

    pub fn box_node() -> Self {
        Self::new(NodeKind::Box)
    }

    pub fn text(block: TextBlock) -> Self {
        Self::new(NodeKind::Text(block))
    }

    pub fn image_asset(asset: Image) -> Self {
        Self::new(NodeKind::ImageAsset(asset))
    }

    pub fn image_source(source: impl Into<AssetImageSource>) -> Self {
        Self::new(NodeKind::ImageSource(source.into()))
    }

    pub fn svg(svg: SvgNode) -> Self {
        Self::new(NodeKind::Svg(svg))
    }

    pub fn math(source: impl Into<String>) -> Self {
        Self::new(NodeKind::Math(MathFragment::new(source)))
    }

    pub fn math_with_options(source: impl Into<String>, options: MathOptions) -> Self {
        Self::new(NodeKind::Math(MathFragment {
            source: source.into(),
            options,
        }))
    }

    pub fn with_style(mut self, style: LayoutStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_stylesheet(mut self, stylesheet: Stylesheet) -> Self {
        self.stylesheet = Some(stylesheet);
        self
    }

    pub fn with_child(mut self, child: Node) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_children(mut self, children: impl IntoIterator<Item = Node>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }

    pub fn style(&self) -> &LayoutStyle {
        &self.style
    }

    pub fn stylesheet(&self) -> Option<&Stylesheet> {
        self.stylesheet.as_ref()
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SafeFont {
    pub descriptor: FontDescriptor,
    pub source: Option<FontSource>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SafeLayoutStyle {
    pub width: Option<Pt>,
    pub height: Option<Pt>,
    pub margin: EdgeInsets,
    pub padding: EdgeInsets,
    pub background_color: Option<Color>,
    pub color: Color,
    pub font: SafeFont,
    pub font_size: Pt,
    pub line_height: Pt,
    pub z_index: i32,
    pub page_break_before: bool,
    pub page_break_after: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SafeNodeKind {
    View,
    Box,
    Text { block: TextBlock, layout: TextLayout },
    ImageAsset { asset: Image },
    ImageSource { source: AssetImageSource },
    Svg { svg: SvgNode },
    Math { source: String, svg: SvgNode },
}

#[derive(Clone, Debug, PartialEq)]
pub struct SafeLayoutNode {
    pub frame: Bounds,
    pub content_frame: Bounds,
    pub style: SafeLayoutStyle,
    pub kind: SafeNodeKind,
    pub children: Vec<SafeLayoutNode>,
    pub page_index: usize,
}

impl SafeLayoutNode {
    pub fn font_descriptor(&self) -> Option<&FontDescriptor> {
        match &self.kind {
            SafeNodeKind::Text { .. } => Some(&self.style.font.descriptor),
            SafeNodeKind::View
            | SafeNodeKind::Box
            | SafeNodeKind::ImageAsset { .. }
            | SafeNodeKind::ImageSource { .. }
            | SafeNodeKind::Svg { .. }
            | SafeNodeKind::Math { .. } => None,
        }
    }

    pub fn children(&self) -> &[SafeLayoutNode] {
        &self.children
    }

    pub fn text_layout(&self) -> Option<&TextLayout> {
        match &self.kind {
            SafeNodeKind::Text { layout, .. } => Some(layout),
            SafeNodeKind::View
            | SafeNodeKind::Box
            | SafeNodeKind::ImageAsset { .. }
            | SafeNodeKind::ImageSource { .. }
            | SafeNodeKind::Svg { .. }
            | SafeNodeKind::Math { .. } => None,
        }
    }

    pub fn z_index(&self) -> i32 {
        self.style.z_index
    }

    pub fn style(&self) -> &SafeLayoutStyle {
        &self.style
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SafeLayoutPage {
    pub size: Size,
    pub style: SafeLayoutStyle,
    pub nodes: Vec<SafeLayoutNode>,
    pub source_page_index: usize,
}

impl SafeLayoutPage {
    pub fn nodes(&self) -> &[SafeLayoutNode] {
        &self.nodes
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SafeLayoutDocument {
    pub metadata: LayoutMetadata,
    pub pages: Vec<SafeLayoutPage>,
    pub pipeline: Vec<LayoutPipelineStep>,
}

impl SafeLayoutDocument {
    pub fn pages(&self) -> &[SafeLayoutPage] {
        &self.pages
    }

    pub fn pipeline(&self) -> &[LayoutPipelineStep] {
        &self.pipeline
    }
}

pub struct LayoutEngine {
    text_engine: TextEngine,
    font_store: FontStore,
    default_page_size: Size,
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            text_engine: TextEngine::default(),
            font_store: FontStore::default(),
            default_page_size: Size::new(DEFAULT_PAGE_WIDTH, DEFAULT_PAGE_HEIGHT),
        }
    }

    pub fn with_text_engine_config(mut self, config: TextEngineConfig) -> Self {
        self.text_engine = TextEngine::new(config);
        self
    }

    pub fn with_default_page_size(mut self, size: Size) -> Result<Self> {
        validate_page_size(size)?;
        self.default_page_size = size;
        Ok(self)
    }

    pub fn text_engine(&self) -> &TextEngine {
        &self.text_engine
    }

    pub fn font_store(&self) -> &FontStore {
        &self.font_store
    }

    pub fn font_store_mut(&mut self) -> &mut FontStore {
        &mut self.font_store
    }

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

    pub fn layout_document(&self, document: &Document) -> Result<SafeLayoutDocument> {
        if document.pages().is_empty() {
            return Err(Error::EmptyDocument);
        }

        let mut pages = Vec::new();
        for (source_page_index, page) in document.pages().iter().enumerate() {
            pages.extend(self.layout_page(page, source_page_index)?);
        }

        Ok(SafeLayoutDocument {
            metadata: document.metadata().clone(),
            pages,
            pipeline: ORDERED_PIPELINE.to_vec(),
        })
    }

    fn layout_page(&self, page: &Page, source_page_index: usize) -> Result<Vec<SafeLayoutPage>> {
        let size = self.resolve_page_size(page)?;
        let page_container = stylesheet_container(size);
        let page_seed = resolve_style_seed(&page.style, page.stylesheet.as_ref(), &page_container);
        let page_style = SafeLayoutStyle::from_seed(page_seed, None);

        let available_width = (size.width - page_style.padding.horizontal()).max(0.0);
        let available_height = (size.height - page_style.padding.vertical()).max(0.0);
        let child_container = StylesheetContainer::new(available_width as f64, available_height as f64);

        let mut measured = Vec::with_capacity(page.nodes().len());
        for node in page.nodes() {
            measured.push(self.measure_node(
                node,
                &page_style,
                &child_container,
                available_width,
                available_height,
            )?);
        }

        self.paginate_page(size, page_style, measured, source_page_index)
    }

    fn resolve_page_size(&self, page: &Page) -> Result<Size> {
        let base_container = stylesheet_container(self.default_page_size);
        let page_seed = resolve_style_seed(&page.style, page.stylesheet.as_ref(), &base_container);

        let width = page
            .size()
            .map(|size| size.width)
            .or_else(|| page_seed.width.map(Pt::value))
            .unwrap_or(self.default_page_size.width);
        let height = page
            .size()
            .map(|size| size.height)
            .or_else(|| page_seed.height.map(Pt::value))
            .unwrap_or(self.default_page_size.height);

        let size = Size::new(width, height);
        validate_page_size(size)?;
        Ok(size)
    }

    fn measure_node(
        &self,
        node: &Node,
        parent_style: &SafeLayoutStyle,
        container: &StylesheetContainer,
        available_width: f32,
        available_height: f32,
    ) -> Result<MeasuredNode> {
        let seed = resolve_style_seed(node.style(), node.stylesheet(), container);
        let style = SafeLayoutStyle::from_seed(seed, Some(parent_style));

        let width = style.width.map(Pt::value).unwrap_or_else(|| {
            (available_width - style.margin.left.value() - style.margin.right.value()).max(0.0)
        });

        let measured = match node.kind() {
            NodeKind::View => self.measure_view(node, style, width, available_height)?,
            NodeKind::Box => self.measure_box(style, width),
            NodeKind::Text(block) => self.measure_text(block, style, width, available_height)?,
            NodeKind::ImageAsset(asset) => self.measure_image_asset(asset.clone(), style, width)?,
            NodeKind::ImageSource(source) => self.measure_image_source(source.clone(), style, width)?,
            NodeKind::Svg(svg) => self.measure_svg(svg.clone(), style, width)?,
            NodeKind::Math(fragment) => self.measure_math(fragment, style, width)?,
        };

        Ok(measured)
    }

    fn measure_view(
        &self,
        node: &Node,
        style: SafeLayoutStyle,
        width: f32,
        available_height: f32,
    ) -> Result<MeasuredNode> {
        let child_available_width = (width - style.padding.horizontal()).max(0.0);
        let child_available_height = style
            .height
            .map(Pt::value)
            .unwrap_or(available_height)
            .max(style.line_height.value());
        let child_container =
            StylesheetContainer::new(child_available_width as f64, child_available_height as f64);

        let mut children = Vec::with_capacity(node.children().len());
        for child in node.children() {
            children.push(self.measure_node(
                child,
                &style,
                &child_container,
                child_available_width,
                child_available_height,
            )?);
        }

        let content_height = children.iter().map(MeasuredNode::outer_height).sum::<f32>();
        let height = style
            .height
            .map(Pt::value)
            .unwrap_or(content_height + style.padding.vertical());

        Ok(MeasuredNode {
            kind: SafeNodeKind::View,
            style,
            size: Size::new(width, height.max(0.0)),
            children,
        })
    }

    fn measure_box(&self, style: SafeLayoutStyle, width: f32) -> MeasuredNode {
        let height = style.height.map(Pt::value).unwrap_or(0.0);

        MeasuredNode {
            kind: SafeNodeKind::Box,
            style,
            size: Size::new(width, height),
            children: Vec::new(),
        }
    }

    fn measure_text(
        &self,
        block: &TextBlock,
        style: SafeLayoutStyle,
        width: f32,
        available_height: f32,
    ) -> Result<MeasuredNode> {
        let attributes = TextAttributes::default()
            .with_font(style.font.descriptor.clone())
            .with_font_size(style.font_size)?;
        let attributed = block
            .to_attributed_string()?
            .with_default_attributes(attributes)?;
        let container_height = style
            .height
            .map(Pt::value)
            .unwrap_or_else(|| available_height.max(style.line_height.value()));
        let container = TextContainer::new(TextRect::from_values(
            0.0,
            0.0,
            width.max(style.line_height.value()).max(1.0),
            container_height.max(style.line_height.value()).max(1.0),
        ))?;
        let layout = self
            .text_engine
            .layout(&attributed, &container, Some(&self.font_store))?;
        let height = style
            .height
            .map(Pt::value)
            .unwrap_or_else(|| layout.bounds().height.value());
        let line_height = style.line_height.value();

        Ok(MeasuredNode {
            kind: SafeNodeKind::Text {
                block: block.clone(),
                layout,
            },
            style,
            size: Size::new(width, height.max(line_height)),
            children: Vec::new(),
        })
    }

    fn measure_image_asset(
        &self,
        asset: Image,
        style: SafeLayoutStyle,
        width: f32,
    ) -> Result<MeasuredNode> {
        let size = resolve_replaced_size(
            Size::new(asset.width(), asset.height()),
            style.width.map(Pt::value).unwrap_or(width),
            style.width.map(Pt::value),
            style.height.map(Pt::value),
            "image asset",
        )?;

        Ok(MeasuredNode {
            kind: SafeNodeKind::ImageAsset { asset },
            style,
            size,
            children: Vec::new(),
        })
    }

    fn measure_image_source(
        &self,
        source: AssetImageSource,
        style: SafeLayoutStyle,
        width: f32,
    ) -> Result<MeasuredNode> {
        let resolved_width = style.width.map(Pt::value).unwrap_or(width);
        let resolved_height = style.height.map(Pt::value).ok_or(Error::UnresolvedAssetDimensions {
            kind: "image source",
        })?;

        Ok(MeasuredNode {
            kind: SafeNodeKind::ImageSource { source },
            style,
            size: Size::new(resolved_width, resolved_height),
            children: Vec::new(),
        })
    }

    fn measure_svg(&self, svg: SvgNode, style: SafeLayoutStyle, width: f32) -> Result<MeasuredNode> {
        let natural = resolve_svg_size(&svg)?;
        let size = resolve_replaced_size(
            natural,
            style.width.map(Pt::value).unwrap_or(width),
            style.width.map(Pt::value),
            style.height.map(Pt::value),
            "svg",
        )?;

        Ok(MeasuredNode {
            kind: SafeNodeKind::Svg { svg },
            style,
            size,
            children: Vec::new(),
        })
    }

    fn measure_math(
        &self,
        fragment: &MathFragment,
        style: SafeLayoutStyle,
        width: f32,
    ) -> Result<MeasuredNode> {
        let rendered = render_math_with_options(&fragment.source, &fragment.options)?;
        let natural = resolve_svg_size(&rendered.svg)?;
        let size = resolve_replaced_size(
            natural,
            style.width.map(Pt::value).unwrap_or(width),
            style.width.map(Pt::value),
            style.height.map(Pt::value),
            "math",
        )?;

        Ok(MeasuredNode {
            kind: SafeNodeKind::Math {
                source: fragment.source.clone(),
                svg: rendered.svg,
            },
            style,
            size,
            children: Vec::new(),
        })
    }

    fn paginate_page(
        &self,
        size: Size,
        page_style: SafeLayoutStyle,
        measured: Vec<MeasuredNode>,
        source_page_index: usize,
    ) -> Result<Vec<SafeLayoutPage>> {
        let page_top = page_style.padding.top.value();
        let page_left = page_style.padding.left.value();
        let page_bottom = size.height - page_style.padding.bottom.value();

        let mut chunked = Vec::<Vec<MeasuredNode>>::new();
        let mut current = Vec::<MeasuredNode>::new();
        let mut cursor_y = page_top;

        for node in measured {
            if node.style.page_break_before && !current.is_empty() {
                chunked.push(std::mem::take(&mut current));
                cursor_y = page_top;
            }

            let outer_height = node.outer_height();
            let next_bottom = cursor_y + outer_height;
            let overflows = next_bottom > page_bottom;
            if overflows && !current.is_empty() {
                chunked.push(std::mem::take(&mut current));
                cursor_y = page_top;
            }

            cursor_y += outer_height;
            let break_after = node.style.page_break_after;
            current.push(node);

            if break_after {
                chunked.push(std::mem::take(&mut current));
                cursor_y = page_top;
            }
        }

        if !current.is_empty() {
            chunked.push(current);
        }

        if chunked.is_empty() {
            chunked.push(Vec::new());
        }

        let mut pages = Vec::with_capacity(chunked.len());
        for (page_index, nodes) in chunked.into_iter().enumerate() {
            let positioned = position_nodes(
                nodes,
                page_left,
                page_top,
                size.width - page_style.padding.horizontal(),
                page_index,
            );
            pages.push(SafeLayoutPage {
                size,
                style: page_style.clone(),
                nodes: sort_by_z_index(positioned),
                source_page_index,
            });
        }

        Ok(pages)
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

#[derive(Clone, Debug)]
struct MeasuredNode {
    kind: SafeNodeKind,
    style: SafeLayoutStyle,
    size: Size,
    children: Vec<MeasuredNode>,
}

impl MeasuredNode {
    fn outer_height(&self) -> f32 {
        self.style.margin.top.value() + self.size.height + self.style.margin.bottom.value()
    }
}

#[derive(Clone, Debug, Default)]
struct StyleSeed {
    width: Option<Pt>,
    height: Option<Pt>,
    margin: Option<EdgeInsets>,
    padding: Option<EdgeInsets>,
    background_color: Option<Color>,
    color: Option<Color>,
    font_family: Option<String>,
    font_style: Option<FontStyle>,
    font_weight: Option<FontWeight>,
    font_source: Option<FontSource>,
    font_size: Option<Pt>,
    line_height: Option<Pt>,
    z_index: Option<i32>,
    page_break_before: Option<bool>,
    page_break_after: Option<bool>,
}

impl SafeLayoutStyle {
    fn from_seed(seed: StyleSeed, parent: Option<&SafeLayoutStyle>) -> Self {
        let fallback_descriptor = parent
            .map(|style| style.font.descriptor.clone())
            .unwrap_or_else(|| FontDescriptor::new(StandardFont::Helvetica.family_name()));
        let family = seed
            .font_family
            .clone()
            .or_else(|| seed.font_source.as_ref().and_then(font_source_family))
            .unwrap_or_else(|| fallback_descriptor.family().to_string());
        let font_style = seed
            .font_style
            .or_else(|| parent.map(|style| style.font.descriptor.font_style()))
            .unwrap_or_else(|| fallback_descriptor.font_style());
        let font_weight = seed
            .font_weight
            .or_else(|| parent.map(|style| style.font.descriptor.font_weight()))
            .unwrap_or_else(|| fallback_descriptor.font_weight());
        let mut descriptor = FontDescriptor::new(family).with_style(font_style);
        descriptor = descriptor.with_weight(font_weight);

        let font_size = seed
            .font_size
            .or_else(|| parent.map(|style| style.font_size))
            .unwrap_or(Pt::new(12.0));
        let line_height = seed
            .line_height
            .or_else(|| parent.map(|style| style.line_height))
            .unwrap_or_else(|| Pt::new(font_size.value() * 1.2));

        Self {
            width: seed.width,
            height: seed.height,
            margin: seed.margin.unwrap_or_default(),
            padding: seed.padding.unwrap_or_default(),
            background_color: seed.background_color,
            color: seed
                .color
                .or_else(|| parent.map(|style| style.color))
                .unwrap_or(Color::BLACK),
            font: SafeFont {
                descriptor,
                source: seed
                    .font_source
                    .or_else(|| parent.and_then(|style| style.font.source.clone())),
            },
            font_size,
            line_height,
            z_index: seed.z_index.unwrap_or(0),
            page_break_before: seed.page_break_before.unwrap_or(false),
            page_break_after: seed.page_break_after.unwrap_or(false),
        }
    }
}

fn resolve_style_seed(
    input: &LayoutStyle,
    stylesheet: Option<&Stylesheet>,
    container: &StylesheetContainer,
) -> StyleSeed {
    let mut seed = StyleSeed::default();
    if let Some(stylesheet) = stylesheet {
        apply_resolved_stylesheet(&mut seed, &stylesheet.resolve(container));
    }
    apply_input_style(&mut seed, input);
    seed
}

fn apply_input_style(seed: &mut StyleSeed, input: &LayoutStyle) {
    if let Some(value) = input.width {
        seed.width = Some(value);
    }
    if let Some(value) = input.height {
        seed.height = Some(value);
    }
    if let Some(value) = input.margin {
        seed.margin = Some(value);
    }
    if let Some(value) = input.padding {
        seed.padding = Some(value);
    }
    if let Some(value) = input.background_color {
        seed.background_color = Some(value);
    }
    if let Some(value) = input.color {
        seed.color = Some(value);
    }
    if let Some(value) = &input.font_family {
        seed.font_family = Some(value.clone());
    }
    if let Some(value) = input.font_style {
        seed.font_style = Some(value);
    }
    if let Some(value) = input.font_weight {
        seed.font_weight = Some(value);
    }
    if let Some(value) = &input.font_source {
        seed.font_source = Some(value.clone());
    }
    if let Some(value) = input.font_size {
        seed.font_size = Some(value);
    }
    if let Some(value) = input.line_height {
        seed.line_height = Some(value);
    }
    if let Some(value) = input.z_index {
        seed.z_index = Some(value);
    }
    if let Some(value) = input.page_break_before {
        seed.page_break_before = Some(value);
    }
    if let Some(value) = input.page_break_after {
        seed.page_break_after = Some(value);
    }
}

fn apply_resolved_stylesheet(seed: &mut StyleSeed, style: &StylesheetMap) {
    if let Some(value) = stylesheet_pt(style, "width") {
        seed.width = Some(value);
    }
    if let Some(value) = stylesheet_pt(style, "height") {
        seed.height = Some(value);
    }
    if let Some(value) = stylesheet_color(style, "backgroundColor") {
        seed.background_color = Some(value);
    }
    if let Some(value) = stylesheet_color(style, "color") {
        seed.color = Some(value);
    }
    if let Some(value) = stylesheet_string(style, "fontFamily") {
        seed.font_family = Some(value.to_string());
    }
    if let Some(value) = stylesheet_font_style(style, "fontStyle") {
        seed.font_style = Some(value);
    }
    if let Some(value) = stylesheet_font_weight(style, "fontWeight") {
        seed.font_weight = Some(value);
    }
    if let Some(value) = stylesheet_string(style, "fontSource") {
        seed.font_source = Some(FontSource::remote(value));
    }
    if let Some(value) = stylesheet_string(style, "fontSourceLocal") {
        seed.font_source = Some(FontSource::local(value));
    }
    if let Some(value) = stylesheet_string(style, "fontSourceDataUri") {
        seed.font_source = Some(FontSource::data_uri(value));
    }
    if let Some(value) = stylesheet_standard_font(style, "fontSourceStandard") {
        seed.font_source = Some(FontSource::standard(value));
    }
    if let Some(value) = stylesheet_pt(style, "fontSize") {
        seed.font_size = Some(value);
    }
    if let Some(value) = stylesheet_pt(style, "lineHeight") {
        seed.line_height = Some(value);
    }
    if let Some(value) = stylesheet_i32(style, "zIndex") {
        seed.z_index = Some(value);
    }
    if let Some(value) = stylesheet_bool(style, "pageBreakBefore") {
        seed.page_break_before = Some(value);
    }
    if let Some(value) = stylesheet_bool(style, "pageBreakAfter") {
        seed.page_break_after = Some(value);
    }

    apply_edge_insets(
        &mut seed.margin,
        style,
        ["marginTop", "marginRight", "marginBottom", "marginLeft"],
    );
    apply_edge_insets(
        &mut seed.padding,
        style,
        ["paddingTop", "paddingRight", "paddingBottom", "paddingLeft"],
    );
}

fn apply_edge_insets(target: &mut Option<EdgeInsets>, style: &StylesheetMap, keys: [&str; 4]) {
    let mut value = target.unwrap_or_default();
    let mut changed = false;

    if let Some(edge) = stylesheet_pt(style, keys[0]) {
        value.top = edge;
        changed = true;
    }
    if let Some(edge) = stylesheet_pt(style, keys[1]) {
        value.right = edge;
        changed = true;
    }
    if let Some(edge) = stylesheet_pt(style, keys[2]) {
        value.bottom = edge;
        changed = true;
    }
    if let Some(edge) = stylesheet_pt(style, keys[3]) {
        value.left = edge;
        changed = true;
    }

    if changed {
        *target = Some(value);
    }
}

fn stylesheet_pt(style: &StylesheetMap, key: &str) -> Option<Pt> {
    stylesheet_f32(style, key).map(Pt::new)
}

fn stylesheet_f32(style: &StylesheetMap, key: &str) -> Option<f32> {
    match style.get(key)? {
        StyleValue::Number(value) => Some(*value as f32),
        StyleValue::String(value) => value.trim().parse::<f32>().ok(),
        _ => None,
    }
}

fn stylesheet_i32(style: &StylesheetMap, key: &str) -> Option<i32> {
    match style.get(key)? {
        StyleValue::Number(value) => Some(*value as i32),
        StyleValue::String(value) => value.trim().parse::<i32>().ok(),
        _ => None,
    }
}

fn stylesheet_bool(style: &StylesheetMap, key: &str) -> Option<bool> {
    match style.get(key)? {
        StyleValue::Bool(value) => Some(*value),
        StyleValue::String(value) => match value.trim().to_ascii_lowercase().as_str() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

fn stylesheet_string<'a>(style: &'a StylesheetMap, key: &str) -> Option<&'a str> {
    match style.get(key)? {
        StyleValue::String(value) => Some(value.as_str()),
        _ => None,
    }
}

fn stylesheet_color(style: &StylesheetMap, key: &str) -> Option<Color> {
    parse_color(stylesheet_string(style, key)?)
}

fn stylesheet_font_style(style: &StylesheetMap, key: &str) -> Option<FontStyle> {
    match stylesheet_string(style, key)?
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "normal" => Some(FontStyle::Normal),
        "italic" => Some(FontStyle::Italic),
        "oblique" => Some(FontStyle::Oblique),
        _ => None,
    }
}

fn stylesheet_font_weight(style: &StylesheetMap, key: &str) -> Option<FontWeight> {
    let value = match style.get(key)? {
        StyleValue::Number(value) => *value as u16,
        StyleValue::String(value) => value.trim().parse::<u16>().ok()?,
        _ => return None,
    };

    FontWeight::new(value).ok()
}

fn stylesheet_standard_font(style: &StylesheetMap, key: &str) -> Option<StandardFont> {
    match stylesheet_string(style, key)?.trim() {
        "Times-Roman" => Some(StandardFont::TimesRoman),
        "Times-Bold" => Some(StandardFont::TimesBold),
        "Times-Italic" => Some(StandardFont::TimesItalic),
        "Times-BoldItalic" => Some(StandardFont::TimesBoldItalic),
        "Helvetica" => Some(StandardFont::Helvetica),
        "Helvetica-Bold" => Some(StandardFont::HelveticaBold),
        "Helvetica-Oblique" => Some(StandardFont::HelveticaOblique),
        "Helvetica-BoldOblique" => Some(StandardFont::HelveticaBoldOblique),
        "Courier" => Some(StandardFont::Courier),
        "Courier-Bold" => Some(StandardFont::CourierBold),
        "Courier-Oblique" => Some(StandardFont::CourierOblique),
        "Courier-BoldOblique" => Some(StandardFont::CourierBoldOblique),
        "Symbol" => Some(StandardFont::Symbol),
        "ZapfDingbats" => Some(StandardFont::ZapfDingbats),
        _ => None,
    }
}

fn font_source_family(source: &FontSource) -> Option<String> {
    match source {
        FontSource::Standard(font) => Some(font.family_name().to_string()),
        FontSource::Local(_) | FontSource::Remote(_) | FontSource::DataUri(_) => None,
    }
}

fn parse_color(value: &str) -> Option<Color> {
    let trimmed = value.trim();
    match trimmed {
        "black" => return Some(Color::BLACK),
        "white" => return Some(Color::WHITE),
        _ => {}
    }

    let hex = trimmed.strip_prefix('#')?;
    match hex.len() {
        6 => Some(Color::rgb(
            u8::from_str_radix(&hex[0..2], 16).ok()?,
            u8::from_str_radix(&hex[2..4], 16).ok()?,
            u8::from_str_radix(&hex[4..6], 16).ok()?,
        )),
        8 => Some(Color::rgba(
            u8::from_str_radix(&hex[0..2], 16).ok()?,
            u8::from_str_radix(&hex[2..4], 16).ok()?,
            u8::from_str_radix(&hex[4..6], 16).ok()?,
            u8::from_str_radix(&hex[6..8], 16).ok()?,
        )),
        _ => None,
    }
}

fn resolve_replaced_size(
    natural: Size,
    fallback_width: f32,
    width: Option<f32>,
    height: Option<f32>,
    kind: &'static str,
) -> Result<Size> {
    let natural_width = natural.width.abs();
    let natural_height = natural.height.abs();
    let aspect_ratio = if natural_width > 0.0 && natural_height > 0.0 {
        Some(natural_width / natural_height)
    } else {
        None
    };

    match (width, height) {
        (Some(width), Some(height)) => Ok(Size::new(width, height)),
        (Some(width), None) => {
            let ratio = aspect_ratio.ok_or(Error::InvalidNaturalDimensions { kind })?;
            Ok(Size::new(width, width / ratio))
        }
        (None, Some(height)) => {
            let ratio = aspect_ratio.ok_or(Error::InvalidNaturalDimensions { kind })?;
            Ok(Size::new(height * ratio, height))
        }
        (None, None) if natural_width > 0.0 && natural_height > 0.0 => {
            Ok(Size::new(natural_width.min(fallback_width).max(0.0), natural_height))
        }
        (None, None) => Err(Error::InvalidNaturalDimensions { kind }),
    }
}

fn resolve_svg_size(svg: &SvgNode) -> Result<Size> {
    let view_box = svg
        .props
        .get("viewBox")
        .and_then(|value| parse_view_box(value));
    let width = svg
        .props
        .get("width")
        .and_then(|value| parse_numeric_dimension(value).ok())
        .or_else(|| view_box.map(|(_, _, width, _)| width))
        .unwrap_or(0.0);
    let height = svg
        .props
        .get("height")
        .and_then(|value| parse_numeric_dimension(value).ok())
        .or_else(|| view_box.map(|(_, _, _, height)| height))
        .unwrap_or(0.0);

    if width <= 0.0 || height <= 0.0 {
        Err(Error::InvalidSvgDimensions)
    } else {
        Ok(Size::new(width, height))
    }
}

fn parse_view_box(value: &str) -> Option<(f32, f32, f32, f32)> {
    let values: Vec<f32> = value
        .split(|character: char| character.is_ascii_whitespace() || character == ',')
        .filter(|part| !part.is_empty())
        .filter_map(|part| part.parse::<f32>().ok())
        .collect();

    match values.as_slice() {
        [x, y, width, height] if *width > 0.0 && *height > 0.0 => Some((*x, *y, *width, *height)),
        _ => None,
    }
}

fn parse_numeric_dimension(value: &str) -> Result<f32> {
    let trimmed = value.trim();
    let mut end = 0usize;
    let mut has_digit = false;
    let mut has_decimal_point = false;

    for (index, character) in trimmed.char_indices() {
        let is_first = index == 0;
        let is_sign = is_first && (character == '+' || character == '-');

        if character.is_ascii_digit() {
            has_digit = true;
            end = index + character.len_utf8();
            continue;
        }
        if character == '.' && !has_decimal_point {
            has_decimal_point = true;
            end = index + character.len_utf8();
            continue;
        }
        if is_sign {
            end = index + character.len_utf8();
            continue;
        }
        break;
    }

    if !has_digit || end == 0 {
        return Err(Error::InvalidDimension {
            input: value.to_string(),
        });
    }

    let (number, suffix) = trimmed.split_at(end);
    let parsed = number.parse::<f32>().map_err(|_| Error::InvalidDimension {
        input: value.to_string(),
    })?;
    let scaled = match suffix.trim().to_ascii_lowercase().as_str() {
        "" | "px" | "pt" => parsed,
        "in" => parsed * 72.0,
        "cm" => parsed * 72.0 / 2.54,
        "mm" => parsed * 72.0 / 25.4,
        _ => parsed,
    };
    Ok(scaled.abs())
}

fn position_nodes(
    measured: Vec<MeasuredNode>,
    origin_x: f32,
    origin_y: f32,
    available_width: f32,
    page_index: usize,
) -> Vec<SafeLayoutNode> {
    let mut cursor_y = origin_y;
    let mut positioned = Vec::with_capacity(measured.len());

    for node in measured {
        let top_margin = node.style.margin.top.value();
        let x = origin_x + node.style.margin.left.value();
        let y = cursor_y + top_margin;
        let frame = Bounds::from_origin_size(x, y, node.size.width, node.size.height);
        let content_x = x + node.style.padding.left.value();
        let content_y = y + node.style.padding.top.value();
        let content_width = (node.size.width - node.style.padding.horizontal()).max(0.0);
        let children = position_nodes(
            node.children,
            content_x,
            content_y,
            available_width.min(content_width),
            page_index,
        );

        positioned.push(SafeLayoutNode {
            frame,
            content_frame: Bounds::from_origin_size(
                content_x,
                content_y,
                content_width,
                (node.size.height - node.style.padding.vertical()).max(0.0),
            ),
            style: node.style.clone(),
            kind: node.kind,
            children: sort_by_z_index(children),
            page_index,
        });

        cursor_y += top_margin + node.size.height + node.style.margin.bottom.value();
    }

    positioned
}

fn sort_by_z_index(mut nodes: Vec<SafeLayoutNode>) -> Vec<SafeLayoutNode> {
    let mut indexed: Vec<_> = nodes.drain(..).enumerate().collect();
    indexed.sort_by_key(|(index, node)| (node.style.z_index, *index));
    indexed.into_iter().map(|(_, node)| node).collect()
}

fn stylesheet_container(size: Size) -> StylesheetContainer {
    StylesheetContainer::new(size.width as f64, size.height as f64)
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

#[cfg(test)]
mod tests {
    use super::*;
    use graphitepdf_image::{ImageFormat, RasterImage};
    use graphitepdf_svg::parse_svg;

    fn stylesheet(entries: impl IntoIterator<Item = (&'static str, StyleValue)>) -> Stylesheet {
        Stylesheet::new(StyleValue::Object(
            entries
                .into_iter()
                .map(|(key, value)| (key.to_string(), value))
                .collect::<StylesheetMap>(),
        ))
    }

    #[test]
    fn resolves_styles_inheritance_and_text_layout_in_pipeline_order() {
        let block = TextBlock::from(
            graphitepdf_textkit::TextSpan::new("Hello layout pipeline")
                .expect("text span should be valid"),
        );
        let document = Document::new().with_page(
            Page::new([Node::text(block)])
                .with_size(Size::new(220.0, 160.0))
                .with_style(LayoutStyle::new().with_padding(EdgeInsets::all(Pt::new(12.0))))
                .with_stylesheet(stylesheet([
                    ("fontFamily", "Helvetica".into()),
                    ("fontSize", 18.into()),
                    ("color", "#112233".into()),
                ])),
        );

        let layout = LayoutEngine::new()
            .layout_document(&document)
            .expect("document should layout");

        assert_eq!(layout.pipeline(), ORDERED_PIPELINE.as_slice());
        assert_eq!(layout.pages().len(), 1);

        let node = &layout.pages()[0].nodes()[0];
        assert_eq!(node.style().font.descriptor.family(), "Helvetica");
        assert_eq!(node.style().font_size, Pt::new(18.0));
        assert_eq!(node.style().color, Color::rgb(0x11, 0x22, 0x33));
        assert!(node.text_layout().is_some());
        assert!(node.frame.origin.x >= 12.0);
        assert!(node.frame.origin.y >= 12.0);
    }

    #[test]
    fn paginates_top_level_nodes_and_resets_origins() {
        let page = Page::new([
            Node::box_node().with_style(LayoutStyle::new().with_height(Pt::new(60.0))),
            Node::box_node().with_style(LayoutStyle::new().with_height(Pt::new(60.0))),
        ])
        .with_size(Size::new(140.0, 100.0))
        .with_style(LayoutStyle::new().with_padding(EdgeInsets::all(Pt::new(10.0))));

        let layout = LayoutEngine::new()
            .layout_document(&Document::new().with_page(page))
            .expect("layout should paginate");

        assert_eq!(layout.pages().len(), 2);
        assert_eq!(layout.pages()[0].nodes().len(), 1);
        assert_eq!(layout.pages()[1].nodes().len(), 1);
        assert_eq!(layout.pages()[0].nodes()[0].frame.origin.y, 10.0);
        assert_eq!(layout.pages()[1].nodes()[0].frame.origin.y, 10.0);
    }

    #[test]
    fn resolves_svg_math_and_z_index_order() {
        let svg = parse_svg(r#"<svg viewBox="0 0 20 10"><rect width="20" height="10"/></svg>"#);
        let page = Page::new([
            Node::svg(svg.clone()).with_style(LayoutStyle::new().with_z_index(5)),
            Node::math("x^2 + y^2")
                .with_style(LayoutStyle::new().with_width(Pt::new(60.0)).with_z_index(1)),
        ])
        .with_size(Size::new(200.0, 200.0));

        let layout = LayoutEngine::new()
            .layout_document(&Document::new().with_page(page))
            .expect("layout should resolve math and svg");

        let nodes = layout.pages()[0].nodes();
        assert_eq!(nodes.len(), 2);
        assert!(matches!(nodes[0].kind, SafeNodeKind::Math { .. }));
        assert!(matches!(nodes[1].kind, SafeNodeKind::Svg { .. }));
        assert_eq!(nodes[1].frame.size.width, 20.0);
        assert_eq!(nodes[1].frame.size.height, 10.0);
        assert_eq!(nodes[0].frame.size.width, 60.0);
        assert!(nodes[0].frame.size.height > 0.0);
    }

    #[test]
    fn resolves_image_asset_dimensions_from_intrinsic_size() {
        let image = Image::Raster(RasterImage {
            width: 200,
            height: 100,
            data: vec![1, 2, 3, 4],
            format: ImageFormat::Png,
            key: None,
        });
        let page = Page::new([Node::image_asset(image).with_style(
            LayoutStyle::new().with_width(Pt::new(50.0)),
        )])
        .with_size(Size::new(200.0, 200.0));

        let layout = LayoutEngine::new()
            .layout_document(&Document::new().with_page(page))
            .expect("layout should resolve image asset");

        let node = &layout.pages()[0].nodes()[0];
        assert_eq!(node.frame.size.width, 50.0);
        assert_eq!(node.frame.size.height, 25.0);
    }
}
