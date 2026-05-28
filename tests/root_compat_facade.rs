use graphitepdf::{
    Document, Node, NodeKind, PdfMetadata, Style, StyleValue, Stylesheet, StylesheetContainer,
    TextNode,
};
use graphitepdf::{layout, render, renderer};
use graphitepdf::{document, style};

#[test]
fn root_layout_facade_accepts_compat_documents() {
    let container = StylesheetContainer::new(240.0, 180.0);
    let stylesheet = Stylesheet::new(StyleValue::Object(
        [
            ("fontFamily".to_string(), "Inter".into()),
            ("fontStyle".to_string(), "italic".into()),
            ("fontWeight".to_string(), 700.into()),
        ]
        .into_iter()
        .collect(),
    ));
    let document = Document::new()
        .set_metadata(PdfMetadata {
            title: Some(String::from("Compat facade")),
            ..PdfMetadata::default()
        })
        .add_page(Node::new(
            NodeKind::View {
                children: vec![Node::from_stylesheet(
                    NodeKind::Text(TextNode::new("Hello facade")),
                    &container,
                    &stylesheet,
                )],
            },
            Style::default(),
        ));

    let layout = layout::LayoutEngine::new()
        .layout_document(&document)
        .expect("compat document should layout through the root facade");

    assert_eq!(layout.metadata.title.as_deref(), Some("Compat facade"));
    assert_eq!(layout.pages().len(), 1);
    assert_eq!(layout.pages()[0].nodes().len(), 1);
    assert_eq!(
        layout.pages()[0].nodes()[0]
            .font_descriptor()
            .map(|descriptor| descriptor.family()),
        Some("Inter")
    );

}

#[test]
fn root_render_and_renderer_facades_work_with_compat_documents() {
    let document = Document::new()
        .set_metadata(PdfMetadata {
            title: Some(String::from("Render facade")),
            ..PdfMetadata::default()
        })
        .add_page(Node::new(
            NodeKind::View {
                children: vec![Node::new(
                    NodeKind::Text(TextNode::new("Render me")),
                    Style {
                        background_color: Some(graphitepdf::Color::rgb(0xee, 0xf2, 0xff)),
                        ..Style::default()
                    },
                )],
            },
            Style::default(),
        ));

    let layout = layout::LayoutEngine::new()
        .layout_document(&document)
        .expect("compat document should layout through the root facade");
    let rendered = render::RenderEngine::new()
        .build(&layout)
        .expect("split render engine should accept the root layout facade output");
    let bytes =
        renderer::render_to_bytes(&document).expect("renderer facade should accept root documents");

    assert_eq!(rendered.pages.len(), 1);
    assert!(rendered.pages[0]
        .commands
        .iter()
        .any(|command| matches!(command, render::RenderCommand::DrawText(_))));
    assert!(bytes.starts_with(b"%PDF-1.7"));

}

#[test]
fn root_compat_document_round_trips_into_split_layout_types() {
    let document = Document::new().add_page(Node::new(
        NodeKind::View {
            children: vec![Node::new(
                NodeKind::Text(TextNode::new("Round trip")),
                Style::default(),
            )],
        },
        Style::default(),
    ));

    let core_document = layout::Document::from(&document);

    assert_eq!(core_document.pages().len(), 1);
    assert_eq!(core_document.pages()[0].nodes().len(), 1);
}

#[test]
fn root_document_module_keeps_legacy_image_height_adapter() {
    let node = document::Node::new(
        document::NodeKind::Image(document::ImageNode::new(
            graphitepdf::RemoteImageSource::new("https://example.com/image.png"),
        )),
        style::Style::default(),
    );

    let layout_node = layout::Node::from(&node);

    assert_eq!(layout_node.style().height, Some(graphitepdf::Pt::new(120.0)));
}

#[test]
fn root_style_module_keeps_compat_style_conversions() {
    let stylesheet = style::Stylesheet::new(style::StyleValue::Object(
        [
            ("fontFamily".to_string(), "Inter".into()),
            ("fontWeight".to_string(), 700.into()),
            ("justifyContent".to_string(), "center".into()),
        ]
        .into_iter()
        .collect(),
    ));
    let compat_style =
        style::Style::from_stylesheet(&style::StylesheetContainer::new(120.0, 80.0), &stylesheet);

    let descriptor = compat_style
        .font_descriptor()
        .expect("compat style should still expose a font descriptor");
    let layout_style = graphitepdf_layout::LayoutStyle::from(&compat_style);
    let round_tripped = style::Style::from(layout_style.clone());

    assert_eq!(descriptor.family(), "Inter");
    assert_eq!(descriptor.font_weight(), graphitepdf::FontVariantWeight::BOLD);
    assert_eq!(compat_style.justify_content, style::JustifyContent::Center);
    assert_eq!(layout_style.font_family.as_deref(), Some("Inter"));
    assert_eq!(round_tripped.font_family.as_deref(), Some("Inter"));
}
