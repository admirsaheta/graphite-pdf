//! # GraphitePDF Kit
//!
//! A pure Rust PDF generation library with no external PDF dependencies.
//!
//! ## Features
//! - Document construction with `DocumentBuilder`
//! - Page sizing (A0-A6, Letter, Legal, Tabloid, custom sizes)
//! - Text rendering with `TextBuilder`
//! - Vector graphics with `Canvas`
//! - Flate compression
//! - Metadata support
//! - Optional tracing support
//!
//! ## Quick Start
//! ```rust,no_run
//! use graphitepdf_kit::{DocumentBuilder, PageSize, TextBuilder, Canvas, Color, Metadata};
//!
//! // Create text content
//! let text = TextBuilder::new()
//!     .font("F1", 24.0)
//!     .position(100.0, 700.0)
//!     .text("Hello, World!")
//!     .finish();
//!
//! // Create vector graphics
//! let graphics = Canvas::new()
//!     .fill_color(Color::RED)
//!     .rect(100.0, 650.0, 200.0, 50.0)
//!     .fill()
//!     .finish();
//!
//! // Combine and build the document
//! let content = [text, graphics].concat();
//! let doc = DocumentBuilder::new()
//!     .metadata(Metadata::new()
//!         .title("My Document")
//!         .author("Me"))
//!     .with_page(PageSize::A4, content);
//!
//! // Save to file
//! doc.save("output.pdf").unwrap();
//! ```

mod compress;
mod document;
mod error;
mod font;
mod metadata;
mod objects;
mod outline;
mod page;
mod pattern;
mod security;
mod svg_render;
#[cfg(test)]
mod tests;
mod text;
mod vector;
mod writer;

#[cfg(feature = "tables")]
mod table;
#[cfg(feature = "tables")]
pub use table::{BorderStyle, TableBuilder, TableCell, TableRow};

pub use compress::flate_encode;
pub use document::{DocumentBuilder, Page};
pub use graphitepdf_errors as errors;
pub use graphitepdf_math as math;
pub use graphitepdf_svg as svg;
pub use error::{GraphitePdfKitError, Result};
pub use font::{Font, FontRegistry, StandardFont};
pub use metadata::Metadata;
pub use objects::Object;
pub use outline::{Outline, OutlineItem};
pub use page::{PageMargins, PageOrientation, PageSize};
pub use pattern::{GradientStop, LinearGradient, Pattern, RadialGradient, TilingPattern};
pub use security::{Permissions, SecurityOptions};
pub use self::svg_render::{
    SvgRenderOptions, ToPdfPageContent, render_math_to_page_content,
    render_math_to_page_content_with_options, render_svg_node_to_page_content,
    render_svg_node_to_page_content_with_options,
};
pub use text::{FontWeight, TextAlignment, TextBuilder, TextRenderingMode};
pub use vector::{Canvas, Color, LineCap, LineJoin};
pub use writer::PdfWriter;
