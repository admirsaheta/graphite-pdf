pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    GraphitepdfErrors(#[from] graphitepdf_errors::GraphitePdfError),

    #[error("text content cannot be empty")]
    EmptyText,

    #[error("font size must be positive, got {size}")]
    InvalidFontSize { size: f32 },
}
