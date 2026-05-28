use std::collections::HashMap;
use std::fmt::Write as _;

use crate::error::{GraphitePdfKitError, Result};
use graphitepdf_math::MathRender;
use graphitepdf_svg::{SvgNode, SvgNodeKind};

const DEFAULT_VIEWPORT_SIZE: f64 = 100.0;
const DEFAULT_FONT_NAME: &str = "F1";
const DEFAULT_FONT_SIZE: f64 = 12.0;
const CIRCLE_BEZIER_KAPPA: f64 = 0.552_284_749_830_793_6;

#[derive(Clone, Debug, PartialEq)]
pub struct SvgRenderOptions {
    pub x: f64,
    pub y: f64,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub font_name: String,
    pub font_size: f64,
}

impl SvgRenderOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn font_name(mut self, font_name: impl Into<String>) -> Self {
        self.font_name = font_name.into();
        self
    }

    pub fn font_size(mut self, font_size: f64) -> Self {
        self.font_size = font_size;
        self
    }
}

impl Default for SvgRenderOptions {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: None,
            height: None,
            font_name: String::from(DEFAULT_FONT_NAME),
            font_size: DEFAULT_FONT_SIZE,
        }
    }
}

pub trait ToPdfPageContent {
    fn to_pdf_page_content(&self) -> Result<Vec<u8>> {
        self.to_pdf_page_content_with_options(&SvgRenderOptions::default())
    }

    fn to_pdf_page_content_with_options(&self, options: &SvgRenderOptions) -> Result<Vec<u8>>;
}

impl ToPdfPageContent for SvgNode {
    fn to_pdf_page_content_with_options(&self, options: &SvgRenderOptions) -> Result<Vec<u8>> {
        render_svg_node_to_page_content_with_options(self, options)
    }
}

impl ToPdfPageContent for MathRender {
    fn to_pdf_page_content_with_options(&self, options: &SvgRenderOptions) -> Result<Vec<u8>> {
        render_math_to_page_content_with_options(self, options)
    }
}

pub fn render_svg_node_to_page_content(svg: &SvgNode) -> Result<Vec<u8>> {
    render_svg_node_to_page_content_with_options(svg, &SvgRenderOptions::default())
}

pub fn render_svg_node_to_page_content_with_options(
    svg: &SvgNode,
    options: &SvgRenderOptions,
) -> Result<Vec<u8>> {
    if svg.kind != SvgNodeKind::Svg {
        return Err(GraphitePdfKitError::Render(
            "SVG page rendering requires an <svg> root node".to_string(),
        ));
    }

    let viewport = SvgViewport::from_node(svg, options)?;
    let mut content = String::new();

    content.push_str("q\n");
    push_matrix(&mut content, Transform::translate(options.x, options.y));
    push_matrix(
        &mut content,
        Transform::new(1.0, 0.0, 0.0, -1.0, 0.0, viewport.height),
    );
    push_matrix(
        &mut content,
        Transform::scale(
            viewport.width / viewport.view_box.width,
            viewport.height / viewport.view_box.height,
        ),
    );
    push_matrix(
        &mut content,
        Transform::translate(-viewport.view_box.min_x, -viewport.view_box.min_y),
    );

    let state = RenderState::from_root(svg, options);
    let definitions = collect_definitions(svg);
    render_node(svg, &state, &mut content, options, &definitions)?;

    content.push_str("Q\n");
    Ok(content.into_bytes())
}

pub fn render_math_to_page_content(math: &MathRender) -> Result<Vec<u8>> {
    render_math_to_page_content_with_options(math, &SvgRenderOptions::default())
}

pub fn render_math_to_page_content_with_options(
    math: &MathRender,
    options: &SvgRenderOptions,
) -> Result<Vec<u8>> {
    render_svg_node_to_page_content_with_options(&math.svg, options)
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SvgViewBox {
    min_x: f64,
    min_y: f64,
    width: f64,
    height: f64,
}

impl SvgViewBox {
    fn aspect_ratio(self) -> f64 {
        self.width / self.height
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SvgViewport {
    view_box: SvgViewBox,
    width: f64,
    height: f64,
}

type DefinitionMap<'a> = HashMap<&'a str, &'a SvgNode>;

impl SvgViewport {
    fn from_node(svg: &SvgNode, options: &SvgRenderOptions) -> Result<Self> {
        let mut width_hint = svg.props.get("width").and_then(|value| parse_length(value).ok());
        let mut height_hint = svg.props.get("height").and_then(|value| parse_length(value).ok());
        let view_box = if let Some(raw_view_box) = svg.props.get("viewBox") {
            parse_view_box(raw_view_box)?
        } else {
            let width = width_hint.unwrap_or(DEFAULT_VIEWPORT_SIZE);
            let height = height_hint.unwrap_or(DEFAULT_VIEWPORT_SIZE);
            SvgViewBox {
                min_x: 0.0,
                min_y: 0.0,
                width,
                height,
            }
        };

        if width_hint.is_none() {
            width_hint = Some(view_box.width);
        }
        if height_hint.is_none() {
            height_hint = Some(view_box.height);
        }

        let (width, height) = match (options.width, options.height) {
            (Some(width), Some(height)) => (width, height),
            (Some(width), None) => (width, width / view_box.aspect_ratio()),
            (None, Some(height)) => (height * view_box.aspect_ratio(), height),
            (None, None) => (
                width_hint.unwrap_or(DEFAULT_VIEWPORT_SIZE),
                height_hint.unwrap_or(DEFAULT_VIEWPORT_SIZE),
            ),
        };

        if width <= 0.0 || height <= 0.0 || view_box.width <= 0.0 || view_box.height <= 0.0 {
            return Err(GraphitePdfKitError::Render(
                "SVG dimensions must be positive".to_string(),
            ));
        }

        Ok(Self {
            view_box,
            width,
            height,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PdfColor {
    r: f64,
    g: f64,
    b: f64,
}

impl PdfColor {
    const BLACK: Self = Self::new(0.0, 0.0, 0.0);

    const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

#[derive(Clone, Debug)]
struct RenderState {
    fill: Option<PdfColor>,
    stroke: Option<PdfColor>,
    line_width: f64,
    line_cap: Option<u8>,
    line_join: Option<u8>,
    font_size: f64,
}

impl RenderState {
    fn from_root(svg: &SvgNode, options: &SvgRenderOptions) -> Self {
        let mut state = Self {
            fill: Some(PdfColor::BLACK),
            stroke: None,
            line_width: 1.0,
            line_cap: None,
            line_join: None,
            font_size: options.font_size,
        };

        state = state.inherit(svg);
        state
    }

    fn inherit(&self, node: &SvgNode) -> Self {
        let mut state = self.clone();

        if let Some(fill) = parse_paint_prop(node, "fill") {
            state.fill = fill;
        }
        if let Some(stroke) = parse_paint_prop(node, "stroke") {
            state.stroke = stroke;
        }
        if let Some(width) = parse_number_prop(node, "strokeWidth") {
            state.line_width = width.max(0.0);
        }
        if let Some(line_cap) = parse_line_cap_prop(node) {
            state.line_cap = Some(line_cap);
        }
        if let Some(line_join) = parse_line_join_prop(node) {
            state.line_join = Some(line_join);
        }
        if let Some(font_size) = parse_number_prop(node, "fontSize") {
            state.font_size = font_size.max(0.0);
        }

        state
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct TextCursor {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Transform {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
}

impl Transform {
    const fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    }

    const fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        Self { a, b, c, d, e, f }
    }

    const fn translate(tx: f64, ty: f64) -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0, tx, ty)
    }

    const fn scale(sx: f64, sy: f64) -> Self {
        Self::new(sx, 0.0, 0.0, sy, 0.0, 0.0)
    }

    fn rotate_degrees(angle_degrees: f64) -> Self {
        let radians = angle_degrees.to_radians();
        let cos = radians.cos();
        let sin = radians.sin();
        Self::new(cos, sin, -sin, cos, 0.0, 0.0)
    }

    fn skew_x_degrees(angle_degrees: f64) -> Self {
        Self::new(1.0, 0.0, angle_degrees.to_radians().tan(), 1.0, 0.0, 0.0)
    }

    fn skew_y_degrees(angle_degrees: f64) -> Self {
        Self::new(1.0, angle_degrees.to_radians().tan(), 0.0, 1.0, 0.0, 0.0)
    }

    fn multiply(self, other: Self) -> Self {
        Self {
            a: self.a * other.a + self.c * other.b,
            b: self.b * other.a + self.d * other.b,
            c: self.a * other.c + self.c * other.d,
            d: self.b * other.c + self.d * other.d,
            e: self.a * other.e + self.c * other.f + self.e,
            f: self.b * other.e + self.d * other.f + self.f,
        }
    }
}

fn render_node(
    node: &SvgNode,
    state: &RenderState,
    content: &mut String,
    options: &SvgRenderOptions,
    definitions: &DefinitionMap<'_>,
) -> Result<()> {
    let state = state.inherit(node);
    let transform = node
        .props
        .get("transform")
        .map(|value| parse_transform(value))
        .transpose()?;

    if let Some(transform) = transform {
        content.push_str("q\n");
        push_matrix(content, transform);
    }

    match node.kind {
        SvgNodeKind::Svg | SvgNodeKind::G => {
            for child in &node.children {
                render_node(child, &state, content, options, definitions)?;
            }
        }
        SvgNodeKind::Rect => render_rect(node, &state, content)?,
        SvgNodeKind::Circle => render_circle(node, &state, content)?,
        SvgNodeKind::Ellipse => render_ellipse(node, &state, content)?,
        SvgNodeKind::Line => render_line(node, &state, content)?,
        SvgNodeKind::Polyline => render_polyline(node, &state, content, false)?,
        SvgNodeKind::Polygon => render_polyline(node, &state, content, true)?,
        SvgNodeKind::Path => render_path(node, &state, content)?,
        SvgNodeKind::Text | SvgNodeKind::Tspan => {
            let _ = render_text_container(
                node,
                &state,
                content,
                options,
                definitions,
                TextCursor::default(),
            )?;
        }
        SvgNodeKind::Use => render_use(node, &state, content, options, definitions)?,
        SvgNodeKind::Defs
        | SvgNodeKind::ClipPath
        | SvgNodeKind::LinearGradient
        | SvgNodeKind::RadialGradient
        | SvgNodeKind::Marker
        | SvgNodeKind::Stop
        | SvgNodeKind::Image
        | SvgNodeKind::TextInstance => {}
    }

    if transform.is_some() {
        content.push_str("Q\n");
    }

    Ok(())
}

fn collect_definitions<'a>(node: &'a SvgNode) -> DefinitionMap<'a> {
    let mut definitions = HashMap::new();
    collect_definitions_into(node, &mut definitions);
    definitions
}

fn collect_definitions_into<'a>(node: &'a SvgNode, definitions: &mut DefinitionMap<'a>) {
    if let Some(id) = node.props.get("id") {
        definitions.insert(id.as_str(), node);
    }

    for child in &node.children {
        collect_definitions_into(child, definitions);
    }
}

fn render_use(
    node: &SvgNode,
    state: &RenderState,
    content: &mut String,
    options: &SvgRenderOptions,
    definitions: &DefinitionMap<'_>,
) -> Result<()> {
    let Some(reference) = extract_use_href(node) else {
        return Ok(());
    };
    let Some(target) = definitions.get(reference) else {
        return Ok(());
    };

    let translate_x = parse_number_prop(node, "x").unwrap_or(0.0);
    let translate_y = parse_number_prop(node, "y").unwrap_or(0.0);
    let needs_translation = translate_x != 0.0 || translate_y != 0.0;

    if needs_translation {
        content.push_str("q\n");
        push_matrix(content, Transform::translate(translate_x, translate_y));
    }

    render_node(target, state, content, options, definitions)?;

    if needs_translation {
        content.push_str("Q\n");
    }

    Ok(())
}

fn render_rect(node: &SvgNode, state: &RenderState, content: &mut String) -> Result<()> {
    let x = parse_number_prop(node, "x").unwrap_or(0.0);
    let y = parse_number_prop(node, "y").unwrap_or(0.0);
    let width = parse_number_prop(node, "width").unwrap_or(0.0);
    let height = parse_number_prop(node, "height").unwrap_or(0.0);

    if width <= 0.0 || height <= 0.0 {
        return Ok(());
    }

    content.push_str("q\n");
    apply_paint_state(content, state);
    let _ = writeln!(content, "{} {} {} {} re", format_number(x), format_number(y), format_number(width), format_number(height));
    apply_paint_operator(content, state, true);
    content.push_str("Q\n");
    Ok(())
}

fn render_circle(node: &SvgNode, state: &RenderState, content: &mut String) -> Result<()> {
    let cx = parse_number_prop(node, "cx").unwrap_or(0.0);
    let cy = parse_number_prop(node, "cy").unwrap_or(0.0);
    let r = parse_number_prop(node, "r").unwrap_or(0.0);

    if r <= 0.0 {
        return Ok(());
    }

    render_ellipse_segments(cx, cy, r, r, state, content);
    Ok(())
}

fn render_ellipse(node: &SvgNode, state: &RenderState, content: &mut String) -> Result<()> {
    let cx = parse_number_prop(node, "cx").unwrap_or(0.0);
    let cy = parse_number_prop(node, "cy").unwrap_or(0.0);
    let rx = parse_number_prop(node, "rx").unwrap_or(0.0);
    let ry = parse_number_prop(node, "ry").unwrap_or(0.0);

    if rx <= 0.0 || ry <= 0.0 {
        return Ok(());
    }

    render_ellipse_segments(cx, cy, rx, ry, state, content);
    Ok(())
}

fn render_ellipse_segments(
    cx: f64,
    cy: f64,
    rx: f64,
    ry: f64,
    state: &RenderState,
    content: &mut String,
) {
    let ox = rx * CIRCLE_BEZIER_KAPPA;
    let oy = ry * CIRCLE_BEZIER_KAPPA;

    content.push_str("q\n");
    apply_paint_state(content, state);
    let _ = writeln!(content, "{} {} m", format_number(cx + rx), format_number(cy));
    let _ = writeln!(
        content,
        "{} {} {} {} {} {} c",
        format_number(cx + rx),
        format_number(cy + oy),
        format_number(cx + ox),
        format_number(cy + ry),
        format_number(cx),
        format_number(cy + ry)
    );
    let _ = writeln!(
        content,
        "{} {} {} {} {} {} c",
        format_number(cx - ox),
        format_number(cy + ry),
        format_number(cx - rx),
        format_number(cy + oy),
        format_number(cx - rx),
        format_number(cy)
    );
    let _ = writeln!(
        content,
        "{} {} {} {} {} {} c",
        format_number(cx - rx),
        format_number(cy - oy),
        format_number(cx - ox),
        format_number(cy - ry),
        format_number(cx),
        format_number(cy - ry)
    );
    let _ = writeln!(
        content,
        "{} {} {} {} {} {} c",
        format_number(cx + ox),
        format_number(cy - ry),
        format_number(cx + rx),
        format_number(cy - oy),
        format_number(cx + rx),
        format_number(cy)
    );
    content.push_str("h\n");
    apply_paint_operator(content, state, true);
    content.push_str("Q\n");
}

fn render_line(node: &SvgNode, state: &RenderState, content: &mut String) -> Result<()> {
    let x1 = parse_number_prop(node, "x1").unwrap_or(0.0);
    let y1 = parse_number_prop(node, "y1").unwrap_or(0.0);
    let x2 = parse_number_prop(node, "x2").unwrap_or(0.0);
    let y2 = parse_number_prop(node, "y2").unwrap_or(0.0);

    content.push_str("q\n");
    apply_paint_state(content, state);
    let _ = writeln!(content, "{} {} m", format_number(x1), format_number(y1));
    let _ = writeln!(content, "{} {} l", format_number(x2), format_number(y2));
    apply_line_operator(content, state);
    content.push_str("Q\n");
    Ok(())
}

fn render_polyline(
    node: &SvgNode,
    state: &RenderState,
    content: &mut String,
    closed: bool,
) -> Result<()> {
    let Some(raw_points) = node.props.get("points") else {
        return Ok(());
    };
    let points = parse_points(raw_points)?;
    if points.len() < 2 {
        return Ok(());
    }

    content.push_str("q\n");
    apply_paint_state(content, state);
    let _ = writeln!(
        content,
        "{} {} m",
        format_number(points[0].0),
        format_number(points[0].1)
    );
    for (x, y) in points.iter().copied().skip(1) {
        let _ = writeln!(content, "{} {} l", format_number(x), format_number(y));
    }
    if closed {
        content.push_str("h\n");
        apply_paint_operator(content, state, true);
    } else {
        apply_line_operator(content, state);
    }
    content.push_str("Q\n");
    Ok(())
}

fn render_path(node: &SvgNode, state: &RenderState, content: &mut String) -> Result<()> {
    let Some(data) = node.props.get("d") else {
        return Ok(());
    };

    content.push_str("q\n");
    apply_paint_state(content, state);
    render_path_data(data, content)?;
    apply_paint_operator(content, state, false);
    content.push_str("Q\n");
    Ok(())
}

fn render_text_container(
    node: &SvgNode,
    state: &RenderState,
    content: &mut String,
    options: &SvgRenderOptions,
    definitions: &DefinitionMap<'_>,
    inherited_cursor: TextCursor,
) -> Result<TextCursor> {
    let mut cursor = inherited_cursor;

    if let Some(x) = parse_number_prop(node, "x") {
        cursor.x = x;
    }
    if let Some(y) = parse_number_prop(node, "y") {
        cursor.y = y;
    }
    if let Some(dx) = parse_number_prop(node, "dx") {
        cursor.x += dx;
    }
    if let Some(dy) = parse_number_prop(node, "dy") {
        cursor.y += dy;
    }

    for child in &node.children {
        match child.kind {
            SvgNodeKind::TextInstance => {
                if let Some(value) = child.value.as_deref() {
                    emit_text_run(content, value, cursor, state, options);
                    cursor.x += estimate_text_advance(value, state.font_size);
                }
            }
            SvgNodeKind::Tspan | SvgNodeKind::Text => {
                cursor = render_node_text_fragment(
                    child,
                    state,
                    content,
                    options,
                    definitions,
                    cursor,
                )?;
            }
            _ => render_node(child, state, content, options, definitions)?,
        }
    }

    Ok(cursor)
}

fn render_node_text_fragment(
    node: &SvgNode,
    state: &RenderState,
    content: &mut String,
    options: &SvgRenderOptions,
    definitions: &DefinitionMap<'_>,
    cursor: TextCursor,
) -> Result<TextCursor> {
    let state = state.inherit(node);
    let transform = node
        .props
        .get("transform")
        .map(|value| parse_transform(value))
        .transpose()?;

    if let Some(transform) = transform {
        content.push_str("q\n");
        push_matrix(content, transform);
    }

    let cursor = render_text_container(node, &state, content, options, definitions, cursor)?;

    if transform.is_some() {
        content.push_str("Q\n");
    }

    Ok(cursor)
}

fn emit_text_run(
    content: &mut String,
    text: &str,
    cursor: TextCursor,
    state: &RenderState,
    options: &SvgRenderOptions,
) {
    if text.is_empty() || (state.fill.is_none() && state.stroke.is_none()) {
        return;
    }

    content.push_str("BT\n");
    let _ = writeln!(
        content,
        "/{} {} Tf",
        options.font_name,
        format_number(state.font_size.max(0.001))
    );
    match (state.fill, state.stroke) {
        (Some(fill), Some(stroke)) => {
            let _ = writeln!(
                content,
                "{} {} {} rg",
                format_number(fill.r),
                format_number(fill.g),
                format_number(fill.b)
            );
            let _ = writeln!(
                content,
                "{} {} {} RG",
                format_number(stroke.r),
                format_number(stroke.g),
                format_number(stroke.b)
            );
            content.push_str("2 Tr\n");
        }
        (Some(fill), None) => {
            let _ = writeln!(
                content,
                "{} {} {} rg",
                format_number(fill.r),
                format_number(fill.g),
                format_number(fill.b)
            );
            content.push_str("0 Tr\n");
        }
        (None, Some(stroke)) => {
            let _ = writeln!(
                content,
                "{} {} {} RG",
                format_number(stroke.r),
                format_number(stroke.g),
                format_number(stroke.b)
            );
            content.push_str("1 Tr\n");
        }
        (None, None) => {}
    }
    let _ = writeln!(
        content,
        "1 0 0 -1 {} {} Tm",
        format_number(cursor.x),
        format_number(cursor.y)
    );
    let _ = writeln!(content, "({}) Tj", escape_pdf_text(text));
    content.push_str("ET\n");
}

fn apply_paint_state(content: &mut String, state: &RenderState) {
    if let Some(fill) = state.fill {
        let _ = writeln!(
            content,
            "{} {} {} rg",
            format_number(fill.r),
            format_number(fill.g),
            format_number(fill.b)
        );
    }
    if let Some(stroke) = state.stroke {
        let _ = writeln!(
            content,
            "{} {} {} RG",
            format_number(stroke.r),
            format_number(stroke.g),
            format_number(stroke.b)
        );
        let _ = writeln!(content, "{} w", format_number(state.line_width));
    }
    if let Some(line_cap) = state.line_cap {
        let _ = writeln!(content, "{} J", line_cap);
    }
    if let Some(line_join) = state.line_join {
        let _ = writeln!(content, "{} j", line_join);
    }
}

fn apply_paint_operator(content: &mut String, state: &RenderState, closed: bool) {
    match (state.fill.is_some(), state.stroke.is_some()) {
        (true, true) => content.push_str(if closed { "B\n" } else { "B\n" }),
        (true, false) => content.push_str(if closed { "f\n" } else { "f\n" }),
        (false, true) => content.push_str(if closed { "S\n" } else { "S\n" }),
        (false, false) => content.push_str("n\n"),
    }
}

fn apply_line_operator(content: &mut String, state: &RenderState) {
    if state.stroke.is_some() {
        content.push_str("S\n");
    } else {
        content.push_str("n\n");
    }
}

fn parse_paint_prop(node: &SvgNode, key: &str) -> Option<Option<PdfColor>> {
    let value = node.props.get(key)?;
    if value.eq_ignore_ascii_case("none") {
        return Some(None);
    }
    parse_color(value).map(Some)
}

fn parse_line_cap_prop(node: &SvgNode) -> Option<u8> {
    match node.props.get("strokeLinecap")?.as_str() {
        "butt" => Some(0),
        "round" => Some(1),
        "square" => Some(2),
        _ => None,
    }
}

fn parse_line_join_prop(node: &SvgNode) -> Option<u8> {
    match node.props.get("strokeLinejoin")?.as_str() {
        "miter" => Some(0),
        "round" => Some(1),
        "bevel" => Some(2),
        _ => None,
    }
}

fn parse_number_prop(node: &SvgNode, key: &str) -> Option<f64> {
    node.props.get(key).and_then(|value| parse_length(value).ok())
}

fn parse_length(value: &str) -> Result<f64> {
    parse_number(value)
}

fn parse_number(value: &str) -> Result<f64> {
    let trimmed = value.trim();
    let mut end = 0usize;
    let mut seen_digit = false;
    let mut seen_decimal = false;
    let mut seen_exponent = false;

    for (index, character) in trimmed.char_indices() {
        let is_first = index == 0;
        if character.is_ascii_digit() {
            seen_digit = true;
            end = index + character.len_utf8();
            continue;
        }
        if (character == '+' || character == '-') && (is_first || matches!(trimmed[..index].chars().last(), Some('e' | 'E'))) {
            end = index + character.len_utf8();
            continue;
        }
        if character == '.' && !seen_decimal && !seen_exponent {
            seen_decimal = true;
            end = index + character.len_utf8();
            continue;
        }
        if (character == 'e' || character == 'E') && seen_digit && !seen_exponent {
            seen_exponent = true;
            seen_decimal = false;
            end = index + character.len_utf8();
            continue;
        }
        break;
    }

    if !seen_digit || end == 0 {
        return Err(GraphitePdfKitError::Render(format!(
            "invalid SVG numeric value `{trimmed}`"
        )));
    }

    trimmed[..end].parse::<f64>().map_err(|_| {
        GraphitePdfKitError::Render(format!("invalid SVG numeric value `{trimmed}`"))
    })
}

fn parse_view_box(value: &str) -> Result<SvgViewBox> {
    let values: Vec<f64> = value
        .split(|character: char| character.is_ascii_whitespace() || character == ',')
        .filter(|part| !part.is_empty())
        .map(parse_number)
        .collect::<Result<Vec<_>>>()?;

    if values.len() != 4 || values[2] <= 0.0 || values[3] <= 0.0 {
        return Err(GraphitePdfKitError::Render(format!(
            "invalid SVG viewBox `{value}`"
        )));
    }

    Ok(SvgViewBox {
        min_x: values[0],
        min_y: values[1],
        width: values[2],
        height: values[3],
    })
}

fn parse_points(value: &str) -> Result<Vec<(f64, f64)>> {
    let numbers = tokenize_numbers(value)?;
    if numbers.len() < 2 {
        return Ok(Vec::new());
    }
    if numbers.len() % 2 != 0 {
        return Err(GraphitePdfKitError::Render(format!(
            "invalid SVG points list `{value}`"
        )));
    }

    Ok(numbers
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect())
}

fn tokenize_numbers(value: &str) -> Result<Vec<f64>> {
    let mut numbers = Vec::new();
    let mut index = 0usize;
    let bytes = value.as_bytes();

    while index < value.len() {
        let character = bytes[index] as char;
        if character.is_ascii_whitespace() || character == ',' {
            index += 1;
            continue;
        }

        let start = index;
        let mut end = index;
        let mut seen_decimal = false;
        let mut seen_exponent = false;

        while end < value.len() {
            let current = bytes[end] as char;
            let previous = if end > start {
                Some(bytes[end - 1] as char)
            } else {
                None
            };

            let is_sign = current == '+' || current == '-';
            if current.is_ascii_digit()
                || (current == '.' && !seen_decimal && !seen_exponent)
                || (current == 'e' || current == 'E') && !seen_exponent
                || (is_sign && end == start)
                || (is_sign && matches!(previous, Some('e' | 'E')))
            {
                if current == '.' {
                    seen_decimal = true;
                } else if current == 'e' || current == 'E' {
                    seen_exponent = true;
                    seen_decimal = false;
                }
                end += 1;
                continue;
            }

            break;
        }

        if start == end {
            return Err(GraphitePdfKitError::Render(format!(
                "invalid SVG numeric token near `{}`",
                &value[index..]
            )));
        }

        numbers.push(value[start..end].parse::<f64>().map_err(|_| {
            GraphitePdfKitError::Render(format!(
                "invalid SVG numeric token `{}`",
                &value[start..end]
            ))
        })?);
        index = end;
    }

    Ok(numbers)
}

fn parse_transform(value: &str) -> Result<Transform> {
    let mut remainder = value.trim();
    let mut transform = Transform::identity();

    while !remainder.is_empty() {
        let Some(open) = remainder.find('(') else {
            break;
        };
        let name = remainder[..open].trim();
        let after_open = &remainder[open + 1..];
        let Some(close) = after_open.find(')') else {
            return Err(GraphitePdfKitError::Render(format!(
                "invalid SVG transform `{value}`"
            )));
        };
        let args = tokenize_numbers(&after_open[..close])?;
        let next = match name {
            "translate" => {
                let tx = args.first().copied().unwrap_or(0.0);
                let ty = args.get(1).copied().unwrap_or(0.0);
                Transform::translate(tx, ty)
            }
            "scale" => {
                let sx = args.first().copied().unwrap_or(1.0);
                let sy = args.get(1).copied().unwrap_or(sx);
                Transform::scale(sx, sy)
            }
            "matrix" if args.len() == 6 => {
                Transform::new(args[0], args[1], args[2], args[3], args[4], args[5])
            }
            "rotate" => match args.as_slice() {
                [angle] => Transform::rotate_degrees(*angle),
                [angle, cx, cy] => Transform::translate(*cx, *cy)
                    .multiply(Transform::rotate_degrees(*angle))
                    .multiply(Transform::translate(-*cx, -*cy)),
                _ => {
                    return Err(GraphitePdfKitError::Render(format!(
                        "invalid rotate transform `{value}`"
                    )));
                }
            },
            "skewX" if args.len() == 1 => Transform::skew_x_degrees(args[0]),
            "skewY" if args.len() == 1 => Transform::skew_y_degrees(args[0]),
            _ => {
                return Err(GraphitePdfKitError::Render(format!(
                    "unsupported SVG transform `{name}`"
                )));
            }
        };

        transform = transform.multiply(next);
        remainder = after_open[close + 1..].trim_start();
    }

    Ok(transform)
}

fn push_matrix(content: &mut String, matrix: Transform) {
    let _ = writeln!(
        content,
        "{} {} {} {} {} {} cm",
        format_number(matrix.a),
        format_number(matrix.b),
        format_number(matrix.c),
        format_number(matrix.d),
        format_number(matrix.e),
        format_number(matrix.f)
    );
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PathToken {
    Command(char),
    Number(f64),
}

fn tokenize_path_data(data: &str) -> Result<Vec<PathToken>> {
    let mut tokens = Vec::new();
    let bytes = data.as_bytes();
    let mut index = 0usize;

    while index < data.len() {
        let character = bytes[index] as char;
        if character.is_ascii_whitespace() || character == ',' {
            index += 1;
            continue;
        }
        if character.is_ascii_alphabetic() {
            tokens.push(PathToken::Command(character));
            index += 1;
            continue;
        }

        let start = index;
        let mut end = index;
        let mut seen_decimal = false;
        let mut seen_exponent = false;

        while end < data.len() {
            let current = bytes[end] as char;
            let previous = if end > start {
                Some(bytes[end - 1] as char)
            } else {
                None
            };
            let is_sign = current == '+' || current == '-';
            let can_continue = current.is_ascii_digit()
                || (current == '.' && !seen_decimal && !seen_exponent)
                || ((current == 'e' || current == 'E') && !seen_exponent)
                || (is_sign && end == start)
                || (is_sign && matches!(previous, Some('e' | 'E')));

            if !can_continue {
                break;
            }

            if current == '.' {
                seen_decimal = true;
            } else if current == 'e' || current == 'E' {
                seen_exponent = true;
                seen_decimal = false;
            }
            end += 1;
        }

        if start == end {
            return Err(GraphitePdfKitError::Render(format!(
                "invalid SVG path token near `{}`",
                &data[index..]
            )));
        }

        let number = data[start..end].parse::<f64>().map_err(|_| {
            GraphitePdfKitError::Render(format!(
                "invalid SVG path number `{}`",
                &data[start..end]
            ))
        })?;
        tokens.push(PathToken::Number(number));
        index = end;
    }

    Ok(tokens)
}

fn render_path_data(data: &str, content: &mut String) -> Result<()> {
    let tokens = tokenize_path_data(data)?;
    let mut index = 0usize;
    let mut current = (0.0, 0.0);
    let mut subpath_start = (0.0, 0.0);
    let mut last_command = 'M';
    let mut last_cubic_ctrl: Option<(f64, f64)> = None;
    let mut last_quad_ctrl: Option<(f64, f64)> = None;

    while index < tokens.len() {
        let command = match tokens[index] {
            PathToken::Command(command) => {
                index += 1;
                last_command = command;
                command
            }
            PathToken::Number(_) => last_command,
        };

        let relative = command.is_ascii_lowercase();
        match command.to_ascii_uppercase() {
            'M' => {
                let first = next_point(&tokens, &mut index, relative, current)?;
                current = first;
                subpath_start = first;
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
                let _ = writeln!(
                    content,
                    "{} {} m",
                    format_number(current.0),
                    format_number(current.1)
                );

                while has_number(&tokens, index) {
                    current = next_point(&tokens, &mut index, relative, current)?;
                    let _ = writeln!(
                        content,
                        "{} {} l",
                        format_number(current.0),
                        format_number(current.1)
                    );
                }
            }
            'L' => {
                while has_number(&tokens, index) {
                    current = next_point(&tokens, &mut index, relative, current)?;
                    last_cubic_ctrl = None;
                    last_quad_ctrl = None;
                    let _ = writeln!(
                        content,
                        "{} {} l",
                        format_number(current.0),
                        format_number(current.1)
                    );
                }
            }
            'H' => {
                while has_number(&tokens, index) {
                    let value = next_number(&tokens, &mut index)?;
                    current.0 = if relative { current.0 + value } else { value };
                    last_cubic_ctrl = None;
                    last_quad_ctrl = None;
                    let _ = writeln!(
                        content,
                        "{} {} l",
                        format_number(current.0),
                        format_number(current.1)
                    );
                }
            }
            'V' => {
                while has_number(&tokens, index) {
                    let value = next_number(&tokens, &mut index)?;
                    current.1 = if relative { current.1 + value } else { value };
                    last_cubic_ctrl = None;
                    last_quad_ctrl = None;
                    let _ = writeln!(
                        content,
                        "{} {} l",
                        format_number(current.0),
                        format_number(current.1)
                    );
                }
            }
            'C' => {
                while has_number(&tokens, index) {
                    let control_1 = next_point(&tokens, &mut index, relative, current)?;
                    let control_2 = next_point(&tokens, &mut index, relative, current)?;
                    let end = next_point(&tokens, &mut index, relative, current)?;
                    let _ = writeln!(
                        content,
                        "{} {} {} {} {} {} c",
                        format_number(control_1.0),
                        format_number(control_1.1),
                        format_number(control_2.0),
                        format_number(control_2.1),
                        format_number(end.0),
                        format_number(end.1)
                    );
                    current = end;
                    last_cubic_ctrl = Some(control_2);
                    last_quad_ctrl = None;
                }
            }
            'S' => {
                while has_number(&tokens, index) {
                    let control_1 = reflect_control_point(last_cubic_ctrl, current);
                    let control_2 = next_point(&tokens, &mut index, relative, current)?;
                    let end = next_point(&tokens, &mut index, relative, current)?;
                    let _ = writeln!(
                        content,
                        "{} {} {} {} {} {} c",
                        format_number(control_1.0),
                        format_number(control_1.1),
                        format_number(control_2.0),
                        format_number(control_2.1),
                        format_number(end.0),
                        format_number(end.1)
                    );
                    current = end;
                    last_cubic_ctrl = Some(control_2);
                    last_quad_ctrl = None;
                }
            }
            'Q' => {
                while has_number(&tokens, index) {
                    let control = next_point(&tokens, &mut index, relative, current)?;
                    let end = next_point(&tokens, &mut index, relative, current)?;
                    let cubic_1 = (
                        current.0 + (control.0 - current.0) * (2.0 / 3.0),
                        current.1 + (control.1 - current.1) * (2.0 / 3.0),
                    );
                    let cubic_2 = (
                        end.0 + (control.0 - end.0) * (2.0 / 3.0),
                        end.1 + (control.1 - end.1) * (2.0 / 3.0),
                    );
                    let _ = writeln!(
                        content,
                        "{} {} {} {} {} {} c",
                        format_number(cubic_1.0),
                        format_number(cubic_1.1),
                        format_number(cubic_2.0),
                        format_number(cubic_2.1),
                        format_number(end.0),
                        format_number(end.1)
                    );
                    current = end;
                    last_cubic_ctrl = Some(cubic_2);
                    last_quad_ctrl = Some(control);
                }
            }
            'T' => {
                while has_number(&tokens, index) {
                    let control = reflect_control_point(last_quad_ctrl, current);
                    let end = next_point(&tokens, &mut index, relative, current)?;
                    let cubic_1 = (
                        current.0 + (control.0 - current.0) * (2.0 / 3.0),
                        current.1 + (control.1 - current.1) * (2.0 / 3.0),
                    );
                    let cubic_2 = (
                        end.0 + (control.0 - end.0) * (2.0 / 3.0),
                        end.1 + (control.1 - end.1) * (2.0 / 3.0),
                    );
                    let _ = writeln!(
                        content,
                        "{} {} {} {} {} {} c",
                        format_number(cubic_1.0),
                        format_number(cubic_1.1),
                        format_number(cubic_2.0),
                        format_number(cubic_2.1),
                        format_number(end.0),
                        format_number(end.1)
                    );
                    current = end;
                    last_cubic_ctrl = Some(cubic_2);
                    last_quad_ctrl = Some(control);
                }
            }
            'Z' => {
                current = subpath_start;
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
                content.push_str("h\n");
            }
            unsupported => {
                return Err(GraphitePdfKitError::Render(format!(
                    "unsupported SVG path command `{unsupported}`"
                )));
            }
        }
    }

    Ok(())
}

fn extract_use_href<'a>(node: &'a SvgNode) -> Option<&'a str> {
    node.props
        .get("href")
        .or_else(|| node.props.get("xlinkHref"))
        .and_then(|value| value.strip_prefix('#'))
}

fn has_number(tokens: &[PathToken], index: usize) -> bool {
    matches!(tokens.get(index), Some(PathToken::Number(_)))
}

fn next_number(tokens: &[PathToken], index: &mut usize) -> Result<f64> {
    match tokens.get(*index) {
        Some(PathToken::Number(value)) => {
            *index += 1;
            Ok(*value)
        }
        _ => Err(GraphitePdfKitError::Render(
            "invalid SVG path command sequence".to_string(),
        )),
    }
}

fn next_point(
    tokens: &[PathToken],
    index: &mut usize,
    relative: bool,
    current: (f64, f64),
) -> Result<(f64, f64)> {
    let x = next_number(tokens, index)?;
    let y = next_number(tokens, index)?;
    Ok(if relative {
        (current.0 + x, current.1 + y)
    } else {
        (x, y)
    })
}

fn reflect_control_point(control: Option<(f64, f64)>, current: (f64, f64)) -> (f64, f64) {
    control.map_or(current, |(x, y)| (2.0 * current.0 - x, 2.0 * current.1 - y))
}

fn parse_color(value: &str) -> Option<PdfColor> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }

    if let Some(hex) = value.strip_prefix('#') {
        return parse_hex_color(hex);
    }

    let lowercase = value.to_ascii_lowercase();
    if lowercase.starts_with("rgb(") && lowercase.ends_with(')') {
        return parse_rgb_function(&lowercase[4..lowercase.len() - 1]);
    }

    match lowercase.as_str() {
        "black" => Some(PdfColor::new(0.0, 0.0, 0.0)),
        "white" => Some(PdfColor::new(1.0, 1.0, 1.0)),
        "red" => Some(PdfColor::new(1.0, 0.0, 0.0)),
        "green" => Some(PdfColor::new(0.0, 0.5, 0.0)),
        "blue" => Some(PdfColor::new(0.0, 0.0, 1.0)),
        "yellow" => Some(PdfColor::new(1.0, 1.0, 0.0)),
        "purple" => Some(PdfColor::new(0.5, 0.0, 0.5)),
        "gray" | "grey" => Some(PdfColor::new(0.5, 0.5, 0.5)),
        "orange" => Some(PdfColor::new(1.0, 0.647, 0.0)),
        "rebeccapurple" => Some(PdfColor::new(0.4, 0.2, 0.6)),
        _ => None,
    }
}

fn parse_hex_color(hex: &str) -> Option<PdfColor> {
    let (r, g, b) = match hex.len() {
        3 => (
            u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?,
            u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?,
            u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?,
        ),
        4 => (
            u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?,
            u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?,
            u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?,
        ),
        6 | 8 => (
            u8::from_str_radix(&hex[0..2], 16).ok()?,
            u8::from_str_radix(&hex[2..4], 16).ok()?,
            u8::from_str_radix(&hex[4..6], 16).ok()?,
        ),
        _ => return None,
    };

    Some(PdfColor::new(
        f64::from(r) / 255.0,
        f64::from(g) / 255.0,
        f64::from(b) / 255.0,
    ))
}

fn parse_rgb_function(values: &str) -> Option<PdfColor> {
    let parts: Vec<&str> = values
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect();
    if parts.len() != 3 {
        return None;
    }

    fn parse_component(value: &str) -> Option<f64> {
        if let Some(percent) = value.strip_suffix('%') {
            let value = percent.parse::<f64>().ok()?;
            Some((value / 100.0).clamp(0.0, 1.0))
        } else {
            let value = value.parse::<f64>().ok()?;
            Some((value / 255.0).clamp(0.0, 1.0))
        }
    }

    Some(PdfColor::new(
        parse_component(parts[0])?,
        parse_component(parts[1])?,
        parse_component(parts[2])?,
    ))
}

fn estimate_text_advance(text: &str, font_size: f64) -> f64 {
    text.chars().count() as f64 * font_size * 0.6
}

fn escape_pdf_text(text: &str) -> String {
    text.chars()
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

fn format_number(value: f64) -> String {
    let rounded = (value * 1000.0).round() / 1000.0;
    let mut rendered = format!("{rounded:.3}");

    while rendered.contains('.') && rendered.ends_with('0') {
        rendered.pop();
    }
    if rendered.ends_with('.') {
        rendered.pop();
    }
    if rendered == "-0" {
        String::from("0")
    } else {
        rendered
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphitepdf_math::{MathOptions, render_math_with_options};
    use graphitepdf_svg::parse_svg;

    #[test]
    fn renders_svg_node_to_pdf_page_content() {
        let svg = parse_svg(
            r##"
            <svg xmlns="http://www.w3.org/2000/svg" width="120" height="80" viewBox="0 0 120 80">
              <rect x="10" y="10" width="50" height="20" fill="#336699"/>
              <path d="M70 10 L110 10 L90 40 Z" fill="none" stroke="red" stroke-width="2"/>
              <text x="15" y="55" font-size="14" fill="blue">Hi</text>
            </svg>"##,
        );

        let rendered = render_svg_node_to_page_content_with_options(
            &svg,
            &SvgRenderOptions::new()
                .position(24.0, 48.0)
                .font_name("F1"),
        )
        .expect("svg page content should render");
        let content = String::from_utf8(rendered).expect("content should be valid ASCII");

        assert!(content.contains("24 48 cm"));
        assert!(content.contains("10 10 50 20 re"));
        assert!(content.contains("0.2 0.4 0.6 rg"));
        assert!(content.contains("1 0 0 RG"));
        assert!(content.contains("/F1 14 Tf"));
        assert!(content.contains("(Hi) Tj"));
    }

    #[test]
    fn renders_math_render_to_pdf_page_content_with_trait() {
        let math = render_math_with_options(
            r"\int_0^1 x^2 \, dx",
            &MathOptions::new().color("rebeccapurple").height(36.0),
        )
        .expect("math should render");

        let rendered = math
            .to_pdf_page_content_with_options(&SvgRenderOptions::new().position(18.0, 32.0))
            .expect("math should convert to page content");
        let content = String::from_utf8(rendered).expect("content should be valid ASCII");

        assert!(content.starts_with("q\n"));
        assert!(content.contains("18 32 cm"));
        assert!(content.contains(" c\n") || content.contains(" l\n"));
        assert!(content.ends_with("Q\n"));
    }

    #[test]
    fn preserves_aspect_ratio_when_only_one_dimension_is_overridden() {
        let svg = parse_svg(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="160" height="80" viewBox="0 0 160 80">
                <rect x="0" y="0" width="160" height="80" fill="black"/>
            </svg>"#,
        );

        let rendered = render_svg_node_to_page_content_with_options(
            &svg,
            &SvgRenderOptions::new().width(200.0),
        )
        .expect("svg should render");
        let content = String::from_utf8(rendered).expect("content should decode");

        assert!(content.contains("1 0 0 -1 0 100 cm"));
        assert!(content.contains("1.25 0 0 1.25 0 0 cm"));
    }
}
