mod support;

use std::sync::Arc;

use anyhow::{Context, Result, anyhow, bail};
use graphitepdf_kit::{
    Canvas, Color, DocumentBuilder, Metadata, PageSize, SvgRenderOptions, TextBuilder,
    ToPdfPageContent,
    font::{FontDescriptor, FontStore, StandardFont},
    image::{Image, LocalImageSource, resolve_image},
};
use graphitepdf_stylesheet::{Container as StylesheetContainer, Style, StyleValue, Stylesheet};
use support::{output_path, workspace_path};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let page_size = PageSize::A4;
    let container = StylesheetContainer::new(page_size.width, page_size.height);
    let stylesheet = stylesheet();
    let resolved = stylesheet.resolve(&container);

    let panel_width = resolved_number(&resolved, "width")?;
    let panel_height = resolved_number(&resolved, "height")?;
    let title_size = resolved_number(&resolved, "fontSize")?;
    let body_size = resolved_number(&resolved, "lineHeight")?;
    let panel_color = color_from_hex(resolved_string(&resolved, "backgroundColor")?)?;
    let title_color = color_from_hex(resolved_string(&resolved, "color")?)?;
    let standard_font = standard_font_from_name(resolved_string(&resolved, "fontSourceStandard")?)?;

    let descriptor = FontDescriptor::new(standard_font.family_name())
        .with_style(standard_font.font_style())
        .with_weight(standard_font.font_weight());
    let loaded_font = FontStore::new()
        .load(&descriptor)
        .await
        .context("failed to load stylesheet-selected font through graphitepdf-font")?;
    let (document, font_name) = DocumentBuilder::new().add_font(loaded_font);

    let png_path = workspace_path("brand/logo/graphitepdf-logo-oss.png");
    let svg_path = workspace_path("brand/logo/graphitepdf-logo-oss.svg");
    let raster_logo = resolve_image(LocalImageSource::new(&png_path))
        .await
        .with_context(|| format!("failed to resolve raster image {}", png_path.display()))?;
    let svg_logo = resolve_image(LocalImageSource::new(&svg_path))
        .await
        .with_context(|| format!("failed to resolve SVG image {}", svg_path.display()))?;

    let svg_content = render_svg_logo(&svg_logo, &font_name, panel_width)?;
    let summary = image_summary(&raster_logo, &svg_logo)?;

    let panel = Canvas::new()
        .fill_color(panel_color)
        .rect(48.0, 500.0, panel_width, panel_height)
        .fill()
        .finish();
    let title = TextBuilder::new()
        .font(&font_name, title_size)
        .set_color(title_color)
        .position(64.0, 770.0)
        .text("GraphitePDF Native Asset Flow")
        .finish();
    let body = TextBuilder::new()
        .font("F1", body_size)
        .set_color(Color::rgb(0.12, 0.16, 0.22))
        .position(64.0, 742.0)
        .leading(body_size + 4.0)
        .text("This example resolves a stylesheet, loads the selected native font,")
        .next_line(0.0, -(body_size + 4.0))
        .text("and inspects native image assets before generating a PDF.")
        .finish();
    let metrics = TextBuilder::new()
        .font("F1", 11.0)
        .set_color(Color::rgb(0.2, 0.25, 0.32))
        .position(64.0, 516.0)
        .leading(15.0)
        .text(&summary)
        .next_line(0.0, -15.0)
        .text("Outputs default to .artifacts/examples/kit unless GRAPHITEPDF_OUTPUT or GRAPHITEPDF_OUTPUT_DIR is set.")
        .finish();

    let content = [panel, title, body, svg_content, metrics].concat();
    let path = output_path("native")?;

    document
        .metadata(
            Metadata::new()
                .title("GraphitePDF Native Asset Example")
                .author("graphitepdf")
                .subject("kit example for font, image, and stylesheet usage")
                .keywords([
                    "graphitepdf",
                    "kit",
                    "font",
                    "image",
                    "stylesheet",
                    "example",
                ]),
        )
        .with_page(page_size, content)
        .save(&path)?;

    println!("Generated {}!", path.display());
    Ok(())
}

fn stylesheet() -> Stylesheet {
    Stylesheet::new(StyleValue::Array(vec![
        StyleValue::Object(style([
            ("width", 499.0.into()),
            ("height", 280.0.into()),
            ("backgroundColor", "#eef2ff".into()),
            ("color", "#312e81".into()),
            ("fontSize", 24.0.into()),
            ("lineHeight", 13.0.into()),
            ("fontSourceStandard", "Courier-Bold".into()),
        ])),
        StyleValue::Object(style([("lineHeight", 12.0.into())])),
    ]))
}

fn style<const N: usize>(entries: [(&str, StyleValue); N]) -> Style {
    entries
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect()
}

fn resolved_number(style: &Style, key: &str) -> Result<f64> {
    match style.get(key) {
        Some(StyleValue::Number(value)) => Ok(*value),
        Some(value) => bail!("expected `{key}` to resolve to a number, found {value:?}"),
        None => bail!("stylesheet is missing `{key}`"),
    }
}

fn resolved_string<'a>(style: &'a Style, key: &str) -> Result<&'a str> {
    match style.get(key) {
        Some(StyleValue::String(value)) => Ok(value.as_str()),
        Some(value) => bail!("expected `{key}` to resolve to a string, found {value:?}"),
        None => bail!("stylesheet is missing `{key}`"),
    }
}

fn color_from_hex(hex: &str) -> Result<Color> {
    let hex = hex.trim().trim_start_matches('#');
    if hex.len() != 6 {
        bail!("expected a 6-digit hex color, found `{hex}`");
    }

    let parse_channel = |range: std::ops::Range<usize>| -> Result<f64> {
        let channel = u8::from_str_radix(&hex[range], 16)
            .with_context(|| format!("invalid hex color channel in `{hex}`"))?;
        Ok(f64::from(channel) / 255.0)
    };

    Ok(Color::rgb(
        parse_channel(0..2)?,
        parse_channel(2..4)?,
        parse_channel(4..6)?,
    ))
}

fn standard_font_from_name(name: &str) -> Result<StandardFont> {
    match name.trim() {
        "Times-Roman" => Ok(StandardFont::TimesRoman),
        "Times-Bold" => Ok(StandardFont::TimesBold),
        "Times-Italic" => Ok(StandardFont::TimesItalic),
        "Times-BoldItalic" => Ok(StandardFont::TimesBoldItalic),
        "Helvetica" => Ok(StandardFont::Helvetica),
        "Helvetica-Bold" => Ok(StandardFont::HelveticaBold),
        "Helvetica-Oblique" => Ok(StandardFont::HelveticaOblique),
        "Helvetica-BoldOblique" => Ok(StandardFont::HelveticaBoldOblique),
        "Courier" => Ok(StandardFont::Courier),
        "Courier-Bold" => Ok(StandardFont::CourierBold),
        "Courier-Oblique" => Ok(StandardFont::CourierOblique),
        "Courier-BoldOblique" => Ok(StandardFont::CourierBoldOblique),
        "Symbol" => Ok(StandardFont::Symbol),
        "ZapfDingbats" => Ok(StandardFont::ZapfDingbats),
        other => Err(anyhow!("unsupported standard font `{other}`")),
    }
}

fn render_svg_logo(svg_logo: &Arc<Image>, font_name: &str, panel_width: f64) -> Result<Vec<u8>> {
    let Image::Svg(svg_logo) = svg_logo.as_ref() else {
        bail!("expected the local SVG logo to resolve to an SVG image");
    };

    Ok(svg_logo.data.to_pdf_page_content_with_options(
        &SvgRenderOptions::new()
            .position(96.0, 565.0)
            .width((panel_width - 96.0).max(160.0))
            .font_name(font_name)
            .font_size(12.0),
    )?)
}

fn image_summary(raster_logo: &Arc<Image>, svg_logo: &Arc<Image>) -> Result<String> {
    let raster = match raster_logo.as_ref() {
        Image::Raster(raster) => raster,
        _ => bail!("expected the local PNG logo to resolve to a raster image"),
    };
    let svg = match svg_logo.as_ref() {
        Image::Svg(svg) => svg,
        _ => bail!("expected the local SVG logo to resolve to an SVG image"),
    };

    Ok(format!(
        "Resolved raster logo as {}x{} {} and SVG logo as {:.0}x{:.0} {}.",
        raster.width,
        raster.height,
        raster.format.as_str(),
        svg.width,
        svg.height,
        "svg",
    ))
}
