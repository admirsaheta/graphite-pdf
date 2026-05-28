use std::io;

pub type Result<T> = std::result::Result<T, GraphitePdfError>;

#[derive(Debug, thiserror::Error)]
pub enum GraphitePdfError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("invalid PDF object: {0}")]
    InvalidObject(String),
    #[error("font error: {0}")]
    FontError(String),
    #[error("image error: {0}")]
    ImageError(String),
    #[error("invalid page size: {0}")]
    InvalidPageSize(String),
    #[error("encoding error: {0}")]
    EncodingError(String),
    #[error("compression error: {0}")]
    CompressionError(String),
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("invalid document: {0}")]
    InvalidDocument(String),
    #[error("layout error: {0}")]
    Layout(String),
    #[error("render error: {0}")]
    Render(String),
    #[error("unsupported feature: {0}")]
    UnsupportedFeature(&'static str),
}
