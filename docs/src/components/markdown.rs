use pulldown_cmark::{Options, Parser, html as cmark_html};
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MarkdownProps {
    pub content: String,
}

#[component]
pub fn Markdown(props: &MarkdownProps) -> Html {
    let container = use_node_ref();

    {
        let container = container.clone();
        let content = props.content.clone();
        use_effect_with(content, move |md| {
            if let Some(el) = container.cast::<Element>() {
                let html_str = render_markdown(md);
                el.set_inner_html(&html_str);
                add_heading_ids(&el);
                highlight_code();
            }
        });
    }

    html! {
        <div ref={container} class="doc-content" />
    }
}

fn render_markdown(md: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(md, opts);
    let mut out = String::with_capacity(md.len() * 2);
    cmark_html::push_html(&mut out, parser);
    out
}

fn add_heading_ids(container: &Element) {
    let selector = "h1, h2, h3, h4";
    if let Ok(list) = container.query_selector_all(selector) {
        for i in 0..list.length() {
            if let Some(node) = list.item(i)
                && let Ok(el) = node.dyn_into::<Element>()
            {
                let text = el.text_content().unwrap_or_default();
                let id = slugify(&text);
                let _ = el.set_attribute("id", &id);
            }
        }
    }
}

fn highlight_code() {
    let _ = js_sys::eval("if(window.hljs){window.hljs.highlightAll();}");
}

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
