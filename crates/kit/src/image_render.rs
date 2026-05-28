use std::fmt::Write as _;
use std::io::Cursor;

use crate::error::{GraphitePdfKitError, Result};
use crate::svg_render::{SvgRenderOptions, render_svg_node_to_page_content_with_options};
use graphitepdf_image::{Image, ImageFormat, RasterImage};

#[derive(Clone, Debug, PartialEq)]
pub struct ImageRenderOptions {
    pub x: f64,
    pub y: f64,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

impl ImageRenderOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
}

impl Default for ImageRenderOptions {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: None,
            height: None,
        }
    }
}

pub fn render_image_to_page_content(image: &Image) -> Result<Vec<u8>> {
    render_image_to_page_content_with_options(image, &ImageRenderOptions::default())
}

pub fn render_image_to_page_content_with_options(
    image: &Image,
    options: &ImageRenderOptions,
) -> Result<Vec<u8>> {
    match image {
        Image::Raster(raster) => render_raster_to_page_content(raster, options),
        Image::Svg(svg) => {
            let (width, height) = resolve_size(svg.width as f64, svg.height as f64, options)?;
            render_svg_node_to_page_content_with_options(
                &svg.data,
                &SvgRenderOptions::new()
                    .position(options.x, options.y)
                    .size(width, height),
            )
        }
    }
}

fn render_raster_to_page_content(
    raster: &RasterImage,
    options: &ImageRenderOptions,
) -> Result<Vec<u8>> {
    let (width, height) = resolve_size(raster.width as f64, raster.height as f64, options)?;
    let decoded = decode_raster_image(raster)?;
    let mut content = String::new();

    content.push_str("q\n");
    let _ = writeln!(
        content,
        "{} 0 0 {} {} {} cm",
        format_number(width),
        format_number(height),
        format_number(options.x),
        format_number(options.y)
    );
    content.push_str("BI\n");
    let _ = writeln!(content, "/Width {}", decoded.width);
    let _ = writeln!(content, "/Height {}", decoded.height);
    let _ = writeln!(content, "/ColorSpace /{}", decoded.color_space);
    content.push_str("/BitsPerComponent 8\n");
    content.push_str("/Filter /ASCIIHexDecode\n");
    content.push_str("ID\n");
    content.push_str(&hex_encode(&decoded.data));
    content.push_str(">\nEI\nQ\n");

    Ok(content.into_bytes())
}

fn resolve_size(
    natural_width: f64,
    natural_height: f64,
    options: &ImageRenderOptions,
) -> Result<(f64, f64)> {
    if natural_width <= 0.0 || natural_height <= 0.0 {
        return Err(GraphitePdfKitError::ImageError(
            "image dimensions must be positive".to_string(),
        ));
    }

    let size = match (options.width, options.height) {
        (Some(width), Some(height)) => (width, height),
        (Some(width), None) => (width, width * (natural_height / natural_width)),
        (None, Some(height)) => (height * (natural_width / natural_height), height),
        (None, None) => (natural_width, natural_height),
    };

    if size.0 <= 0.0 || size.1 <= 0.0 {
        Err(GraphitePdfKitError::ImageError(
            "rendered image dimensions must be positive".to_string(),
        ))
    } else {
        Ok(size)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DecodedRaster {
    width: u32,
    height: u32,
    color_space: &'static str,
    data: Vec<u8>,
}

#[cfg(feature = "images")]
fn decode_raster_image(raster: &RasterImage) -> Result<DecodedRaster> {
    match raster.format {
        ImageFormat::Png => decode_png(raster),
        ImageFormat::Jpeg => decode_jpeg(raster),
        ImageFormat::Svg => Err(GraphitePdfKitError::UnsupportedFeature(
            "SVG raster decoding is not supported",
        )),
    }
}

#[cfg(not(feature = "images"))]
fn decode_raster_image(_raster: &RasterImage) -> Result<DecodedRaster> {
    Err(GraphitePdfKitError::UnsupportedFeature(
        "image decoding requires the `images` feature",
    ))
}

#[cfg(feature = "images")]
fn decode_png(raster: &RasterImage) -> Result<DecodedRaster> {
    let mut decoder = png::Decoder::new(Cursor::new(&raster.data));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().map_err(|error| {
        GraphitePdfKitError::ImageError(format!("failed to decode PNG: {error}"))
    })?;
    let output_size = reader.output_buffer_size().ok_or_else(|| {
        GraphitePdfKitError::ImageError("PNG decoder did not report an output size".to_string())
    })?;
    let mut buffer = vec![0; output_size];
    let info = reader.next_frame(&mut buffer).map_err(|error| {
        GraphitePdfKitError::ImageError(format!("failed to read PNG frame: {error}"))
    })?;
    let data = &buffer[..info.buffer_size()];

    let rgb = match info.color_type {
        png::ColorType::Rgb => data.to_vec(),
        png::ColorType::Rgba => data
            .chunks_exact(4)
            .flat_map(|chunk| [chunk[0], chunk[1], chunk[2]])
            .collect(),
        png::ColorType::Grayscale => data
            .iter()
            .flat_map(|value| [*value, *value, *value])
            .collect(),
        png::ColorType::GrayscaleAlpha => data
            .chunks_exact(2)
            .flat_map(|chunk| [chunk[0], chunk[0], chunk[0]])
            .collect(),
        other => {
            return Err(GraphitePdfKitError::ImageError(format!(
                "unsupported PNG color type {other:?}"
            )));
        }
    };

    Ok(DecodedRaster {
        width: info.width,
        height: info.height,
        color_space: "DeviceRGB",
        data: rgb,
    })
}

#[cfg(feature = "images")]
fn decode_jpeg(raster: &RasterImage) -> Result<DecodedRaster> {
    let mut decoder = jpeg_decoder::Decoder::new(Cursor::new(&raster.data));
    let pixels = decoder.decode().map_err(|error| {
        GraphitePdfKitError::ImageError(format!("failed to decode JPEG: {error}"))
    })?;
    let info = decoder.info().ok_or_else(|| {
        GraphitePdfKitError::ImageError("JPEG decoder did not return image info".to_string())
    })?;

    let rgb = match info.pixel_format {
        jpeg_decoder::PixelFormat::L8 => pixels
            .iter()
            .flat_map(|value| [*value, *value, *value])
            .collect(),
        jpeg_decoder::PixelFormat::RGB24 => pixels,
        jpeg_decoder::PixelFormat::CMYK32 => pixels
            .chunks_exact(4)
            .flat_map(|chunk| cmyk_to_rgb(chunk[0], chunk[1], chunk[2], chunk[3]))
            .collect(),
        other => {
            return Err(GraphitePdfKitError::ImageError(format!(
                "unsupported JPEG pixel format {other:?}"
            )));
        }
    };

    Ok(DecodedRaster {
        width: u32::from(info.width),
        height: u32::from(info.height),
        color_space: "DeviceRGB",
        data: rgb,
    })
}

#[cfg(feature = "images")]
fn cmyk_to_rgb(c: u8, m: u8, y: u8, k: u8) -> [u8; 3] {
    let c = f32::from(c) / 255.0;
    let m = f32::from(m) / 255.0;
    let y = f32::from(y) / 255.0;
    let k = f32::from(k) / 255.0;

    [
        ((1.0 - (c * (1.0 - k) + k)) * 255.0).round() as u8,
        ((1.0 - (m * (1.0 - k) + k)) * 255.0).round() as u8,
        ((1.0 - (y * (1.0 - k) + k)) * 255.0).round() as u8,
    ]
}

fn hex_encode(data: &[u8]) -> String {
    let mut encoded = String::with_capacity(data.len() * 2);
    for byte in data {
        let _ = write!(encoded, "{byte:02X}");
    }
    encoded
}

fn format_number(value: f64) -> String {
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
