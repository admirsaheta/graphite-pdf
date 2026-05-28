use crate::compress::flate_encode;
use crate::error::Result;
use crate::font::{Font, FontRegistry, StandardFont};
use crate::metadata::Metadata;
use crate::objects::Object;
use crate::page::PageSize;
use std::io::Write;

#[cfg(feature = "tracing")]
use tracing::instrument;

#[derive(Clone, Debug)]
struct PageInfo {
    size: PageSize,
    content: Vec<u8>,
}

pub struct PdfWriter {
    buffer: Vec<u8>,
    pages: Vec<PageInfo>,
    metadata: Metadata,
    fonts: FontRegistry,
}

impl PdfWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            pages: Vec::new(),
            metadata: Metadata::new(),
            fonts: FontRegistry::new(),
        }
    }

    pub fn with_metadata(metadata: Metadata) -> Self {
        let mut fonts = FontRegistry::new();
        // Add default Helvetica font
        fonts.register(Font::standard(StandardFont::Helvetica));

        Self {
            buffer: Vec::new(),
            pages: Vec::new(),
            metadata,
            fonts,
        }
    }

    pub fn add_page(&mut self, size: PageSize, content: Vec<u8>) {
        self.pages.push(PageInfo { size, content });
    }

    pub fn add_font(&mut self, font: Font) -> String {
        self.fonts.register(font)
    }

    #[cfg_attr(feature = "tracing", instrument(skip(self)))]
    pub fn write_all(mut self) -> Result<Vec<u8>> {
        self.buffer.extend(b"%PDF-1.7\n");
        self.buffer.extend(b"%\xc3\xa4\xc3\xb1\xc3\xb6\xc3\x9f\n");

        let mut objects = Vec::new();

        // 1. Add all fonts
        let mut font_ids = std::collections::HashMap::new();
        for (name, (font, _)) in &self.fonts.fonts {
            let font_obj = Object::dict([
                ("Type", Object::name("Font")),
                ("Subtype", Object::name("Type1")),
                ("BaseFont", Object::name(&font.name)),
                ("Encoding", Object::name("WinAnsiEncoding")),
            ]);
            font_ids.insert(name.clone(), objects.len() + 1);
            objects.push(font_obj);
        }

        // 2. Add all page content streams
        let mut content_ids = Vec::new();
        for page in &self.pages {
            let compressed = flate_encode(&page.content)?;
            let content_obj = Object::Stream {
                dict: vec![
                    ("Filter".to_string(), Object::name("FlateDecode")),
                ],
                data: compressed,
            };
            content_ids.push(objects.len() + 1);
            objects.push(content_obj);
        }

        // 3. Add all page objects
        let mut page_ids = Vec::new();
        for (i, page) in self.pages.iter().enumerate() {
            let mut font_dict = vec![];
            for (name, &id) in &font_ids {
                font_dict.push((name.clone(), Object::Ref(id as u64)));
            }

            let page_obj = Object::dict([
                ("Type", Object::name("Page")),
                ("MediaBox", Object::array([
                    0.0,
                    0.0,
                    page.size.width,
                    page.size.height,
                ])),
                ("Contents", Object::Ref(content_ids[i] as u64)),
                ("Resources", Object::dict([
                    ("Font", Object::Dict(font_dict)),
                ])),
            ]);
            page_ids.push(objects.len() + 1);
            objects.push(page_obj);
        }

        // 4. Add pages tree
        let pages_tree_id = objects.len() + 1;
        objects.push(Object::dict([
            ("Type", Object::name("Pages")),
            ("Count", Object::Integer(page_ids.len() as i64)),
            ("Kids", Object::Array(page_ids.into_iter().map(|id| Object::Ref(id as u64)).collect())),
        ]));

        // 5. Add catalog
        let catalog_id = objects.len() + 1;
        objects.push(Object::dict([
            ("Type", Object::name("Catalog")),
            ("Pages", Object::Ref(pages_tree_id as u64)),
        ]));

        // 6. Add info dictionary
        let mut info_dict_entries = vec![
            ("Producer".to_string(), Object::string("GraphitePDF Kit")),
        ];
        if let Some(title) = &self.metadata.title {
            info_dict_entries.push(("Title".to_string(), Object::string(title)));
        }
        if let Some(author) = &self.metadata.author {
            info_dict_entries.push(("Author".to_string(), Object::string(author)));
        }
        if let Some(subject) = &self.metadata.subject {
            info_dict_entries.push(("Subject".to_string(), Object::string(subject)));
        }
        if !self.metadata.keywords.is_empty() {
            info_dict_entries.push((
                "Keywords".to_string(),
                Object::string(self.metadata.keywords.join(", ")),
            ));
        }
        if let Some(creator) = &self.metadata.creator {
            info_dict_entries.push(("Creator".to_string(), Object::string(creator)));
        }
        let info_id = objects.len() + 1;
        objects.push(Object::Dict(info_dict_entries));

        let mut offsets = Vec::new();
        for obj in objects {
            offsets.push(self.buffer.len() as u64);
            writeln!(&mut self.buffer, "{} 0 obj", offsets.len())?;
            obj.write(&mut self.buffer)?;
            self.buffer.push(b'\n');
            writeln!(&mut self.buffer, "endobj")?;
        }

        let xref_offset = self.buffer.len() as u64;
        writeln!(&mut self.buffer, "xref")?;
        writeln!(&mut self.buffer, "0 {}", offsets.len() + 1)?;
        writeln!(&mut self.buffer, "0000000000 65535 f ")?;
        for offset in &offsets {
            writeln!(&mut self.buffer, "{:010} 00000 n ", offset)?;
        }

        writeln!(&mut self.buffer, "trailer")?;
        Object::dict([
            ("Size", Object::Integer((offsets.len() + 1) as i64)),
            ("Root", Object::Ref(catalog_id as u64)),
            ("Info", Object::Ref(info_id as u64)),
        ]).write(&mut self.buffer)?;
        writeln!(&mut self.buffer)?;

        writeln!(&mut self.buffer, "startxref")?;
        writeln!(&mut self.buffer, "{}", xref_offset)?;
        writeln!(&mut self.buffer, "%%EOF")?;

        Ok(self.buffer)
    }
}

impl Default for PdfWriter {
    fn default() -> Self {
        Self::new()
    }
}
