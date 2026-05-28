pub mod error {
    pub use graphitepdf_render::error::*;
}

pub use graphitepdf_render::*;

#[cfg(test)]
mod tests {
    use super::*;

    use graphitepdf_font::{FontDescriptor, FontSource, StandardFont};
    use graphitepdf_layout::LayoutMetadata;
    use graphitepdf_layout::{Document, LayoutStyle, Node, Page};
    use graphitepdf_primitives::{Bounds, Color, Pt, Size};
    use std::io::Cursor;

    fn simple_render_document(label: &str) -> RenderDocument {
        RenderDocument {
            metadata: LayoutMetadata {
                title: Some(String::from(label)),
                ..LayoutMetadata::default()
            },
            pages: vec![RenderPage {
                size: Size::new(240.0, 180.0),
                source_page_index: 0,
                commands: vec![
                    RenderCommand::FillRect(FillRectOp {
                        context: graphitepdf_render::RenderContext {
                            page_index: 0,
                            source_page_index: 0,
                            path: vec![0],
                            node_kind: graphitepdf_render::RenderNodeKind::Box,
                            z_index: 0,
                            frame: Bounds::from_origin_size(12.0, 18.0, 120.0, 48.0),
                            content_frame: Bounds::from_origin_size(12.0, 18.0, 120.0, 48.0),
                        },
                        bounds: Bounds::from_origin_size(12.0, 18.0, 120.0, 48.0),
                        color: Color::rgb(0x22, 0x66, 0xaa),
                        role: graphitepdf_render::PaintRole::Background,
                    }),
                    RenderCommand::DrawText(TextRenderOp {
                        context: graphitepdf_render::RenderContext {
                            page_index: 0,
                            source_page_index: 0,
                            path: vec![1],
                            node_kind: graphitepdf_render::RenderNodeKind::Text,
                            z_index: 1,
                            frame: Bounds::from_origin_size(18.0, 26.0, 180.0, 24.0),
                            content_frame: Bounds::from_origin_size(18.0, 26.0, 180.0, 24.0),
                        },
                        text: label.to_string(),
                        color: Color::BLACK,
                        font: Some(FontDescriptor::new("Helvetica")),
                        font_source: Some(FontSource::standard(StandardFont::Helvetica)),
                        font_size: Pt::new(12.0),
                        line_height: Some(Pt::new(14.0)),
                        block: None,
                        layout: None,
                    }),
                ],
            }],
        }
    }

    #[test]
    fn renders_render_documents_to_pdf_bytes() {
        let document = simple_render_document("Hello renderer");
        let bytes = render_to_bytes(&document).expect("render document should serialize");

        assert!(bytes.starts_with(b"%PDF-1.7"));
        assert!(
            bytes
                .windows(b"/Type /Page".len())
                .any(|window| window == b"/Type /Page")
        );
        assert!(
            bytes
                .windows(b"%%EOF".len())
                .any(|window| window == b"%%EOF")
        );
    }

    #[test]
    fn session_tracks_revisions_and_refreshes_after_updates() {
        let document = Document::new().with_page(
            Page::new([Node::box_node().with_style(
                LayoutStyle::new().with_background_color(Color::rgb(0xdd, 0xee, 0xff)),
            )])
            .with_size(Size::new(200.0, 120.0)),
        );
        let mut session = RendererSession::new(document);

        let first_revision = session
            .render_snapshot()
            .expect("initial render should succeed")
            .revision();
        let first_bytes = session
            .to_bytes()
            .expect("session should produce pdf bytes");

        assert_eq!(first_revision, 0);
        assert_eq!(
            session
                .render_snapshot()
                .expect("cached render should still be available")
                .revision(),
            0
        );

        let updated_revision = session.update_document(|document| {
            document.add_page(Page::new([Node::box_node()]).with_size(Size::new(200.0, 120.0)));
        });
        let (updated_snapshot_revision, updated_page_count) = {
            let updated_snapshot = session
                .render_snapshot()
                .expect("updated render should succeed");
            (
                updated_snapshot.revision(),
                updated_snapshot.document().pages.len(),
            )
        };
        let updated_bytes = session
            .to_bytes()
            .expect("updated session should serialize");

        assert_eq!(updated_revision, 1);
        assert_eq!(updated_snapshot_revision, 1);
        assert_eq!(updated_page_count, 2);
        assert_ne!(first_bytes, updated_bytes);
    }

    #[test]
    fn writer_and_file_helpers_emit_pdf_output() {
        let document = simple_render_document("Writer helper");
        let mut buffer = Cursor::new(Vec::new());

        render_to_writer(&document, &mut buffer).expect("writer helper should succeed");
        assert!(buffer.get_ref().starts_with(b"%PDF-1.7"));

        let path = std::env::temp_dir().join(format!(
            "graphitepdf-renderer-test-{}-{}.pdf",
            std::process::id(),
            std::thread::current().name().unwrap_or("unnamed")
        ));
        render_to_file(&document, &path).expect("file helper should succeed");
        let saved = std::fs::read(&path).expect("saved PDF should be readable");
        let _ = std::fs::remove_file(&path);

        assert!(saved.starts_with(b"%PDF-1.7"));
    }
}
