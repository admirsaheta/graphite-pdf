#[cfg(feature = "fonts-engine")]
use crate::error::{GraphitePdfKitError, Result};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Font {
    pub name: String,
    pub data: Vec<u8>,
}

impl Font {
    #[cfg(feature = "fonts-engine")]
    pub fn from_bytes(name: impl Into<String>, data: Vec<u8>) -> Result<Self> {
        // Verify font data with ttf-parser
        let _ = ttf_parser::Face::parse(&data, 0)
            .map_err(|e| GraphitePdfKitError::FontError(format!("Invalid font data: {:?}", e)))?;

        Ok(Self {
            name: name.into(),
            data,
        })
    }

    pub fn standard(name: StandardFont) -> Self {
        Self {
            name: name.as_str().to_string(),
            data: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    #[cfg(feature = "fonts-engine")]
    pub fn measure_text_width(&self, text: &str, font_size: f64) -> Result<f64> {
        if self.data.is_empty() {
            // For standard fonts, use rough estimate
            Ok(text.len() as f64 * font_size * 0.6)
        } else {
            let face = ttf_parser::Face::parse(&self.data, 0)
                .map_err(|e| GraphitePdfKitError::FontError(format!("Failed to parse font: {:?}", e)))?;

            let units_per_em = face.units_per_em() as f64;
            let scale = font_size / units_per_em;

            let mut width = 0.0;
            for c in text.chars() {
                if let Some(glyph_id) = face.glyph_index(c) {
                    if let Some(advance) = face.glyph_hor_advance(glyph_id) {
                        width += advance as f64 * scale;
                    }
                }
            }
            Ok(width)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StandardFont {
    TimesRoman,
    TimesBold,
    TimesItalic,
    TimesBoldItalic,
    Helvetica,
    HelveticaBold,
    HelveticaOblique,
    HelveticaBoldOblique,
    Courier,
    CourierBold,
    CourierOblique,
    CourierBoldOblique,
    Symbol,
    ZapfDingbats,
}

impl StandardFont {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TimesRoman => "Times-Roman",
            Self::TimesBold => "Times-Bold",
            Self::TimesItalic => "Times-Italic",
            Self::TimesBoldItalic => "Times-BoldItalic",
            Self::Helvetica => "Helvetica",
            Self::HelveticaBold => "Helvetica-Bold",
            Self::HelveticaOblique => "Helvetica-Oblique",
            Self::HelveticaBoldOblique => "Helvetica-BoldOblique",
            Self::Courier => "Courier",
            Self::CourierBold => "Courier-Bold",
            Self::CourierOblique => "Courier-Oblique",
            Self::CourierBoldOblique => "Courier-BoldOblique",
            Self::Symbol => "Symbol",
            Self::ZapfDingbats => "ZapfDingbats",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct FontRegistry {
    pub fonts: HashMap<String, (Font, u64)>,
    next_id: u64,
}

impl FontRegistry {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn register(&mut self, font: Font) -> String {
        let id = self.next_id;
        let name = format!("F{}", id);
        self.next_id += 1;
        self.fonts.insert(name.clone(), (font, id));
        name
    }

    pub fn get(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name).map(|(font, _)| font)
    }
}
