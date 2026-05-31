use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Error, Expr, ExprLit, ExprPath, Lit, LitStr, Result, Token, braced, parse_macro_input};

#[proc_macro]
pub fn pdf(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as PdfRoot);

    match Generator::new().generate_document(&root.root) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn styles(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StylesInput);

    match generate_styles(&input.entries) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn stylesheet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TemplateStylesheetInput);

    match generate_template_stylesheet(&input.entries) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

struct PdfRoot {
    root: ElementNode,
}

struct StylesInput {
    entries: Punctuated<StyleEntry, Token![,]>,
}

struct TemplateStylesheetInput {
    entries: Punctuated<TemplateStylesheetEntry, Token![,]>,
}

struct StyleEntry {
    key: Ident,
    value: Expr,
}

struct TemplateStylesheetEntry {
    name: Ident,
    styles: StylesInput,
}

impl Parse for PdfRoot {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let root = Parser::parse_element(input)?;
        if !input.is_empty() {
            return Err(input.error("unexpected trailing tokens after pdf! root"));
        }

        Ok(Self { root })
    }
}

impl Parse for StylesInput {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            entries: Punctuated::parse_terminated(input)?,
        })
    }
}

impl Parse for TemplateStylesheetInput {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            entries: Punctuated::parse_terminated(input)?,
        })
    }
}

impl Parse for StyleEntry {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let key = input.parse()?;
        input.parse::<Token![:]>()?;
        let value = input.parse()?;

        Ok(Self { key, value })
    }
}

impl Parse for TemplateStylesheetEntry {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        input.parse::<Token![.]>()?;
        let name = input.parse()?;
        input.parse::<Token![=>]>()?;

        let content;
        braced!(content in input);
        let styles = StylesInput::parse(&content)?;
        if !content.is_empty() {
            return Err(content.error("unexpected trailing tokens in stylesheet! entry"));
        }

        Ok(Self { name, styles })
    }
}

#[derive(Clone)]
enum PropValue {
    Bool,
    Expr(Expr),
    String(LitStr),
}

#[derive(Clone)]
struct Prop {
    name: Ident,
    value: PropValue,
}

#[derive(Clone)]
enum PdfAstNode {
    Element(ElementNode),
    Text(LitStr),
    Expression(Expr),
}

#[derive(Clone)]
struct ElementNode {
    name: Ident,
    props: Vec<Prop>,
    children: Vec<PdfAstNode>,
    self_closing: bool,
}

struct Parser;

impl Parser {
    fn parse_element(input: ParseStream<'_>) -> Result<ElementNode> {
        input.parse::<Token![<]>()?;
        let name: Ident = input.parse()?;
        let props = Self::parse_props(input)?;

        if input.peek(Token![/]) {
            input.parse::<Token![/]>()?;
            input.parse::<Token![>]>()?;
            return Ok(ElementNode {
                name,
                props,
                children: Vec::new(),
                self_closing: true,
            });
        }

        input.parse::<Token![>]>()?;

        let mut children = Vec::new();
        while !Self::starts_closing_tag(input) {
            if input.is_empty() {
                return Err(Error::new(
                    name.span(),
                    format!("missing closing tag for <{}>", name),
                ));
            }

            if input.peek(Token![<]) {
                children.push(PdfAstNode::Element(Self::parse_element(input)?));
            } else if input.peek(LitStr) {
                children.push(PdfAstNode::Text(input.parse()?));
            } else if input.peek(syn::token::Brace) {
                children.push(PdfAstNode::Expression(Self::parse_braced_expr(input)?));
            } else {
                return Err(input.error(
                    "unsupported token inside pdf! element; expected a child element, string literal, or { expression }",
                ));
            }
        }

        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        let closing: Ident = input.parse()?;
        if closing != name {
            return Err(Error::new(
                closing.span(),
                format!(
                    "mismatched closing tag: expected </{}> but found </{}>",
                    name, closing
                ),
            ));
        }
        input.parse::<Token![>]>()?;

        Ok(ElementNode {
            name,
            props,
            children,
            self_closing: false,
        })
    }

    fn parse_props(input: ParseStream<'_>) -> Result<Vec<Prop>> {
        let mut props = Vec::new();

        while !(input.peek(Token![>]) || (input.peek(Token![/]) && input.peek2(Token![>]))) {
            let name: Ident = input.parse()?;
            let value = if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;
                if input.peek(syn::token::Brace) {
                    PropValue::Expr(Self::parse_braced_expr(input)?)
                } else if input.peek(LitStr) {
                    PropValue::String(input.parse()?)
                } else {
                    return Err(
                        input.error("expected a string literal or { expression } for prop value")
                    );
                }
            } else {
                PropValue::Bool
            };

            props.push(Prop { name, value });
        }

        Ok(props)
    }

    fn starts_closing_tag(input: ParseStream<'_>) -> bool {
        let fork = input.fork();
        fork.parse::<Token![<]>().is_ok() && fork.parse::<Token![/]>().is_ok()
    }

    fn parse_braced_expr(input: ParseStream<'_>) -> Result<Expr> {
        let content;
        braced!(content in input);
        let expr: Expr = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("unexpected trailing tokens inside { expression }"));
        }
        Ok(expr)
    }
}

fn generate_styles(entries: &Punctuated<StyleEntry, Token![,]>) -> Result<TokenStream2> {
    for (index, entry) in entries.iter().enumerate() {
        if entries
            .iter()
            .skip(index + 1)
            .any(|candidate| candidate.key == entry.key)
        {
            return Err(Error::new(
                entry.key.span(),
                format!("duplicate style key `{}` in styles! macro", entry.key),
            ));
        }
    }

    let setters = entries
        .iter()
        .map(generate_style_entry)
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {{
        let mut __graphitepdf_style = ::graphitepdf::template::LayoutStyle::default();
        #(#setters)*
        __graphitepdf_style
    }})
}

fn generate_template_stylesheet(
    entries: &Punctuated<TemplateStylesheetEntry, Token![,]>,
) -> Result<TokenStream2> {
    for (index, entry) in entries.iter().enumerate() {
        if entries
            .iter()
            .skip(index + 1)
            .any(|candidate| candidate.name == entry.name)
        {
            return Err(Error::new(
                entry.name.span(),
                format!(
                    "duplicate stylesheet key `{}` in stylesheet! macro",
                    entry.name
                ),
            ));
        }
    }

    let type_name = format_ident!("__GraphitepdfTemplateStylesheet");
    let fields = entries.iter().map(|entry| {
        let name = &entry.name;
        quote! { pub #name: ::graphitepdf::template::LayoutStyle }
    });
    let values = entries
        .iter()
        .map(|entry| {
            let name = &entry.name;
            let styles = generate_styles(&entry.styles.entries)?;
            Ok(quote! { #name: #styles })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {{
        #[derive(Clone, Debug)]
        struct #type_name {
            #(#fields,)*
        }

        #type_name {
            #(#values,)*
        }
    }})
}

fn generate_style_entry(entry: &StyleEntry) -> Result<TokenStream2> {
    let key = entry.key.to_string();
    let value = &entry.value;

    let setter = match key.as_str() {
        "width" => pt_style_assignment("with_width", value)?,
        "height" => pt_style_assignment("with_height", value)?,
        "font_size" => pt_style_assignment("with_font_size", value)?,
        "line_height" => pt_style_assignment("with_line_height", value)?,
        "color" => color_style_assignment("with_color", value)?,
        "background_color" => color_style_assignment("with_background_color", value)?,
        "font_family" => string_style_assignment("with_font_family", value)?,
        "font_style" => font_style_assignment(value)?,
        "font_weight" => font_weight_assignment(value)?,
        "z_index" => i32_style_assignment("with_z_index", value)?,
        "page_break_before" => bool_style_assignment("with_page_break_before", value)?,
        "page_break_after" => bool_style_assignment("with_page_break_after", value)?,
        _ => {
            return Err(Error::new(
                entry.key.span(),
                format!("unsupported style key `{key}` in styles! macro"),
            ));
        }
    };

    Ok(quote! {
        __graphitepdf_style = __graphitepdf_style.#setter;
    })
}

fn pt_style_assignment(method: &str, value: &Expr) -> Result<TokenStream2> {
    let method = Ident::new(method, Span::call_site());

    match numeric_literal_as_f32(value)? {
        Some(number) => Ok(quote! { #method(::graphitepdf::Pt::new(#number)) }),
        None => Ok(quote! { #method(#value) }),
    }
}

fn i32_style_assignment(method: &str, value: &Expr) -> Result<TokenStream2> {
    let method = Ident::new(method, Span::call_site());

    match integer_literal_tokens(value)? {
        Some(number) => Ok(quote! { #method(#number) }),
        None => Ok(quote! { #method(#value) }),
    }
}

fn bool_style_assignment(method: &str, value: &Expr) -> Result<TokenStream2> {
    let method = Ident::new(method, Span::call_site());

    if let Some(boolean) = bool_literal(value) {
        Ok(quote! { #method(#boolean) })
    } else {
        Ok(quote! { #method(#value) })
    }
}

fn string_style_assignment(method: &str, value: &Expr) -> Result<TokenStream2> {
    let method = Ident::new(method, Span::call_site());

    if let Some(text) = string_literal(value) {
        Ok(quote! { #method(#text) })
    } else {
        Ok(quote! { #method(#value) })
    }
}

fn color_style_assignment(method: &str, value: &Expr) -> Result<TokenStream2> {
    let method = Ident::new(method, Span::call_site());

    let color = if let Some(text) = string_literal(value) {
        color_tokens_from_str(text)?
    } else {
        quote! { #value }
    };

    Ok(quote! { #method(#color) })
}

fn font_style_assignment(value: &Expr) -> Result<TokenStream2> {
    let font_style = if let Some(ident) = single_path_ident(value) {
        match ident.to_string().as_str() {
            "normal" => quote!(::graphitepdf::FontStyle::Normal),
            "italic" => quote!(::graphitepdf::FontStyle::Italic),
            "oblique" => quote!(::graphitepdf::FontStyle::Oblique),
            _ => {
                return Err(Error::new(
                    ident.span(),
                    "unsupported font_style value; use normal, italic, oblique, or a FontStyle expression",
                ));
            }
        }
    } else {
        quote! { #value }
    };

    Ok(quote! { with_font_style(#font_style) })
}

fn font_weight_assignment(value: &Expr) -> Result<TokenStream2> {
    let font_weight = if let Some(ident) = single_path_ident(value) {
        match ident.to_string().as_str() {
            "thin" => quote!(::graphitepdf::font::FontWeight::THIN),
            "extra_light" => quote!(::graphitepdf::font::FontWeight::EXTRA_LIGHT),
            "light" => quote!(::graphitepdf::font::FontWeight::LIGHT),
            "normal" => quote!(::graphitepdf::font::FontWeight::NORMAL),
            "medium" => quote!(::graphitepdf::font::FontWeight::MEDIUM),
            "semi_bold" => quote!(::graphitepdf::font::FontWeight::SEMI_BOLD),
            "bold" => quote!(::graphitepdf::font::FontWeight::BOLD),
            "extra_bold" => quote!(::graphitepdf::font::FontWeight::EXTRA_BOLD),
            "black" => quote!(::graphitepdf::font::FontWeight::BLACK),
            _ => {
                return Err(Error::new(
                    ident.span(),
                    "unsupported font_weight value; use named weights like bold or a FontWeight expression",
                ));
            }
        }
    } else if let Some(number) = integer_literal_tokens(value)? {
        quote!(::graphitepdf::font::FontWeight::new(#number).expect("styles! validated font_weight range"))
    } else {
        quote! { #value }
    };

    Ok(quote! { with_font_weight(#font_weight) })
}

fn color_tokens_from_str(value: &LitStr) -> Result<TokenStream2> {
    let text = value.value();
    let normalized = text.trim();

    let tokens = match normalized.to_ascii_lowercase().as_str() {
        "black" => quote!(::graphitepdf::Color::BLACK),
        "white" => quote!(::graphitepdf::Color::WHITE),
        _ => {
            let hex = normalized.strip_prefix('#').ok_or_else(|| {
                Error::new(
                    value.span(),
                    "unsupported color literal; use black, white, #RRGGBB, #RRGGBBAA, or a Color expression",
                )
            })?;

            match hex.len() {
                6 => {
                    let red = parse_hex_byte(value, &hex[0..2])?;
                    let green = parse_hex_byte(value, &hex[2..4])?;
                    let blue = parse_hex_byte(value, &hex[4..6])?;
                    quote!(::graphitepdf::Color::rgb(#red, #green, #blue))
                }
                8 => {
                    let red = parse_hex_byte(value, &hex[0..2])?;
                    let green = parse_hex_byte(value, &hex[2..4])?;
                    let blue = parse_hex_byte(value, &hex[4..6])?;
                    let alpha = parse_hex_byte(value, &hex[6..8])?;
                    quote!(::graphitepdf::Color::rgba(#red, #green, #blue, #alpha))
                }
                _ => {
                    return Err(Error::new(
                        value.span(),
                        "unsupported color literal; use #RRGGBB or #RRGGBBAA",
                    ));
                }
            }
        }
    };

    Ok(tokens)
}

fn parse_hex_byte(source: &LitStr, value: &str) -> Result<u8> {
    u8::from_str_radix(value, 16)
        .map_err(|_| Error::new(source.span(), "invalid hex color literal in styles! macro"))
}

fn string_literal(value: &Expr) -> Option<&LitStr> {
    match value {
        Expr::Lit(ExprLit {
            lit: Lit::Str(text),
            ..
        }) => Some(text),
        _ => None,
    }
}

fn bool_literal(value: &Expr) -> Option<bool> {
    match value {
        Expr::Lit(ExprLit {
            lit: Lit::Bool(boolean),
            ..
        }) => Some(boolean.value),
        _ => None,
    }
}

fn integer_literal_tokens(value: &Expr) -> Result<Option<TokenStream2>> {
    match value {
        Expr::Lit(ExprLit {
            lit: Lit::Int(number),
            ..
        }) => Ok(Some(quote! { #number })),
        Expr::Lit(ExprLit {
            lit: Lit::Float(_), ..
        }) => Err(Error::new(
            value.span(),
            "expected an integer literal for this style value",
        )),
        _ => Ok(None),
    }
}

fn numeric_literal_as_f32(value: &Expr) -> Result<Option<TokenStream2>> {
    match value {
        Expr::Lit(ExprLit {
            lit: Lit::Int(number),
            ..
        }) => Ok(Some(quote! { #number as f32 })),
        Expr::Lit(ExprLit {
            lit: Lit::Float(number),
            ..
        }) => Ok(Some(quote! { #number as f32 })),
        Expr::Lit(ExprLit {
            lit: Lit::Str(_), ..
        }) => Err(Error::new(
            value.span(),
            "expected a numeric literal or typed expression for this style value",
        )),
        _ => Ok(None),
    }
}

fn single_path_ident(value: &Expr) -> Option<&Ident> {
    match value {
        Expr::Path(ExprPath {
            attrs: _,
            qself: None,
            path,
        }) if path.segments.len() == 1 => Some(&path.segments[0].ident),
        _ => None,
    }
}

struct Generator;

impl Generator {
    fn new() -> Self {
        Self
    }

    fn generate_document(&self, root: &ElementNode) -> Result<TokenStream2> {
        if root.name != "Document" {
            return Err(Error::new(
                root.name.span(),
                "pdf! root must be a <Document> element in the current prototype",
            ));
        }

        self.validate_props(root, &["metadata"])?;

        let child_pages = root
            .children
            .iter()
            .map(|child| self.generate_document_child(child))
            .collect::<Result<Vec<_>>>()?;

        let metadata = self
            .prop("metadata", root)?
            .map(|prop| match &prop.value {
                PropValue::Expr(expr) => Ok(quote! {
                    __graphitepdf_document = __graphitepdf_document.with_metadata(#expr);
                }),
                _ => Err(Error::new(
                    prop.name.span(),
                    "Document metadata must be provided as a Rust expression",
                )),
            })
            .transpose()?;

        Ok(quote! {{
            let mut __graphitepdf_document = ::graphitepdf::template::__private::LayoutDocument::new();
            #metadata
            #(
                #child_pages
            )*
            __graphitepdf_document
        }})
    }

    fn generate_document_child(&self, child: &PdfAstNode) -> Result<TokenStream2> {
        match child {
            PdfAstNode::Element(element) if element.name == "Page" => {
                let page = self.generate_page(element)?;
                Ok(quote! {
                    __graphitepdf_document.add_page(#page);
                })
            }
            PdfAstNode::Element(element) => Err(Error::new(
                element.name.span(),
                "Document children must be <Page> elements in the current prototype",
            )),
            PdfAstNode::Text(text) => Err(Error::new(
                text.span(),
                "Document children cannot be raw text in the current prototype",
            )),
            PdfAstNode::Expression(expr) => Ok(quote! {
                for __graphitepdf_page in ::graphitepdf::template::__private::into_layout_pages(#expr) {
                    __graphitepdf_document.add_page(__graphitepdf_page);
                }
            }),
        }
    }

    fn generate_page(&self, page: &ElementNode) -> Result<TokenStream2> {
        self.validate_props(page, &["size", "style"])?;

        let children = page
            .children
            .iter()
            .map(|child| self.generate_layout_child_append(child, "Page", "__graphitepdf_children"))
            .collect::<Result<Vec<_>>>()?;

        let size = self
            .prop("size", page)?
            .map(|prop| self.generate_page_size(prop))
            .transpose()?;
        let style = self
            .prop("style", page)?
            .map(|prop| self.generate_style_assignment(prop, "__graphitepdf_page"))
            .transpose()?;

        Ok(quote! {{
            let mut __graphitepdf_children = ::std::vec::Vec::new();
            #(#children)*
            let mut __graphitepdf_page = ::graphitepdf::template::__private::LayoutPage::new(__graphitepdf_children);
            #size
            #style
            __graphitepdf_page
        }})
    }

    fn generate_layout_child_append(
        &self,
        child: &PdfAstNode,
        parent: &str,
        target: &str,
    ) -> Result<TokenStream2> {
        let target = Ident::new(target, Span::call_site());

        match child {
            PdfAstNode::Element(element) => {
                let child_expr = self.generate_layout_element(element, parent)?;
                Ok(quote! {
                    #target.push(#child_expr);
                })
            }
            PdfAstNode::Text(text) => Err(Error::new(
                text.span(),
                format!("{parent} children cannot be raw text in the current prototype"),
            )),
            PdfAstNode::Expression(expr) => Ok(quote! {
                #target.extend(::graphitepdf::template::__private::into_layout_nodes(#expr));
            }),
        }
    }

    fn generate_layout_element(&self, element: &ElementNode, parent: &str) -> Result<TokenStream2> {
        if element.name == "View" {
            self.generate_view(element)
        } else if element.name == "Text" {
            self.generate_text(element)
        } else if element.name == "Image" {
            self.generate_image(element)
        } else {
            Err(Error::new(
                element.name.span(),
                format!(
                    "{parent} children must be <View>, <Text>, or <Image> elements in the current prototype"
                ),
            ))
        }
    }

    fn generate_view(&self, view: &ElementNode) -> Result<TokenStream2> {
        self.validate_props(view, &["style"])?;

        let children = view
            .children
            .iter()
            .map(|child| self.generate_layout_child_append(child, "View", "__graphitepdf_children"))
            .collect::<Result<Vec<_>>>()?;

        let style = self
            .prop("style", view)?
            .map(|prop| self.generate_style_assignment(prop, "__graphitepdf_node"))
            .transpose()?;

        Ok(quote! {{
            let mut __graphitepdf_children = ::std::vec::Vec::new();
            #(#children)*
            let mut __graphitepdf_node = ::graphitepdf::template::__private::LayoutNode::view(__graphitepdf_children);
            #style
            __graphitepdf_node
        }})
    }

    fn generate_text(&self, text: &ElementNode) -> Result<TokenStream2> {
        self.validate_props(text, &["style"])?;

        let mut fragments = Vec::new();
        for child in &text.children {
            match child {
                PdfAstNode::Text(literal) => {
                    fragments.push(quote! {
                        __graphitepdf_text.push_str(#literal);
                    });
                }
                PdfAstNode::Expression(expr) => {
                    fragments.push(quote! {
                        __graphitepdf_text.push_str(&(#expr).to_string());
                    });
                }
                PdfAstNode::Element(element) => {
                    return Err(Error::new(
                        element.name.span(),
                        "Text children cannot contain nested elements in the current prototype",
                    ));
                }
            }
        }

        let style = self
            .prop("style", text)?
            .map(|prop| self.generate_style_assignment(prop, "__graphitepdf_node"))
            .transpose()?;

        let node_expr = if fragments.is_empty() {
            quote! {
                ::graphitepdf::template::__private::text_node_from_str("")
            }
        } else {
            quote! {{
                let mut __graphitepdf_text = ::std::string::String::new();
                #(#fragments)*
                ::graphitepdf::template::__private::text_node_from_string(__graphitepdf_text)
            }}
        };

        Ok(quote! {{
            let mut __graphitepdf_node = #node_expr;
            #style
            __graphitepdf_node
        }})
    }

    fn generate_image(&self, image: &ElementNode) -> Result<TokenStream2> {
        self.validate_props(image, &["src", "style"])?;

        if !image.self_closing && !image.children.is_empty() {
            return Err(Error::new(
                image.name.span(),
                "Image cannot have children in the current prototype",
            ));
        }

        let Some(src_prop) = self.prop("src", image)? else {
            return Err(Error::new(
                image.name.span(),
                "Image requires a src prop in the current prototype",
            ));
        };

        let src = match &src_prop.value {
            PropValue::Expr(expr) => quote! { #expr },
            _ => {
                return Err(Error::new(
                    src_prop.name.span(),
                    "Image src must be provided as a Rust expression in the current prototype",
                ));
            }
        };

        let style = self
            .prop("style", image)?
            .map(|prop| self.generate_style_assignment(prop, "__graphitepdf_node"))
            .transpose()?;

        Ok(quote! {{
            let mut __graphitepdf_node = ::graphitepdf::template::__private::LayoutNode::image_source(#src);
            #style
            __graphitepdf_node
        }})
    }

    fn generate_page_size(&self, prop: &Prop) -> Result<TokenStream2> {
        let size = match &prop.value {
            PropValue::Expr(expr) => quote! {
                __graphitepdf_page = __graphitepdf_page.with_size(
                    ::graphitepdf::template::__private::into_pdf_size(#expr)
                );
            },
            PropValue::String(value) => {
                let page_size = match value.value().to_ascii_uppercase().as_str() {
                    "A0" => quote!(::graphitepdf::template::__private::PageSize::A0),
                    "A1" => quote!(::graphitepdf::template::__private::PageSize::A1),
                    "A2" => quote!(::graphitepdf::template::__private::PageSize::A2),
                    "A3" => quote!(::graphitepdf::template::__private::PageSize::A3),
                    "A4" => quote!(::graphitepdf::template::__private::PageSize::A4),
                    "A5" => quote!(::graphitepdf::template::__private::PageSize::A5),
                    "A6" => quote!(::graphitepdf::template::__private::PageSize::A6),
                    "LETTER" => quote!(::graphitepdf::template::__private::PageSize::LETTER),
                    "LEGAL" => quote!(::graphitepdf::template::__private::PageSize::LEGAL),
                    "TABLOID" => quote!(::graphitepdf::template::__private::PageSize::TABLOID),
                    _ => {
                        return Err(Error::new(
                            value.span(),
                            "unsupported page size literal; use a known size like \"A4\" or a typed Rust expression",
                        ));
                    }
                };

                quote! {
                    __graphitepdf_page = __graphitepdf_page.with_size(
                        ::graphitepdf::template::__private::into_pdf_size(#page_size)
                    );
                }
            }
            PropValue::Bool => {
                return Err(Error::new(
                    prop.name.span(),
                    "Page size must be a string literal like \"A4\" or a typed Rust expression",
                ));
            }
        };

        Ok(size)
    }

    fn generate_style_assignment(&self, prop: &Prop, target: &str) -> Result<TokenStream2> {
        let target = Ident::new(target, Span::call_site());

        match &prop.value {
            PropValue::Expr(expr) => Ok(quote! {
                #target = #target.with_style(#expr);
            }),
            _ => Err(Error::new(
                prop.name.span(),
                "style must be provided as a Rust expression in the current prototype",
            )),
        }
    }

    fn validate_props(&self, element: &ElementNode, allowed: &[&str]) -> Result<()> {
        for prop in &element.props {
            let prop_name = prop.name.to_string();
            if !allowed.iter().any(|candidate| *candidate == prop_name) {
                return Err(Error::new(
                    prop.name.span(),
                    format!(
                        "unsupported prop `{prop_name}` on <{}> in the current prototype",
                        element.name
                    ),
                ));
            }
        }

        for allowed_name in allowed {
            let count = element
                .props
                .iter()
                .filter(|prop| prop.name == *allowed_name)
                .count();
            if count > 1 {
                return Err(Error::new(
                    element.name.span(),
                    format!("duplicate `{allowed_name}` prop on <{}>", element.name),
                ));
            }
        }

        Ok(())
    }

    fn prop<'a>(&self, name: &str, element: &'a ElementNode) -> Result<Option<&'a Prop>> {
        Ok(element.props.iter().find(|prop| prop.name == name))
    }
}
