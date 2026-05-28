pub type Result<T> = std::result::Result<T, Error>;

use std::str::Utf8Error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    GraphitepdfErrors(#[from] graphitepdf_errors::GraphitePdfError),

    #[error(transparent)]
    QuickXml(#[from] quick_xml::Error),

    #[error(transparent)]
    QuickXmlEncoding(#[from] quick_xml::encoding::EncodingError),

    #[error(transparent)]
    QuickXmlEscape(#[from] quick_xml::escape::EscapeError),

    #[error(transparent)]
    QuickXmlAttr(#[from] quick_xml::events::attributes::AttrError),

    #[error(transparent)]
    Utf8(#[from] Utf8Error),
}
