pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    GraphitepdfErrors(#[from] graphitepdf_errors::GraphitePdfError),

    #[error(transparent)]
    Font(#[from] graphitepdf_font::Error),

    #[error("text content cannot be empty")]
    EmptyText,

    #[error("font size must be positive, got {size}")]
    InvalidFontSize { size: f32 },

    #[error("text range `{start}..{end}` is invalid for content length {len}")]
    InvalidTextRange {
        start: usize,
        end: usize,
        len: usize,
    },

    #[error("text range `{start}..{end}` must align to UTF-8 boundaries")]
    NonCharacterBoundaryRange { start: usize, end: usize },

    #[error("text container must have positive dimensions, got width={width} height={height}")]
    InvalidTextContainer { width: f32, height: f32 },

    #[error("no registered or fallback font could satisfy family `{family}`")]
    UnresolvedFont { family: String },
}
