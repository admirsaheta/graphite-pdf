pub mod error;

pub use error::*;

use std::collections::BTreeMap;
use std::fmt;
use std::str::from_utf8;

use graphitepdf_primitives as P;
use quick_xml::Reader;
use quick_xml::XmlVersion;
use quick_xml::escape::unescape;
use quick_xml::events::{BytesCData, BytesRef, BytesStart, BytesText, Event};

pub type SvgProps = BTreeMap<String, String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SvgNodeKind {
    Svg,
    G,
    Path,
    Rect,
    Circle,
    Ellipse,
    Line,
    Polyline,
    Polygon,
    Text,
    Tspan,
    Defs,
    ClipPath,
    LinearGradient,
    RadialGradient,
    Marker,
    Stop,
    Image,
    Use,
    TextInstance,
}

impl SvgNodeKind {
    pub const fn primitive_name(self) -> &'static str {
        match self {
            Self::Svg => P::Svg,
            Self::G => P::G,
            Self::Path => P::Path,
            Self::Rect => P::Rect,
            Self::Circle => P::Circle,
            Self::Ellipse => P::Ellipse,
            Self::Line => P::Line,
            Self::Polyline => P::Polyline,
            Self::Polygon => P::Polygon,
            Self::Text => P::Text,
            Self::Tspan => P::Tspan,
            Self::Defs => P::Defs,
            Self::ClipPath => P::ClipPath,
            Self::LinearGradient => P::LinearGradient,
            Self::RadialGradient => P::RadialGradient,
            Self::Marker => P::Marker,
            Self::Stop => P::Stop,
            Self::Image => P::Image,
            Self::Use => P::Use,
            Self::TextInstance => P::TextInstance,
        }
    }

    fn from_tag_name(tag_name: &str) -> Option<Self> {
        match tag_name {
            "svg" => Some(Self::Svg),
            "g" => Some(Self::G),
            "path" => Some(Self::Path),
            "rect" => Some(Self::Rect),
            "circle" => Some(Self::Circle),
            "ellipse" => Some(Self::Ellipse),
            "line" => Some(Self::Line),
            "polyline" => Some(Self::Polyline),
            "polygon" => Some(Self::Polygon),
            "text" => Some(Self::Text),
            "tspan" => Some(Self::Tspan),
            "defs" => Some(Self::Defs),
            "clippath" => Some(Self::ClipPath),
            "lineargradient" => Some(Self::LinearGradient),
            "radialgradient" => Some(Self::RadialGradient),
            "marker" => Some(Self::Marker),
            "stop" => Some(Self::Stop),
            "image" => Some(Self::Image),
            "use" => Some(Self::Use),
            _ => None,
        }
    }

    const fn is_text_container(self) -> bool {
        matches!(self, Self::Text | Self::Tspan)
    }
}

impl fmt::Display for SvgNodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.primitive_name())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SvgNode {
    pub kind: SvgNodeKind,
    pub r#type: &'static str,
    pub props: SvgProps,
    pub children: Vec<SvgNode>,
    pub value: Option<String>,
}

impl SvgNode {
    pub fn new(kind: SvgNodeKind) -> Self {
        Self {
            kind,
            r#type: kind.primitive_name(),
            props: SvgProps::new(),
            children: Vec::new(),
            value: None,
        }
    }

    pub fn empty_svg() -> Self {
        Self::new(SvgNodeKind::Svg)
    }

    pub fn type_name(&self) -> &'static str {
        self.r#type
    }

    fn text_instance(value: String) -> Self {
        let mut node = Self::new(SvgNodeKind::TextInstance);
        node.value = Some(value);
        node
    }
}

impl Default for SvgNode {
    fn default() -> Self {
        Self::empty_svg()
    }
}

#[derive(Debug)]
struct OpenNode {
    tag_name: String,
    node: SvgNode,
}

fn is_skipped_element(tag_name: &str) -> bool {
    matches!(
        tag_name,
        "script"
            | "foreignobject"
            | "filter"
            | "mask"
            | "pattern"
            | "symbol"
            | "animate"
            | "animatetransform"
            | "animatemotion"
            | "set"
    )
}

fn to_camel_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut capitalize_next = false;

    for character in input.chars() {
        if character == '-' || character == ':' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(character.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(character);
        }
    }

    result
}

fn parse_style_attribute(style_string: &str) -> SvgProps {
    let mut props = SvgProps::new();

    for declaration in style_string.split(';') {
        let declaration = declaration.trim();
        if declaration.is_empty() {
            continue;
        }

        let Some(colon_index) = declaration.find(':') else {
            continue;
        };

        let property = declaration[..colon_index].trim();
        let value = declaration[colon_index + 1..].trim();

        if !property.is_empty() && !value.is_empty() {
            props.insert(to_camel_case(property), value.to_string());
        }
    }

    props
}

fn convert_attributes(attributes: impl IntoIterator<Item = (String, String)>) -> SvgProps {
    let mut props = SvgProps::new();

    for (name, value) in attributes {
        if name == "style" {
            props.extend(parse_style_attribute(&value));
        } else {
            props.insert(to_camel_case(&name), value);
        }
    }

    props
}

fn get_attributes_from_start(event: &BytesStart<'_>) -> Result<Vec<(String, String)>> {
    let mut attributes = Vec::new();

    for attribute in event.attributes() {
        let attribute = attribute?;
        let key = from_utf8(attribute.key.as_ref())?.to_string();
        let value = attribute
            .normalized_value(XmlVersion::Implicit1_0)?
            .into_owned();
        attributes.push((key, value));
    }

    Ok(attributes)
}

fn decode_tag_name(bytes: &[u8]) -> Result<String> {
    Ok(from_utf8(bytes)?.to_ascii_lowercase())
}

fn attach_node(root: &mut Option<SvgNode>, stack: &mut [OpenNode], node: SvgNode) {
    if let Some(parent) = stack.last_mut() {
        parent.node.children.push(node);
    } else if root.is_none() {
        *root = Some(node);
    }
}

fn push_text_if_relevant(stack: &mut [OpenNode], text: String) {
    let Some(parent) = stack.last_mut() else {
        return;
    };

    if !parent.node.kind.is_text_container() {
        return;
    }

    if text.trim().is_empty() {
        if let Some(last_child) = parent.node.children.last_mut() {
            if last_child.kind == SvgNodeKind::TextInstance {
                if let Some(existing_value) = last_child.value.as_mut() {
                    existing_value.push_str(&text);
                }
            }
        }
        return;
    }

    if let Some(last_child) = parent.node.children.last_mut() {
        if last_child.kind == SvgNodeKind::TextInstance {
            if let Some(existing_value) = last_child.value.as_mut() {
                existing_value.push_str(&text);
                return;
            }
        }
    }

    parent
        .node
        .children
        .push(SvgNode::text_instance(text.trim_start().to_string()));
}

fn trim_last_text_instance(node: &mut SvgNode) {
    if !node.kind.is_text_container() {
        return;
    }

    let Some(last_child) = node.children.last_mut() else {
        return;
    };

    if last_child.kind != SvgNodeKind::TextInstance {
        return;
    }

    let Some(value) = last_child.value.as_mut() else {
        return;
    };

    let trimmed = value.trim_end();
    if trimmed.len() != value.len() {
        *value = trimmed.to_string();
    }
}

fn decode_cdata_text(event: &BytesCData<'_>) -> Result<String> {
    Ok(event.decode()?.into_owned())
}

fn decode_text(event: &BytesText<'_>) -> Result<String> {
    Ok(event.decode()?.into_owned())
}

fn decode_general_reference(reference: &BytesRef<'_>) -> Result<String> {
    let decoded = reference.decode()?.into_owned();
    let escaped = if decoded.starts_with('&') {
        decoded
    } else {
        format!("&{decoded};")
    };

    Ok(unescape(&escaped)?.into_owned())
}

fn collapse_stack(mut stack: Vec<OpenNode>, root: Option<SvgNode>) -> SvgNode {
    if let Some(root) = root {
        return root;
    }

    while stack.len() > 1 {
        let child = stack.pop().expect("stack length checked").node;
        if let Some(parent) = stack.last_mut() {
            parent.node.children.push(child);
        }
    }

    stack.pop().map(|entry| entry.node).unwrap_or_default()
}

pub fn try_parse_svg(svg_string: &str) -> Result<SvgNode> {
    let mut reader = Reader::from_str(svg_string);
    let mut buffer = Vec::new();
    let mut stack: Vec<OpenNode> = Vec::new();
    let mut root: Option<SvgNode> = None;
    let mut skip_depth = 0usize;

    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(event) => {
                let tag_name = decode_tag_name(event.name().as_ref())?;

                if skip_depth > 0 {
                    skip_depth += 1;
                    buffer.clear();
                    continue;
                }

                if is_skipped_element(&tag_name) {
                    skip_depth = 1;
                    buffer.clear();
                    continue;
                }

                let Some(kind) = SvgNodeKind::from_tag_name(&tag_name) else {
                    skip_depth = 1;
                    buffer.clear();
                    continue;
                };

                if let Some(parent) = stack.last_mut() {
                    trim_last_text_instance(&mut parent.node);
                }

                let mut node = SvgNode::new(kind);
                node.props = convert_attributes(get_attributes_from_start(&event)?);

                stack.push(OpenNode { tag_name, node });
            }
            Event::Empty(event) => {
                let tag_name = decode_tag_name(event.name().as_ref())?;

                if skip_depth > 0 || is_skipped_element(&tag_name) {
                    buffer.clear();
                    continue;
                }

                let Some(kind) = SvgNodeKind::from_tag_name(&tag_name) else {
                    buffer.clear();
                    continue;
                };

                if let Some(parent) = stack.last_mut() {
                    trim_last_text_instance(&mut parent.node);
                }

                let mut node = SvgNode::new(kind);
                node.props = convert_attributes(get_attributes_from_start(&event)?);
                attach_node(&mut root, &mut stack, node);
            }
            Event::End(event) => {
                let tag_name = decode_tag_name(event.name().as_ref())?;

                if skip_depth > 0 {
                    skip_depth -= 1;
                    buffer.clear();
                    continue;
                }

                let Some(last_tag_name) = stack.last().map(|entry| entry.tag_name.as_str()) else {
                    buffer.clear();
                    continue;
                };

                if last_tag_name != tag_name {
                    buffer.clear();
                    continue;
                }

                if let Some(last) = stack.last_mut() {
                    trim_last_text_instance(&mut last.node);
                }

                let node = stack.pop().expect("stack is not empty").node;
                attach_node(&mut root, &mut stack, node);
            }
            Event::Text(event) => {
                if skip_depth == 0 {
                    push_text_if_relevant(&mut stack, decode_text(&event)?);
                }
            }
            Event::CData(event) => {
                if skip_depth == 0 {
                    push_text_if_relevant(&mut stack, decode_cdata_text(&event)?);
                }
            }
            Event::GeneralRef(reference) => {
                if skip_depth == 0 {
                    push_text_if_relevant(&mut stack, decode_general_reference(&reference)?);
                }
            }
            Event::Comment(_) | Event::Decl(_) | Event::DocType(_) | Event::PI(_) => {}
            Event::Eof => break,
        }

        buffer.clear();
    }

    Ok(collapse_stack(stack, root))
}

pub fn parse_svg(svg_string: &str) -> SvgNode {
    try_parse_svg(svg_string).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn props(entries: &[(&str, &str)]) -> SvgProps {
        entries
            .iter()
            .map(|(name, value)| (String::from(*name), String::from(*value)))
            .collect()
    }

    fn node(kind: SvgNodeKind, props_entries: &[(&str, &str)], children: Vec<SvgNode>) -> SvgNode {
        SvgNode {
            kind,
            r#type: kind.primitive_name(),
            props: props(props_entries),
            children,
            value: None,
        }
    }

    fn text(value: &str) -> SvgNode {
        SvgNode {
            kind: SvgNodeKind::TextInstance,
            r#type: SvgNodeKind::TextInstance.primitive_name(),
            props: SvgProps::new(),
            children: Vec::new(),
            value: Some(value.to_string()),
        }
    }

    #[test]
    fn parses_dimensions_variants() {
        let unitless =
            parse_svg(r#"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="150"></svg>"#);
        assert_eq!(
            unitless,
            node(
                SvgNodeKind::Svg,
                &[
                    ("height", "150"),
                    ("width", "200"),
                    ("xmlns", "http://www.w3.org/2000/svg")
                ],
                vec![],
            )
        );

        let px = parse_svg(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="96px" height="48px"></svg>"#,
        );
        assert_eq!(px.props.get("width"), Some(&"96px".to_string()));
        assert_eq!(px.props.get("height"), Some(&"48px".to_string()));

        let pt = parse_svg(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="72pt" height="36pt"></svg>"#,
        );
        assert_eq!(pt.props.get("width"), Some(&"72pt".to_string()));
        assert_eq!(pt.props.get("height"), Some(&"36pt".to_string()));

        let inches =
            parse_svg(r#"<svg xmlns="http://www.w3.org/2000/svg" width="1in" height="2in"></svg>"#);
        assert_eq!(inches.props.get("width"), Some(&"1in".to_string()));
        assert_eq!(inches.props.get("height"), Some(&"2in".to_string()));

        let cm = parse_svg(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="2.54cm" height="5.08cm"></svg>"#,
        );
        assert_eq!(cm.props.get("width"), Some(&"2.54cm".to_string()));
        assert_eq!(cm.props.get("height"), Some(&"5.08cm".to_string()));

        let mm = parse_svg(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="25.4mm" height="50.8mm"></svg>"#,
        );
        assert_eq!(mm.props.get("width"), Some(&"25.4mm".to_string()));
        assert_eq!(mm.props.get("height"), Some(&"50.8mm".to_string()));
    }

    #[test]
    fn parses_viewbox_and_missing_dimensions() {
        let view_box =
            parse_svg(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="10 20 300 200"></svg>"#);
        assert_eq!(
            view_box.props.get("viewBox"),
            Some(&"10 20 300 200".to_string())
        );

        let no_dimensions = parse_svg(r#"<svg xmlns="http://www.w3.org/2000/svg"></svg>"#);
        assert_eq!(
            no_dimensions,
            node(
                SvgNodeKind::Svg,
                &[("xmlns", "http://www.w3.org/2000/svg")],
                vec![],
            )
        );
    }

    #[test]
    fn maps_basic_shapes() {
        let svg = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
              <rect x="10" y="10" width="80" height="80" fill="red"/>
              <circle cx="50" cy="50" r="25" fill="blue"/>
              <ellipse cx="50" cy="50" rx="40" ry="20"/>
              <line x1="0" y1="0" x2="100" y2="100" stroke="black"/>
              <polyline points="0,0 50,50 100,0"/>
              <polygon points="50,0 100,100 0,100"/>
            </svg>"#,
        );

        assert_eq!(
            svg,
            node(
                SvgNodeKind::Svg,
                &[
                    ("height", "100"),
                    ("width", "100"),
                    ("xmlns", "http://www.w3.org/2000/svg")
                ],
                vec![
                    node(
                        SvgNodeKind::Rect,
                        &[
                            ("fill", "red"),
                            ("height", "80"),
                            ("width", "80"),
                            ("x", "10"),
                            ("y", "10")
                        ],
                        vec![],
                    ),
                    node(
                        SvgNodeKind::Circle,
                        &[("cx", "50"), ("cy", "50"), ("fill", "blue"), ("r", "25")],
                        vec![],
                    ),
                    node(
                        SvgNodeKind::Ellipse,
                        &[("cx", "50"), ("cy", "50"), ("rx", "40"), ("ry", "20")],
                        vec![],
                    ),
                    node(
                        SvgNodeKind::Line,
                        &[
                            ("stroke", "black"),
                            ("x1", "0"),
                            ("x2", "100"),
                            ("y1", "0"),
                            ("y2", "100")
                        ],
                        vec![],
                    ),
                    node(
                        SvgNodeKind::Polyline,
                        &[("points", "0,0 50,50 100,0")],
                        vec![]
                    ),
                    node(
                        SvgNodeKind::Polygon,
                        &[("points", "50,0 100,100 0,100")],
                        vec![]
                    ),
                ],
            )
        );
    }

    #[test]
    fn maps_path_gradients_groups_clip_path_and_image() {
        let path = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
              <path d="M10 10 H 90 V 90 H 10 Z" fill="none" stroke="black"/>
            </svg>"#,
        );
        assert_eq!(path.children[0].kind, SvgNodeKind::Path);

        let gradients = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
              <defs>
                <linearGradient id="lg1" x1="0%" y1="0%" x2="100%" y2="0%">
                  <stop offset="0%" stop-color="red"/>
                  <stop offset="100%" stop-color="blue"/>
                </linearGradient>
                <radialGradient id="rg1" cx="50%" cy="50%" r="50%">
                  <stop offset="0%" stop-color="white"/>
                  <stop offset="100%" stop-color="black"/>
                </radialGradient>
              </defs>
            </svg>"#,
        );
        assert_eq!(gradients.children[0].kind, SvgNodeKind::Defs);
        assert_eq!(
            gradients.children[0].children[0].kind,
            SvgNodeKind::LinearGradient
        );
        assert_eq!(
            gradients.children[0].children[1].kind,
            SvgNodeKind::RadialGradient
        );
        assert_eq!(
            gradients.children[0].children[0].children[0]
                .props
                .get("stopColor"),
            Some(&"red".to_string())
        );

        let groups = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
              <g transform="translate(10,10)">
                <g opacity="0.5">
                  <rect width="50" height="50"/>
                </g>
                <circle cx="75" cy="75" r="10"/>
              </g>
            </svg>"#,
        );
        assert_eq!(groups.children[0].kind, SvgNodeKind::G);
        assert_eq!(groups.children[0].children[0].kind, SvgNodeKind::G);
        assert_eq!(groups.children[0].children[1].kind, SvgNodeKind::Circle);

        let clip_path = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
              <defs>
                <clipPath id="clip1">
                  <rect width="50" height="50"/>
                </clipPath>
              </defs>
            </svg>"#,
        );
        assert_eq!(
            clip_path.children[0].children[0].kind,
            SvgNodeKind::ClipPath
        );

        let image = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
              <image href="photo.png" x="0" y="0" width="100" height="100"/>
            </svg>"#,
        );
        assert_eq!(
            image.children[0],
            node(
                SvgNodeKind::Image,
                &[
                    ("height", "100"),
                    ("href", "photo.png"),
                    ("width", "100"),
                    ("x", "0"),
                    ("y", "0")
                ],
                vec![],
            )
        );
    }

    #[test]
    fn handles_text_and_tspan_content() {
        let text_svg = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="200" height="50">
              <text x="10" y="30" font-size="20">Hello World</text>
            </svg>"#,
        );
        assert_eq!(
            text_svg.children[0],
            node(
                SvgNodeKind::Text,
                &[("fontSize", "20"), ("x", "10"), ("y", "30")],
                vec![text("Hello World")],
            )
        );

        let tspan_svg = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="200" height="50">
              <text x="10" y="30">
                <tspan fill="red">Red</tspan>
                <tspan fill="blue">Blue</tspan>
              </text>
            </svg>"#,
        );
        assert_eq!(
            tspan_svg.children[0],
            node(
                SvgNodeKind::Text,
                &[("x", "10"), ("y", "30")],
                vec![
                    node(SvgNodeKind::Tspan, &[("fill", "red")], vec![text("Red")]),
                    node(SvgNodeKind::Tspan, &[("fill", "blue")], vec![text("Blue")]),
                ],
            )
        );

        let non_text = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
              <rect>some text</rect>
            </svg>"#,
        );
        assert_eq!(non_text.children[0], node(SvgNodeKind::Rect, &[], vec![]));
    }

    #[test]
    fn converts_attributes_and_styles() {
        let camel_case = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
              <rect stroke-width="2" fill-opacity="0.5" stroke-dasharray="5,3" stroke-linecap="round"/>
            </svg>"#,
        );
        assert_eq!(
            camel_case.children[0],
            node(
                SvgNodeKind::Rect,
                &[
                    ("fillOpacity", "0.5"),
                    ("strokeDasharray", "5,3"),
                    ("strokeLinecap", "round"),
                    ("strokeWidth", "2"),
                ],
                vec![],
            )
        );

        let inline_style = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
              <rect style="fill:red;stroke:blue;stroke-width:2px;opacity:0.8"/>
            </svg>"#,
        );
        assert_eq!(
            inline_style.children[0],
            node(
                SvgNodeKind::Rect,
                &[
                    ("fill", "red"),
                    ("opacity", "0.8"),
                    ("stroke", "blue"),
                    ("strokeWidth", "2px")
                ],
                vec![],
            )
        );

        let merged = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
              <rect fill="green" width="50" height="50" style="stroke:blue;stroke-width:3"/>
            </svg>"#,
        );
        assert_eq!(
            merged.children[0],
            node(
                SvgNodeKind::Rect,
                &[
                    ("fill", "green"),
                    ("height", "50"),
                    ("stroke", "blue"),
                    ("strokeWidth", "3"),
                    ("width", "50"),
                ],
                vec![],
            )
        );

        let single_quoted = parse_svg(
            r#"
            <svg xmlns='http://www.w3.org/2000/svg' width='100' height='100'>
              <rect fill='red' width='50' height='50'/>
            </svg>"#,
        );
        assert_eq!(
            single_quoted.children[0],
            node(
                SvgNodeKind::Rect,
                &[("fill", "red"), ("height", "50"), ("width", "50")],
                vec![],
            )
        );
    }

    #[test]
    fn decodes_entities_and_cdata_in_text() {
        let entities = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
              <text x="10" y="30">&lt;hello&gt; &amp; &quot;world&quot;</text>
            </svg>"#,
        );
        assert_eq!(
            entities.children[0].children,
            vec![text("<hello> & \"world\"")]
        );

        let cdata = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="200" height="50">
              <text x="10" y="30"><![CDATA[Some <special> text]]></text>
            </svg>"#,
        );
        assert_eq!(
            cdata.children[0].children,
            vec![text("Some <special> text")]
        );
    }

    #[test]
    fn skips_unsupported_and_unknown_elements_like_ts_parser() {
        let skipped = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
              <script>alert(1)</script>
              <rect width="50" height="50"/>
              <foreignObject><div>hi</div></foreignObject>
              <circle cx="50" cy="50" r="10"/>
              <filter id="f1"><feGaussianBlur/></filter>
              <mask id="m1"><rect/></mask>
            </svg>"#,
        );
        assert_eq!(
            skipped,
            node(
                SvgNodeKind::Svg,
                &[
                    ("height", "100"),
                    ("width", "100"),
                    ("xmlns", "http://www.w3.org/2000/svg")
                ],
                vec![
                    node(
                        SvgNodeKind::Rect,
                        &[("height", "50"), ("width", "50")],
                        vec![]
                    ),
                    node(
                        SvgNodeKind::Circle,
                        &[("cx", "50"), ("cy", "50"), ("r", "10")],
                        vec![]
                    ),
                ],
            )
        );

        let unknown = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
              <custom-element foo="bar"/>
              <rect width="50" height="50"/>
            </svg>"#,
        );
        assert_eq!(unknown.children.len(), 1);
        assert_eq!(unknown.children[0].kind, SvgNodeKind::Rect);
    }

    #[test]
    fn parses_use_nodes_and_xlink_references() {
        let svg = parse_svg(
            r##"
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
              <defs>
                <path id="shape" d="M0 0 L10 0 L10 10 Z"/>
              </defs>
              <use href="#shape" x="20" y="30"/>
              <use xlink:href="#shape" transform="translate(40,0)"/>
            </svg>"##,
        );

        assert_eq!(svg.children[0].kind, SvgNodeKind::Defs);
        assert_eq!(svg.children[1].kind, SvgNodeKind::Use);
        assert_eq!(
            svg.children[1].props.get("href"),
            Some(&"#shape".to_string())
        );
        assert_eq!(svg.children[1].props.get("x"), Some(&"20".to_string()));
        assert_eq!(svg.children[2].kind, SvgNodeKind::Use);
        assert_eq!(
            svg.children[2].props.get("xlinkHref"),
            Some(&"#shape".to_string())
        );
    }

    #[test]
    fn ignores_xml_preamble_doctype_and_comments() {
        let xml_decl = parse_svg(
            r#"<?xml version="1.0" encoding="UTF-8"?><svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><rect width="50" height="50"/></svg>"#,
        );
        assert_eq!(xml_decl.children[0].kind, SvgNodeKind::Rect);

        let doctype = parse_svg(
            r#"<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd"><svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><rect/></svg>"#,
        );
        assert_eq!(doctype.children[0].kind, SvgNodeKind::Rect);

        let comments = parse_svg(
            r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
              <!-- this is a comment -->
              <rect width="50" height="50"/>
            </svg>"#,
        );
        assert_eq!(comments.children[0].kind, SvgNodeKind::Rect);
    }

    #[test]
    fn handles_invalid_input_like_ts_parser() {
        let non_svg_root = parse_svg("<div>not svg</div>");
        assert_eq!(non_svg_root, SvgNode::empty_svg());

        let empty = parse_svg("");
        assert_eq!(empty, SvgNode::empty_svg());

        let invalid_view_box = parse_svg(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="bad"></svg>"#,
        );
        assert_eq!(
            invalid_view_box.props.get("viewBox"),
            Some(&"bad".to_string())
        );

        let short_view_box = parse_svg(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100"></svg>"#,
        );
        assert_eq!(
            short_view_box.props.get("viewBox"),
            Some(&"0 0 100".to_string())
        );
    }

    #[test]
    fn parses_real_world_like_examples() {
        let icon = parse_svg(
            r##"
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24">
              <defs>
                <linearGradient id="grad" x1="0" y1="0" x2="1" y2="1">
                  <stop offset="0%" stop-color="#ff6b6b"/>
                  <stop offset="100%" stop-color="#4ecdc4"/>
                </linearGradient>
              </defs>
              <g fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 2L2 7l10 5 10-5-10-5z"/>
                <path d="M2 17l10 5 10-5"/>
                <path d="M2 12l10 5 10-5"/>
              </g>
            </svg>"##,
        );
        assert_eq!(icon.children[0].kind, SvgNodeKind::Defs);
        assert_eq!(icon.children[1].kind, SvgNodeKind::G);
        assert_eq!(icon.children[1].children.len(), 3);
        assert_eq!(
            icon.children[1].props,
            props(&[
                ("fill", "none"),
                ("strokeLinecap", "round"),
                ("strokeLinejoin", "round"),
                ("strokeWidth", "2"),
            ])
        );

        let chart = parse_svg(
            r##"
            <svg xmlns="http://www.w3.org/2000/svg" width="200" height="100" viewBox="0 0 200 100">
              <rect width="200" height="100" fill="#f0f0f0"/>
              <g transform="translate(20,80)">
                <line x1="0" y1="0" x2="160" y2="0" stroke="#ccc"/>
                <rect x="0" y="-60" width="30" height="60" style="fill:#4ecdc4;opacity:0.9"/>
                <rect x="40" y="-40" width="30" height="40" style="fill:#ff6b6b;opacity:0.9"/>
                <rect x="80" y="-75" width="30" height="75" style="fill:#45b7d1;opacity:0.9"/>
              </g>
            </svg>"##,
        );
        assert_eq!(chart.children.len(), 2);
        assert_eq!(chart.children[0].kind, SvgNodeKind::Rect);
        assert_eq!(chart.children[1].kind, SvgNodeKind::G);
        assert_eq!(chart.children[1].children.len(), 4);
        assert_eq!(
            chart.children[1].children[1].props,
            props(&[
                ("fill", "#4ecdc4"),
                ("height", "60"),
                ("opacity", "0.9"),
                ("width", "30"),
                ("x", "0"),
                ("y", "-60"),
            ])
        );
    }

    #[test]
    fn exposes_typed_and_fallible_api() {
        let parsed =
            try_parse_svg(r#"<svg xmlns="http://www.w3.org/2000/svg"><text>Hello</text></svg>"#)
                .expect("valid SVG should parse");

        assert_eq!(parsed.kind, SvgNodeKind::Svg);
        assert_eq!(parsed.type_name(), P::Svg);
        assert_eq!(parsed.children[0].kind, SvgNodeKind::Text);
    }
}
