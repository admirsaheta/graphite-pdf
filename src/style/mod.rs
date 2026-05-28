pub use graphitepdf_font::{
    FontDescriptor, FontSource, FontStyle, FontWeight as FontVariantWeight, StandardFont,
};
use graphitepdf_primitives::{Color, Pt};

pub use graphitepdf_stylesheet::{
    Container as StylesheetContainer, ExpandedStyle as StylesheetExpandedStyle,
    SafeStyle as StylesheetSafeStyle, Style as StylesheetMap, StyleValue, Stylesheet,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Style {
    pub width: Option<Pt>,
    pub height: Option<Pt>,
    pub margin: EdgeInsets,
    pub padding: EdgeInsets,
    pub background_color: Option<Color>,
    pub color: Option<Color>,
    pub font_size: Option<Pt>,
    pub font_family: Option<String>,
    pub font_style: Option<FontStyle>,
    pub font_weight: Option<FontVariantWeight>,
    pub font_source: Option<FontSource>,
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            margin: EdgeInsets::default(),
            padding: EdgeInsets::default(),
            background_color: None,
            color: Some(Color::BLACK),
            font_size: Some(Pt::new(12.0)),
            font_family: None,
            font_style: None,
            font_weight: None,
            font_source: None,
            flex_direction: FlexDirection::default(),
            justify_content: JustifyContent::default(),
            align_items: AlignItems::default(),
        }
    }
}

impl Style {
    pub fn from_stylesheet(container: &StylesheetContainer, stylesheet: &Stylesheet) -> Self {
        let mut style = Self::default();
        style.apply_stylesheet(container, stylesheet);
        style
    }

    pub fn apply_stylesheet(&mut self, container: &StylesheetContainer, stylesheet: &Stylesheet) {
        let resolved = stylesheet.resolve(container);
        self.apply_resolved_stylesheet(&resolved);
    }

    pub fn apply_resolved_stylesheet(&mut self, style: &StylesheetMap) {
        if let Some(value) = stylesheet_pt(style, "width") {
            self.width = Some(value);
        }
        if let Some(value) = stylesheet_pt(style, "height") {
            self.height = Some(value);
        }

        apply_edge_insets(
            &mut self.margin,
            style,
            ["marginTop", "marginRight", "marginBottom", "marginLeft"],
        );
        apply_edge_insets(
            &mut self.padding,
            style,
            ["paddingTop", "paddingRight", "paddingBottom", "paddingLeft"],
        );

        if let Some(value) = stylesheet_color(style, "backgroundColor") {
            self.background_color = Some(value);
        }
        if let Some(value) = stylesheet_color(style, "color") {
            self.color = Some(value);
        }
        if let Some(value) = stylesheet_pt(style, "fontSize") {
            self.font_size = Some(value);
        }
        if let Some(value) = stylesheet_string(style, "fontFamily") {
            self.font_family = Some(value.to_string());
        }
        if let Some(value) = stylesheet_font_style(style, "fontStyle") {
            self.font_style = Some(value);
        }
        if let Some(value) = stylesheet_font_weight(style, "fontWeight") {
            self.font_weight = Some(value);
        }
        if let Some(value) = stylesheet_string(style, "fontSource") {
            self.font_source = Some(FontSource::remote(value));
        }
        if let Some(value) = stylesheet_string(style, "fontSourceLocal") {
            self.font_source = Some(FontSource::local(value));
        }
        if let Some(value) = stylesheet_string(style, "fontSourceDataUri") {
            self.font_source = Some(FontSource::data_uri(value));
        }
        if let Some(value) = stylesheet_standard_font(style, "fontSourceStandard") {
            self.font_source = Some(FontSource::standard(value));
        }
        if let Some(value) = stylesheet_flex_direction(style, "flexDirection") {
            self.flex_direction = value;
        }
        if let Some(value) = stylesheet_justify_content(style, "justifyContent") {
            self.justify_content = value;
        }
        if let Some(value) = stylesheet_align_items(style, "alignItems") {
            self.align_items = value;
        }
    }

    pub fn font_descriptor(&self) -> Option<FontDescriptor> {
        let mut descriptor = FontDescriptor::new(self.font_family.clone()?);

        if let Some(value) = self.font_style {
            descriptor = descriptor.with_style(value);
        }
        if let Some(value) = self.font_weight {
            descriptor = descriptor.with_weight(value);
        }

        Some(descriptor)
    }
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
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexDirection {
    #[default]
    Column,
    Row,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum JustifyContent {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlignItems {
    #[default]
    Start,
    Center,
    End,
    Stretch,
}

fn apply_edge_insets(target: &mut EdgeInsets, style: &StylesheetMap, keys: [&str; 4]) {
    if let Some(value) = stylesheet_pt(style, keys[0]) {
        target.top = value;
    }
    if let Some(value) = stylesheet_pt(style, keys[1]) {
        target.right = value;
    }
    if let Some(value) = stylesheet_pt(style, keys[2]) {
        target.bottom = value;
    }
    if let Some(value) = stylesheet_pt(style, keys[3]) {
        target.left = value;
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

fn stylesheet_string<'a>(style: &'a StylesheetMap, key: &str) -> Option<&'a str> {
    match style.get(key)? {
        StyleValue::String(value) => Some(value.as_str()),
        _ => None,
    }
}

fn stylesheet_color(style: &StylesheetMap, key: &str) -> Option<Color> {
    let value = stylesheet_string(style, key)?;
    parse_color(value)
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

fn stylesheet_font_weight(style: &StylesheetMap, key: &str) -> Option<FontVariantWeight> {
    let value = style.get(key)?;
    let number = match value {
        StyleValue::Number(value) => *value as u16,
        StyleValue::String(value) => value.trim().parse::<u16>().ok()?,
        _ => return None,
    };

    FontVariantWeight::new(number).ok()
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

fn stylesheet_flex_direction(style: &StylesheetMap, key: &str) -> Option<FlexDirection> {
    match stylesheet_string(style, key)?.trim() {
        "column" => Some(FlexDirection::Column),
        "row" => Some(FlexDirection::Row),
        _ => None,
    }
}

fn stylesheet_justify_content(style: &StylesheetMap, key: &str) -> Option<JustifyContent> {
    match stylesheet_string(style, key)?.trim() {
        "start" | "flex-start" => Some(JustifyContent::Start),
        "center" => Some(JustifyContent::Center),
        "end" | "flex-end" => Some(JustifyContent::End),
        "space-between" => Some(JustifyContent::SpaceBetween),
        _ => None,
    }
}

fn stylesheet_align_items(style: &StylesheetMap, key: &str) -> Option<AlignItems> {
    match stylesheet_string(style, key)?.trim() {
        "start" | "flex-start" => Some(AlignItems::Start),
        "center" => Some(AlignItems::Center),
        "end" | "flex-end" => Some(AlignItems::End),
        "stretch" => Some(AlignItems::Stretch),
        _ => None,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn stylesheet_style(entries: [(&str, StyleValue); 11]) -> StylesheetMap {
        entries
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect()
    }

    #[test]
    fn builds_style_from_stylesheet_and_exposes_font_descriptor() {
        let container = StylesheetContainer {
            width: 200.0,
            height: 300.0,
            dpi: None,
            rem_base: Some(10.0),
            orientation: None,
        };
        let stylesheet = Stylesheet::new(StyleValue::Object(stylesheet_style([
            ("width", 24.into()),
            ("marginTop", 12.into()),
            ("marginRight", 14.into()),
            ("paddingLeft", 8.into()),
            ("backgroundColor", "#112233".into()),
            ("color", "#AABBCCDD".into()),
            ("fontFamily", "Inter".into()),
            ("fontStyle", "italic".into()),
            ("fontWeight", 600.into()),
            ("fontSourceStandard", "Helvetica-Bold".into()),
            ("justifyContent", "center".into()),
        ])));

        let style = Style::from_stylesheet(&container, &stylesheet);

        assert_eq!(style.width, Some(Pt::new(24.0)));
        assert_eq!(style.margin.top, Pt::new(12.0));
        assert_eq!(style.margin.right, Pt::new(14.0));
        assert_eq!(style.padding.left, Pt::new(8.0));
        assert_eq!(style.background_color, Some(Color::rgb(0x11, 0x22, 0x33)));
        assert_eq!(style.color, Some(Color::rgba(0xAA, 0xBB, 0xCC, 0xDD)));
        assert_eq!(style.font_style, Some(FontStyle::Italic));
        assert_eq!(style.font_weight, Some(FontVariantWeight::SEMI_BOLD));
        assert_eq!(
            style.font_source,
            Some(FontSource::standard(StandardFont::HelveticaBold))
        );
        assert_eq!(style.justify_content, JustifyContent::Center);

        let descriptor = style
            .font_descriptor()
            .expect("font descriptor should exist");
        assert_eq!(descriptor.family(), "Inter");
        assert_eq!(descriptor.font_style(), FontStyle::Italic);
        assert_eq!(descriptor.font_weight(), FontVariantWeight::SEMI_BOLD);
    }

    #[test]
    fn applying_partial_stylesheet_preserves_existing_values() {
        let mut style = Style {
            width: Some(Pt::new(42.0)),
            font_family: Some(String::from("Existing")),
            ..Style::default()
        };
        let resolved = stylesheet_style([
            ("height", 100.into()),
            ("marginTop", 3.into()),
            ("marginRight", 0.into()),
            ("paddingLeft", 0.into()),
            ("backgroundColor", "#000000".into()),
            ("color", "#FFFFFFFF".into()),
            ("fontFamily", StyleValue::Null),
            ("fontStyle", StyleValue::Null),
            ("fontWeight", StyleValue::Null),
            ("fontSourceStandard", StyleValue::Null),
            ("alignItems", "stretch".into()),
        ]);

        style.apply_resolved_stylesheet(&resolved);

        assert_eq!(style.width, Some(Pt::new(42.0)));
        assert_eq!(style.height, Some(Pt::new(100.0)));
        assert_eq!(style.margin.top, Pt::new(3.0));
        assert_eq!(style.font_family.as_deref(), Some("Existing"));
        assert_eq!(style.align_items, AlignItems::Stretch);
    }
}
