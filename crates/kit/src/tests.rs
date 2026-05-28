#[cfg(test)]
mod tests {
    use crate::{
        Canvas, Color, DocumentBuilder, Metadata, Object, PageSize, TextBuilder,
    };

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
        let canvas = Canvas::new()
            .rect(0.0, 0.0, 100.0, 100.0)
            .fill()
            .finish();
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
        assert!(buf.len() > 0);
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
