use crate::primitives::{Color, Pt};

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
            flex_direction: FlexDirection::default(),
            justify_content: JustifyContent::default(),
            align_items: AlignItems::default(),
        }
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
