pub mod error;

pub use error::*;

use graphitepdf_font::FontDescriptor;
use graphitepdf_primitives::Pt;

#[derive(Clone, Debug, PartialEq)]
pub struct TextSpan {
    content: String,
    font: Option<FontDescriptor>,
    font_size: Pt,
}

impl TextSpan {
    pub fn new(content: impl Into<String>) -> Result<Self> {
        let content = content.into();
        if content.trim().is_empty() {
            return Err(Error::EmptyText);
        }

        Ok(Self {
            content,
            font: None,
            font_size: Pt::new(12.0),
        })
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn font(&self) -> Option<&FontDescriptor> {
        self.font.as_ref()
    }

    pub const fn font_size(&self) -> Pt {
        self.font_size
    }

    pub fn with_font(mut self, font: FontDescriptor) -> Self {
        self.font = Some(font);
        self
    }

    pub fn with_font_size(mut self, font_size: Pt) -> Result<Self> {
        if font_size.value() <= 0.0 {
            return Err(Error::InvalidFontSize {
                size: font_size.value(),
            });
        }

        self.font_size = font_size;
        Ok(self)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextBlock {
    spans: Vec<TextSpan>,
}

impl TextBlock {
    pub fn new(spans: impl IntoIterator<Item = TextSpan>) -> Self {
        Self {
            spans: spans.into_iter().collect(),
        }
    }

    pub fn push(&mut self, span: TextSpan) {
        self.spans.push(span);
    }

    pub fn spans(&self) -> &[TextSpan] {
        &self.spans
    }

    pub fn plain_text(&self) -> String {
        self.spans
            .iter()
            .map(TextSpan::content)
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn is_empty(&self) -> bool {
        self.spans.is_empty()
    }
}

impl From<TextSpan> for TextBlock {
    fn from(value: TextSpan) -> Self {
        Self { spans: vec![value] }
    }
}
