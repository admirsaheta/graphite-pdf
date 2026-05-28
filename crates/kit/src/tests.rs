#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::{Canvas, Color, DocumentBuilder, Metadata, Object, PageSize, TextBuilder};

    #[test]
    fn test_document_builder() {
        let _doc = DocumentBuilder::new();
    }

    #[test]
    fn test_metadata_builder() {
        let meta = Metadata::new()
            .title("Test")
            .author("Author")
            .subject("Subject")
            .keywords(["rust", "pdf"]);
        assert_eq!(meta.title, Some("Test".to_string()));
        assert_eq!(meta.author, Some("Author".to_string()));
    }

    #[test]
    fn test_text_builder() {
        let text = TextBuilder::new()
            .font("F1", 12.0)
            .position(100.0, 700.0)
            .text("Hello")
            .finish();
        assert!(!text.is_empty());
    }

    #[test]
    fn test_canvas_builder() {
        let canvas = Canvas::new().rect(0.0, 0.0, 100.0, 100.0).fill().finish();
        assert!(!canvas.is_empty());
    }

    #[test]
    fn test_write_empty_document() {
        let mut buf = Vec::new();
        let doc = DocumentBuilder::new();
        doc.write(&mut buf).unwrap();
        assert!(buf.starts_with(b"%PDF-1.7\n"));
        assert!(buf.ends_with(b"%%EOF\n"));
    }

    #[test]
    fn test_write_simple_document() {
        let text = TextBuilder::new()
            .font("F1", 24.0)
            .position(100.0, 700.0)
            .text("Hello, World!")
            .finish();
        let doc = DocumentBuilder::new().with_page(PageSize::A4, text);
        let mut buf = Vec::new();
        doc.write(&mut buf).unwrap();
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_custom_font_name_stays_stable_when_writing() {
        let (doc, font_name) = DocumentBuilder::new().add_font(crate::Font::standard(
            crate::font::StandardFont::CourierBold,
        ));
        let text = TextBuilder::new()
            .font(&font_name, 24.0)
            .position(100.0, 700.0)
            .text("Hello, Courier!")
            .finish();

        let mut buf = Vec::new();
        doc.with_page(PageSize::A4, text).write(&mut buf).unwrap();

        let pdf = String::from_utf8_lossy(&buf);
        assert_eq!(font_name, "F2");
        assert!(pdf.contains(&format!("/{font_name} ")));
        assert!(pdf.contains("/BaseFont /Courier-Bold"));
    }

    #[test]
    fn test_font_registry_accepts_concrete_graphitepdf_font_types() {
        let mut registry = crate::FontRegistry::with_default_font();
        let font_name = registry.register(graphitepdf_font::StandardFont::CourierBold);

        assert_eq!(font_name, "F2");
        assert!(matches!(
            registry.get(&font_name),
            Some(font) if font.standard_font() == Some(graphitepdf_font::StandardFont::CourierBold)
        ));
    }

    #[test]
    fn test_document_builder_accepts_concrete_graphitepdf_font_types() {
        let (doc, font_name) =
            DocumentBuilder::new().add_font(graphitepdf_font::StandardFont::CourierBold);
        let text = TextBuilder::new()
            .font(&font_name, 24.0)
            .position(100.0, 700.0)
            .text("Hello from shared font")
            .finish();

        let mut buf = Vec::new();
        doc.with_page(PageSize::A4, text).write(&mut buf).unwrap();

        let pdf = String::from_utf8_lossy(&buf);
        assert_eq!(font_name, "F2");
        assert!(pdf.contains("/BaseFont /Courier-Bold"));
    }

    #[test]
    fn test_shared_font_and_image_modules_are_re_exported() {
        let font_source = crate::font::FontSource::local("/tmp/example.ttf");
        let image_source = crate::image::ImageSource::from(crate::image::LocalImageSource::new(
            "/tmp/example.png",
        ));

        assert!(matches!(
            font_source,
            crate::font::FontSource::Local(path) if path.as_path() == std::path::Path::new("/tmp/example.ttf")
        ));
        assert!(matches!(
            image_source,
            crate::image::ImageSource::Local(source)
                if source.path.as_path() == std::path::Path::new("/tmp/example.png")
        ));
    }

    #[test]
    fn test_page_sizes() {
        let a4 = PageSize::A4;
        // Now Page sizes are now in POINTS (not mm!
        assert_eq!(a4.width, 595.276);
        assert_eq!(a4.height, 841.89);
        let letter = PageSize::LETTER;
        assert_eq!(letter.width, 612.0);
        assert_eq!(letter.height, 792.0);
    }

    #[test]
    fn test_color_conversions() {
        let red = Color::RED;
        assert_eq!(red.r, 1.0);
        assert_eq!(red.g, 0.0);
        assert_eq!(red.b, 0.0);
        let from_tuple: Color = (1.0, 0.0, 1.0).into();
        assert_eq!(from_tuple.r, 1.0);
        assert_eq!(from_tuple.b, 1.0);
    }

    #[test]
    fn test_object_methods() {
        let mut dict = Object::dict([("Type", "Catalog")]);
        dict.insert("Version", "1.7").unwrap();
        let value = dict.get("Type");
        assert!(value.is_some());
    }
}
