#![cfg(feature = "template")]

use graphitepdf::font::FontWeight;
use graphitepdf::template::LayoutStyle;
use graphitepdf::{FontStyle, PageSize, Pt, pdf, renderer, styles, stylesheet};

#[test]
fn pdf_macro_builds_a_document_that_renders_to_pdf_bytes() {
    let title_style = LayoutStyle {
        font_size: Some(graphitepdf::Pt::new(24.0)),
        ..LayoutStyle::default()
    };

    let document = pdf! {
        <Document>
            <Page size="A4">
                <View>
                    <Text style={title_style.clone()}>"Hello GraphitePDF template"</Text>
                    <Text>{"Page size is "}{format!("{:.0}pt", PageSize::A4.width)}</Text>
                </View>
            </Page>
        </Document>
    };

    let bytes = renderer::render_to_bytes(&document).expect("template document should render");

    assert!(bytes.starts_with(b"%PDF-1.7"));
}

#[test]
fn pdf_macro_supports_expression_children_for_document_page_and_view() {
    let extra_view_children = Some(vec![graphitepdf::template::__private::text_node_from_str(
        "Expression child inside View",
    )]);
    let extra_page_nodes = Some(vec![graphitepdf::template::__private::LayoutNode::view(
        vec![graphitepdf::template::__private::text_node_from_str(
            "Expression child inside Page",
        )],
    )]);
    let extra_document_page = Some(
        graphitepdf::template::__private::LayoutPage::new(vec![
            graphitepdf::template::__private::text_node_from_str(
                "Expression child inside Document",
            ),
        ])
        .with_size(graphitepdf::template::__private::into_pdf_size(
            PageSize::A5,
        )),
    );

    let document = pdf! {
        <Document>
            <Page size="A4">
                {extra_page_nodes}
                <View>
                    {extra_view_children}
                    <Text>{"Static child inside View"}</Text>
                </View>
            </Page>
            {extra_document_page}
        </Document>
    };

    assert_eq!(document.pages().len(), 2);

    let bytes = renderer::render_to_bytes(&document).expect("template document should render");

    assert!(bytes.starts_with(b"%PDF-1.7"));
}

#[test]
fn styles_macro_coerces_supported_literals_and_integrates_with_pdf_macro() {
    let title_style = styles! {
        font_size: 24.0,
        line_height: 30.0,
        color: "#1E1E1C",
        background_color: "white",
        font_family: "Inter",
        font_style: italic,
        font_weight: bold,
        z_index: 3,
        page_break_before: false,
        page_break_after: true,
    };

    assert_eq!(title_style.font_size, Some(Pt::new(24.0)));
    assert_eq!(title_style.line_height, Some(Pt::new(30.0)));
    assert_eq!(
        title_style.color,
        Some(graphitepdf::Color::rgb(0x1E, 0x1E, 0x1C))
    );
    assert_eq!(
        title_style.background_color,
        Some(graphitepdf::Color::WHITE)
    );
    assert_eq!(title_style.font_family.as_deref(), Some("Inter"));
    assert_eq!(title_style.font_style, Some(FontStyle::Italic));
    assert_eq!(title_style.font_weight, Some(FontWeight::BOLD));
    assert_eq!(title_style.z_index, Some(3));
    assert_eq!(title_style.page_break_before, Some(false));
    assert_eq!(title_style.page_break_after, Some(true));

    let document = pdf! {
        <Document>
            <Page size={PageSize::A4}>
                <Text style={title_style.clone()}>"Styled title"</Text>
            </Page>
        </Document>
    };

    assert_eq!(document.pages()[0].style(), &LayoutStyle::default());

    let bytes =
        renderer::render_to_bytes(&document).expect("styled template document should render");

    assert!(bytes.starts_with(b"%PDF-1.7"));
}

#[test]
fn stylesheet_macro_returns_reusable_named_layout_styles() {
    let invoice = stylesheet! {
        .title => {
            font_size: 20.0,
            font_weight: bold,
            color: "#223355",
        },
        .body => {
            font_size: 12.0,
            line_height: 16.0,
            font_family: "Helvetica",
        },
        .callout => {
            background_color: "#F3F4F6",
            z_index: 4,
        },
    };

    assert_eq!(invoice.title.font_size, Some(Pt::new(20.0)));
    assert_eq!(invoice.title.font_weight, Some(FontWeight::BOLD));
    assert_eq!(invoice.body.line_height, Some(Pt::new(16.0)));
    assert_eq!(invoice.callout.z_index, Some(4));

    let document = pdf! {
        <Document>
            <Page size="A4">
                <View style={invoice.callout.clone()}>
                    <Text style={invoice.title.clone()}>"Reusable stylesheet heading"</Text>
                    <Text style={invoice.body.clone()}>"Body copy reuses a named style."</Text>
                </View>
            </Page>
        </Document>
    };

    let bytes = renderer::render_to_bytes(&document)
        .expect("stylesheet-based template document should render");

    assert!(bytes.starts_with(b"%PDF-1.7"));
}
