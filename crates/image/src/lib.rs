pub mod error;

pub use error::*;

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::path::PathBuf;
use std::str::from_utf8;
use std::sync::{Arc, Mutex, OnceLock};

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use graphitepdf_svg::{SvgNode, try_parse_svg};

const DEFAULT_CACHE_LIMIT: usize = 30;
const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    Jpeg,
    Png,
    Svg,
}

impl ImageFormat {
    fn from_str(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "png" => Some(Self::Png),
            "svg" | "svg+xml" => Some(Self::Svg),
            _ => None,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Jpeg => "jpeg",
            Self::Png => "png",
            Self::Svg => "svg",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataImageSource {
    pub data: Vec<u8>,
    pub format: ImageFormat,
}

impl DataImageSource {
    pub fn new(data: impl Into<Vec<u8>>, format: ImageFormat) -> Self {
        Self {
            data: data.into(),
            format,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalImageSource {
    pub path: PathBuf,
    pub format: Option<ImageFormat>,
}

impl LocalImageSource {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            format: None,
        }
    }

    pub fn with_format(mut self, format: ImageFormat) -> Self {
        self.format = Some(format);
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RemoteMethod {
    #[default]
    Get,
    Head,
    Post,
    Put,
    Delete,
    Patch,
}

impl RemoteMethod {
    fn as_reqwest_method(self) -> reqwest::Method {
        match self {
            Self::Get => reqwest::Method::GET,
            Self::Head => reqwest::Method::HEAD,
            Self::Post => reqwest::Method::POST,
            Self::Put => reqwest::Method::PUT,
            Self::Delete => reqwest::Method::DELETE,
            Self::Patch => reqwest::Method::PATCH,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RemoteCredentials {
    Omit,
    SameOrigin,
    Include,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemoteImageSource {
    pub uri: String,
    pub method: RemoteMethod,
    pub headers: BTreeMap<String, String>,
    pub format: Option<ImageFormat>,
    pub body: Option<Vec<u8>>,
    pub credentials: Option<RemoteCredentials>,
}

impl RemoteImageSource {
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            method: RemoteMethod::Get,
            headers: BTreeMap::new(),
            format: None,
            body: None,
            credentials: None,
        }
    }

    pub fn with_method(mut self, method: RemoteMethod) -> Self {
        self.method = method;
        self
    }

    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    pub fn with_body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn with_format(mut self, format: ImageFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub fn with_credentials(mut self, credentials: RemoteCredentials) -> Self {
        self.credentials = Some(credentials);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataUriImageSource {
    pub uri: String,
}

impl DataUriImageSource {
    pub fn new(uri: impl Into<String>) -> Self {
        Self { uri: uri.into() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImageSource {
    Bytes(Vec<u8>),
    Data(DataImageSource),
    Local(LocalImageSource),
    Remote(RemoteImageSource),
    DataUri(DataUriImageSource),
}

impl ImageSource {
    fn cache_key(&self) -> Option<String> {
        match self {
            Self::Bytes(_) => None,
            Self::Data(source) => Some(BASE64.encode(&source.data)),
            Self::Local(source) => Some(source.path.to_string_lossy().into_owned()),
            Self::Remote(source) => Some(source.uri.clone()),
            Self::DataUri(source) => Some(source.uri.clone()),
        }
    }
}

impl From<Vec<u8>> for ImageSource {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl From<&[u8]> for ImageSource {
    fn from(value: &[u8]) -> Self {
        Self::Bytes(value.to_vec())
    }
}

impl From<DataImageSource> for ImageSource {
    fn from(value: DataImageSource) -> Self {
        Self::Data(value)
    }
}

impl From<LocalImageSource> for ImageSource {
    fn from(value: LocalImageSource) -> Self {
        Self::Local(value)
    }
}

impl From<RemoteImageSource> for ImageSource {
    fn from(value: RemoteImageSource) -> Self {
        Self::Remote(value)
    }
}

impl From<DataUriImageSource> for ImageSource {
    fn from(value: DataUriImageSource) -> Self {
        Self::DataUri(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RasterImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub format: ImageFormat,
    pub key: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SvgImage {
    pub width: f32,
    pub height: f32,
    pub data: SvgNode,
    pub raw_data: Vec<u8>,
    pub key: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Image {
    Raster(RasterImage),
    Svg(SvgImage),
}

pub type ImageAsset = Image;

impl Image {
    pub fn format(&self) -> ImageFormat {
        match self {
            Self::Raster(image) => image.format,
            Self::Svg(_) => ImageFormat::Svg,
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            Self::Raster(image) => image.width as f32,
            Self::Svg(image) => image.width,
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            Self::Raster(image) => image.height as f32,
            Self::Svg(image) => image.height,
        }
    }

    pub fn key(&self) -> Option<&str> {
        match self {
            Self::Raster(image) => image.key.as_deref(),
            Self::Svg(image) => image.key.as_deref(),
        }
    }

    fn set_key(&mut self, key: String) {
        match self {
            Self::Raster(image) => image.key = Some(key),
            Self::Svg(image) => image.key = Some(key),
        }
    }
}

#[derive(Debug)]
pub struct ImageCache {
    limit: usize,
    state: Mutex<CacheState>,
}

#[derive(Debug, Default)]
struct CacheState {
    entries: HashMap<String, Arc<Image>>,
    order: VecDeque<String>,
}

impl ImageCache {
    pub fn new(limit: usize) -> Self {
        Self {
            limit: limit.max(1),
            state: Mutex::new(CacheState::default()),
        }
    }

    pub fn get(&self, key: &str) -> Option<Arc<Image>> {
        let mut state = self.state.lock().expect("image cache mutex poisoned");
        let value = state.entries.get(key).cloned();

        if value.is_some() {
            touch_key(&mut state.order, key);
        }

        value
    }

    pub fn set(&self, key: impl Into<String>, value: Arc<Image>) {
        let key = key.into();
        let mut state = self.state.lock().expect("image cache mutex poisoned");

        state.entries.insert(key.clone(), value);
        touch_key(&mut state.order, &key);

        while state.entries.len() > self.limit {
            if let Some(oldest) = state.order.pop_front() {
                if oldest != key {
                    state.entries.remove(&oldest);
                }
            } else {
                break;
            }
        }
    }

    pub fn reset(&self) {
        let mut state = self.state.lock().expect("image cache mutex poisoned");
        state.entries.clear();
        state.order.clear();
    }

    pub fn len(&self) -> usize {
        let state = self.state.lock().expect("image cache mutex poisoned");
        state.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn touch_key(order: &mut VecDeque<String>, key: &str) {
    if let Some(index) = order.iter().position(|existing| existing == key) {
        order.remove(index);
    }

    order.push_back(key.to_string());
}

fn global_image_cache() -> &'static ImageCache {
    static CACHE: OnceLock<ImageCache> = OnceLock::new();
    CACHE.get_or_init(|| ImageCache::new(DEFAULT_CACHE_LIMIT))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolveImageOptions {
    pub cache: bool,
}

impl Default for ResolveImageOptions {
    fn default() -> Self {
        Self { cache: true }
    }
}

pub async fn resolve_image(source: impl Into<ImageSource>) -> Result<Arc<Image>> {
    resolve_image_with_options(source, ResolveImageOptions::default()).await
}

pub async fn resolve_image_with_options(
    source: impl Into<ImageSource>,
    options: ResolveImageOptions,
) -> Result<Arc<Image>> {
    resolve_image_with_cache(source.into(), &options, global_image_cache()).await
}

async fn resolve_image_with_cache(
    source: ImageSource,
    options: &ResolveImageOptions,
    cache: &ImageCache,
) -> Result<Arc<Image>> {
    let cache_key = source.cache_key();

    if options.cache
        && let Some(ref key) = cache_key
        && let Some(image) = cache.get(key)
    {
        return Ok(image);
    }

    let mut image = match source {
        ImageSource::Bytes(bytes) => resolve_bytes_image(bytes, None)?,
        ImageSource::Data(source) => resolve_data_image(source)?,
        ImageSource::Local(source) => resolve_local_image(source).await?,
        ImageSource::Remote(source) => resolve_remote_image(source).await?,
        ImageSource::DataUri(source) => resolve_data_uri_image(source)?,
    };

    if let Some(key) = cache_key {
        image.set_key(key.clone());

        let image = Arc::new(image);
        if options.cache {
            cache.set(key, Arc::clone(&image));
        }

        return Ok(image);
    }

    Ok(Arc::new(image))
}

fn resolve_data_image(source: DataImageSource) -> Result<Image> {
    parse_image(source.data, source.format)
}

async fn resolve_local_image(source: LocalImageSource) -> Result<Image> {
    let bytes = tokio::fs::read(source.path).await?;
    resolve_bytes_image(bytes, source.format)
}

async fn resolve_remote_image(source: RemoteImageSource) -> Result<Image> {
    let mut request =
        reqwest::Client::new().request(source.method.as_reqwest_method(), &source.uri);

    for (name, value) in source.headers {
        request = request.header(name, value);
    }

    if let Some(body) = source.body {
        request = request.body(body);
    }

    let bytes = request
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?
        .to_vec();

    resolve_bytes_image(bytes, source.format)
}

fn resolve_data_uri_image(source: DataUriImageSource) -> Result<Image> {
    let payload =
        source
            .uri
            .strip_prefix("data:image/")
            .ok_or_else(|| Error::InvalidImageData {
                message: format!("invalid image data URI: {}", source.uri),
            })?;

    let (metadata, encoded) = payload
        .split_once(',')
        .ok_or_else(|| Error::InvalidImageData {
            message: format!("invalid image data URI: {}", source.uri),
        })?;
    let (format, encoding) = metadata
        .split_once(';')
        .ok_or_else(|| Error::InvalidImageData {
            message: format!("invalid image data URI metadata: {metadata}"),
        })?;

    if !encoding.eq_ignore_ascii_case("base64") {
        return Err(Error::InvalidImageData {
            message: format!("unsupported image data URI encoding: {encoding}"),
        });
    }

    let format = ImageFormat::from_str(format).ok_or_else(|| Error::UnsupportedImageFormat {
        format: format.to_string(),
    })?;
    let data = BASE64.decode(encoded)?;

    parse_image(data, format)
}

fn resolve_bytes_image(bytes: Vec<u8>, declared_format: Option<ImageFormat>) -> Result<Image> {
    let format = sniff_image_format(&bytes)
        .or(declared_format)
        .ok_or_else(|| Error::InvalidImageData {
            message: "unable to determine image format from bytes".to_string(),
        })?;

    parse_image(bytes, format)
}

fn parse_image(bytes: Vec<u8>, format: ImageFormat) -> Result<Image> {
    match format {
        ImageFormat::Png => parse_png(bytes).map(Image::Raster),
        ImageFormat::Jpeg => parse_jpeg(bytes).map(Image::Raster),
        ImageFormat::Svg => parse_svg(bytes).map(Image::Svg),
    }
}

fn parse_png(data: Vec<u8>) -> Result<RasterImage> {
    if !is_png(&data) {
        return Err(Error::InvalidImageData {
            message: "PNG signature not found".to_string(),
        });
    }

    if data.len() < 24 || &data[12..16] != b"IHDR" {
        return Err(Error::InvalidImageData {
            message: "PNG missing IHDR chunk".to_string(),
        });
    }

    let width = read_be_u32(&data[16..20])?;
    let height = read_be_u32(&data[20..24])?;

    Ok(RasterImage {
        width,
        height,
        data,
        format: ImageFormat::Png,
        key: None,
    })
}

fn parse_jpeg(data: Vec<u8>) -> Result<RasterImage> {
    if !is_jpeg(&data) {
        return Err(Error::InvalidImageData {
            message: "SOI not found in JPEG".to_string(),
        });
    }

    let mut offset = 2;
    let mut width = None;
    let mut height = None;
    let mut orientation = None;

    while offset + 1 < data.len() {
        if data[offset] != 0xFF {
            offset += 1;
            continue;
        }

        while offset < data.len() && data[offset] == 0xFF {
            offset += 1;
        }

        if offset >= data.len() {
            break;
        }

        let marker = data[offset];
        offset += 1;

        if marker == 0xD9 || marker == 0xDA {
            break;
        }

        if matches!(marker, 0x01 | 0xD0..=0xD7) {
            continue;
        }

        if offset + 2 > data.len() {
            break;
        }

        let segment_length = read_be_u16(&data[offset..offset + 2])? as usize;
        if segment_length < 2 || offset + segment_length > data.len() {
            return Err(Error::InvalidImageData {
                message: "JPEG segment exceeds input length".to_string(),
            });
        }

        let segment_start = offset + 2;
        let segment_end = offset + segment_length;
        let segment = &data[segment_start..segment_end];

        if marker == 0xE1 {
            orientation = parse_exif_orientation(segment)?;
        } else if is_start_of_frame(marker) {
            if segment.len() < 5 {
                return Err(Error::InvalidImageData {
                    message: "JPEG SOF segment too short".to_string(),
                });
            }

            height = Some(read_be_u16(&segment[1..3])? as u32);
            width = Some(read_be_u16(&segment[3..5])? as u32);
        }

        offset = segment_end;
    }

    let mut width = width.ok_or_else(|| Error::InvalidImageData {
        message: "JPEG dimensions not found".to_string(),
    })?;
    let mut height = height.ok_or_else(|| Error::InvalidImageData {
        message: "JPEG dimensions not found".to_string(),
    })?;

    if matches!(orientation, Some(5..=8)) {
        std::mem::swap(&mut width, &mut height);
    }

    Ok(RasterImage {
        width,
        height,
        data,
        format: ImageFormat::Jpeg,
        key: None,
    })
}

fn parse_svg(data: Vec<u8>) -> Result<SvgImage> {
    if !is_svg(&data) {
        return Err(Error::InvalidImageData {
            message: "SVG signature not found".to_string(),
        });
    }

    let svg_string = from_utf8(strip_utf8_bom(&data))?;
    let parsed = try_parse_svg(svg_string)?;
    let view_box = parsed
        .props
        .get("viewBox")
        .and_then(|value| parse_view_box(value));
    let width = parsed
        .props
        .get("width")
        .and_then(|value| parse_svg_dimension(value))
        .or_else(|| view_box.map(|view_box| view_box.width))
        .unwrap_or(0.0);
    let height = parsed
        .props
        .get("height")
        .and_then(|value| parse_svg_dimension(value))
        .or_else(|| view_box.map(|view_box| view_box.height))
        .unwrap_or(0.0);

    Ok(SvgImage {
        width,
        height,
        data: parsed,
        raw_data: data,
        key: None,
    })
}

fn sniff_image_format(data: &[u8]) -> Option<ImageFormat> {
    if is_jpeg(data) {
        Some(ImageFormat::Jpeg)
    } else if is_png(data) {
        Some(ImageFormat::Png)
    } else if is_svg(data) {
        Some(ImageFormat::Svg)
    } else {
        None
    }
}

fn is_png(data: &[u8]) -> bool {
    data.starts_with(&PNG_SIGNATURE)
}

fn is_jpeg(data: &[u8]) -> bool {
    data.len() >= 2 && data[0] == 0xFF && data[1] == 0xD8
}

fn is_svg(data: &[u8]) -> bool {
    let Ok(text) = from_utf8(strip_utf8_bom(data)) else {
        return false;
    };

    let trimmed = text.trim_start();
    trimmed.starts_with("<?xml") || trimmed.starts_with("<svg")
}

fn strip_utf8_bom(data: &[u8]) -> &[u8] {
    data.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(data)
}

fn is_start_of_frame(marker: u8) -> bool {
    matches!(
        marker,
        0xC0 | 0xC1 | 0xC2 | 0xC3 | 0xC5 | 0xC6 | 0xC7 | 0xC9 | 0xCA | 0xCB | 0xCD | 0xCE | 0xCF
    )
}

fn parse_exif_orientation(segment: &[u8]) -> Result<Option<u16>> {
    if !segment.starts_with(b"Exif\0\0") {
        return Ok(None);
    }

    let tiff = &segment[6..];
    if tiff.len() < 8 {
        return Ok(None);
    }

    let big_endian = match &tiff[..2] {
        b"MM" => true,
        b"II" => false,
        _ => return Ok(None),
    };

    let ifd_offset = read_endian_u32(&tiff[4..8], big_endian)? as usize;
    if ifd_offset + 2 > tiff.len() {
        return Ok(None);
    }

    let entry_count = read_endian_u16(&tiff[ifd_offset..ifd_offset + 2], big_endian)? as usize;
    let mut entry_offset = ifd_offset + 2;

    for _ in 0..entry_count {
        if entry_offset + 12 > tiff.len() {
            return Ok(None);
        }

        let entry = &tiff[entry_offset..entry_offset + 12];
        let tag = read_endian_u16(&entry[0..2], big_endian)?;
        let field_type = read_endian_u16(&entry[2..4], big_endian)?;
        let count = read_endian_u32(&entry[4..8], big_endian)?;

        if tag == 0x0112 && field_type == 3 && count >= 1 {
            let value = if big_endian {
                read_be_u16(&entry[8..10])?
            } else {
                read_le_u16(&entry[8..10])?
            };

            return Ok(Some(value));
        }

        entry_offset += 12;
    }

    Ok(None)
}

fn read_be_u16(bytes: &[u8]) -> Result<u16> {
    let array = bytes.try_into().map_err(|_| Error::InvalidImageData {
        message: "expected a 2-byte big-endian integer".to_string(),
    })?;
    Ok(u16::from_be_bytes(array))
}

fn read_be_u32(bytes: &[u8]) -> Result<u32> {
    let array = bytes.try_into().map_err(|_| Error::InvalidImageData {
        message: "expected a 4-byte big-endian integer".to_string(),
    })?;
    Ok(u32::from_be_bytes(array))
}

fn read_le_u16(bytes: &[u8]) -> Result<u16> {
    let array = bytes.try_into().map_err(|_| Error::InvalidImageData {
        message: "expected a 2-byte little-endian integer".to_string(),
    })?;
    Ok(u16::from_le_bytes(array))
}

fn read_endian_u16(bytes: &[u8], big_endian: bool) -> Result<u16> {
    if big_endian {
        read_be_u16(bytes)
    } else {
        read_le_u16(bytes)
    }
}

fn read_endian_u32(bytes: &[u8], big_endian: bool) -> Result<u32> {
    let array: [u8; 4] = bytes.try_into().map_err(|_| Error::InvalidImageData {
        message: "expected a 4-byte endian integer".to_string(),
    })?;

    Ok(if big_endian {
        u32::from_be_bytes(array)
    } else {
        u32::from_le_bytes(array)
    })
}

#[derive(Clone, Copy, Debug)]
struct ViewBox {
    width: f32,
    height: f32,
}

fn parse_view_box(value: &str) -> Option<ViewBox> {
    let parts: Vec<_> = value
        .split(|character: char| character.is_ascii_whitespace() || character == ',')
        .filter(|part| !part.is_empty())
        .collect();

    if parts.len() != 4 {
        return None;
    }

    let width = parts[2].parse::<f32>().ok()?;
    let height = parts[3].parse::<f32>().ok()?;

    Some(ViewBox { width, height })
}

fn parse_svg_dimension(value: &str) -> Option<f32> {
    let value = value.trim();

    for (suffix, multiplier) in [
        ("px", 72.0 / 96.0),
        ("pt", 1.0),
        ("in", 72.0),
        ("cm", 72.0 / 2.54),
        ("mm", 72.0 / 25.4),
    ] {
        if let Some(number) = value.strip_suffix(suffix) {
            return number
                .trim()
                .parse::<f32>()
                .ok()
                .map(|value| value * multiplier);
        }
    }

    value.parse::<f32>().ok()
}

#[cfg(test)]
#[allow(clippy::await_holding_lock)]
mod tests {
    use super::*;

    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

    const PNG_1X1: &[u8] = &[
        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6,
        0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 13, 73, 68, 65, 84, 120, 218, 99, 248, 207, 192, 0, 0,
        3, 1, 1, 0, 201, 254, 146, 239, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
    ];

    fn test_guard() -> std::sync::MutexGuard<'static, ()> {
        TEST_MUTEX
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("test mutex poisoned")
    }

    fn reset_cache() {
        global_image_cache().reset();
    }

    fn unique_temp_path(extension: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("graphitepdf-image-{timestamp}.{extension}"))
    }

    fn png_data_uri() -> String {
        format!("data:image/png;base64,{}", BASE64.encode(PNG_1X1))
    }

    fn svg_bytes() -> Vec<u8> {
        br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 180"></svg>"#.to_vec()
    }

    fn jpeg_with_orientation(width: u16, height: u16, orientation: u16) -> Vec<u8> {
        let exif_payload = [
            b'E',
            b'x',
            b'i',
            b'f',
            0,
            0,
            b'M',
            b'M',
            0,
            42,
            0,
            0,
            0,
            8,
            0,
            1,
            0x01,
            0x12,
            0,
            3,
            0,
            0,
            0,
            1,
            (orientation >> 8) as u8,
            orientation as u8,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let app1_length = (exif_payload.len() + 2) as u16;

        let mut bytes = vec![0xFF, 0xD8, 0xFF, 0xE1];
        bytes.extend_from_slice(&app1_length.to_be_bytes());
        bytes.extend_from_slice(&exif_payload);
        bytes.extend_from_slice(&[
            0xFF,
            0xC0,
            0x00,
            0x11,
            0x08,
            (height >> 8) as u8,
            height as u8,
            (width >> 8) as u8,
            width as u8,
            0x03,
            0x01,
            0x11,
            0x00,
            0x02,
            0x11,
            0x01,
            0x03,
            0x11,
            0x01,
            0xFF,
            0xD9,
        ]);
        bytes
    }

    async fn serve_once(response_body: Vec<u8>) -> String {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("listener should bind");
        let address = listener
            .local_addr()
            .expect("listener should have local address");

        tokio::spawn(async move {
            let (mut stream, _) = listener
                .accept()
                .await
                .expect("connection should be accepted");
            let mut request = vec![0_u8; 2048];
            let _ = stream.read(&mut request).await;

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: image/png\r\nConnection: close\r\n\r\n",
                response_body.len()
            );

            stream
                .write_all(response.as_bytes())
                .await
                .expect("headers should be written");
            stream
                .write_all(&response_body)
                .await
                .expect("body should be written");
        });

        format!("http://{address}/image.png")
    }

    #[tokio::test]
    async fn resolves_png_bytes_without_using_cache() {
        let _guard = test_guard();
        reset_cache();

        let first = resolve_image(PNG_1X1.to_vec())
            .await
            .expect("first byte image should resolve");
        let second = resolve_image(PNG_1X1.to_vec())
            .await
            .expect("second byte image should resolve");

        assert!(
            matches!(&*first, Image::Raster(image) if image.width == 1 && image.height == 1 && image.format == ImageFormat::Png)
        );
        assert!(!Arc::ptr_eq(&first, &second));
        assert_eq!(global_image_cache().len(), 0);
    }

    #[tokio::test]
    async fn resolves_data_source_and_reuses_cached_result() {
        let _guard = test_guard();
        reset_cache();

        let source = DataImageSource::new(PNG_1X1, ImageFormat::Png);

        let first = resolve_image(source.clone())
            .await
            .expect("data source should resolve");
        let second = resolve_image(source)
            .await
            .expect("cached data source should resolve");

        assert!(Arc::ptr_eq(&first, &second));
        assert_eq!(first.key(), Some(BASE64.encode(PNG_1X1).as_str()));
        assert_eq!(global_image_cache().len(), 1);
    }

    #[tokio::test]
    async fn resolves_png_from_data_uri() {
        let _guard = test_guard();
        reset_cache();

        let image = resolve_image(DataUriImageSource::new(png_data_uri()))
            .await
            .expect("data URI image should resolve");

        assert!(
            matches!(&*image, Image::Raster(raster) if raster.width == 1 && raster.height == 1)
        );
        assert_eq!(image.format(), ImageFormat::Png);
        assert_eq!(global_image_cache().len(), 1);
    }

    #[tokio::test]
    async fn resolves_png_from_local_file() {
        let _guard = test_guard();
        reset_cache();

        let path = unique_temp_path("png");
        fs::write(&path, PNG_1X1).expect("temp PNG should be written");

        let result = resolve_image(LocalImageSource::new(&path)).await;
        let _ = fs::remove_file(&path);

        let image = result.expect("local image should resolve");
        assert!(
            matches!(&*image, Image::Raster(raster) if raster.width == 1 && raster.height == 1)
        );
        assert_eq!(image.key(), Some(path.to_string_lossy().as_ref()));
    }

    #[tokio::test]
    async fn resolves_png_from_remote_url() {
        let _guard = test_guard();
        reset_cache();

        let uri = serve_once(PNG_1X1.to_vec()).await;
        let image = resolve_image(RemoteImageSource::new(uri.clone()))
            .await
            .expect("remote image should resolve");

        assert!(
            matches!(&*image, Image::Raster(raster) if raster.width == 1 && raster.height == 1)
        );
        assert_eq!(image.key(), Some(uri.as_str()));
    }

    #[tokio::test]
    async fn parses_svg_dimensions_from_view_box() {
        let _guard = test_guard();
        reset_cache();

        let image = resolve_image(svg_bytes())
            .await
            .expect("SVG bytes should resolve");

        assert!(matches!(&*image, Image::Svg(svg) if svg.width == 320.0 && svg.height == 180.0));
        assert_eq!(image.format(), ImageFormat::Svg);
    }

    #[tokio::test]
    async fn parses_jpeg_and_applies_exif_orientation() {
        let _guard = test_guard();
        reset_cache();

        let jpeg = jpeg_with_orientation(3, 2, 6);
        let image = resolve_image(jpeg).await.expect("JPEG should resolve");

        assert!(
            matches!(&*image, Image::Raster(raster) if raster.width == 2 && raster.height == 3 && raster.format == ImageFormat::Jpeg)
        );
    }

    #[tokio::test]
    async fn supports_disabling_cache() {
        let _guard = test_guard();
        reset_cache();

        let source = DataImageSource::new(PNG_1X1, ImageFormat::Png);
        let options = ResolveImageOptions { cache: false };

        let first = resolve_image_with_options(source.clone(), options.clone())
            .await
            .expect("uncached image should resolve");
        let second = resolve_image_with_options(source, options)
            .await
            .expect("second uncached image should resolve");

        assert!(!Arc::ptr_eq(&first, &second));
        assert_eq!(global_image_cache().len(), 0);
    }

    #[tokio::test]
    async fn evicts_least_recently_used_entries() {
        let _guard = test_guard();

        let cache = ImageCache::new(2);
        let first = Arc::new(Image::Raster(RasterImage {
            width: 1,
            height: 1,
            data: PNG_1X1.to_vec(),
            format: ImageFormat::Png,
            key: Some("first".to_string()),
        }));
        let second = Arc::new(Image::Raster(RasterImage {
            width: 1,
            height: 1,
            data: PNG_1X1.to_vec(),
            format: ImageFormat::Png,
            key: Some("second".to_string()),
        }));
        let third = Arc::new(Image::Raster(RasterImage {
            width: 1,
            height: 1,
            data: PNG_1X1.to_vec(),
            format: ImageFormat::Png,
            key: Some("third".to_string()),
        }));

        cache.set("first", Arc::clone(&first));
        cache.set("second", Arc::clone(&second));
        let cached_first = cache.get("first").expect("first entry should be present");
        assert!(Arc::ptr_eq(&cached_first, &first));

        cache.set("third", Arc::clone(&third));

        assert!(cache.get("first").is_some());
        assert!(cache.get("third").is_some());
        assert!(cache.get("second").is_none());
    }

    #[test]
    fn parses_svg_dimensions_with_supported_units() {
        assert_eq!(parse_svg_dimension("96px"), Some(72.0));
        assert_eq!(parse_svg_dimension("2in"), Some(144.0));
        assert_eq!(parse_svg_dimension("10"), Some(10.0));
    }

    #[test]
    fn reports_invalid_data_uris() {
        let error =
            resolve_data_uri_image(DataUriImageSource::new("data:text/plain;base64,SGVsbG8="))
                .expect_err("non-image data URI should be rejected");

        assert!(matches!(error, Error::InvalidImageData { .. }));
    }
}
