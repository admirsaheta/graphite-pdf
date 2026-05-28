pub mod error;

pub use error::*;

use indexmap::IndexMap;

const DEFAULT_DPI: f64 = 72.0;
const DEFAULT_REM_BASE: f64 = 18.0;
const MM_PER_INCH: f64 = 25.4;
const CM_PER_INCH: f64 = 2.54;

pub type Style = IndexMap<String, StyleValue>;
pub type SafeStyle = Style;
pub type ExpandedStyle = Style;

#[derive(Clone, Debug, PartialEq)]
pub enum StyleValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<StyleValue>),
    Object(Style),
}

impl StyleValue {
    pub fn as_object(&self) -> Option<&Style> {
        match self {
            Self::Object(style) => Some(style),
            _ => None,
        }
    }

    fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(number) => Some(*number),
            Self::String(text) => parse_float_like(text),
            _ => None,
        }
    }

    fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(text) => Some(text),
            _ => None,
        }
    }
}

impl Default for StyleValue {
    fn default() -> Self {
        Self::Object(Style::new())
    }
}

impl From<bool> for StyleValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for StyleValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<f32> for StyleValue {
    fn from(value: f32) -> Self {
        Self::Number(f64::from(value))
    }
}

impl From<i64> for StyleValue {
    fn from(value: i64) -> Self {
        Self::Number(value as f64)
    }
}

impl From<i32> for StyleValue {
    fn from(value: i32) -> Self {
        Self::Number(f64::from(value))
    }
}

impl From<usize> for StyleValue {
    fn from(value: usize) -> Self {
        Self::Number(value as f64)
    }
}

impl From<String> for StyleValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for StyleValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<Vec<StyleValue>> for StyleValue {
    fn from(value: Vec<StyleValue>) -> Self {
        Self::Array(value)
    }
}

impl From<Style> for StyleValue {
    fn from(value: Style) -> Self {
        Self::Object(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Landscape,
    Portrait,
}

impl Orientation {
    fn from_str(value: &str) -> Option<Self> {
        match value.trim() {
            "landscape" => Some(Self::Landscape),
            "portrait" => Some(Self::Portrait),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Container {
    pub width: f64,
    pub height: f64,
    pub dpi: Option<f64>,
    pub rem_base: Option<f64>,
    pub orientation: Option<Orientation>,
}

impl Container {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            dpi: None,
            rem_base: None,
            orientation: None,
        }
    }

    fn resolved_orientation(self) -> Orientation {
        self.orientation.unwrap_or_else(|| {
            if self.width > self.height {
                Orientation::Landscape
            } else {
                Orientation::Portrait
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stylesheet {
    input: StyleValue,
}

impl Default for Stylesheet {
    fn default() -> Self {
        Self {
            input: StyleValue::Object(Style::new()),
        }
    }
}

impl Stylesheet {
    pub fn new(input: impl Into<StyleValue>) -> Self {
        Self {
            input: input.into(),
        }
    }

    pub fn input(&self) -> &StyleValue {
        &self.input
    }

    pub fn is_empty(&self) -> bool {
        match &self.input {
            StyleValue::Null => true,
            StyleValue::Array(items) => items.is_empty(),
            StyleValue::Object(style) => style.is_empty(),
            _ => false,
        }
    }

    pub fn resolve(&self, container: &Container) -> Style {
        resolve_styles(container, &self.input)
    }
}

pub fn flatten(input: &StyleValue) -> Style {
    let mut flattened = Style::new();
    flatten_into(input, &mut flattened);
    flattened
}

pub fn resolve_media_queries(container: &Container, style: &Style) -> Style {
    let mut resolved = Style::new();

    for (key, value) in style {
        if key.starts_with("@media") {
            if matches_media_query(container, key) {
                if let StyleValue::Object(media_style) = value {
                    merge_style(&mut resolved, media_style.clone());
                }
            }
        } else {
            resolved.insert(key.clone(), value.clone());
        }
    }

    resolved
}

pub fn resolve_style(container: &Container, style: &Style) -> Style {
    let mut resolved = Style::new();

    for (key, value) in style {
        if key.starts_with("@media") {
            continue;
        }

        let expanded = resolve_property(key, value, container, style);
        merge_style(&mut resolved, expanded);
    }

    resolved
}

pub fn resolve_styles(container: &Container, input: &StyleValue) -> Style {
    let flattened = flatten(input);
    let media_resolved = resolve_media_queries(container, &flattened);
    resolve_style(container, &media_resolved)
}

pub fn transform_color(value: &str) -> String {
    let trimmed = value.trim();

    if let Some(hex) = parse_rgb_color(trimmed) {
        return hex;
    }

    if let Some(hex) = parse_hsl_color(trimmed) {
        return hex;
    }

    trimmed.to_string()
}

fn flatten_into(input: &StyleValue, flattened: &mut Style) {
    match input {
        StyleValue::Null => {}
        StyleValue::Array(items) => {
            for item in items {
                flatten_into(item, flattened);
            }
        }
        StyleValue::Object(style) => {
            for (key, value) in style {
                if !matches!(value, StyleValue::Null) {
                    flattened.insert(key.clone(), value.clone());
                }
            }
        }
        StyleValue::Bool(_) | StyleValue::Number(_) | StyleValue::String(_) => {}
    }
}

fn merge_style(target: &mut Style, source: Style) {
    for (key, value) in source {
        target.insert(key, value);
    }
}

fn style_with(key: impl Into<String>, value: impl Into<StyleValue>) -> Style {
    let mut style = Style::new();
    style.insert(key.into(), value.into());
    style
}

fn resolve_property(key: &str, value: &StyleValue, container: &Container, style: &Style) -> Style {
    match key {
        "backgroundColor" | "color" | "textDecorationColor" | "fill" | "stroke" => {
            process_color_value(key, value)
        }
        "opacity" | "fillOpacity" | "strokeOpacity" | "aspectRatio" | "zIndex" | "maxLines"
        | "flexGrow" | "flexShrink" => process_number_value(key, value),
        "height"
        | "maxHeight"
        | "maxWidth"
        | "minHeight"
        | "minWidth"
        | "width"
        | "bottom"
        | "left"
        | "right"
        | "top"
        | "fontSize"
        | "letterSpacing"
        | "strokeWidth"
        | "borderBottomLeftRadius"
        | "borderBottomRightRadius"
        | "borderBottomWidth"
        | "borderLeftWidth"
        | "borderRightWidth"
        | "borderTopLeftRadius"
        | "borderTopRightRadius"
        | "borderTopWidth"
        | "columnGap"
        | "rowGap"
        | "flexBasis" => process_unit_value(key, value, container),
        "display"
        | "position"
        | "overflow"
        | "direction"
        | "fontFamily"
        | "fontStyle"
        | "textAlign"
        | "textDecoration"
        | "textDecorationStyle"
        | "textIndent"
        | "textOverflow"
        | "textTransform"
        | "verticalAlign"
        | "alignContent"
        | "alignItems"
        | "alignSelf"
        | "flexDirection"
        | "flexFlow"
        | "flexWrap"
        | "justifyContent"
        | "justifySelf"
        | "objectFit"
        | "strokeDasharray"
        | "fillRule"
        | "textAnchor"
        | "strokeLinecap"
        | "strokeLinejoin"
        | "visibility"
        | "clipPath"
        | "dominantBaseline"
        | "borderBottomStyle"
        | "borderLeftStyle"
        | "borderRightStyle"
        | "borderTopStyle" => process_noop_value(key, value),
        "fontWeight" => process_font_weight(value),
        "lineHeight" => process_line_height(value, style, container),
        "margin" => expand_margin(value, container),
        "marginTop" | "marginRight" | "marginBottom" | "marginLeft" => {
            expand_margin_single(key, value, container)
        }
        "marginHorizontal" => expand_margin_horizontal(value, container),
        "marginVertical" => expand_margin_vertical(value, container),
        "padding" => expand_padding(value, container),
        "paddingTop" | "paddingRight" | "paddingBottom" | "paddingLeft" => {
            expand_padding_single(key, value, container)
        }
        "paddingHorizontal" => expand_padding_horizontal(value, container),
        "paddingVertical" => expand_padding_vertical(value, container),
        "gap" => process_gap(value, container),
        "flex" => process_flex(value, container),
        "objectPosition" => process_object_position(value, container),
        "objectPositionX" | "objectPositionY" => {
            process_object_position_value(key, value, container)
        }
        "transform" | "gradientTransform" => process_transform(key, value),
        "transformOrigin" => process_transform_origin(value, container),
        "transformOriginX" | "transformOriginY" => {
            process_transform_origin_value(key, value, container)
        }
        "border" | "borderTop" | "borderRight" | "borderBottom" | "borderLeft" | "borderColor"
        | "borderStyle" | "borderWidth" | "borderRadius" => {
            process_border_shorthand(key, value, container)
        }
        "borderBottomColor" | "borderLeftColor" | "borderRightColor" | "borderTopColor" => {
            process_color_value(key, value)
        }
        _ => style_with(key, value.clone()),
    }
}

fn process_noop_value(key: &str, value: &StyleValue) -> Style {
    style_with(key, value.clone())
}

fn process_number_value(key: &str, value: &StyleValue) -> Style {
    match value.as_f64() {
        Some(number) => style_with(key, number),
        None => Style::new(),
    }
}

fn process_unit_value(key: &str, value: &StyleValue, container: &Container) -> Style {
    style_with(key, transform_unit(container, value))
}

fn process_color_value(key: &str, value: &StyleValue) -> Style {
    match value {
        StyleValue::String(text) => style_with(key, transform_color(text)),
        _ => style_with(key, value.clone()),
    }
}

fn process_font_weight(value: &StyleValue) -> Style {
    let weight = match value {
        StyleValue::Number(number) => *number,
        StyleValue::String(text) => match text.to_ascii_lowercase().as_str() {
            "thin" | "hairline" => 100.0,
            "ultralight" | "extralight" => 200.0,
            "light" => 300.0,
            "normal" => 400.0,
            "medium" => 500.0,
            "semibold" | "demibold" => 600.0,
            "bold" => 700.0,
            "ultrabold" | "extrabold" => 800.0,
            "heavy" | "black" => 900.0,
            _ => parse_int_like(text).map(f64::from).unwrap_or(400.0),
        },
        _ => 400.0,
    };

    style_with("fontWeight", weight)
}

fn process_line_height(value: &StyleValue, style: &Style, container: &Container) -> Style {
    let font_size = style
        .get("fontSize")
        .map(|value| transform_unit(container, value))
        .and_then(|value| match value {
            StyleValue::Number(number) => Some(number),
            _ => None,
        })
        .unwrap_or(DEFAULT_REM_BASE);

    let resolved = match value {
        StyleValue::String(text) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                StyleValue::String(String::new())
            } else if let Some(percent) = parse_percent(trimmed) {
                StyleValue::Number(percent * font_size)
            } else if is_plain_number(trimmed) {
                StyleValue::Number(parse_float_like(trimmed).unwrap_or(0.0) * font_size)
            } else {
                transform_unit(container, value)
            }
        }
        StyleValue::Number(number) => StyleValue::Number(number * font_size),
        _ => value.clone(),
    };

    style_with("lineHeight", resolved)
}

fn expand_margin(value: &StyleValue, container: &Container) -> Style {
    expand_box_model(value, container, 4, true, |parts| {
        style_from_pairs(vec![
            ("marginTop", parts[0].clone()),
            ("marginRight", parts[1].clone()),
            ("marginBottom", parts[2].clone()),
            ("marginLeft", parts[3].clone()),
        ])
    })
}

fn expand_margin_horizontal(value: &StyleValue, container: &Container) -> Style {
    expand_box_model(value, container, 2, true, |parts| {
        style_from_pairs(vec![
            ("marginRight", parts[0].clone()),
            ("marginLeft", parts[1].clone()),
        ])
    })
}

fn expand_margin_vertical(value: &StyleValue, container: &Container) -> Style {
    expand_box_model(value, container, 2, true, |parts| {
        style_from_pairs(vec![
            ("marginTop", parts[0].clone()),
            ("marginBottom", parts[1].clone()),
        ])
    })
}

fn expand_margin_single(key: &str, value: &StyleValue, container: &Container) -> Style {
    expand_box_model(value, container, 1, true, |parts| {
        style_with(key, parts[0].clone())
    })
}

fn expand_padding(value: &StyleValue, container: &Container) -> Style {
    expand_box_model(value, container, 4, false, |parts| {
        style_from_pairs(vec![
            ("paddingTop", parts[0].clone()),
            ("paddingRight", parts[1].clone()),
            ("paddingBottom", parts[2].clone()),
            ("paddingLeft", parts[3].clone()),
        ])
    })
}

fn expand_padding_horizontal(value: &StyleValue, container: &Container) -> Style {
    expand_box_model(value, container, 2, false, |parts| {
        style_from_pairs(vec![
            ("paddingRight", parts[0].clone()),
            ("paddingLeft", parts[1].clone()),
        ])
    })
}

fn expand_padding_vertical(value: &StyleValue, container: &Container) -> Style {
    expand_box_model(value, container, 2, false, |parts| {
        style_from_pairs(vec![
            ("paddingTop", parts[0].clone()),
            ("paddingBottom", parts[1].clone()),
        ])
    })
}

fn expand_padding_single(key: &str, value: &StyleValue, container: &Container) -> Style {
    expand_box_model(value, container, 1, false, |parts| {
        style_with(key, parts[0].clone())
    })
}

fn expand_box_model(
    value: &StyleValue,
    container: &Container,
    max_values: usize,
    auto_supported: bool,
    builder: impl Fn([StyleValue; 4]) -> Style,
) -> Style {
    let Some(mut parts) = parse_box_model_parts(value, container, max_values, auto_supported)
    else {
        return Style::new();
    };

    let first = parts.remove(0);
    let second = parts.first().cloned().unwrap_or_else(|| first.clone());
    let third = parts.get(1).cloned().unwrap_or_else(|| first.clone());
    let fourth = parts
        .get(2)
        .cloned()
        .unwrap_or_else(|| parts.first().cloned().unwrap_or_else(|| first.clone()));

    builder([first, second, third, fourth])
}

fn parse_box_model_parts(
    value: &StyleValue,
    container: &Container,
    max_values: usize,
    auto_supported: bool,
) -> Option<Vec<StyleValue>> {
    match value {
        StyleValue::Number(number) => Some(vec![StyleValue::Number(*number)]),
        StyleValue::String(text) => {
            let trimmed = text.trim();
            if trimmed.is_empty() || contains_unsupported_box_syntax(trimmed) {
                return None;
            }

            let mut parts = Vec::new();
            for token in trimmed.split_whitespace() {
                if token == "auto" && auto_supported {
                    parts.push(StyleValue::String("auto".to_string()));
                    continue;
                }

                if !is_valid_box_model_token(token) {
                    return None;
                }

                parts.push(transform_unit(
                    container,
                    &StyleValue::String(token.to_string()),
                ));
            }

            if parts.is_empty() || parts.len() > max_values {
                return None;
            }

            Some(parts)
        }
        _ => None,
    }
}

fn contains_unsupported_box_syntax(text: &str) -> bool {
    ['(', ')', '"', '\'', ',', '/']
        .into_iter()
        .any(|character| text.contains(character))
}

fn is_valid_box_model_token(token: &str) -> bool {
    token.ends_with('%') || parse_length_token(token).is_some()
}

fn process_gap(value: &StyleValue, container: &Container) -> Style {
    let parts = match value {
        StyleValue::Number(_) => vec![value.clone()],
        StyleValue::String(text) => text
            .split_whitespace()
            .map(|part| StyleValue::String(part.to_string()))
            .collect(),
        _ => return Style::new(),
    };

    if parts.is_empty() {
        return Style::new();
    }

    let row_gap = transform_unit(container, &parts[0]);
    let column_gap = transform_unit(container, parts.get(1).unwrap_or(&parts[0]));

    style_from_pairs(vec![("rowGap", row_gap), ("columnGap", column_gap)])
}

fn process_flex(value: &StyleValue, container: &Container) -> Style {
    let mut parts: Vec<String> = Vec::new();
    let defaults: [&str; 3] = match value {
        StyleValue::String(text) if text == "auto" => ["1", "1", "auto"],
        StyleValue::String(text) if text == "none" => ["0", "0", "auto"],
        StyleValue::String(text) if text == "initial" => ["0", "1", "auto"],
        StyleValue::String(text) => {
            parts = text.split_whitespace().map(ToOwned::to_owned).collect();
            ["1", "1", "0"]
        }
        StyleValue::Number(number) => {
            parts.push(number.to_string());
            ["1", "1", "0"]
        }
        _ => return Style::new(),
    };

    let flex_grow =
        parse_float_like(parts.first().map(String::as_str).unwrap_or(defaults[0])).unwrap_or(0.0);
    let flex_shrink =
        parse_float_like(parts.get(1).map(String::as_str).unwrap_or(defaults[1])).unwrap_or(0.0);
    let flex_basis_input = parts
        .get(2)
        .map(|value| StyleValue::String(value.clone()))
        .unwrap_or_else(|| StyleValue::String(defaults[2].to_string()));
    let flex_basis = transform_unit(container, &flex_basis_input);

    style_from_pairs(vec![
        ("flexGrow", StyleValue::Number(flex_grow)),
        ("flexShrink", StyleValue::Number(flex_shrink)),
        ("flexBasis", flex_basis),
    ])
}

fn process_object_position(value: &StyleValue, container: &Container) -> Style {
    let StyleValue::String(text) = value else {
        return Style::new();
    };

    let parts: Vec<&str> = text.split_whitespace().collect();
    if parts.is_empty() {
        return Style::new();
    }

    let (x_value, y_value) = if parts.len() == 1 {
        if matches!(parts[0], "top" | "bottom") {
            ("center", parts[0])
        } else {
            (parts[0], "center")
        }
    } else {
        (parts[0], parts[1])
    };

    style_from_pairs(vec![
        (
            "objectPositionX",
            offset_keyword(transform_unit(
                container,
                &StyleValue::String(x_value.to_string()),
            )),
        ),
        (
            "objectPositionY",
            offset_keyword(transform_unit(
                container,
                &StyleValue::String(y_value.to_string()),
            )),
        ),
    ])
}

fn process_object_position_value(key: &str, value: &StyleValue, container: &Container) -> Style {
    style_with(key, offset_keyword(transform_unit(container, value)))
}

fn process_transform_origin(value: &StyleValue, container: &Container) -> Style {
    let StyleValue::String(text) = value else {
        return Style::new();
    };

    let parts: Vec<&str> = text.split_whitespace().collect();
    let pair = transform_origin_pair(&parts);

    style_from_pairs(vec![
        (
            "transformOriginX",
            normalize_transform_origin_value(transform_unit(
                container,
                &StyleValue::String(pair.0.to_string()),
            )),
        ),
        (
            "transformOriginY",
            normalize_transform_origin_value(transform_unit(
                container,
                &StyleValue::String(pair.1.to_string()),
            )),
        ),
    ])
}

fn process_transform_origin_value(key: &str, value: &StyleValue, container: &Container) -> Style {
    style_with(
        key,
        normalize_transform_origin_value(transform_unit(container, value)),
    )
}

fn transform_origin_pair<'a>(parts: &'a [&'a str]) -> (&'a str, &'a str) {
    if parts.is_empty() {
        return ("center", "center");
    }

    let mut pair = if parts.len() == 1 {
        [parts[0], "center"]
    } else {
        [parts[0], parts[1]]
    };

    if matches!(pair[0], "top" | "bottom") {
        pair.swap(0, 1);
    }

    (pair[0], pair[1])
}

fn normalize_transform_origin_value(value: StyleValue) -> StyleValue {
    let mapped = offset_keyword(value);
    cast_float_value(mapped)
}

fn process_transform(key: &str, value: &StyleValue) -> Style {
    match value {
        StyleValue::String(text) => style_with(key, parse_transform(text)),
        StyleValue::Array(_) => style_with(key, value.clone()),
        _ => Style::new(),
    }
}

fn parse_transform(input: &str) -> StyleValue {
    let mut operations = Vec::new();
    let mut remainder = input.trim();

    if !remainder.contains('(') {
        return StyleValue::Array(vec![
            style_from_pairs(vec![
                ("operation", StyleValue::String(remainder.to_string())),
                ("value", StyleValue::Bool(true)),
            ])
            .into(),
        ]);
    }

    while let Some(start) = remainder.find('(') {
        let name = remainder[..start].trim();
        let after_start = &remainder[start + 1..];
        let Some(end) = after_start.find(')') else {
            break;
        };

        let raw_values = after_start[..end].trim();
        let values: Vec<&str> = if raw_values.contains(',') {
            raw_values
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .collect()
        } else {
            raw_values
                .split_whitespace()
                .filter(|value| !value.is_empty())
                .collect()
        };

        operations.push(normalize_transform_operation(name, &values).into());
        remainder = after_start[end + 1..].trim();
    }

    StyleValue::Array(operations)
}

fn normalize_transform_operation(name: &str, values: &[&str]) -> Style {
    match name {
        "scale" => {
            let x = parse_float_like(values.first().copied().unwrap_or("0")).unwrap_or(0.0);
            let y = parse_float_like(
                values
                    .get(1)
                    .copied()
                    .unwrap_or(values.first().copied().unwrap_or("0")),
            )
            .unwrap_or(x);
            transform_operation("scale", vec![x, y])
        }
        "scaleX" => {
            let x = parse_float_like(values.first().copied().unwrap_or("0")).unwrap_or(0.0);
            transform_operation("scale", vec![x, 1.0])
        }
        "scaleY" => {
            let y = parse_float_like(values.first().copied().unwrap_or("0")).unwrap_or(0.0);
            transform_operation("scale", vec![1.0, y])
        }
        "translate" => {
            let x = parse_float_like(values.first().copied().unwrap_or("0")).unwrap_or(0.0);
            let y = parse_float_like(values.get(1).copied().unwrap_or("0")).unwrap_or(0.0);
            transform_operation("translate", vec![x, y])
        }
        "translateX" => {
            let x = parse_float_like(values.first().copied().unwrap_or("0")).unwrap_or(0.0);
            transform_operation("translate", vec![x, 0.0])
        }
        "translateY" => {
            let y = parse_float_like(values.first().copied().unwrap_or("0")).unwrap_or(0.0);
            transform_operation("translate", vec![0.0, y])
        }
        "rotate" => {
            let angle = parse_angle(values.first().copied().unwrap_or("0"));
            let cx = parse_float_like(values.get(1).copied().unwrap_or("0")).unwrap_or(0.0);
            let cy = parse_float_like(values.get(2).copied().unwrap_or("0")).unwrap_or(0.0);
            transform_operation("rotate", vec![angle, cx, cy])
        }
        "skew" => {
            let parsed = values
                .iter()
                .map(|value| parse_angle(value))
                .collect::<Vec<_>>();
            transform_operation("skew", parsed)
        }
        "skewX" => {
            let angle = parse_angle(values.first().copied().unwrap_or("0"));
            transform_operation("skew", vec![angle, 0.0])
        }
        "skewY" => {
            let angle = parse_angle(values.first().copied().unwrap_or("0"));
            transform_operation("skew", vec![0.0, angle])
        }
        other => {
            let parsed = values
                .iter()
                .map(|value| parse_float_like(value).unwrap_or(0.0))
                .collect::<Vec<_>>();
            transform_operation(other, parsed)
        }
    }
}

fn transform_operation(operation: &str, values: Vec<f64>) -> Style {
    style_from_pairs(vec![
        ("operation", operation.into()),
        (
            "value",
            StyleValue::Array(values.into_iter().map(StyleValue::Number).collect()),
        ),
    ])
}

fn parse_angle(value: &str) -> f64 {
    let trimmed = value.trim();
    if let Some(number) = trimmed.strip_suffix("rad").and_then(parse_float_like) {
        (number * 180.0) / std::f64::consts::PI
    } else if let Some(number) = trimmed.strip_suffix("deg").and_then(parse_float_like) {
        number
    } else {
        parse_float_like(trimmed).unwrap_or(0.0)
    }
}

fn process_border_shorthand(key: &str, value: &StyleValue, container: &Container) -> Style {
    let Some(text) = value.as_string() else {
        return match key {
            "borderWidth" | "borderRadius" => spread_value(key, transform_unit(container, value)),
            _ => Style::new(),
        };
    };

    let parts: Vec<&str> = text.split_whitespace().collect();
    if parts.len() >= 3 {
        let width = transform_unit(container, &StyleValue::String(parts[0].to_string()));
        let style = StyleValue::String(parts[1].to_string());
        let color = StyleValue::String(transform_color(&parts[2..].join(" ")));

        return if matches!(
            key,
            "borderTop" | "borderRight" | "borderBottom" | "borderLeft"
        ) {
            let prefix = key;
            style_from_pairs(vec![
                (format!("{prefix}Color").as_str(), color),
                (format!("{prefix}Style").as_str(), style),
                (format!("{prefix}Width").as_str(), width),
            ])
        } else {
            style_from_pairs(vec![
                ("borderTopColor", color.clone()),
                ("borderTopStyle", style.clone()),
                ("borderTopWidth", width.clone()),
                ("borderRightColor", color.clone()),
                ("borderRightStyle", style.clone()),
                ("borderRightWidth", width.clone()),
                ("borderBottomColor", color.clone()),
                ("borderBottomStyle", style.clone()),
                ("borderBottomWidth", width.clone()),
                ("borderLeftColor", color),
                ("borderLeftStyle", style),
                ("borderLeftWidth", width),
            ])
        };
    }

    match key {
        "borderColor" => spread_border_color(value),
        "borderStyle" => spread_border_style(value),
        "borderWidth" => spread_border_width(transform_unit(container, value)),
        "borderRadius" => spread_border_radius(transform_unit(container, value)),
        _ => style_with(key, value.clone()),
    }
}

fn spread_border_color(value: &StyleValue) -> Style {
    let resolved = match value {
        StyleValue::String(text) => StyleValue::String(transform_color(text)),
        _ => value.clone(),
    };

    style_from_pairs(vec![
        ("borderTopColor", resolved.clone()),
        ("borderRightColor", resolved.clone()),
        ("borderBottomColor", resolved.clone()),
        ("borderLeftColor", resolved),
    ])
}

fn spread_border_style(value: &StyleValue) -> Style {
    style_from_pairs(vec![
        ("borderTopStyle", value.clone()),
        ("borderRightStyle", value.clone()),
        ("borderBottomStyle", value.clone()),
        ("borderLeftStyle", value.clone()),
    ])
}

fn spread_border_width(value: StyleValue) -> Style {
    style_from_pairs(vec![
        ("borderTopWidth", value.clone()),
        ("borderRightWidth", value.clone()),
        ("borderBottomWidth", value.clone()),
        ("borderLeftWidth", value),
    ])
}

fn spread_border_radius(value: StyleValue) -> Style {
    style_from_pairs(vec![
        ("borderTopLeftRadius", value.clone()),
        ("borderTopRightRadius", value.clone()),
        ("borderBottomRightRadius", value.clone()),
        ("borderBottomLeftRadius", value),
    ])
}

fn spread_value(key: &str, value: StyleValue) -> Style {
    match key {
        "borderWidth" => spread_border_width(value),
        "borderRadius" => spread_border_radius(value),
        _ => style_with(key, value),
    }
}

fn transform_unit(container: &Container, value: &StyleValue) -> StyleValue {
    match value {
        StyleValue::Number(number) => StyleValue::Number(*number),
        StyleValue::String(text) => transform_unit_text(container, text),
        _ => value.clone(),
    }
}

fn transform_unit_text(container: &Container, text: &str) -> StyleValue {
    let trimmed = text.trim();

    if trimmed.ends_with('%') {
        return StyleValue::String(trimmed.to_string());
    }

    let Some((number, unit)) = parse_length_token(trimmed) else {
        return StyleValue::String(trimmed.to_string());
    };

    let dpi = container.dpi.unwrap_or(DEFAULT_DPI);
    let value = match unit.as_deref() {
        Some("rem") => number * container.rem_base.unwrap_or(DEFAULT_REM_BASE),
        Some("in") => number * DEFAULT_DPI,
        Some("mm") => number * (DEFAULT_DPI / MM_PER_INCH),
        Some("cm") => number * (DEFAULT_DPI / CM_PER_INCH),
        Some("vh") => number * (container.height / 100.0),
        Some("vw") => number * (container.width / 100.0),
        Some("px") => (number * (DEFAULT_DPI / dpi)).round(),
        Some("pt") | None => number,
        Some(_) => return StyleValue::String(trimmed.to_string()),
    };

    StyleValue::Number(value)
}

fn parse_length_token(token: &str) -> Option<(f64, Option<String>)> {
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut number_end = 0usize;
    for (index, character) in trimmed.char_indices() {
        let valid =
            character.is_ascii_digit() || character == '.' || (index == 0 && character == '-');
        if valid {
            number_end = index + character.len_utf8();
        } else {
            break;
        }
    }

    if number_end == 0 {
        return None;
    }

    let number = trimmed[..number_end].parse::<f64>().ok()?;
    let unit = trimmed[number_end..].trim();

    if unit.is_empty() {
        Some((number, None))
    } else {
        Some((number, Some(unit.to_string())))
    }
}

fn parse_float_like(text: &str) -> Option<f64> {
    let trimmed = text.trim();
    let mut end = 0usize;

    for (index, character) in trimmed.char_indices() {
        let valid =
            character.is_ascii_digit() || character == '.' || (index == 0 && character == '-');
        if valid {
            end = index + character.len_utf8();
        } else {
            break;
        }
    }

    if end == 0 {
        None
    } else {
        trimmed[..end].parse::<f64>().ok()
    }
}

fn parse_int_like(text: &str) -> Option<i32> {
    let trimmed = text.trim();
    let mut end = 0usize;

    for (index, character) in trimmed.char_indices() {
        let valid = character.is_ascii_digit() || (index == 0 && character == '-');
        if valid {
            end = index + character.len_utf8();
        } else {
            break;
        }
    }

    if end == 0 {
        None
    } else {
        trimmed[..end].parse::<i32>().ok()
    }
}

fn is_plain_number(text: &str) -> bool {
    text.chars().enumerate().all(|(index, character)| {
        character.is_ascii_digit() || character == '.' || (index == 0 && character == '-')
    })
}

fn parse_percent(text: &str) -> Option<f64> {
    text.strip_suffix('%')
        .and_then(parse_float_like)
        .map(|percent| percent / 100.0)
}

fn offset_keyword(value: StyleValue) -> StyleValue {
    match value {
        StyleValue::String(text) => match text.as_str() {
            "top" | "left" => StyleValue::String("0%".to_string()),
            "right" | "bottom" => StyleValue::String("100%".to_string()),
            "center" => StyleValue::String("50%".to_string()),
            _ => StyleValue::String(text),
        },
        other => other,
    }
}

fn cast_float_value(value: StyleValue) -> StyleValue {
    match value {
        StyleValue::String(text) if is_plain_number(&text) => {
            StyleValue::Number(parse_float_like(&text).unwrap_or(0.0))
        }
        other => other,
    }
}

fn style_from_pairs(pairs: Vec<(&str, StyleValue)>) -> Style {
    pairs
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect()
}

fn matches_media_query(container: &Container, query: &str) -> bool {
    let Some(query) = query.strip_prefix("@media") else {
        return false;
    };

    query.split(',').map(str::trim).any(|branch| {
        branch
            .split(" and ")
            .all(|clause| matches_media_clause(container, clause.trim()))
    })
}

fn matches_media_clause(container: &Container, clause: &str) -> bool {
    let trimmed = clause.trim().trim_start_matches('(').trim_end_matches(')');
    let Some((feature, raw_value)) = trimmed.split_once(':') else {
        return false;
    };

    let feature = feature.trim();
    let raw_value = raw_value.trim();

    match feature {
        "max-height" => {
            compare_media_dimension(container.height, raw_value, container, |lhs, rhs| {
                lhs <= rhs
            })
        }
        "min-height" => {
            compare_media_dimension(container.height, raw_value, container, |lhs, rhs| {
                lhs >= rhs
            })
        }
        "max-width" => {
            compare_media_dimension(container.width, raw_value, container, |lhs, rhs| lhs <= rhs)
        }
        "min-width" => {
            compare_media_dimension(container.width, raw_value, container, |lhs, rhs| lhs >= rhs)
        }
        "orientation" => Orientation::from_str(raw_value)
            .map(|orientation| orientation == container.resolved_orientation())
            .unwrap_or(false),
        _ => false,
    }
}

fn compare_media_dimension(
    actual: f64,
    raw_value: &str,
    container: &Container,
    predicate: impl Fn(f64, f64) -> bool,
) -> bool {
    match transform_unit(container, &StyleValue::String(raw_value.to_string())) {
        StyleValue::Number(number) => predicate(actual, number),
        _ => false,
    }
}

fn parse_rgb_color(value: &str) -> Option<String> {
    let lower = value.to_ascii_lowercase();
    let has_alpha = lower.starts_with("rgba(");
    if !has_alpha && !lower.starts_with("rgb(") {
        return None;
    }

    let start = value.find('(')? + 1;
    let end = value.rfind(')')?;
    let parts = value[start..end]
        .split(',')
        .map(str::trim)
        .collect::<Vec<_>>();

    if (!has_alpha && parts.len() != 3) || (has_alpha && parts.len() != 4) {
        return None;
    }

    let red = clamp_byte(parse_float_like(parts[0])?);
    let green = clamp_byte(parse_float_like(parts[1])?);
    let blue = clamp_byte(parse_float_like(parts[2])?);
    let alpha = if has_alpha {
        Some(clamp_alpha(parse_float_like(parts[3])?))
    } else {
        None
    };

    Some(rgb_to_hex(red, green, blue, alpha))
}

fn parse_hsl_color(value: &str) -> Option<String> {
    let lower = value.to_ascii_lowercase();
    let has_alpha = lower.starts_with("hsla(");
    if !has_alpha && !lower.starts_with("hsl(") {
        return None;
    }

    let start = value.find('(')? + 1;
    let end = value.rfind(')')?;
    let parts = value[start..end]
        .split(',')
        .map(str::trim)
        .collect::<Vec<_>>();

    if (!has_alpha && parts.len() != 3) || (has_alpha && parts.len() != 4) {
        return None;
    }

    let hue = parse_float_like(parts[0])?;
    let saturation = parse_percent(parts[1])?;
    let lightness = parse_percent(parts[2])?;
    let alpha = if has_alpha {
        Some(clamp_alpha(parse_float_like(parts[3])?))
    } else {
        None
    };

    let (red, green, blue) = hsl_to_rgb(hue, saturation, lightness);
    Some(rgb_to_hex(red, green, blue, alpha))
}

fn hsl_to_rgb(hue: f64, saturation: f64, lightness: f64) -> (u8, u8, u8) {
    let hue = (hue % 360.0 + 360.0) % 360.0 / 360.0;

    if saturation == 0.0 {
        let value = clamp_byte(lightness * 255.0);
        return (value, value, value);
    }

    let q = if lightness < 0.5 {
        lightness * (1.0 + saturation)
    } else {
        lightness + saturation - lightness * saturation
    };
    let p = 2.0 * lightness - q;

    (
        clamp_byte(hue_to_rgb(p, q, hue + (1.0 / 3.0)) * 255.0),
        clamp_byte(hue_to_rgb(p, q, hue) * 255.0),
        clamp_byte(hue_to_rgb(p, q, hue - (1.0 / 3.0)) * 255.0),
    )
}

fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * ((2.0 / 3.0) - t) * 6.0;
    }
    p
}

fn clamp_byte(value: f64) -> u8 {
    value.round().clamp(0.0, 255.0) as u8
}

fn clamp_alpha(value: f64) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn rgb_to_hex(red: u8, green: u8, blue: u8, alpha: Option<u8>) -> String {
    match alpha {
        Some(alpha) if alpha < 255 => format!("#{red:02X}{green:02X}{blue:02X}{alpha:02X}"),
        _ => format!("#{red:02X}{green:02X}{blue:02X}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn container() -> Container {
        Container {
            width: 200.0,
            height: 400.0,
            dpi: None,
            rem_base: Some(10.0),
            orientation: None,
        }
    }

    fn style(entries: Vec<(&str, StyleValue)>) -> Style {
        entries
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect()
    }

    fn media(entries: Vec<(&str, StyleValue)>) -> StyleValue {
        StyleValue::Object(style(entries))
    }

    #[test]
    fn flattens_nested_styles_and_ignores_nullish_entries() {
        let input = StyleValue::Array(vec![
            StyleValue::Null,
            StyleValue::Object(style(vec![("backgroundColor", "black".into())])),
            false.into(),
            StyleValue::Array(vec![StyleValue::Object(style(vec![
                ("color", "red".into()),
                ("textAlign", "center".into()),
            ]))]),
        ]);

        let flattened = flatten(&input);

        assert_eq!(
            flattened,
            style(vec![
                ("backgroundColor", "black".into()),
                ("color", "red".into()),
                ("textAlign", "center".into()),
            ])
        );
    }

    #[test]
    fn resolves_media_queries_with_and_or_and_ordered_overrides() {
        let container = Container {
            width: 400.0,
            height: 300.0,
            dpi: None,
            rem_base: None,
            orientation: Some(Orientation::Landscape),
        };
        let styles = style(vec![
            ("color", "black".into()),
            (
                "@media min-width: 300 and max-width: 500",
                media(vec![("color", "red".into())]),
            ),
            (
                "@media max-width: 300, orientation: landscape",
                media(vec![("backgroundColor", "blue".into())]),
            ),
            (
                "@media max-height: 400",
                media(vec![("color", "green".into())]),
            ),
        ]);

        let resolved = resolve_media_queries(&container, &styles);

        assert_eq!(
            resolved,
            style(vec![
                ("color", "green".into()),
                ("backgroundColor", "blue".into()),
            ])
        );
    }

    #[test]
    fn resolves_margins_and_preserves_auto_and_percent_values() {
        let resolved = resolve_style(
            &container(),
            &style(vec![("margin", "auto 20 30 40%".into())]),
        );

        assert_eq!(
            resolved,
            style(vec![
                ("marginTop", "auto".into()),
                ("marginRight", 20.into()),
                ("marginBottom", 30.into()),
                ("marginLeft", "40%".into()),
            ])
        );
    }

    #[test]
    fn ignores_invalid_padding_syntax() {
        let resolved = resolve_style(
            &container(),
            &style(vec![("padding", "calc(100% - 10px)".into())]),
        );

        assert!(resolved.is_empty());
    }

    #[test]
    fn resolves_border_shorthand_and_color_conversion() {
        let resolved = resolve_style(
            &container(),
            &style(vec![("border", "1in solid rgba(0, 255, 0, 0.5)".into())]),
        );

        assert_eq!(
            resolved,
            style(vec![
                ("borderTopColor", "#00FF0080".into()),
                ("borderTopStyle", "solid".into()),
                ("borderTopWidth", 72.into()),
                ("borderRightColor", "#00FF0080".into()),
                ("borderRightStyle", "solid".into()),
                ("borderRightWidth", 72.into()),
                ("borderBottomColor", "#00FF0080".into()),
                ("borderBottomStyle", "solid".into()),
                ("borderBottomWidth", 72.into()),
                ("borderLeftColor", "#00FF0080".into()),
                ("borderLeftStyle", "solid".into()),
                ("borderLeftWidth", 72.into()),
            ])
        );
    }

    #[test]
    fn resolves_gap_and_flex_shorthands() {
        let resolved = resolve_style(
            &container(),
            &style(vec![
                ("gap", "10px 20%".into()),
                ("flex", "2 3 1rem".into()),
            ]),
        );

        assert_eq!(resolved.get("rowGap"), Some(&10.into()));
        assert_eq!(resolved.get("columnGap"), Some(&"20%".into()));
        assert_eq!(resolved.get("flexGrow"), Some(&2.into()));
        assert_eq!(resolved.get("flexShrink"), Some(&3.into()));
        assert_eq!(resolved.get("flexBasis"), Some(&10.into()));
    }

    #[test]
    fn resolves_object_position_keywords_and_lengths() {
        let resolved = resolve_style(
            &container(),
            &style(vec![("objectPosition", "left 2rem".into())]),
        );

        assert_eq!(
            resolved,
            style(vec![
                ("objectPositionX", "0%".into()),
                ("objectPositionY", 20.into())
            ])
        );
    }

    #[test]
    fn resolves_text_handlers_and_color_transforms() {
        let resolved = resolve_style(
            &container(),
            &style(vec![
                ("fontWeight", "semibold".into()),
                ("fontSize", "2rem".into()),
                ("lineHeight", "150%".into()),
                ("textDecorationColor", "hsl(0, 100%, 50%)".into()),
            ]),
        );

        assert_eq!(resolved.get("fontWeight"), Some(&600.into()));
        assert_eq!(resolved.get("fontSize"), Some(&20.into()));
        assert_eq!(resolved.get("lineHeight"), Some(&30.into()));
        assert_eq!(resolved.get("textDecorationColor"), Some(&"#FF0000".into()));
    }

    #[test]
    fn resolves_transform_origin_and_transform_operations() {
        let resolved = resolve_style(
            &container(),
            &style(vec![
                ("transformOrigin", "top left".into()),
                (
                    "transform",
                    "translate(10px, 20px) rotate(90deg) skewX(30deg) matrix(1, 0, 0, 1, 5, 10)"
                        .into(),
                ),
            ]),
        );

        assert_eq!(resolved.get("transformOriginX"), Some(&"0%".into()));
        assert_eq!(resolved.get("transformOriginY"), Some(&"0%".into()));

        let StyleValue::Array(operations) = resolved.get("transform").cloned().unwrap() else {
            panic!("expected parsed transform array");
        };

        assert_eq!(operations.len(), 4);
        assert_eq!(
            operations[0],
            StyleValue::Object(style(vec![
                ("operation", "translate".into()),
                ("value", StyleValue::Array(vec![10.into(), 20.into()])),
            ]))
        );
        assert_eq!(
            operations[1],
            StyleValue::Object(style(vec![
                ("operation", "rotate".into()),
                (
                    "value",
                    StyleValue::Array(vec![90.into(), 0.into(), 0.into()])
                ),
            ]))
        );
    }

    #[test]
    fn resolves_svg_handlers() {
        let resolved = resolve_style(
            &container(),
            &style(vec![
                ("fill", "rgb(255, 0, 255)".into()),
                ("strokeWidth", "2rem".into()),
                ("fillOpacity", "0.5".into()),
            ]),
        );

        assert_eq!(resolved.get("fill"), Some(&"#FF00FF".into()));
        assert_eq!(resolved.get("strokeWidth"), Some(&20.into()));
        assert_eq!(resolved.get("fillOpacity"), Some(&0.5.into()));
    }

    #[test]
    fn resolves_end_to_end_style_pipeline() {
        let input = StyleValue::Array(vec![
            StyleValue::Object(style(vec![("margin", "10px".into())])),
            StyleValue::Object(style(vec![
                ("padding", "2rem".into()),
                ("width", "1in".into()),
                (
                    "@media min-width: 100",
                    media(vec![("backgroundColor", "rgb(255, 0, 0)".into())]),
                ),
            ])),
        ]);

        let resolved = resolve_styles(&container(), &input);

        assert_eq!(resolved.get("marginTop"), Some(&10.into()));
        assert_eq!(resolved.get("marginRight"), Some(&10.into()));
        assert_eq!(resolved.get("paddingLeft"), Some(&20.into()));
        assert_eq!(resolved.get("paddingBottom"), Some(&20.into()));
        assert_eq!(resolved.get("width"), Some(&72.into()));
        assert_eq!(resolved.get("backgroundColor"), Some(&"#FF0000".into()));
    }

    #[test]
    fn transforms_rgb_and_hsl_colors() {
        assert_eq!(transform_color("rgb(255, 0, 0)"), "#FF0000");
        assert_eq!(transform_color("rgba(0, 255, 0, 0.5)"), "#00FF0080");
        assert_eq!(transform_color("hsl(0, 100%, 50%)"), "#FF0000");
        assert_eq!(transform_color("hsla(0, 100%, 50%, 0.5)"), "#FF000080");
    }

    #[test]
    fn stylesheet_wrapper_resolves_input() {
        let stylesheet = Stylesheet::new(StyleValue::Object(style(vec![("width", "2rem".into())])));
        let resolved = stylesheet.resolve(&container());

        assert!(!stylesheet.is_empty());
        assert_eq!(resolved.get("width"), Some(&20.into()));
    }
}
