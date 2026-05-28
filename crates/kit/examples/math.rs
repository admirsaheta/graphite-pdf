mod support;

use anyhow::Result;
use graphitepdf_kit::math::{MathOptions, render_math_with_options};
use graphitepdf_kit::{
    DocumentBuilder, Metadata, PageSize, SvgRenderOptions, TextBuilder, ToPdfPageContent,
};
use support::output_path;

fn main() -> Result<()> {
    let display_formula = render_math_with_options(
        r"\int_0^1 x^3 \, dx = \frac{1}{4}",
        &MathOptions::new().height(42.0).color("rebeccapurple"),
    )?;
    let inline_formula = render_math_with_options(
        r"e^{i\pi} + 1 = 0",
        &MathOptions::new()
            .inline(true)
            .height(24.0)
            .color("#0f766e"),
    )?;

    let title = TextBuilder::new()
        .font("F1", 20.0)
        .position(48.0, 788.0)
        .text("GraphitePDF Math Example")
        .finish();
    let body = TextBuilder::new()
        .font("F1", 12.0)
        .position(48.0, 748.0)
        .text("MathJax-backed formulas can be rendered to SVG and placed into PDF page content.")
        .text("Inline math is rendered separately below:")
        .finish();

    let display_content = display_formula.to_pdf_page_content_with_options(
        &SvgRenderOptions::new()
            .position(56.0, 620.0)
            .font_name("F1"),
    )?;
    let inline_label = TextBuilder::new()
        .font("F1", 12.0)
        .position(48.0, 560.0)
        .text("Inline identity:")
        .finish();
    let inline_content = inline_formula.to_pdf_page_content_with_options(
        &SvgRenderOptions::new()
            .position(132.0, 544.0)
            .font_name("F1"),
    )?;

    let content = [title, body, display_content, inline_label, inline_content].concat();
    let path = output_path("math")?;

    DocumentBuilder::new()
        .metadata(
            Metadata::new()
                .title("GraphitePDF Math Example")
                .author("graphitepdf")
                .subject("Math to PDF page content")
                .keywords(["graphitepdf", "math", "latex", "pdf", "example"]),
        )
        .with_page(PageSize::A4, content)
        .save(&path)?;

    println!("Generated {}!", path.display());
    Ok(())
}
