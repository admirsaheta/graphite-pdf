pub mod error;

pub use error::*;
pub use graphitepdf_svg::{SvgNode, SvgNodeKind};

use std::fmt;

use graphitepdf_primitives::Color;
use graphitepdf_svg::SvgProps;
use mathjax_svg_rs::{HorizontalAlign, Options as BackendOptions, render_tex};

const DEFAULT_HEIGHT: f32 = 22.0;

#[derive(Clone, Debug, PartialEq)]
pub enum MathDimension {
    Number(f32),
    Value(String),
}

impl MathDimension {
    fn to_svg_value(&self) -> String {
        match self {
            Self::Number(value) => format_number(*value),
            Self::Value(value) => value.trim().to_string(),
        }
    }

    fn parse_numeric_value(&self) -> Result<(f32, String)> {
        match self {
            Self::Number(value) => Ok((*value, String::new())),
            Self::Value(value) => parse_numeric_with_unit(value),
        }
    }
}

impl fmt::Display for MathDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_svg_value())
    }
}

impl From<f32> for MathDimension {
    fn from(value: f32) -> Self {
        Self::Number(value)
    }
}

impl From<f64> for MathDimension {
    fn from(value: f64) -> Self {
        Self::Number(value as f32)
    }
}

impl From<i32> for MathDimension {
    fn from(value: i32) -> Self {
        Self::Number(value as f32)
    }
}

impl From<u32> for MathDimension {
    fn from(value: u32) -> Self {
        Self::Number(value as f32)
    }
}

impl From<String> for MathDimension {
    fn from(value: String) -> Self {
        Self::Value(value)
    }
}

impl From<&str> for MathDimension {
    fn from(value: &str) -> Self {
        Self::Value(value.to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MathOptions {
    pub inline: bool,
    pub width: Option<MathDimension>,
    pub height: Option<MathDimension>,
    pub color: String,
    pub debug: bool,
}

impl MathOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }

    pub fn width(mut self, width: impl Into<MathDimension>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<MathDimension>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    pub fn color_from_primitives(mut self, color: Color) -> Self {
        self.color = format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            color.red, color.green, color.blue, color.alpha
        );
        self
    }

    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}

impl Default for MathOptions {
    fn default() -> Self {
        Self {
            inline: false,
            width: None,
            height: None,
            color: String::from("black"),
            debug: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MathRender {
    pub source: String,
    pub raw_svg: String,
    pub svg: SvgNode,
}

impl MathRender {
    pub fn into_svg(self) -> SvgNode {
        self.svg
    }
}

pub fn render_math(latex: &str) -> Result<MathRender> {
    render_math_with_options(latex, &MathOptions::default())
}

pub fn render_math_with_options(latex: &str, options: &MathOptions) -> Result<MathRender> {
    let raw_svg = render_latex_to_svg(latex, options)?;
    let mut svg = graphitepdf_svg::try_parse_svg(&raw_svg)?;

    if svg.kind != SvgNodeKind::Svg {
        return Err(Error::InvalidSvgRoot);
    }

    let (width, height) = resolve_dimensions(&svg.props, options)?;
    svg.props.insert(String::from("width"), width);
    svg.props.insert(String::from("height"), height);
    svg.props
        .insert(String::from("color"), options.color.clone());

    if options.debug {
        svg.props.insert(String::from("debug"), String::from("true"));
    }

    resolve_current_color(&mut svg, &options.color);

    Ok(MathRender {
        source: latex.to_string(),
        raw_svg,
        svg,
    })
}

fn render_latex_to_svg(latex: &str, options: &MathOptions) -> Result<String> {
    let wrapped_latex = wrap_latex_for_mode(latex, options.inline);
    let backend_options = BackendOptions {
        horizontal_align: if options.inline {
            HorizontalAlign::Left
        } else {
            HorizontalAlign::Center
        },
        ..BackendOptions::default()
    };

    render_tex(&wrapped_latex, &backend_options).map_err(Error::MathBackend)
}

fn wrap_latex_for_mode(latex: &str, inline: bool) -> String {
    let style = if inline {
        r"\textstyle"
    } else {
        r"\displaystyle"
    };

    format!("{{{style} {latex}}}")
}

fn resolve_dimensions(props: &SvgProps, options: &MathOptions) -> Result<(String, String)> {
    let aspect_ratio = extract_aspect_ratio(props)?;

    match (options.width.as_ref(), options.height.as_ref()) {
        (Some(width), Some(height)) => Ok((width.to_svg_value(), height.to_svg_value())),
        (Some(width), None) => {
            let (width_value, suffix) = width.parse_numeric_value()?;
            let height = width_value / aspect_ratio;
            Ok((width.to_svg_value(), format_length(height, &suffix)))
        }
        (None, Some(height)) => {
            let (height_value, suffix) = height.parse_numeric_value()?;
            let width = height_value * aspect_ratio;
            Ok((format_length(width, &suffix), height.to_svg_value()))
        }
        (None, None) => Ok((
            format_number(DEFAULT_HEIGHT * aspect_ratio),
            format_number(DEFAULT_HEIGHT),
        )),
    }
}

fn extract_aspect_ratio(props: &SvgProps) -> Result<f32> {
    if let Some(view_box) = props.get("viewBox") {
        let values: Vec<f32> = view_box
            .split(|character: char| character.is_ascii_whitespace() || character == ',')
            .filter(|part| !part.is_empty())
            .filter_map(|part| part.parse::<f32>().ok())
            .collect();

        if values.len() == 4 && values[2].is_finite() && values[3].is_finite() && values[3] != 0.0 {
            return Ok(values[2].abs() / values[3].abs());
        }
    }

    if let (Some(width), Some(height)) = (props.get("width"), props.get("height")) {
        let (width_value, _) = parse_numeric_with_unit(width)?;
        let (height_value, _) = parse_numeric_with_unit(height)?;

        if height_value != 0.0 {
            return Ok(width_value.abs() / height_value.abs());
        }
    }

    Err(Error::InvalidViewBox)
}

fn resolve_current_color(node: &mut SvgNode, color: &str) {
    for value in node.props.values_mut() {
        if value == "currentColor" {
            *value = color.to_string();
        }
    }

    for child in &mut node.children {
        resolve_current_color(child, color);
    }
}

fn parse_numeric_with_unit(input: &str) -> Result<(f32, String)> {
    let trimmed = input.trim();
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
            input: input.to_string(),
        });
    }

    let (number, suffix) = trimmed.split_at(end);
    let value = number.parse::<f32>().map_err(|_| Error::InvalidDimension {
        input: input.to_string(),
    })?;

    Ok((value, suffix.trim().to_string()))
}

fn format_length(value: f32, suffix: &str) -> String {
    format!("{}{}", format_number(value), suffix)
}

fn format_number(value: f32) -> String {
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

    fn parse_dimension(value: &str) -> f32 {
        parse_numeric_with_unit(value)
            .expect("dimension should be numeric")
            .0
    }

    fn contains_current_color(node: &SvgNode) -> bool {
        node.props.values().any(|value| value == "currentColor")
            || node.children.iter().any(contains_current_color)
    }

    #[test]
    fn renders_display_math_with_default_dimensions() {
        let rendered = render_math(r"\frac{-b \pm \sqrt{b^2 - 4ac}}{2a}")
            .expect("display math should render");

        assert_eq!(rendered.svg.kind, SvgNodeKind::Svg);
        assert!(rendered.raw_svg.contains("<svg"));
        assert_eq!(rendered.svg.props.get("height"), Some(&String::from("22")));
        assert!(rendered.svg.props.contains_key("width"));
        assert_eq!(rendered.svg.props.get("color"), Some(&String::from("black")));
        assert!(!contains_current_color(&rendered.svg));
    }

    #[test]
    fn supports_explicit_dimensions_color_and_debug() {
        let rendered = render_math_with_options(
            r"e^{i\pi} + 1 = 0",
            &MathOptions::new()
                .width("180px")
                .height(40.0)
                .color("rebeccapurple")
                .debug(true),
        )
        .expect("math with explicit options should render");

        assert_eq!(rendered.svg.props.get("width"), Some(&String::from("180px")));
        assert_eq!(rendered.svg.props.get("height"), Some(&String::from("40")));
        assert_eq!(
            rendered.svg.props.get("color"),
            Some(&String::from("rebeccapurple"))
        );
        assert_eq!(rendered.svg.props.get("debug"), Some(&String::from("true")));
        assert!(!contains_current_color(&rendered.svg));
    }

    #[test]
    fn derives_missing_dimension_from_view_box_aspect_ratio() {
        let rendered = render_math_with_options(
            r"\sum_{n=1}^{\infty} \frac{1}{n^2}",
            &MathOptions::new().width(180.0),
        )
        .expect("math with one dimension should render");

        let width = parse_dimension(
            rendered
                .svg
                .props
                .get("width")
                .expect("width should be populated"),
        );
        let height = parse_dimension(
            rendered
                .svg
                .props
                .get("height")
                .expect("height should be derived"),
        );
        let aspect_ratio = extract_aspect_ratio(&rendered.svg.props).expect("viewBox should exist");

        assert!((width / height - aspect_ratio).abs() < 0.01);
    }

    #[test]
    fn differentiates_inline_and_display_rendering() {
        let display = render_math_with_options(
            r"\int_0^\infty e^{-x^2} \, dx = \sqrt{\pi}",
            &MathOptions::default(),
        )
        .expect("display math should render");
        let inline = render_math_with_options(
            r"\int_0^\infty e^{-x^2} \, dx = \sqrt{\pi}",
            &MathOptions::new().inline(true),
        )
        .expect("inline math should render");

        assert_ne!(display.raw_svg, inline.raw_svg);
    }

    #[test]
    fn rejects_non_numeric_single_dimension_strings() {
        let error = render_math_with_options(
            r"E = mc^2",
            &MathOptions::new().width("wide"),
        )
        .expect_err("non-numeric width should fail when deriving height");

        assert!(matches!(error, Error::InvalidDimension { .. }));
    }

    #[test]
    fn supports_primitive_color_conversion() {
        let options = MathOptions::new().color_from_primitives(Color::rgba(16, 32, 48, 255));
        let rendered = render_math_with_options(r"x + y", &options).expect("math should render");

        assert_eq!(
            rendered.svg.props.get("color"),
            Some(&String::from("#102030ff"))
        );
    }
}
