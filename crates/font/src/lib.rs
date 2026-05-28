pub mod error;

pub use error::*;

use base64::Engine;
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

impl fmt::Display for FontStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Normal => "normal",
            Self::Italic => "italic",
            Self::Oblique => "oblique",
        };

        f.write_str(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FontWeight(u16);

impl FontWeight {
    pub const THIN: Self = Self(100);
    pub const EXTRA_LIGHT: Self = Self(200);
    pub const LIGHT: Self = Self(300);
    pub const NORMAL: Self = Self(400);
    pub const MEDIUM: Self = Self(500);
    pub const SEMI_BOLD: Self = Self(600);
    pub const BOLD: Self = Self(700);
    pub const EXTRA_BOLD: Self = Self(800);
    pub const BLACK: Self = Self(900);

    pub fn new(weight: u16) -> Result<Self> {
        if (1..=1000).contains(&weight) {
            Ok(Self(weight))
        } else {
            Err(Error::InvalidFontWeight { weight })
        }
    }

    pub const fn value(self) -> u16 {
        self.0
    }
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl TryFrom<u16> for FontWeight {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self> {
        Self::new(value)
    }
}

impl From<FontWeight> for u16 {
    fn from(value: FontWeight) -> Self {
        value.value()
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
    pub const fn family_name(self) -> &'static str {
        match self {
            Self::TimesRoman | Self::TimesBold | Self::TimesItalic | Self::TimesBoldItalic => {
                "Times-Roman"
            }
            Self::Helvetica
            | Self::HelveticaBold
            | Self::HelveticaOblique
            | Self::HelveticaBoldOblique => "Helvetica",
            Self::Courier | Self::CourierBold | Self::CourierOblique | Self::CourierBoldOblique => {
                "Courier"
            }
            Self::Symbol => "Symbol",
            Self::ZapfDingbats => "ZapfDingbats",
        }
    }

    pub const fn font_style(self) -> FontStyle {
        match self {
            Self::TimesItalic | Self::TimesBoldItalic => FontStyle::Italic,
            Self::HelveticaOblique
            | Self::HelveticaBoldOblique
            | Self::CourierOblique
            | Self::CourierBoldOblique => FontStyle::Oblique,
            _ => FontStyle::Normal,
        }
    }

    pub const fn font_weight(self) -> FontWeight {
        match self {
            Self::TimesBold
            | Self::TimesBoldItalic
            | Self::HelveticaBold
            | Self::HelveticaBoldOblique
            | Self::CourierBold
            | Self::CourierBoldOblique => FontWeight::BOLD,
            _ => FontWeight::NORMAL,
        }
    }

    pub const fn as_str(self) -> &'static str {
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FontSource {
    Local(PathBuf),
    Remote(String),
    DataUri(String),
    Standard(StandardFont),
}

impl FontSource {
    pub fn local(path: impl Into<PathBuf>) -> Self {
        Self::Local(path.into())
    }

    pub fn remote(url: impl Into<String>) -> Self {
        Self::Remote(url.into())
    }

    pub fn data_uri(uri: impl Into<String>) -> Self {
        Self::DataUri(uri.into())
    }

    pub const fn standard(font: StandardFont) -> Self {
        Self::Standard(font)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FontDescriptor {
    family: String,
    font_style: FontStyle,
    font_weight: FontWeight,
}

impl FontDescriptor {
    pub fn new(family: impl Into<String>) -> Self {
        Self {
            family: family.into(),
            font_style: FontStyle::Normal,
            font_weight: FontWeight::NORMAL,
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub const fn font_style(&self) -> FontStyle {
        self.font_style
    }

    pub const fn font_weight(&self) -> FontWeight {
        self.font_weight
    }

    pub fn with_style(mut self, font_style: FontStyle) -> Self {
        self.font_style = font_style;
        self
    }

    pub fn with_weight(mut self, font_weight: FontWeight) -> Self {
        self.font_weight = font_weight;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FontRegistration {
    family: String,
    source: FontSource,
    font_style: FontStyle,
    font_weight: FontWeight,
}

impl FontRegistration {
    pub fn new(family: impl Into<String>, source: FontSource) -> Self {
        Self {
            family: family.into(),
            source,
            font_style: FontStyle::Normal,
            font_weight: FontWeight::NORMAL,
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub const fn source(&self) -> &FontSource {
        &self.source
    }

    pub const fn font_style(&self) -> FontStyle {
        self.font_style
    }

    pub const fn font_weight(&self) -> FontWeight {
        self.font_weight
    }

    pub fn with_style(mut self, font_style: FontStyle) -> Self {
        self.font_style = font_style;
        self
    }

    pub fn with_weight(mut self, font_weight: FontWeight) -> Self {
        self.font_weight = font_weight;
        self
    }

    fn descriptor(&self) -> FontDescriptor {
        FontDescriptor::new(self.family.clone())
            .with_style(self.font_style)
            .with_weight(self.font_weight)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FontVariantRegistration {
    source: FontSource,
    font_style: FontStyle,
    font_weight: FontWeight,
}

impl FontVariantRegistration {
    pub fn new(source: FontSource) -> Self {
        Self {
            source,
            font_style: FontStyle::Normal,
            font_weight: FontWeight::NORMAL,
        }
    }

    pub const fn source(&self) -> &FontSource {
        &self.source
    }

    pub const fn font_style(&self) -> FontStyle {
        self.font_style
    }

    pub const fn font_weight(&self) -> FontWeight {
        self.font_weight
    }

    pub fn with_style(mut self, font_style: FontStyle) -> Self {
        self.font_style = font_style;
        self
    }

    pub fn with_weight(mut self, font_weight: FontWeight) -> Self {
        self.font_weight = font_weight;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FontFamilyRegistration {
    family: String,
    fonts: Vec<FontVariantRegistration>,
}

impl FontFamilyRegistration {
    pub fn new(
        family: impl Into<String>,
        fonts: impl IntoIterator<Item = FontVariantRegistration>,
    ) -> Self {
        Self {
            family: family.into(),
            fonts: fonts.into_iter().collect(),
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn fonts(&self) -> &[FontVariantRegistration] {
        &self.fonts
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisteredFont {
    descriptor: FontDescriptor,
    source: FontSource,
}

impl RegisteredFont {
    pub fn descriptor(&self) -> &FontDescriptor {
        &self.descriptor
    }

    pub const fn source(&self) -> &FontSource {
        &self.source
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadedFontData {
    Binary(Vec<u8>),
    Standard(StandardFont),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadedFont {
    descriptor: FontDescriptor,
    source: FontSource,
    data: LoadedFontData,
}

impl LoadedFont {
    pub fn descriptor(&self) -> &FontDescriptor {
        &self.descriptor
    }

    pub const fn source(&self) -> &FontSource {
        &self.source
    }

    pub fn data(&self) -> &LoadedFontData {
        &self.data
    }

    pub fn bytes(&self) -> Option<&[u8]> {
        match &self.data {
            LoadedFontData::Binary(bytes) => Some(bytes.as_slice()),
            LoadedFontData::Standard(_) => None,
        }
    }

    pub fn standard_font(&self) -> Option<StandardFont> {
        match &self.data {
            LoadedFontData::Standard(font) => Some(*font),
            LoadedFontData::Binary(_) => None,
        }
    }
}

pub type HyphenationCallback = Arc<dyn Fn(&str) -> Vec<String> + Send + Sync>;
pub type EmojiUrlBuilder = Arc<dyn Fn(&str) -> String + Send + Sync>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EmojiFormat {
    Png,
    Svg,
    Jpeg,
    Gif,
    Webp,
}

impl EmojiFormat {
    pub const fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Svg => "svg",
            Self::Jpeg => "jpg",
            Self::Gif => "gif",
            Self::Webp => "webp",
        }
    }
}

#[derive(Clone)]
pub enum EmojiSource {
    Url {
        base_url: String,
        format: EmojiFormat,
        with_variation_selectors: bool,
    },
    Builder {
        builder: EmojiUrlBuilder,
        format: EmojiFormat,
        with_variation_selectors: bool,
    },
}

impl fmt::Debug for EmojiSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Url {
                base_url,
                format,
                with_variation_selectors,
            } => f
                .debug_struct("EmojiSource::Url")
                .field("base_url", base_url)
                .field("format", format)
                .field("with_variation_selectors", with_variation_selectors)
                .finish(),
            Self::Builder {
                format,
                with_variation_selectors,
                ..
            } => f
                .debug_struct("EmojiSource::Builder")
                .field("format", format)
                .field("with_variation_selectors", with_variation_selectors)
                .finish_non_exhaustive(),
        }
    }
}

impl EmojiSource {
    pub fn url(base_url: impl Into<String>, format: EmojiFormat) -> Self {
        Self::Url {
            base_url: base_url.into(),
            format,
            with_variation_selectors: false,
        }
    }

    pub fn builder<F>(builder: F, format: EmojiFormat) -> Self
    where
        F: Fn(&str) -> String + Send + Sync + 'static,
    {
        Self::Builder {
            builder: Arc::new(builder),
            format,
            with_variation_selectors: false,
        }
    }

    pub fn with_variation_selectors(mut self, with_variation_selectors: bool) -> Self {
        match &mut self {
            Self::Url {
                with_variation_selectors: value,
                ..
            }
            | Self::Builder {
                with_variation_selectors: value,
                ..
            } => {
                *value = with_variation_selectors;
            }
        }

        self
    }

    pub fn resolve_url(&self, emoji: &str) -> Option<String> {
        if emoji.is_empty() {
            return None;
        }

        let code = match self {
            Self::Url {
                with_variation_selectors,
                ..
            }
            | Self::Builder {
                with_variation_selectors,
                ..
            } => emoji_codepoint_string(emoji, *with_variation_selectors)?,
        };

        match self {
            Self::Url {
                base_url, format, ..
            } => {
                let trimmed = base_url.trim_end_matches('/');
                Some(format!("{trimmed}/{code}.{}", format.extension()))
            }
            Self::Builder { builder, .. } => Some(builder(&code)),
        }
    }
}

#[derive(Clone)]
pub struct FontStore {
    families: HashMap<String, FontFamily>,
    emoji_source: Option<EmojiSource>,
    hyphenation_callback: HyphenationCallback,
}

impl Default for FontStore {
    fn default() -> Self {
        Self::new()
    }
}

impl FontStore {
    pub fn new() -> Self {
        let mut store = Self {
            families: HashMap::new(),
            emoji_source: None,
            hyphenation_callback: Arc::new(|word| vec![word.to_string()]),
        };
        store.register_standard_fonts();
        store
    }

    pub fn register_font(&mut self, registration: FontRegistration) -> Result<()> {
        validate_family_name(registration.family())?;
        self.insert_font(registration);
        Ok(())
    }

    pub fn register_family(&mut self, registration: FontFamilyRegistration) -> Result<()> {
        validate_family_name(registration.family())?;

        let FontFamilyRegistration { family, fonts } = registration;

        for font in fonts {
            let registration = FontRegistration::new(family.clone(), font.source)
                .with_style(font.font_style)
                .with_weight(font.font_weight);
            self.insert_font(registration);
        }

        Ok(())
    }

    pub fn get_font(&self, descriptor: &FontDescriptor) -> Result<RegisteredFont> {
        let family =
            self.families
                .get(descriptor.family())
                .ok_or_else(|| Error::UnknownFontFamily {
                    family: descriptor.family().to_string(),
                })?;

        let weights =
            family
                .fonts
                .get(&descriptor.font_style())
                .ok_or_else(|| Error::UnknownFontStyle {
                    family: descriptor.family().to_string(),
                    style: descriptor.font_style().to_string(),
                })?;

        let resolved_weight =
            resolve_font_weight(weights.keys().copied(), descriptor.font_weight()).ok_or_else(
                || Error::UnknownFontWeight {
                    family: descriptor.family().to_string(),
                    style: descriptor.font_style().to_string(),
                    weight: descriptor.font_weight().value(),
                },
            )?;

        weights
            .get(&resolved_weight)
            .cloned()
            .ok_or_else(|| Error::UnknownFontWeight {
                family: descriptor.family().to_string(),
                style: descriptor.font_style().to_string(),
                weight: descriptor.font_weight().value(),
            })
    }

    pub async fn load(&self, descriptor: &FontDescriptor) -> Result<LoadedFont> {
        let font = self.get_font(descriptor)?;
        let data = load_source(&font.source).await?;

        Ok(LoadedFont {
            descriptor: font.descriptor,
            source: font.source,
            data,
        })
    }

    pub fn register_emoji_source(&mut self, source: EmojiSource) {
        self.emoji_source = Some(source);
    }

    pub fn emoji_source(&self) -> Option<&EmojiSource> {
        self.emoji_source.as_ref()
    }

    pub fn resolve_emoji_url(&self, emoji: &str) -> Option<String> {
        self.emoji_source.as_ref()?.resolve_url(emoji)
    }

    pub fn register_hyphenation_callback<F>(&mut self, callback: F)
    where
        F: Fn(&str) -> Vec<String> + Send + Sync + 'static,
    {
        self.hyphenation_callback = Arc::new(callback);
    }

    pub fn hyphenate(&self, word: &str) -> Vec<String> {
        (self.hyphenation_callback)(word)
    }

    fn insert_font(&mut self, registration: FontRegistration) {
        let family = self
            .families
            .entry(registration.family.clone())
            .or_default();

        family
            .fonts
            .entry(registration.font_style)
            .or_default()
            .insert(
                registration.font_weight,
                RegisteredFont {
                    descriptor: registration.descriptor(),
                    source: registration.source,
                },
            );
    }

    fn register_standard_fonts(&mut self) {
        self.insert_standard_font(StandardFont::Helvetica);
        self.insert_standard_font(StandardFont::HelveticaBold);
        self.insert_standard_font(StandardFont::HelveticaOblique);
        self.insert_standard_font(StandardFont::HelveticaBoldOblique);
        self.insert_standard_font(StandardFont::Courier);
        self.insert_standard_font(StandardFont::CourierBold);
        self.insert_standard_font(StandardFont::CourierOblique);
        self.insert_standard_font(StandardFont::CourierBoldOblique);
        self.insert_standard_font(StandardFont::TimesRoman);
        self.insert_standard_font(StandardFont::TimesBold);
        self.insert_standard_font(StandardFont::TimesItalic);
        self.insert_standard_font(StandardFont::TimesBoldItalic);
        self.insert_standard_font(StandardFont::Symbol);
        self.insert_standard_font(StandardFont::ZapfDingbats);
    }

    fn insert_standard_font(&mut self, font: StandardFont) {
        self.insert_font(
            FontRegistration::new(font.family_name(), FontSource::standard(font))
                .with_style(font.font_style())
                .with_weight(font.font_weight()),
        );
    }
}

#[derive(Clone, Debug, Default)]
struct FontFamily {
    fonts: HashMap<FontStyle, BTreeMap<FontWeight, RegisteredFont>>,
}

fn validate_family_name(family: &str) -> Result<()> {
    if family.trim().is_empty() {
        Err(Error::InvalidFontSource {
            message: String::from("font family cannot be empty"),
        })
    } else {
        Ok(())
    }
}

fn resolve_font_weight(
    weights: impl IntoIterator<Item = FontWeight>,
    target: FontWeight,
) -> Option<FontWeight> {
    let weights: Vec<_> = weights.into_iter().collect();
    if weights.is_empty() {
        return None;
    }

    if let Some(weight) = weights.iter().copied().find(|weight| *weight == target) {
        return Some(weight);
    }

    let target_value = target.value();
    let exact_or_between = |start: u16, end: u16| {
        weights
            .iter()
            .copied()
            .filter(|weight| {
                let value = weight.value();
                value >= start && value <= end
            })
            .min_by_key(|weight| weight.value())
    };
    let below_desc = || {
        weights
            .iter()
            .copied()
            .filter(|weight| weight.value() < target_value)
            .max_by_key(|weight| weight.value())
    };
    let above_asc_from = |threshold: u16| {
        weights
            .iter()
            .copied()
            .filter(|weight| weight.value() > threshold)
            .min_by_key(|weight| weight.value())
    };
    let above_target = || {
        weights
            .iter()
            .copied()
            .filter(|weight| weight.value() > target_value)
            .min_by_key(|weight| weight.value())
    };

    if (400..=500).contains(&target_value) {
        exact_or_between(target_value, 500)
            .or_else(below_desc)
            .or_else(|| above_asc_from(500))
    } else if target_value < 400 {
        below_desc().or_else(above_target)
    } else {
        above_target().or_else(below_desc)
    }
}

async fn load_source(source: &FontSource) -> Result<LoadedFontData> {
    match source {
        FontSource::Standard(font) => Ok(LoadedFontData::Standard(*font)),
        FontSource::Local(path) => {
            let bytes = tokio::fs::read(path)
                .await
                .map_err(|error| Error::LocalFontLoad {
                    path: path.display().to_string(),
                    message: error.to_string(),
                })?;
            Ok(LoadedFontData::Binary(bytes))
        }
        FontSource::Remote(url) => {
            let parsed = reqwest::Url::parse(url).map_err(|error| Error::InvalidFontSource {
                message: format!("invalid remote font URL `{url}`: {error}"),
            })?;
            let scheme = parsed.scheme();
            if scheme != "http" && scheme != "https" {
                return Err(Error::UnsupportedRemoteScheme {
                    scheme: scheme.to_string(),
                });
            }

            let response = reqwest::get(parsed)
                .await
                .map_err(|error| Error::RemoteFontLoad {
                    url: url.clone(),
                    message: error.to_string(),
                })?
                .error_for_status()
                .map_err(|error| Error::RemoteFontLoad {
                    url: url.clone(),
                    message: error.to_string(),
                })?;

            let bytes = response
                .bytes()
                .await
                .map_err(|error| Error::RemoteFontLoad {
                    url: url.clone(),
                    message: error.to_string(),
                })?;

            Ok(LoadedFontData::Binary(bytes.to_vec()))
        }
        FontSource::DataUri(uri) => Ok(LoadedFontData::Binary(decode_data_uri(uri)?)),
    }
}

fn decode_data_uri(uri: &str) -> Result<Vec<u8>> {
    let encoded = uri
        .strip_prefix("data:")
        .ok_or_else(|| Error::InvalidDataUri {
            message: String::from("data URI must start with `data:`"),
        })?;

    let (metadata, payload) = encoded
        .split_once(',')
        .ok_or_else(|| Error::InvalidDataUri {
            message: String::from("data URI must contain a metadata and payload separator"),
        })?;

    let is_base64 = metadata
        .split(';')
        .any(|part| part.eq_ignore_ascii_case("base64"));

    if !is_base64 {
        return Err(Error::InvalidDataUri {
            message: String::from("only base64-encoded font data URIs are supported"),
        });
    }

    base64::engine::general_purpose::STANDARD
        .decode(payload)
        .map_err(|error| Error::InvalidDataUri {
            message: error.to_string(),
        })
}

fn emoji_codepoint_string(emoji: &str, with_variation_selectors: bool) -> Option<String> {
    let codes = emoji
        .chars()
        .filter(|character| with_variation_selectors || *character != '\u{fe0f}')
        .map(|character| format!("{:x}", character as u32))
        .collect::<Vec<_>>();

    if codes.is_empty() {
        None
    } else {
        Some(codes.join("-"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use tempfile::tempdir;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[test]
    fn registers_standard_fonts_by_default() {
        let store = FontStore::new();

        let descriptor = FontDescriptor::new("Helvetica").with_weight(FontWeight::BOLD);
        let font = store
            .get_font(&descriptor)
            .expect("standard font should resolve");

        assert_eq!(
            font.source(),
            &FontSource::standard(StandardFont::HelveticaBold)
        );
    }

    #[test]
    fn resolves_fallback_weights_using_css_rules() {
        let mut store = FontStore::new();
        store
            .register_family(FontFamilyRegistration::new(
                "Inter",
                [
                    FontVariantRegistration::new(FontSource::data_uri(font_data_uri(b"regular")))
                        .with_weight(FontWeight::NORMAL),
                    FontVariantRegistration::new(FontSource::data_uri(font_data_uri(b"medium")))
                        .with_weight(FontWeight::MEDIUM),
                    FontVariantRegistration::new(FontSource::data_uri(font_data_uri(b"bold")))
                        .with_weight(FontWeight::BOLD),
                ],
            ))
            .expect("family registration should succeed");

        let mediumish = FontDescriptor::new("Inter")
            .with_weight(FontWeight::new(450).expect("450 is a valid weight"));
        let heavy = FontDescriptor::new("Inter")
            .with_weight(FontWeight::new(800).expect("800 is a valid weight"));

        let mediumish_font = store
            .get_font(&mediumish)
            .expect("450 should resolve to 500");
        let heavy_font = store.get_font(&heavy).expect("800 should resolve to 700");

        assert_eq!(
            mediumish_font.descriptor().font_weight(),
            FontWeight::MEDIUM
        );
        assert_eq!(heavy_font.descriptor().font_weight(), FontWeight::BOLD);
    }

    #[tokio::test]
    async fn loads_local_fonts_asynchronously() {
        let mut store = FontStore::new();
        let directory = tempdir().expect("temporary directory should be created");
        let path = directory.path().join("local-font.ttf");
        let expected = b"local-font-bytes";
        std::fs::write(&path, expected).expect("font file should be written");

        store
            .register_font(FontRegistration::new(
                "LocalFamily",
                FontSource::local(&path),
            ))
            .expect("font registration should succeed");

        let loaded = store
            .load(&FontDescriptor::new("LocalFamily"))
            .await
            .expect("local font should load");

        assert_eq!(loaded.bytes(), Some(expected.as_slice()));
    }

    #[tokio::test]
    async fn loads_remote_fonts_asynchronously() {
        let mut store = FontStore::new();
        let expected = b"remote-font-bytes".to_vec();
        let url = spawn_font_server(expected.clone())
            .await
            .expect("test server should start");

        store
            .register_font(FontRegistration::new(
                "RemoteFamily",
                FontSource::remote(url),
            ))
            .expect("font registration should succeed");

        let loaded = store
            .load(&FontDescriptor::new("RemoteFamily"))
            .await
            .expect("remote font should load");

        assert_eq!(loaded.bytes(), Some(expected.as_slice()));
    }

    #[tokio::test]
    async fn loads_data_uri_fonts_asynchronously() {
        let mut store = FontStore::new();
        let expected = b"data-uri-font-bytes";

        store
            .register_font(FontRegistration::new(
                "DataUriFamily",
                FontSource::data_uri(font_data_uri(expected)),
            ))
            .expect("font registration should succeed");

        let loaded = store
            .load(&FontDescriptor::new("DataUriFamily"))
            .await
            .expect("data URI font should load");

        assert_eq!(loaded.bytes(), Some(expected.as_slice()));
    }

    #[tokio::test]
    async fn resolves_standard_fonts_without_binary_loading() {
        let store = FontStore::new();
        let loaded = store
            .load(&FontDescriptor::new("Times-Roman").with_style(FontStyle::Italic))
            .await
            .expect("standard font should load");

        assert_eq!(loaded.standard_font(), Some(StandardFont::TimesItalic));
        assert_eq!(loaded.bytes(), None);
    }

    #[test]
    fn registers_emoji_and_hyphenation_handlers() {
        let mut store = FontStore::new();
        store.register_hyphenation_callback(|word| {
            vec![word[..5].to_string(), word[5..].to_string()]
        });
        store.register_emoji_source(EmojiSource::url(
            "https://example.com/emojis/",
            EmojiFormat::Png,
        ));

        assert_eq!(
            store.hyphenate("graphite"),
            vec![String::from("graph"), String::from("ite")]
        );
        assert_eq!(
            store.resolve_emoji_url("☺️"),
            Some(String::from("https://example.com/emojis/263a.png"))
        );

        store.register_emoji_source(
            EmojiSource::url("https://example.com/emojis", EmojiFormat::Png)
                .with_variation_selectors(true),
        );

        assert_eq!(
            store.resolve_emoji_url("☺️"),
            Some(String::from("https://example.com/emojis/263a-fe0f.png"))
        );
    }

    fn font_data_uri(bytes: &[u8]) -> String {
        format!(
            "data:font/ttf;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(bytes)
        )
    }

    async fn spawn_font_server(body: Vec<u8>) -> io::Result<String> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;

        tokio::spawn(async move {
            let (mut stream, _) = listener
                .accept()
                .await
                .expect("test server should accept a connection");
            let mut request_buffer = [0_u8; 1024];
            let _ = stream.read(&mut request_buffer).await;

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            stream
                .write_all(response.as_bytes())
                .await
                .expect("test server should write headers");
            stream
                .write_all(&body)
                .await
                .expect("test server should write body");
        });

        Ok(format!("http://{address}/font.ttf"))
    }
}
