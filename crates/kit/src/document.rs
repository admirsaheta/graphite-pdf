use crate::backend_font::{Font, FontRegistry};
use crate::error::Result;
use crate::metadata::Metadata;
use crate::page::{PageMargins, PageSize};
use crate::writer::PdfWriter;

#[cfg(feature = "tracing")]
use tracing::instrument;

#[derive(Clone, Debug)]
pub struct Page {
    size: PageSize,
    margins: PageMargins,
    content: Vec<u8>,
}

impl Page {
    pub fn new(size: PageSize) -> Self {
        Self {
            size,
            margins: PageMargins::default(),
            content: Vec::new(),
        }
    }

    pub fn with_margins(mut self, margins: PageMargins) -> Self {
        self.margins = margins;
        self
    }

    pub fn with_content(mut self, content: Vec<u8>) -> Self {
        self.content = content;
        self
    }
}

#[derive(Clone, Debug)]
pub struct DocumentBuilder {
    metadata: Metadata,
    pages: Vec<Page>,
    fonts: FontRegistry,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        Self {
            metadata: Metadata::new(),
            pages: Vec::new(),
            fonts: FontRegistry::with_default_font(),
        }
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn add_font(mut self, font: impl Into<Font>) -> (Self, String) {
        let name = self.fonts.register(font);
        (self, name)
    }

    pub fn add_page(mut self, page: Page) -> Self {
        self.pages.push(page);
        self
    }

    pub fn with_page(mut self, size: PageSize, content: impl Into<Vec<u8>>) -> Self {
        self.pages.push(Page {
            size,
            margins: PageMargins::default(),
            content: content.into(),
        });
        self
    }

    #[cfg_attr(feature = "tracing", instrument(skip(self, writer)))]
    pub fn write<W: std::io::Write>(self, mut writer: W) -> Result<()> {
        let mut pdf_writer = PdfWriter::with_metadata_and_fonts(self.metadata, self.fonts);
        for page in self.pages {
            pdf_writer.add_page(page.size, page.content);
        }
        let buffer = pdf_writer.write_all()?;
        writer.write_all(&buffer)?;
        Ok(())
    }

    #[cfg_attr(feature = "tracing", instrument(skip(self, path)))]
    pub fn save(self, path: impl AsRef<std::path::Path>) -> Result<()> {
        let mut file = std::fs::File::create(path)?;
        self.write(&mut file)?;
        Ok(())
    }
}

impl Default for DocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
