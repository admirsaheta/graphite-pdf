use crate::document::{ImageSource, PdfMetadata};
use crate::error::Result;
use crate::layout::{LayoutContent, LayoutDocument, LayoutNode, LayoutPage};
use crate::style::{FontDescriptor, Style};
use graphitepdf_image::{ImageAsset, ImageSource as AssetImageSource};
use graphitepdf_primitives::Bounds;

pub trait RenderBackend {
    fn begin_document(&mut self, _metadata: &PdfMetadata) -> Result<()> {
        Ok(())
    }

    fn begin_page(&mut self, _page: &LayoutPage) -> Result<()> {
        Ok(())
    }

    fn fill_rect(&mut self, _frame: Bounds) -> Result<()> {
        Ok(())
    }

    fn draw_text(&mut self, _frame: Bounds, _style: &Style, _text: &str) -> Result<()> {
        Ok(())
    }

    fn draw_text_with_font(
        &mut self,
        frame: Bounds,
        style: &Style,
        text: &str,
        font: Option<&FontDescriptor>,
    ) -> Result<()> {
        let _ = font;
        self.draw_text(frame, style, text)
    }

    fn draw_image(&mut self, _frame: Bounds, _source: &ImageSource) -> Result<()> {
        Ok(())
    }

    fn draw_image_with_asset(&mut self, frame: Bounds, image: &ImageAsset) -> Result<()> {
        let legacy_source = ImageSource::from(image.clone());
        self.draw_image(frame, &legacy_source)
    }

    fn draw_image_asset(&mut self, frame: Bounds, source: &AssetImageSource) -> Result<()> {
        let legacy_source = ImageSource::from(source.clone());
        self.draw_image(frame, &legacy_source)
    }

    fn end_page(&mut self, _page: &LayoutPage) -> Result<()> {
        Ok(())
    }

    fn end_document(&mut self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct NoopRenderBackend;

impl RenderBackend for NoopRenderBackend {}

pub struct Renderer<B: RenderBackend> {
    backend: B,
}

impl<B: RenderBackend> Renderer<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn render_document(
        &mut self,
        metadata: &PdfMetadata,
        layout: &LayoutDocument,
    ) -> Result<()> {
        let _ = &layout.metadata;

        self.backend.begin_document(metadata)?;

        for page in &layout.pages {
            self.backend.begin_page(page)?;
            for node in &page.nodes {
                self.render_node(node)?;
            }
            self.backend.end_page(page)?;
        }

        self.backend.end_document()
    }

    fn render_node(&mut self, node: &LayoutNode) -> Result<()> {
        if node.style.background_color.is_some() {
            self.backend.fill_rect(node.frame)?;
        }

        match &node.content {
            LayoutContent::View { children } => {
                for child in children {
                    self.render_node(child)?;
                }
            }
            LayoutContent::Text { text } => {
                let font = node.font_descriptor();
                self.backend
                    .draw_text_with_font(node.frame, &node.style, text, font.as_ref())?;
            }
            LayoutContent::Image { source } => {
                if let Some(asset) = source.as_asset() {
                    self.backend.draw_image_with_asset(node.frame, asset)?;
                } else {
                    let asset_source = source.as_asset_source();
                    self.backend.draw_image_asset(node.frame, &asset_source)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::LayoutPage;
    use graphitepdf_font::{FontStyle, FontWeight as FontVariantWeight};
    use graphitepdf_image::RemoteImageSource;
    use graphitepdf_primitives::{Point, Size};

    #[derive(Default)]
    struct RecordingBackend {
        text_calls: Vec<(String, Option<FontDescriptor>)>,
        image_calls: Vec<ImageSource>,
        image_asset_calls: Vec<ImageAsset>,
    }

    impl RenderBackend for RecordingBackend {
        fn draw_text_with_font(
            &mut self,
            _frame: Bounds,
            _style: &Style,
            text: &str,
            font: Option<&FontDescriptor>,
        ) -> Result<()> {
            self.text_calls.push((text.to_string(), font.cloned()));
            Ok(())
        }

        fn draw_image(&mut self, _frame: Bounds, source: &ImageSource) -> Result<()> {
            self.image_calls.push(source.clone());
            Ok(())
        }

        fn draw_image_with_asset(&mut self, _frame: Bounds, image: &ImageAsset) -> Result<()> {
            self.image_asset_calls.push(image.clone());
            Ok(())
        }
    }

    #[test]
    fn renderer_passes_font_descriptor_to_extended_backend_hook() {
        let node = LayoutNode {
            frame: Bounds::from_origin_size(0.0, 0.0, 100.0, 20.0),
            style: Style {
                font_family: Some(String::from("Inter")),
                font_style: Some(FontStyle::Italic),
                font_weight: Some(FontVariantWeight::BOLD),
                ..Style::default()
            },
            content: LayoutContent::Text {
                text: String::from("Hello"),
            },
        };
        let layout = LayoutDocument {
            metadata: PdfMetadata::default(),
            pages: vec![LayoutPage {
                size: Size::new(100.0, 100.0),
                nodes: vec![node],
            }],
        };
        let mut renderer = Renderer::new(RecordingBackend::default());

        renderer
            .render_document(&PdfMetadata::default(), &layout)
            .expect("render should succeed");

        let (text, font) = &renderer.backend().text_calls[0];
        assert_eq!(text, "Hello");
        assert_eq!(
            font.as_ref().map(|descriptor| descriptor.family()),
            Some("Inter")
        );
    }

    #[test]
    fn renderer_keeps_legacy_image_backend_behavior_via_default_adapter() {
        let remote = RemoteImageSource::new("https://example.com/image.png");
        let node = LayoutNode {
            frame: Bounds {
                origin: Point::new(0.0, 0.0),
                size: Size::new(50.0, 50.0),
            },
            style: Style::default(),
            content: LayoutContent::Image {
                source: ImageSource::from(remote.clone()),
            },
        };
        let layout = LayoutDocument {
            metadata: PdfMetadata::default(),
            pages: vec![LayoutPage {
                size: Size::new(50.0, 50.0),
                nodes: vec![node],
            }],
        };
        let mut renderer = Renderer::new(RecordingBackend::default());

        renderer
            .render_document(&PdfMetadata::default(), &layout)
            .expect("render should succeed");

        assert_eq!(
            renderer.backend().image_calls,
            vec![ImageSource::from(AssetImageSource::from(remote))]
        );
    }

    #[test]
    fn renderer_passes_concrete_image_asset_to_extended_backend_hook() {
        let asset = ImageAsset::Raster(graphitepdf_image::RasterImage {
            width: 2,
            height: 3,
            data: vec![9, 8, 7, 6],
            format: graphitepdf_image::ImageFormat::Png,
            key: Some(String::from("hero")),
        });
        let node = LayoutNode {
            frame: Bounds {
                origin: Point::new(0.0, 0.0),
                size: Size::new(50.0, 50.0),
            },
            style: Style::default(),
            content: LayoutContent::Image {
                source: ImageSource::from(asset.clone()),
            },
        };
        let layout = LayoutDocument {
            metadata: PdfMetadata::default(),
            pages: vec![LayoutPage {
                size: Size::new(50.0, 50.0),
                nodes: vec![node],
            }],
        };
        let mut renderer = Renderer::new(RecordingBackend::default());

        renderer
            .render_document(&PdfMetadata::default(), &layout)
            .expect("render should succeed");

        assert!(renderer.backend().image_calls.is_empty());
        assert_eq!(renderer.backend().image_asset_calls, vec![asset]);
    }
}
