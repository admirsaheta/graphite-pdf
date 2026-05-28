#![allow(non_upper_case_globals)]

pub const G: &str = "G";
pub const Svg: &str = "SVG";
pub const View: &str = "VIEW";
pub const Text: &str = "TEXT";
pub const Link: &str = "LINK";
pub const Page: &str = "PAGE";
pub const Note: &str = "NOTE";
pub const Path: &str = "PATH";
pub const Rect: &str = "RECT";
pub const Line: &str = "LINE";
pub const FieldSet: &str = "FIELD_SET";
pub const TextInput: &str = "TEXT_INPUT";
pub const Select: &str = "SELECT";
pub const Checkbox: &str = "CHECKBOX";
pub const List: &str = "LIST";
pub const Stop: &str = "STOP";
pub const Defs: &str = "DEFS";
pub const Image: &str = "IMAGE";
pub const ImageBackground: &str = "IMAGE_BACKGROUND";
pub const Tspan: &str = "TSPAN";
pub const Use: &str = "USE";
pub const Canvas: &str = "CANVAS";
pub const Circle: &str = "CIRCLE";
pub const Ellipse: &str = "ELLIPSE";
pub const Polygon: &str = "POLYGON";
pub const Document: &str = "DOCUMENT";
pub const Polyline: &str = "POLYLINE";
pub const ClipPath: &str = "CLIP_PATH";
pub const TextInstance: &str = "TEXT_INSTANCE";
pub const LinearGradient: &str = "LINEAR_GRADIENT";
pub const RadialGradient: &str = "RADIAL_GRADIENT";
pub const Marker: &str = "MARKER";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exports_match_react_pdf_values() {
        assert_eq!(View, "VIEW");
        assert_eq!(Text, "TEXT");
        assert_eq!(Page, "PAGE");
        assert_eq!(Document, "DOCUMENT");
        assert_eq!(Rect, "RECT");
        assert_eq!(LinearGradient, "LINEAR_GRADIENT");
    }
}
