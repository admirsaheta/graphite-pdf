pub mod error;

pub use error::*;

use std::{collections::HashMap, fmt::Write as _, io::Write, path::Path, sync::Arc};

use graphitepdf_font::{FontDescriptor, FontSource, StandardFont};
use graphitepdf_image::{Image, ImageSource as AssetImageSource, resolve_image};
use graphitepdf_kit::{
    Font as PdfFont, ImageRenderOptions, Metadata as PdfMetadata, PdfWriter, SvgRenderOptions,
    render_image_to_page_content_with_options, render_svg_node_to_page_content_with_options,
};
use graphitepdf_layout::{
    Document as SourceLayoutDocument, EdgeInsets, LayoutContent,
    LayoutDocument as LegacyLayoutDocument, LayoutEngine, LayoutMetadata,
    LayoutNode as LegacyLayoutNode, SafeFont, SafeLayoutDocument, SafeLayoutNode, SafeLayoutPage,
    SafeLayoutStyle, SafeNodeKind,
};
use graphitepdf_primitives::{Bounds, Color, Pt, Size};
use graphitepdf_svg::SvgNode;
use graphitepdf_textkit::{TextBlock, TextLayout};
use graphitepdf_utils::{match_percent, parse_float};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderDocument {
    pub metadata: LayoutMetadata,
    pub pages: Vec<RenderPage>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderPage {
    pub size: Size,
    pub source_page_index: usize,
    pub commands: Vec<RenderCommand>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RenderCommand {
    FillRect(FillRectOp),
    StrokeBorder(BorderRenderOp),
    DrawBox(BoxRenderOp),
    DrawText(TextRenderOp),
    DrawImage(ImageRenderOp),
    DrawSvg(SvgRenderOp),
    PushTransform(TransformRenderOp),
    PopTransform(RenderContext),
    DrawDebug(DebugRenderOp),
    DrawForm(FormRenderOp),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FillRectOp {
    pub context: RenderContext,
    pub bounds: Bounds,
    pub color: Color,
    pub role: PaintRole,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BorderRenderOp {
    pub context: RenderContext,
    pub bounds: Bounds,
    pub side: BorderSidePosition,
    pub border: BorderSide,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoxRenderOp {
    pub context: RenderContext,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextRenderOp {
    pub context: RenderContext,
    pub text: String,
    pub color: Color,
    pub font: Option<FontDescriptor>,
    pub font_source: Option<FontSource>,
    pub font_size: Pt,
    pub line_height: Option<Pt>,
    pub block: Option<TextBlock>,
    pub layout: Option<TextLayout>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImageRenderOp {
    pub context: RenderContext,
    pub source: RenderImageSource,
    pub fit: ObjectFit,
    pub destination: Bounds,
    pub source_size: Option<Size>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SvgRenderOp {
    pub context: RenderContext,
    pub source: SvgRenderSource,
    pub natural_size: Size,
    pub view_box: Option<ViewBox>,
    pub fit: ObjectFit,
    pub destination: Bounds,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransformRenderOp {
    pub context: RenderContext,
    pub operations: Vec<TransformOperation>,
    pub matrix: AffineTransform,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DebugRenderOp {
    pub context: RenderContext,
    pub label: String,
    pub color: Color,
    pub content_color: Option<Color>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormRenderOp {
    pub context: RenderContext,
    pub name: String,
    pub bounds: Bounds,
    pub commands: Vec<RenderCommand>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaintRole {
    PageBackground,
    Background,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BorderSidePosition {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BorderStyle {
    Solid,
    Dashed,
    Dotted,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderSide {
    pub width: Pt,
    pub color: Color,
    pub style: BorderStyle,
}

impl BorderSide {
    pub const fn new(width: Pt, color: Color, style: BorderStyle) -> Self {
        Self {
            width,
            color,
            style,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BorderRadius {
    pub top_left: Pt,
    pub top_right: Pt,
    pub bottom_right: Pt,
    pub bottom_left: Pt,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Border {
    pub top: Option<BorderSide>,
    pub right: Option<BorderSide>,
    pub bottom: Option<BorderSide>,
    pub left: Option<BorderSide>,
    pub radius: BorderRadius,
}

impl Border {
    pub fn all(side: BorderSide) -> Self {
        Self {
            top: Some(side),
            right: Some(side),
            bottom: Some(side),
            left: Some(side),
            radius: BorderRadius::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RenderImageSource {
    Asset(Image),
    Source(AssetImageSource),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SvgRenderSource {
    Svg(SvgNode),
    Math { source: String, svg: SvgNode },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ObjectFit {
    Fill,
    #[default]
    Contain,
    Cover,
    None,
    ScaleDown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ObjectFitResult {
    pub bounds: Bounds,
    pub scale_x: f32,
    pub scale_y: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RenderNodeKind {
    #[default]
    Page,
    View,
    Box,
    Text,
    ImageAsset,
    ImageSource,
    Svg,
    Math,
}

impl RenderNodeKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Page => "page",
            Self::View => "view",
            Self::Box => "box",
            Self::Text => "text",
            Self::ImageAsset => "image-asset",
            Self::ImageSource => "image-source",
            Self::Svg => "svg",
            Self::Math => "math",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderContext {
    pub page_index: usize,
    pub source_page_index: usize,
    pub path: Vec<usize>,
    pub node_kind: RenderNodeKind,
    pub z_index: i32,
    pub frame: Bounds,
    pub content_frame: Bounds,
}

impl RenderContext {
    pub fn label(&self) -> String {
        if self.path.is_empty() {
            format!("{}:{}", self.node_kind.as_str(), self.page_index)
        } else {
            let path = self
                .path
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(".");
            format!("{}:{}:{}", self.node_kind.as_str(), self.page_index, path)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransformOperation {
    Translate {
        x: f32,
        y: f32,
    },
    Scale {
        x: f32,
        y: f32,
    },
    Rotate {
        degrees: f32,
        cx: f32,
        cy: f32,
    },
    Skew {
        x_degrees: f32,
        y_degrees: f32,
    },
    Matrix {
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        e: f32,
        f: f32,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AffineTransform {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

impl AffineTransform {
    pub const fn identity() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }

    pub const fn translate(x: f32, y: f32) -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: x,
            f: y,
        }
    }

    pub const fn scale(x: f32, y: f32) -> Self {
        Self {
            a: x,
            b: 0.0,
            c: 0.0,
            d: y,
            e: 0.0,
            f: 0.0,
        }
    }

    pub fn rotate(degrees: f32) -> Self {
        let radians = degrees.to_radians();
        let cos = radians.cos();
        let sin = radians.sin();
        Self {
            a: cos,
            b: sin,
            c: -sin,
            d: cos,
            e: 0.0,
            f: 0.0,
        }
    }

    pub fn skew(x_degrees: f32, y_degrees: f32) -> Self {
        Self {
            a: 1.0,
            b: y_degrees.to_radians().tan(),
            c: x_degrees.to_radians().tan(),
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }

    pub fn multiply(self, other: Self) -> Self {
        Self {
            a: self.a * other.a + self.c * other.b,
            b: self.b * other.a + self.d * other.b,
            c: self.a * other.c + self.c * other.d,
            d: self.b * other.c + self.d * other.d,
            e: self.a * other.e + self.c * other.f + self.e,
            f: self.b * other.e + self.d * other.f + self.f,
        }
    }

    pub fn is_identity(self) -> bool {
        const EPSILON: f32 = 0.000_1;
        (self.a - 1.0).abs() < EPSILON
            && self.b.abs() < EPSILON
            && self.c.abs() < EPSILON
            && (self.d - 1.0).abs() < EPSILON
            && self.e.abs() < EPSILON
            && self.f.abs() < EPSILON
    }
}

impl Default for AffineTransform {
    fn default() -> Self {
        Self::identity()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DebugRenderOptions {
    pub color: Color,
    pub content_color: Option<Color>,
    pub label_nodes: bool,
}

impl Default for DebugRenderOptions {
    fn default() -> Self {
        Self {
            color: Color::rgba(255, 0, 255, 160),
            content_color: Some(Color::rgba(0, 255, 255, 160)),
            label_nodes: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderEngineOptions {
    pub image_fit: ObjectFit,
    pub svg_fit: ObjectFit,
    pub wrap_views_in_forms: bool,
    pub debug: Option<DebugRenderOptions>,
}

impl Default for RenderEngineOptions {
    fn default() -> Self {
        Self {
            image_fit: ObjectFit::Contain,
            svg_fit: ObjectFit::Contain,
            wrap_views_in_forms: false,
            debug: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderEngine {
    options: RenderEngineOptions,
}

impl RenderEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_options(mut self, options: RenderEngineOptions) -> Self {
        self.options = options;
        self
    }

    pub fn options(&self) -> &RenderEngineOptions {
        &self.options
    }

    pub fn build<T>(&self, source: &T) -> Result<RenderDocument>
    where
        T: RenderSource + ?Sized,
    {
        source.build_render_document(self)
    }

    fn build_safe_document(&self, layout: &SafeLayoutDocument) -> Result<RenderDocument> {
        let pages = layout
            .pages
            .iter()
            .enumerate()
            .map(|(page_index, page)| self.render_safe_page(page_index, page))
            .collect::<Result<Vec<_>>>()?;

        Ok(RenderDocument {
            metadata: layout.metadata.clone(),
            pages,
        })
    }

    fn build_legacy_document(&self, layout: &LegacyLayoutDocument) -> Result<RenderDocument> {
        let pages = layout
            .pages
            .iter()
            .enumerate()
            .map(|(page_index, page)| {
                let commands = page
                    .nodes
                    .iter()
                    .enumerate()
                    .map(|(node_index, node)| self.render_legacy_node(page_index, node_index, node))
                    .collect::<Vec<_>>();

                Ok(RenderPage {
                    size: page.size,
                    source_page_index: page_index,
                    commands: commands.into_iter().flatten().collect(),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(RenderDocument {
            metadata: LayoutMetadata::default(),
            pages,
        })
    }

    fn render_safe_page(&self, page_index: usize, page: &SafeLayoutPage) -> Result<RenderPage> {
        let mut commands = Vec::new();
        let context = RenderContext {
            page_index,
            source_page_index: page.source_page_index,
            path: Vec::new(),
            node_kind: RenderNodeKind::Page,
            z_index: page.style.z_index,
            frame: Bounds::from_origin_size(0.0, 0.0, page.size.width, page.size.height),
            content_frame: inset_bounds(
                Bounds::from_origin_size(0.0, 0.0, page.size.width, page.size.height),
                page.style.padding,
            ),
        };

        commands.extend(background_commands(
            &context,
            page.style.background_color,
            PaintRole::PageBackground,
        ));

        if let Some(debug) = &self.options.debug {
            commands.push(RenderCommand::DrawDebug(DebugRenderOp {
                context: context.clone(),
                label: maybe_label(&context, debug.label_nodes),
                color: debug.color,
                content_color: debug.content_color,
            }));
        }

        for (node_index, node) in page.nodes.iter().enumerate() {
            commands.extend(self.render_safe_node(
                page_index,
                page.source_page_index,
                vec![node_index],
                node,
            )?);
        }

        Ok(RenderPage {
            size: page.size,
            source_page_index: page.source_page_index,
            commands,
        })
    }

    fn render_safe_node(
        &self,
        page_index: usize,
        source_page_index: usize,
        path: Vec<usize>,
        node: &SafeLayoutNode,
    ) -> Result<Vec<RenderCommand>> {
        let context = RenderContext {
            page_index,
            source_page_index,
            path: path.clone(),
            node_kind: safe_node_kind(node),
            z_index: node.z_index(),
            frame: node.frame,
            content_frame: node.content_frame,
        };

        if self.options.wrap_views_in_forms && matches!(node.kind, SafeNodeKind::View) {
            let commands = self.render_safe_node_commands(&context, node, path)?;
            return Ok(vec![RenderCommand::DrawForm(FormRenderOp {
                name: format!("form-{}", context.label().replace(':', "-")),
                bounds: context.frame,
                context,
                commands,
            })]);
        }

        self.render_safe_node_commands(&context, node, path)
    }

    fn render_safe_node_commands(
        &self,
        context: &RenderContext,
        node: &SafeLayoutNode,
        path: Vec<usize>,
    ) -> Result<Vec<RenderCommand>> {
        let mut commands =
            background_commands(context, node.style.background_color, PaintRole::Background);

        match &node.kind {
            SafeNodeKind::View => {}
            SafeNodeKind::Box => {
                commands.push(RenderCommand::DrawBox(BoxRenderOp {
                    context: context.clone(),
                }));
            }
            SafeNodeKind::Text { block, layout } => {
                commands.push(RenderCommand::DrawText(TextRenderOp {
                    context: context.clone(),
                    text: block.plain_text(),
                    color: node.style.color,
                    font: Some(node.style.font.descriptor.clone()),
                    font_source: node.style.font.source.clone(),
                    font_size: node.style.font_size,
                    line_height: Some(node.style.line_height),
                    block: Some(block.clone()),
                    layout: Some(layout.clone()),
                }));
            }
            SafeNodeKind::ImageAsset { asset } => {
                let fit = self.options.image_fit;
                let destination = fit_object(
                    Size::new(asset.width(), asset.height()),
                    node.content_frame,
                    fit,
                )
                .bounds;
                commands.push(RenderCommand::DrawImage(ImageRenderOp {
                    context: context.clone(),
                    source: RenderImageSource::Asset(asset.clone()),
                    fit,
                    destination,
                    source_size: Some(Size::new(asset.width(), asset.height())),
                }));
            }
            SafeNodeKind::ImageSource { source } => {
                commands.push(RenderCommand::DrawImage(ImageRenderOp {
                    context: context.clone(),
                    source: RenderImageSource::Source(source.clone()),
                    fit: self.options.image_fit,
                    destination: node.content_frame,
                    source_size: None,
                }));
            }
            SafeNodeKind::Svg { svg } => {
                commands.extend(self.render_svg_like_node(
                    context,
                    SvgRenderSource::Svg(svg.clone()),
                    svg,
                )?);
            }
            SafeNodeKind::Math { source, svg } => {
                commands.extend(self.render_svg_like_node(
                    context,
                    SvgRenderSource::Math {
                        source: source.clone(),
                        svg: svg.clone(),
                    },
                    svg,
                )?);
            }
        }

        for (child_index, child) in node.children.iter().enumerate() {
            let mut child_path = path.clone();
            child_path.push(child_index);
            commands.extend(self.render_safe_node(
                context.page_index,
                context.source_page_index,
                child_path,
                child,
            )?);
        }

        if let Some(debug) = &self.options.debug {
            commands.push(RenderCommand::DrawDebug(DebugRenderOp {
                context: context.clone(),
                label: maybe_label(context, debug.label_nodes),
                color: debug.color,
                content_color: debug.content_color,
            }));
        }

        Ok(commands)
    }

    fn render_svg_like_node(
        &self,
        context: &RenderContext,
        source: SvgRenderSource,
        svg: &SvgNode,
    ) -> Result<Vec<RenderCommand>> {
        let natural_size = resolve_svg_size(svg)?;
        let view_box = svg
            .props
            .get("viewBox")
            .and_then(|value| parse_view_box(value));
        let fit = self.options.svg_fit;
        let fitted = fit_object(natural_size, context.content_frame, fit);
        let mut commands = Vec::new();
        let transform = svg_fit_transform(context, natural_size, fitted, view_box);

        if !transform.matrix.is_identity() {
            commands.push(RenderCommand::PushTransform(TransformRenderOp {
                context: context.clone(),
                operations: transform.operations,
                matrix: transform.matrix,
            }));
        }

        commands.push(RenderCommand::DrawSvg(SvgRenderOp {
            context: context.clone(),
            source,
            natural_size,
            view_box,
            fit,
            destination: fitted.bounds,
        }));

        if !transform.matrix.is_identity() {
            commands.push(RenderCommand::PopTransform(context.clone()));
        }

        Ok(commands)
    }

    fn render_legacy_node(
        &self,
        page_index: usize,
        node_index: usize,
        node: &LegacyLayoutNode,
    ) -> Vec<RenderCommand> {
        let kind = match &node.content {
            LayoutContent::Text(_) => RenderNodeKind::Text,
            LayoutContent::Box => RenderNodeKind::Box,
        };
        let context = RenderContext {
            page_index,
            source_page_index: page_index,
            path: vec![node_index],
            node_kind: kind,
            z_index: 0,
            frame: node.frame,
            content_frame: node.frame,
        };

        match &node.content {
            LayoutContent::Text(block) => {
                let first_span = block.spans().first();
                vec![RenderCommand::DrawText(TextRenderOp {
                    context,
                    text: block.plain_text(),
                    color: Color::BLACK,
                    font: node.font_descriptor().cloned(),
                    font_source: None,
                    font_size: first_span
                        .map(|span| span.font_size())
                        .unwrap_or(Pt::new(12.0)),
                    line_height: None,
                    block: Some(block.clone()),
                    layout: None,
                })]
            }
            LayoutContent::Box => vec![RenderCommand::DrawBox(BoxRenderOp { context })],
        }
    }
}

pub trait RenderSource {
    fn build_render_document(&self, engine: &RenderEngine) -> Result<RenderDocument>;
}

impl RenderSource for SafeLayoutDocument {
    fn build_render_document(&self, engine: &RenderEngine) -> Result<RenderDocument> {
        engine.build_safe_document(self)
    }
}

impl RenderSource for LegacyLayoutDocument {
    fn build_render_document(&self, engine: &RenderEngine) -> Result<RenderDocument> {
        engine.build_legacy_document(self)
    }
}

pub fn background_commands(
    context: &RenderContext,
    color: Option<Color>,
    role: PaintRole,
) -> Vec<RenderCommand> {
    color
        .map(|color| {
            vec![RenderCommand::FillRect(FillRectOp {
                context: context.clone(),
                bounds: context.frame,
                color,
                role,
            })]
        })
        .unwrap_or_default()
}

pub fn border_commands(context: &RenderContext, border: &Border) -> Vec<RenderCommand> {
    let mut commands = Vec::new();

    for (side, value) in [
        (BorderSidePosition::Top, border.top),
        (BorderSidePosition::Right, border.right),
        (BorderSidePosition::Bottom, border.bottom),
        (BorderSidePosition::Left, border.left),
    ] {
        if let Some(border) = value
            && border.width.value() > 0.0
        {
            commands.push(RenderCommand::StrokeBorder(BorderRenderOp {
                context: context.clone(),
                bounds: context.frame,
                side,
                border,
            }));
        }
    }

    commands
}

pub fn fit_object(source: Size, container: Bounds, fit: ObjectFit) -> ObjectFitResult {
    if container.size.width <= 0.0 || container.size.height <= 0.0 {
        return ObjectFitResult {
            bounds: Bounds::from_origin_size(container.origin.x, container.origin.y, 0.0, 0.0),
            scale_x: 0.0,
            scale_y: 0.0,
        };
    }

    let valid_source = source.width > 0.0 && source.height > 0.0;
    if !valid_source {
        return ObjectFitResult {
            bounds: container,
            scale_x: 1.0,
            scale_y: 1.0,
        };
    }

    let source_width = source.width.abs();
    let source_height = source.height.abs();
    let x_scale = container.size.width / source_width;
    let y_scale = container.size.height / source_height;

    let (scale_x, scale_y) = match fit {
        ObjectFit::Fill => (x_scale, y_scale),
        ObjectFit::Contain => {
            let scale = x_scale.min(y_scale);
            (scale, scale)
        }
        ObjectFit::Cover => {
            let scale = x_scale.max(y_scale);
            (scale, scale)
        }
        ObjectFit::None => (1.0, 1.0),
        ObjectFit::ScaleDown => {
            if source_width <= container.size.width && source_height <= container.size.height {
                (1.0, 1.0)
            } else {
                let scale = x_scale.min(y_scale);
                (scale, scale)
            }
        }
    };

    let fitted_width = source_width * scale_x;
    let fitted_height = source_height * scale_y;
    let x = container.origin.x + ((container.size.width - fitted_width) * 0.5);
    let y = container.origin.y + ((container.size.height - fitted_height) * 0.5);

    ObjectFitResult {
        bounds: Bounds::from_origin_size(x, y, fitted_width, fitted_height),
        scale_x,
        scale_y,
    }
}

pub fn parse_color(input: &str) -> Result<Color> {
    let trimmed = input.trim();
    let lower = trimmed.to_ascii_lowercase();

    if let Some(color) = named_color(lower.as_str()) {
        return Ok(color);
    }

    if let Some(color) = parse_hex_color(trimmed) {
        return Ok(color);
    }

    if lower.starts_with("rgb(") && lower.ends_with(')') {
        let inner = &trimmed[4..trimmed.len() - 1];
        let parts = split_csv(inner);
        if parts.len() == 3 {
            return Ok(Color::rgb(
                parse_rgb_channel(parts[0])?,
                parse_rgb_channel(parts[1])?,
                parse_rgb_channel(parts[2])?,
            ));
        }
    }

    if lower.starts_with("rgba(") && lower.ends_with(')') {
        let inner = &trimmed[5..trimmed.len() - 1];
        let parts = split_csv(inner);
        if parts.len() == 4 {
            return Ok(Color::rgba(
                parse_rgb_channel(parts[0])?,
                parse_rgb_channel(parts[1])?,
                parse_rgb_channel(parts[2])?,
                parse_alpha_channel(parts[3])?,
            ));
        }
    }

    Err(Error::InvalidColor {
        input: input.to_string(),
    })
}

pub fn parse_transform(input: &str) -> Result<Vec<TransformOperation>> {
    let mut operations = Vec::new();
    let mut remainder = input.trim();

    while !remainder.is_empty() {
        let Some(start) = remainder.find('(') else {
            return Err(Error::InvalidTransform {
                input: input.to_string(),
            });
        };
        let name = remainder[..start].trim();
        let after_start = &remainder[start + 1..];
        let Some(end) = after_start.find(')') else {
            return Err(Error::InvalidTransform {
                input: input.to_string(),
            });
        };
        let values = parse_transform_values(&after_start[..end]);
        operations.push(normalize_transform(name, &values, input)?);
        remainder = after_start[end + 1..].trim_start();
    }

    if operations.is_empty() {
        Err(Error::InvalidTransform {
            input: input.to_string(),
        })
    } else {
        Ok(operations)
    }
}

pub fn compose_transform(operations: &[TransformOperation]) -> AffineTransform {
    operations
        .iter()
        .fold(AffineTransform::identity(), |matrix, op| {
            matrix.multiply(transform_matrix(*op))
        })
}

pub fn parse_view_box(input: &str) -> Option<ViewBox> {
    let values = input
        .split(|character: char| character.is_ascii_whitespace() || character == ',')
        .filter(|part| !part.is_empty())
        .map(|part| part.parse::<f32>().ok())
        .collect::<Option<Vec<_>>>()?;

    match values.as_slice() {
        [x, y, width, height] if *width > 0.0 && *height > 0.0 => Some(ViewBox {
            x: *x,
            y: *y,
            width: *width,
            height: *height,
        }),
        _ => None,
    }
}

pub fn resolve_svg_size(svg: &SvgNode) -> Result<Size> {
    let view_box = svg
        .props
        .get("viewBox")
        .and_then(|value| parse_view_box(value));
    let width = svg
        .props
        .get("width")
        .and_then(|value| parse_dimension(value).ok())
        .or_else(|| view_box.map(|view_box| view_box.width))
        .unwrap_or(0.0);
    let height = svg
        .props
        .get("height")
        .and_then(|value| parse_dimension(value).ok())
        .or_else(|| view_box.map(|view_box| view_box.height))
        .unwrap_or(0.0);

    if width <= 0.0 || height <= 0.0 {
        Err(Error::InvalidSvgDimensions)
    } else {
        Ok(Size::new(width, height))
    }
}

pub fn parse_dimension(input: &str) -> Result<f32> {
    let trimmed = input.trim();
    let number = parse_float(trimmed).ok_or_else(|| Error::InvalidDimension {
        input: input.to_string(),
    })?;
    let suffix = trimmed
        .trim_start_matches(|character: char| {
            character.is_ascii_digit() || matches!(character, '.' | '+' | '-')
        })
        .trim()
        .to_ascii_lowercase();

    let scaled = match suffix.as_str() {
        "" | "px" | "pt" => number,
        "in" => number * 72.0,
        "cm" => number * 72.0 / 2.54,
        "mm" => number * 72.0 / 25.4,
        _ => number,
    };

    Ok(scaled.abs())
}

fn inset_bounds(bounds: Bounds, insets: EdgeInsets) -> Bounds {
    Bounds::from_origin_size(
        bounds.origin.x + insets.left.value(),
        bounds.origin.y + insets.top.value(),
        (bounds.size.width - insets.left.value() - insets.right.value()).max(0.0),
        (bounds.size.height - insets.top.value() - insets.bottom.value()).max(0.0),
    )
}

fn safe_node_kind(node: &SafeLayoutNode) -> RenderNodeKind {
    match node.kind {
        SafeNodeKind::View => RenderNodeKind::View,
        SafeNodeKind::Box => RenderNodeKind::Box,
        SafeNodeKind::Text { .. } => RenderNodeKind::Text,
        SafeNodeKind::ImageAsset { .. } => RenderNodeKind::ImageAsset,
        SafeNodeKind::ImageSource { .. } => RenderNodeKind::ImageSource,
        SafeNodeKind::Svg { .. } => RenderNodeKind::Svg,
        SafeNodeKind::Math { .. } => RenderNodeKind::Math,
    }
}

fn maybe_label(context: &RenderContext, enabled: bool) -> String {
    if enabled {
        context.label()
    } else {
        String::new()
    }
}

fn named_color(name: &str) -> Option<Color> {
    match name {
        "black" => Some(Color::BLACK),
        "white" => Some(Color::WHITE),
        "red" => Some(Color::rgb(255, 0, 0)),
        "green" => Some(Color::rgb(0, 128, 0)),
        "blue" => Some(Color::rgb(0, 0, 255)),
        "transparent" => Some(Color::rgba(0, 0, 0, 0)),
        _ => None,
    }
}

fn parse_hex_color(input: &str) -> Option<Color> {
    let hex = input.strip_prefix('#')?;
    match hex.len() {
        3 => Some(Color::rgb(
            expand_hex_digit(hex.as_bytes()[0])?,
            expand_hex_digit(hex.as_bytes()[1])?,
            expand_hex_digit(hex.as_bytes()[2])?,
        )),
        4 => Some(Color::rgba(
            expand_hex_digit(hex.as_bytes()[0])?,
            expand_hex_digit(hex.as_bytes()[1])?,
            expand_hex_digit(hex.as_bytes()[2])?,
            expand_hex_digit(hex.as_bytes()[3])?,
        )),
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

fn expand_hex_digit(value: u8) -> Option<u8> {
    let digit = char::from(value).to_digit(16)? as u8;
    Some((digit << 4) | digit)
}

fn split_csv(input: &str) -> Vec<&str> {
    input
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect()
}

fn parse_rgb_channel(input: &str) -> Result<u8> {
    if let Some(percent) = match_percent(input) {
        return Ok(clamp_u8(percent.percent * 255.0));
    }

    let value = parse_float(input).ok_or_else(|| Error::InvalidColor {
        input: input.to_string(),
    })?;
    Ok(clamp_u8(value))
}

fn parse_alpha_channel(input: &str) -> Result<u8> {
    if let Some(percent) = match_percent(input) {
        return Ok(clamp_u8(percent.percent * 255.0));
    }

    let value = parse_float(input).ok_or_else(|| Error::InvalidColor {
        input: input.to_string(),
    })?;
    let alpha = if value <= 1.0 { value * 255.0 } else { value };
    Ok(clamp_u8(alpha))
}

fn clamp_u8(value: f32) -> u8 {
    value.round().clamp(0.0, 255.0) as u8
}

fn parse_transform_values(input: &str) -> Vec<&str> {
    if input.contains(',') {
        input
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .collect()
    } else {
        input.split_whitespace().collect()
    }
}

fn normalize_transform(name: &str, values: &[&str], original: &str) -> Result<TransformOperation> {
    match name {
        "translate" => Ok(TransformOperation::Translate {
            x: parse_required_f32(values.first().copied(), original)?,
            y: parse_optional_f32(values.get(1).copied()).unwrap_or(0.0),
        }),
        "translateX" => Ok(TransformOperation::Translate {
            x: parse_required_f32(values.first().copied(), original)?,
            y: 0.0,
        }),
        "translateY" => Ok(TransformOperation::Translate {
            x: 0.0,
            y: parse_required_f32(values.first().copied(), original)?,
        }),
        "scale" => {
            let x = parse_required_f32(values.first().copied(), original)?;
            Ok(TransformOperation::Scale {
                x,
                y: parse_optional_f32(values.get(1).copied()).unwrap_or(x),
            })
        }
        "scaleX" => Ok(TransformOperation::Scale {
            x: parse_required_f32(values.first().copied(), original)?,
            y: 1.0,
        }),
        "scaleY" => Ok(TransformOperation::Scale {
            x: 1.0,
            y: parse_required_f32(values.first().copied(), original)?,
        }),
        "rotate" => Ok(TransformOperation::Rotate {
            degrees: parse_angle(values.first().copied(), original)?,
            cx: parse_optional_f32(values.get(1).copied()).unwrap_or(0.0),
            cy: parse_optional_f32(values.get(2).copied()).unwrap_or(0.0),
        }),
        "skew" => Ok(TransformOperation::Skew {
            x_degrees: parse_angle(values.first().copied(), original)?,
            y_degrees: values
                .get(1)
                .copied()
                .map(|value| parse_angle(Some(value), original))
                .transpose()?
                .unwrap_or(0.0),
        }),
        "skewX" => Ok(TransformOperation::Skew {
            x_degrees: parse_angle(values.first().copied(), original)?,
            y_degrees: 0.0,
        }),
        "skewY" => Ok(TransformOperation::Skew {
            x_degrees: 0.0,
            y_degrees: parse_angle(values.first().copied(), original)?,
        }),
        "matrix" if values.len() == 6 => Ok(TransformOperation::Matrix {
            a: parse_required_f32(values.first().copied(), original)?,
            b: parse_required_f32(values.get(1).copied(), original)?,
            c: parse_required_f32(values.get(2).copied(), original)?,
            d: parse_required_f32(values.get(3).copied(), original)?,
            e: parse_required_f32(values.get(4).copied(), original)?,
            f: parse_required_f32(values.get(5).copied(), original)?,
        }),
        _ => Err(Error::InvalidTransform {
            input: original.to_string(),
        }),
    }
}

fn parse_required_f32(value: Option<&str>, original: &str) -> Result<f32> {
    parse_optional_f32(value).ok_or_else(|| Error::InvalidTransform {
        input: original.to_string(),
    })
}

fn parse_optional_f32(value: Option<&str>) -> Option<f32> {
    parse_float(value?.trim())
}

fn parse_angle(value: Option<&str>, original: &str) -> Result<f32> {
    let value = value.ok_or_else(|| Error::InvalidTransform {
        input: original.to_string(),
    })?;
    let trimmed = value.trim();
    if let Some(raw) = trimmed.strip_suffix("rad") {
        let radians = parse_float(raw.trim()).ok_or_else(|| Error::InvalidTransform {
            input: original.to_string(),
        })?;
        Ok(radians.to_degrees())
    } else {
        parse_float(trimmed.trim_end_matches("deg")).ok_or_else(|| Error::InvalidTransform {
            input: original.to_string(),
        })
    }
}

fn transform_matrix(operation: TransformOperation) -> AffineTransform {
    match operation {
        TransformOperation::Translate { x, y } => AffineTransform::translate(x, y),
        TransformOperation::Scale { x, y } => AffineTransform::scale(x, y),
        TransformOperation::Rotate { degrees, cx, cy } => AffineTransform::translate(cx, cy)
            .multiply(AffineTransform::rotate(degrees))
            .multiply(AffineTransform::translate(-cx, -cy)),
        TransformOperation::Skew {
            x_degrees,
            y_degrees,
        } => AffineTransform::skew(x_degrees, y_degrees),
        TransformOperation::Matrix { a, b, c, d, e, f } => AffineTransform { a, b, c, d, e, f },
    }
}

struct SvgFitTransform {
    operations: Vec<TransformOperation>,
    matrix: AffineTransform,
}

fn svg_fit_transform(
    context: &RenderContext,
    natural_size: Size,
    fitted: ObjectFitResult,
    view_box: Option<ViewBox>,
) -> SvgFitTransform {
    let scale_x = if natural_size.width > 0.0 {
        fitted.bounds.size.width / natural_size.width
    } else {
        1.0
    };
    let scale_y = if natural_size.height > 0.0 {
        fitted.bounds.size.height / natural_size.height
    } else {
        1.0
    };

    let mut operations = vec![TransformOperation::Translate {
        x: fitted.bounds.origin.x,
        y: fitted.bounds.origin.y,
    }];
    if let Some(view_box) = view_box {
        operations.push(TransformOperation::Translate {
            x: -view_box.x,
            y: -view_box.y,
        });
    }
    operations.push(TransformOperation::Scale {
        x: scale_x,
        y: scale_y,
    });

    let mut matrix = compose_transform(&operations);
    if context.content_frame.origin == fitted.bounds.origin
        && context.content_frame.size == fitted.bounds.size
        && scale_x == 1.0
        && scale_y == 1.0
        && view_box.is_none()
    {
        matrix = AffineTransform::identity();
    }

    SvgFitTransform { operations, matrix }
}

fn default_safe_font() -> SafeFont {
    SafeFont {
        descriptor: FontDescriptor::new(StandardFont::Helvetica.family_name()),
        source: Some(FontSource::standard(StandardFont::Helvetica)),
    }
}

#[allow(dead_code)]
fn default_safe_style() -> SafeLayoutStyle {
    SafeLayoutStyle {
        width: None,
        height: None,
        margin: EdgeInsets::default(),
        padding: EdgeInsets::default(),
        background_color: None,
        color: Color::BLACK,
        font: default_safe_font(),
        font_size: Pt::new(12.0),
        line_height: Pt::new(14.4),
        z_index: 0,
        page_break_before: false,
        page_break_after: false,
    }
}

pub trait RenderBackend {
    type Output;

    fn render_document(&mut self, document: &RenderDocument) -> Result<Self::Output>;
}

#[derive(Debug, Default)]
pub struct NoopRenderBackend;

impl RenderBackend for NoopRenderBackend {
    type Output = ();

    fn render_document(&mut self, _document: &RenderDocument) -> Result<Self::Output> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Renderer<B: RenderBackend> {
    backend: B,
}

impl<B: RenderBackend> Renderer<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn render(&mut self, document: &RenderDocument) -> Result<B::Output> {
        self.backend.render_document(document)
    }
}

pub trait RendererDocumentSource {
    fn build_render_document(
        &self,
        layout_engine: &LayoutEngine,
        render_engine: &RenderEngine,
    ) -> Result<RenderDocument>;
}

impl RendererDocumentSource for SourceLayoutDocument {
    fn build_render_document(
        &self,
        layout_engine: &LayoutEngine,
        render_engine: &RenderEngine,
    ) -> Result<RenderDocument> {
        let layout = layout_engine.layout_document(self)?;
        render_engine.build(&layout)
    }
}

impl RendererDocumentSource for SafeLayoutDocument {
    fn build_render_document(
        &self,
        _layout_engine: &LayoutEngine,
        render_engine: &RenderEngine,
    ) -> Result<RenderDocument> {
        render_engine.build(self)
    }
}

impl RendererDocumentSource for LegacyLayoutDocument {
    fn build_render_document(
        &self,
        _layout_engine: &LayoutEngine,
        render_engine: &RenderEngine,
    ) -> Result<RenderDocument> {
        render_engine.build(self)
    }
}

impl RendererDocumentSource for RenderDocument {
    fn build_render_document(
        &self,
        _layout_engine: &LayoutEngine,
        _render_engine: &RenderEngine,
    ) -> Result<RenderDocument> {
        Ok(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct DocumentContainer<T> {
    document: T,
    revision: u64,
}

impl<T> DocumentContainer<T> {
    pub fn new(document: T) -> Self {
        Self {
            document,
            revision: 0,
        }
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn document(&self) -> &T {
        &self.document
    }

    pub fn into_inner(self) -> T {
        self.document
    }

    pub fn replace(&mut self, document: T) -> u64 {
        self.document = document;
        self.bump_revision()
    }

    pub fn update(&mut self, update: impl FnOnce(&mut T)) -> u64 {
        update(&mut self.document);
        self.bump_revision()
    }

    fn bump_revision(&mut self) -> u64 {
        self.revision = self.revision.saturating_add(1);
        self.revision
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderSnapshot {
    revision: u64,
    document: RenderDocument,
}

impl RenderSnapshot {
    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn document(&self) -> &RenderDocument {
        &self.document
    }

    pub fn into_document(self) -> RenderDocument {
        self.document
    }
}

pub struct RendererSession<T> {
    container: DocumentContainer<T>,
    layout_engine: LayoutEngine,
    render_engine: RenderEngine,
    rendered: Option<RenderSnapshot>,
}

impl<T> RendererSession<T> {
    pub fn new(document: T) -> Self {
        Self {
            container: DocumentContainer::new(document),
            layout_engine: LayoutEngine::new(),
            render_engine: RenderEngine::new(),
            rendered: None,
        }
    }

    pub fn with_layout_engine(mut self, layout_engine: LayoutEngine) -> Self {
        self.layout_engine = layout_engine;
        self
    }

    pub fn with_render_engine(mut self, render_engine: RenderEngine) -> Self {
        self.render_engine = render_engine;
        self
    }

    pub fn revision(&self) -> u64 {
        self.container.revision()
    }

    pub fn document(&self) -> &T {
        self.container.document()
    }

    pub fn rendered(&self) -> Option<&RenderSnapshot> {
        self.rendered.as_ref()
    }

    pub fn replace_document(&mut self, document: T) -> u64 {
        self.rendered = None;
        self.container.replace(document)
    }

    pub fn update_document(&mut self, update: impl FnOnce(&mut T)) -> u64 {
        self.rendered = None;
        self.container.update(update)
    }
}

impl<T> RendererSession<T>
where
    T: RendererDocumentSource,
{
    pub fn render_snapshot(&mut self) -> Result<&RenderSnapshot> {
        let revision = self.revision();
        let should_render = self
            .rendered
            .as_ref()
            .map(|snapshot| snapshot.revision() != revision)
            .unwrap_or(true);

        if should_render {
            let document = self
                .container
                .document()
                .build_render_document(&self.layout_engine, &self.render_engine)?;
            self.rendered = Some(RenderSnapshot { revision, document });
        }

        Ok(self
            .rendered
            .as_ref()
            .expect("renderer session should populate a snapshot before returning"))
    }

    pub fn render_document(&mut self) -> Result<&RenderDocument> {
        Ok(self.render_snapshot()?.document())
    }

    pub fn render_with<B: RenderBackend>(
        &mut self,
        renderer: &mut Renderer<B>,
    ) -> Result<B::Output> {
        renderer.render(self.render_document()?)
    }

    pub fn to_bytes(&mut self) -> Result<Vec<u8>> {
        let mut renderer = Renderer::new(PdfRenderBackend::default());
        self.render_with(&mut renderer)
    }

    pub fn write<W: Write>(&mut self, writer: W) -> Result<()> {
        let mut backend = PdfRenderBackend::default();
        backend.render_to_writer(self.render_document()?, writer)
    }

    pub fn save(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let mut backend = PdfRenderBackend::default();
        backend.render_to_file(self.render_document()?, path)
    }
}

#[derive(Debug, Default)]
pub struct PdfRenderBackend {
    registered_fonts: HashMap<FontBinding, String>,
    fallback_font_name: Option<String>,
}

impl PdfRenderBackend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render_to_writer<W: Write>(
        &mut self,
        document: &RenderDocument,
        mut writer: W,
    ) -> Result<()> {
        let bytes = self.render_document(document)?;
        writer.write_all(&bytes)?;
        Ok(())
    }

    pub fn render_to_file(
        &mut self,
        document: &RenderDocument,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        let mut file = std::fs::File::create(path)?;
        self.render_to_writer(document, &mut file)
    }

    fn encode_document(&mut self, document: &RenderDocument) -> Result<Vec<u8>> {
        let mut writer = PdfWriter::with_metadata(metadata_to_pdf(&document.metadata));
        self.registered_fonts.clear();
        self.fallback_font_name = None;

        for page in &document.pages {
            let content = self.encode_page(page, &mut writer)?;
            writer.add_page(
                graphitepdf_kit::PageSize::new(page.size.width as f64, page.size.height as f64),
                content,
            );
        }

        Ok(writer.write_all()?)
    }

    fn encode_page(&mut self, page: &RenderPage, writer: &mut PdfWriter) -> Result<Vec<u8>> {
        let mut content = String::new();
        writeln!(
            &mut content,
            "% graphitepdf-render page {}",
            page.source_page_index
        )
        .map_err(string_write_error)?;

        for command in &page.commands {
            self.encode_command(page, command, writer, &mut content)?;
        }

        Ok(content.into_bytes())
    }

    fn encode_command(
        &mut self,
        page: &RenderPage,
        command: &RenderCommand,
        writer: &mut PdfWriter,
        content: &mut String,
    ) -> Result<()> {
        match command {
            RenderCommand::FillRect(operation) => self.write_fill_rect(page, operation, content),
            RenderCommand::StrokeBorder(operation) => self.write_border(page, operation, content),
            RenderCommand::DrawBox(operation) => {
                write_comment(content, &format!("box {}", operation.context.label()))
            }
            RenderCommand::DrawText(operation) => self.write_text(page, operation, writer, content),
            RenderCommand::DrawImage(operation) => self.write_image(page, operation, content),
            RenderCommand::DrawSvg(operation) => self.write_svg(page, operation, content),
            RenderCommand::PushTransform(operation) => self.write_transform(operation, content),
            RenderCommand::PopTransform(context) => {
                write_comment(content, &format!("pop {}", context.label()))?;
                content.push_str("Q\n");
                Ok(())
            }
            RenderCommand::DrawDebug(operation) => self.write_debug(page, operation, content),
            RenderCommand::DrawForm(operation) => self.write_form(page, operation, writer, content),
        }
    }

    fn write_fill_rect(
        &self,
        page: &RenderPage,
        operation: &FillRectOp,
        content: &mut String,
    ) -> Result<()> {
        if operation.color.alpha == 0 {
            return Ok(());
        }

        let bounds = pdf_bounds(page, operation.bounds);
        write_comment(content, &format!("fill {}", operation.context.label()))?;
        content.push_str("q\n");
        push_fill_color(content, operation.color)?;
        writeln!(
            content,
            "{} {} {} {} re",
            bounds.origin.x, bounds.origin.y, bounds.size.width, bounds.size.height
        )
        .map_err(string_write_error)?;
        content.push_str("f\nQ\n");
        Ok(())
    }

    fn write_border(
        &self,
        page: &RenderPage,
        operation: &BorderRenderOp,
        content: &mut String,
    ) -> Result<()> {
        if operation.border.width.value() <= 0.0 || operation.border.color.alpha == 0 {
            return Ok(());
        }

        let bounds = pdf_bounds(page, operation.bounds);
        let half_width = operation.border.width.value() * 0.5;
        let (x1, y1, x2, y2) = match operation.side {
            BorderSidePosition::Top => (
                bounds.origin.x,
                bounds.origin.y + bounds.size.height - half_width,
                bounds.origin.x + bounds.size.width,
                bounds.origin.y + bounds.size.height - half_width,
            ),
            BorderSidePosition::Right => (
                bounds.origin.x + bounds.size.width - half_width,
                bounds.origin.y,
                bounds.origin.x + bounds.size.width - half_width,
                bounds.origin.y + bounds.size.height,
            ),
            BorderSidePosition::Bottom => (
                bounds.origin.x,
                bounds.origin.y + half_width,
                bounds.origin.x + bounds.size.width,
                bounds.origin.y + half_width,
            ),
            BorderSidePosition::Left => (
                bounds.origin.x + half_width,
                bounds.origin.y,
                bounds.origin.x + half_width,
                bounds.origin.y + bounds.size.height,
            ),
        };

        write_comment(content, &format!("border {}", operation.context.label()))?;
        content.push_str("q\n");
        push_stroke_color(content, operation.border.color)?;
        writeln!(content, "{} w", operation.border.width.value()).map_err(string_write_error)?;
        writeln!(content, "{} {} m", x1, y1).map_err(string_write_error)?;
        writeln!(content, "{} {} l", x2, y2).map_err(string_write_error)?;
        content.push_str("S\nQ\n");
        Ok(())
    }

    fn write_text(
        &mut self,
        page: &RenderPage,
        operation: &TextRenderOp,
        writer: &mut PdfWriter,
        content: &mut String,
    ) -> Result<()> {
        if operation.text.is_empty() || operation.color.alpha == 0 {
            return Ok(());
        }

        write_comment(content, &format!("text {}", operation.context.label()))?;
        if let Some(layout) = &operation.layout {
            return self.write_text_layout(page, operation, layout, writer, content);
        }

        let font_name = self.ensure_font(
            writer,
            operation.font.as_ref(),
            operation.font_source.as_ref(),
        );
        let origin = text_origin(page, operation);
        let line_height = operation
            .line_height
            .map(|value| value.value())
            .unwrap_or_else(|| operation.font_size.value() * 1.2);

        content.push_str("BT\n");
        writeln!(content, "/{} {} Tf", font_name, operation.font_size.value())
            .map_err(string_write_error)?;
        push_fill_color(content, operation.color)?;
        writeln!(content, "{} TL", line_height).map_err(string_write_error)?;
        writeln!(content, "1 0 0 1 {} {} Tm", origin.0, origin.1).map_err(string_write_error)?;

        let mut lines = operation.text.lines();
        if let Some(first_line) = lines.next() {
            writeln!(content, "({}) Tj", escape_pdf_text(first_line))
                .map_err(string_write_error)?;
            for line in lines {
                content.push_str("T*\n");
                writeln!(content, "({}) Tj", escape_pdf_text(line)).map_err(string_write_error)?;
            }
        }

        content.push_str("ET\n");
        Ok(())
    }

    fn write_text_layout(
        &mut self,
        page: &RenderPage,
        operation: &TextRenderOp,
        layout: &TextLayout,
        writer: &mut PdfWriter,
        content: &mut String,
    ) -> Result<()> {
        for fragment in layout.fragments() {
            if fragment.text().is_empty() {
                continue;
            }

            let font_source = layout
                .runs()
                .iter()
                .find(|run| {
                    run.range().start() <= fragment.range().start()
                        && fragment.range().end() <= run.range().end()
                })
                .and_then(|run| run.font_source().cloned())
                .or_else(|| operation.font_source.clone());
            let font_name = self.ensure_font(writer, Some(fragment.font()), font_source.as_ref());
            let x = operation.context.content_frame.origin.x + fragment.rect().x.value();
            let y = page.size.height
                - (operation.context.content_frame.origin.y + fragment.baseline().value());

            content.push_str("BT\n");
            writeln!(
                content,
                "/{} {} Tf",
                font_name,
                fragment.font_size().value()
            )
            .map_err(string_write_error)?;
            push_fill_color(content, operation.color)?;
            writeln!(content, "1 0 0 1 {} {} Tm", x, y).map_err(string_write_error)?;
            writeln!(content, "({}) Tj", escape_pdf_text(fragment.text()))
                .map_err(string_write_error)?;
            content.push_str("ET\n");
        }

        Ok(())
    }

    fn write_image(
        &self,
        page: &RenderPage,
        operation: &ImageRenderOp,
        content: &mut String,
    ) -> Result<()> {
        let pdf_y =
            page.size.height - operation.destination.origin.y - operation.destination.size.height;

        let rendered = match &operation.source {
            RenderImageSource::Asset(image) => render_image_to_page_content_with_options(
                image,
                &ImageRenderOptions::new()
                    .position(operation.destination.origin.x as f64, pdf_y as f64)
                    .size(
                        operation.destination.size.width as f64,
                        operation.destination.size.height as f64,
                    ),
            )?,
            RenderImageSource::Source(source) => {
                let image = resolve_image_source_blocking(source.clone())?;
                render_image_to_page_content_with_options(
                    image.as_ref(),
                    &ImageRenderOptions::new()
                        .position(operation.destination.origin.x as f64, pdf_y as f64)
                        .size(
                            operation.destination.size.width as f64,
                            operation.destination.size.height as f64,
                        ),
                )?
            }
        };

        write_comment(content, &format!("image {}", operation.context.label()))?;
        content.push_str(
            std::str::from_utf8(&rendered).map_err(|error| Error::Backend {
                message: format!("image content was not valid UTF-8: {error}"),
            })?,
        );
        if !content.ends_with('\n') {
            content.push('\n');
        }
        Ok(())
    }

    fn write_svg(
        &self,
        page: &RenderPage,
        operation: &SvgRenderOp,
        content: &mut String,
    ) -> Result<()> {
        let pdf_y =
            page.size.height - operation.destination.origin.y - operation.destination.size.height;
        let options = SvgRenderOptions::new()
            .position(operation.destination.origin.x as f64, pdf_y as f64)
            .size(
                operation.destination.size.width as f64,
                operation.destination.size.height as f64,
            );

        let bytes = match &operation.source {
            SvgRenderSource::Svg(svg) => {
                render_svg_node_to_page_content_with_options(svg, &options)?
            }
            SvgRenderSource::Math { svg, .. } => {
                render_svg_node_to_page_content_with_options(svg, &options)?
            }
        };

        write_comment(content, &format!("svg {}", operation.context.label()))?;
        content.push_str(std::str::from_utf8(&bytes).map_err(|error| Error::Backend {
            message: format!("svg content was not valid UTF-8: {error}"),
        })?);
        if !content.ends_with('\n') {
            content.push('\n');
        }
        Ok(())
    }

    fn write_transform(&self, operation: &TransformRenderOp, content: &mut String) -> Result<()> {
        write_comment(content, &format!("push {}", operation.context.label()))?;
        content.push_str("q\n");
        writeln!(
            content,
            "{} {} {} {} {} {} cm",
            operation.matrix.a,
            operation.matrix.b,
            operation.matrix.c,
            operation.matrix.d,
            operation.matrix.e,
            operation.matrix.f
        )
        .map_err(string_write_error)?;
        Ok(())
    }

    fn write_debug(
        &self,
        page: &RenderPage,
        operation: &DebugRenderOp,
        content: &mut String,
    ) -> Result<()> {
        let bounds = pdf_bounds(page, operation.context.frame);
        write_comment(content, &format!("debug {}", operation.context.label()))?;
        content.push_str("q\n");
        push_stroke_color(content, operation.color)?;
        writeln!(content, "0.75 w").map_err(string_write_error)?;
        writeln!(
            content,
            "{} {} {} {} re",
            bounds.origin.x, bounds.origin.y, bounds.size.width, bounds.size.height
        )
        .map_err(string_write_error)?;
        content.push_str("S\nQ\n");
        Ok(())
    }

    fn write_form(
        &mut self,
        page: &RenderPage,
        operation: &FormRenderOp,
        writer: &mut PdfWriter,
        content: &mut String,
    ) -> Result<()> {
        write_comment(
            content,
            &format!("form {} {}", operation.name, operation.context.label()),
        )?;
        content.push_str("q\n");
        for command in &operation.commands {
            self.encode_command(page, command, writer, content)?;
        }
        content.push_str("Q\n");
        Ok(())
    }

    fn ensure_font(
        &mut self,
        writer: &mut PdfWriter,
        descriptor: Option<&FontDescriptor>,
        source: Option<&FontSource>,
    ) -> String {
        let binding = FontBinding {
            descriptor: descriptor.cloned(),
            source: source.cloned(),
        };

        if let Some(name) = self.registered_fonts.get(&binding) {
            return name.clone();
        }

        let font_name = match source {
            Some(FontSource::Standard(font)) => self.standard_font_name(writer, *font),
            _ => match descriptor.and_then(resolve_standard_font) {
                Some(font) => self.standard_font_name(writer, font),
                None => self.fallback_font_name(writer),
            },
        };

        self.registered_fonts.insert(binding, font_name.clone());
        font_name
    }

    fn standard_font_name(&mut self, writer: &mut PdfWriter, font: StandardFont) -> String {
        if font == StandardFont::Helvetica {
            return String::from("F1");
        }

        let binding = FontBinding {
            descriptor: Some(
                FontDescriptor::new(font.family_name())
                    .with_style(font.font_style())
                    .with_weight(font.font_weight()),
            ),
            source: Some(FontSource::standard(font)),
        };

        if let Some(name) = self.registered_fonts.get(&binding) {
            return name.clone();
        }

        let name = writer.add_font(PdfFont::standard(font));
        self.registered_fonts.insert(binding, name.clone());
        name
    }

    fn fallback_font_name(&mut self, writer: &mut PdfWriter) -> String {
        if let Some(name) = &self.fallback_font_name {
            return name.clone();
        }

        let name = String::from("F1");
        let _ = writer;
        self.fallback_font_name = Some(name.clone());
        name
    }
}

fn resolve_image_source_blocking(source: AssetImageSource) -> Result<Arc<Image>> {
    if tokio::runtime::Handle::try_current().is_ok() {
        let join_handle = std::thread::spawn(move || -> Result<Arc<Image>> {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|error| Error::Backend {
                    message: format!("failed to build image resolution runtime: {error}"),
                })?;
            runtime.block_on(resolve_image(source)).map_err(Into::into)
        });

        return join_handle.join().map_err(|_| Error::Backend {
            message: String::from("image resolution thread panicked"),
        })?;
    }

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| Error::Backend {
            message: format!("failed to build image resolution runtime: {error}"),
        })?;

    runtime.block_on(resolve_image(source)).map_err(Into::into)
}

impl RenderBackend for PdfRenderBackend {
    type Output = Vec<u8>;

    fn render_document(&mut self, document: &RenderDocument) -> Result<Self::Output> {
        self.encode_document(document)
    }
}

pub fn render_to_bytes<T>(document: &T) -> Result<Vec<u8>>
where
    T: RendererDocumentSource + ?Sized,
{
    let render_document =
        document.build_render_document(&LayoutEngine::new(), &RenderEngine::new())?;
    let mut backend = PdfRenderBackend::default();
    backend.render_document(&render_document)
}

pub fn render_to_writer<T, W>(document: &T, mut writer: W) -> Result<()>
where
    T: RendererDocumentSource + ?Sized,
    W: Write,
{
    let bytes = render_to_bytes(document)?;
    writer.write_all(&bytes)?;
    Ok(())
}

pub fn render_to_file<T>(document: &T, path: impl AsRef<Path>) -> Result<()>
where
    T: RendererDocumentSource + ?Sized,
{
    let mut file = std::fs::File::create(path)?;
    render_to_writer(document, &mut file)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct FontBinding {
    descriptor: Option<FontDescriptor>,
    source: Option<FontSource>,
}

fn metadata_to_pdf(metadata: &LayoutMetadata) -> PdfMetadata {
    let mut pdf = PdfMetadata::new();
    pdf.title = metadata.title.clone();
    pdf.author = metadata.author.clone();
    pdf.subject = metadata.subject.clone();
    pdf.keywords = metadata.keywords.clone();
    pdf.creator = metadata.creator.clone();
    pdf.producer = metadata.producer.clone();
    pdf
}

fn pdf_bounds(page: &RenderPage, bounds: Bounds) -> Bounds {
    Bounds::from_origin_size(
        bounds.origin.x,
        page.size.height - bounds.origin.y - bounds.size.height,
        bounds.size.width,
        bounds.size.height,
    )
}

fn text_origin(page: &RenderPage, operation: &TextRenderOp) -> (f32, f32) {
    let x = operation.context.content_frame.origin.x;
    let y =
        page.size.height - operation.context.content_frame.origin.y - operation.font_size.value();
    (x, y)
}

fn write_comment(content: &mut String, comment: &str) -> Result<()> {
    writeln!(content, "% {comment}").map_err(string_write_error)
}

fn push_fill_color(content: &mut String, color: Color) -> Result<()> {
    let (r, g, b) = pdf_color(color);
    writeln!(content, "{r} {g} {b} rg").map_err(string_write_error)
}

fn push_stroke_color(content: &mut String, color: Color) -> Result<()> {
    let (r, g, b) = pdf_color(color);
    writeln!(content, "{r} {g} {b} RG").map_err(string_write_error)
}

fn pdf_color(color: Color) -> (f32, f32, f32) {
    (
        f32::from(color.red) / 255.0,
        f32::from(color.green) / 255.0,
        f32::from(color.blue) / 255.0,
    )
}

fn resolve_standard_font(descriptor: &FontDescriptor) -> Option<StandardFont> {
    let family = descriptor.family().trim();
    let is_bold = descriptor.font_weight() >= graphitepdf_font::FontWeight::BOLD;

    if family.eq_ignore_ascii_case("Helvetica") {
        return Some(match (descriptor.font_style(), is_bold) {
            (graphitepdf_font::FontStyle::Italic | graphitepdf_font::FontStyle::Oblique, true) => {
                StandardFont::HelveticaBoldOblique
            }
            (graphitepdf_font::FontStyle::Italic | graphitepdf_font::FontStyle::Oblique, false) => {
                StandardFont::HelveticaOblique
            }
            (_, true) => StandardFont::HelveticaBold,
            _ => StandardFont::Helvetica,
        });
    }

    if family.eq_ignore_ascii_case("Times")
        || family.eq_ignore_ascii_case("Times-Roman")
        || family.eq_ignore_ascii_case("Times New Roman")
    {
        return Some(match (descriptor.font_style(), is_bold) {
            (graphitepdf_font::FontStyle::Italic | graphitepdf_font::FontStyle::Oblique, true) => {
                StandardFont::TimesBoldItalic
            }
            (graphitepdf_font::FontStyle::Italic | graphitepdf_font::FontStyle::Oblique, false) => {
                StandardFont::TimesItalic
            }
            (_, true) => StandardFont::TimesBold,
            _ => StandardFont::TimesRoman,
        });
    }

    if family.eq_ignore_ascii_case("Courier") {
        return Some(match (descriptor.font_style(), is_bold) {
            (graphitepdf_font::FontStyle::Italic | graphitepdf_font::FontStyle::Oblique, true) => {
                StandardFont::CourierBoldOblique
            }
            (graphitepdf_font::FontStyle::Italic | graphitepdf_font::FontStyle::Oblique, false) => {
                StandardFont::CourierOblique
            }
            (_, true) => StandardFont::CourierBold,
            _ => StandardFont::Courier,
        });
    }

    if family.eq_ignore_ascii_case("Symbol") {
        return Some(StandardFont::Symbol);
    }

    if family.eq_ignore_ascii_case("ZapfDingbats") {
        return Some(StandardFont::ZapfDingbats);
    }

    None
}

fn escape_pdf_text(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            '(' => String::from("\\("),
            ')' => String::from("\\)"),
            '\\' => String::from("\\\\"),
            '\n' => String::from("\\n"),
            '\r' => String::from("\\r"),
            '\t' => String::from("\\t"),
            '\x08' => String::from("\\b"),
            '\x0c' => String::from("\\f"),
            _ => character.to_string(),
        })
        .collect()
}

fn string_write_error(error: std::fmt::Error) -> Error {
    Error::Backend {
        message: format!("failed to build PDF content stream: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use graphitepdf_image::{ImageFormat, RasterImage};
    use graphitepdf_layout::{Document, LayoutEngine, LayoutStyle, Node, Page};
    use graphitepdf_svg::parse_svg;
    use graphitepdf_textkit::{TextBlock, TextSpan};

    fn render_document(document: Document, options: RenderEngineOptions) -> RenderDocument {
        let layout = LayoutEngine::new()
            .layout_document(&document)
            .expect("document should layout");
        RenderEngine::new()
            .with_options(options)
            .build(&layout)
            .expect("document should render")
    }

    fn text_block(value: &str) -> TextBlock {
        TextBlock::from(TextSpan::new(value).expect("text span should be valid"))
    }

    #[test]
    fn renders_safe_layout_documents_with_page_backgrounds_forms_and_debug() {
        let document = Document::new().with_page(
            Page::new([
                Node::view([Node::text(text_block("Hello render"))]).with_style(
                    LayoutStyle::new().with_background_color(Color::rgb(0xee, 0xf2, 0xff)),
                ),
            ])
            .with_size(Size::new(220.0, 140.0))
            .with_style(
                LayoutStyle::new()
                    .with_padding(EdgeInsets::all(Pt::new(12.0)))
                    .with_background_color(Color::rgb(0x11, 0x22, 0x33)),
            ),
        );

        let rendered = render_document(
            document,
            RenderEngineOptions {
                wrap_views_in_forms: true,
                debug: Some(DebugRenderOptions::default()),
                ..RenderEngineOptions::default()
            },
        );

        assert_eq!(rendered.pages.len(), 1);
        assert_eq!(rendered.pages[0].size, Size::new(220.0, 140.0));
        assert!(matches!(
            &rendered.pages[0].commands[0],
            RenderCommand::FillRect(FillRectOp {
                role: PaintRole::PageBackground,
                color,
                ..
            }) if *color == Color::rgb(0x11, 0x22, 0x33)
        ));
        assert!(
            rendered.pages[0]
                .commands
                .iter()
                .any(|command| matches!(command, RenderCommand::DrawDebug(_)))
        );

        let form = rendered.pages[0]
            .commands
            .iter()
            .find_map(|command| match command {
                RenderCommand::DrawForm(form) => Some(form),
                _ => None,
            })
            .expect("view should render as form");
        assert!(
            form.commands
                .iter()
                .any(|command| matches!(command, RenderCommand::DrawText(_)))
        );
        assert!(
            form.commands
                .iter()
                .any(|command| matches!(command, RenderCommand::FillRect(_)))
        );
    }

    #[test]
    fn renders_text_images_svg_and_math_with_typed_operations() {
        let image = Image::Raster(RasterImage {
            width: 200,
            height: 100,
            data: vec![1, 2, 3, 4],
            format: ImageFormat::Png,
            key: Some(String::from("hero")),
        });
        let svg = parse_svg(r#"<svg viewBox="0 0 20 10"><rect width="20" height="10"/></svg>"#);
        let document = Document::new().with_page(
            Page::new([
                Node::text(text_block("Typed text")),
                Node::image_asset(image).with_style(LayoutStyle::new().with_width(Pt::new(50.0))),
                Node::svg(svg).with_style(LayoutStyle::new().with_width(Pt::new(40.0))),
                Node::math("x^2 + y^2").with_style(LayoutStyle::new().with_width(Pt::new(60.0))),
            ])
            .with_size(Size::new(240.0, 240.0))
            .with_style(LayoutStyle::new().with_padding(EdgeInsets::all(Pt::new(10.0)))),
        );

        let rendered = render_document(document, RenderEngineOptions::default());
        let commands = &rendered.pages[0].commands;

        assert!(commands.iter().any(|command| match command {
            RenderCommand::DrawText(operation) => {
                operation.text == "Typed text" && operation.layout.is_some()
            }
            _ => false,
        }));
        assert!(commands.iter().any(|command| match command {
            RenderCommand::DrawImage(operation) => {
                matches!(operation.source, RenderImageSource::Asset(_))
                    && operation.destination.size.width == 50.0
                    && operation.destination.size.height == 25.0
            }
            _ => false,
        }));
        assert!(
            commands
                .iter()
                .any(|command| matches!(command, RenderCommand::PushTransform(_)))
        );
        assert!(commands.iter().any(|command| match command {
            RenderCommand::DrawSvg(operation) =>
                matches!(operation.source, SvgRenderSource::Svg(_)),
            _ => false,
        }));
        assert!(commands.iter().any(|command| match command {
            RenderCommand::DrawSvg(operation) => {
                matches!(operation.source, SvgRenderSource::Math { .. })
            }
            _ => false,
        }));
    }

    #[test]
    fn supports_legacy_layout_documents_for_basic_text_and_box_rendering() {
        let legacy = LayoutEngine::new()
            .layout_text_block(Size::new(180.0, 60.0), text_block("Legacy text"))
            .expect("legacy layout should build");
        let rendered = RenderEngine::new()
            .build(&legacy)
            .expect("legacy layout should render");

        assert_eq!(rendered.pages.len(), 1);
        assert!(matches!(
            &rendered.pages[0].commands[0],
            RenderCommand::DrawText(TextRenderOp { text, layout, .. })
                if text == "Legacy text" && layout.is_none()
        ));
    }

    #[test]
    fn parses_colors_and_dimensions() {
        assert_eq!(
            parse_color("#1234").expect("short hex color should parse"),
            Color::rgba(0x11, 0x22, 0x33, 0x44)
        );
        assert_eq!(
            parse_color("rgba(255, 0, 0, 0.5)").expect("rgba color should parse"),
            Color::rgba(255, 0, 0, 128)
        );
        assert_eq!(
            parse_color("rgb(100%, 0%, 0%)").expect("percent rgb color should parse"),
            Color::rgb(255, 0, 0)
        );
        assert_eq!(parse_dimension("2.54cm").expect("cm should parse"), 72.0);
    }

    #[test]
    fn fits_objects_and_parses_transform_matrices() {
        let fitted = fit_object(
            Size::new(200.0, 100.0),
            Bounds::from_origin_size(0.0, 0.0, 50.0, 50.0),
            ObjectFit::Contain,
        );
        assert_eq!(fitted.bounds.size.width, 50.0);
        assert_eq!(fitted.bounds.size.height, 25.0);
        assert_eq!(fitted.bounds.origin.y, 12.5);

        let operations =
            parse_transform("translate(10, 20) scale(2) rotate(90)").expect("transform parses");
        assert_eq!(
            operations,
            vec![
                TransformOperation::Translate { x: 10.0, y: 20.0 },
                TransformOperation::Scale { x: 2.0, y: 2.0 },
                TransformOperation::Rotate {
                    degrees: 90.0,
                    cx: 0.0,
                    cy: 0.0,
                },
            ]
        );

        let matrix = compose_transform(&operations);
        assert!(!matrix.is_identity());
    }

    #[test]
    fn emits_border_commands_for_each_visible_side() {
        let context = RenderContext {
            page_index: 0,
            source_page_index: 0,
            path: vec![0],
            node_kind: RenderNodeKind::Box,
            z_index: 0,
            frame: Bounds::from_origin_size(10.0, 20.0, 30.0, 40.0),
            content_frame: Bounds::from_origin_size(10.0, 20.0, 30.0, 40.0),
        };
        let border = Border::all(BorderSide::new(
            Pt::new(2.0),
            Color::rgb(0x11, 0x22, 0x33),
            BorderStyle::Solid,
        ));

        let commands = border_commands(&context, &border);
        assert_eq!(commands.len(), 4);
        assert!(matches!(
            &commands[0],
            RenderCommand::StrokeBorder(BorderRenderOp {
                side: BorderSidePosition::Top,
                ..
            })
        ));
    }
}
