pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    GraphitepdfErrors(#[from] graphitepdf_errors::GraphitePdfError),

    #[error(transparent)]
    Text(#[from] graphitepdf_textkit::Error),

    #[error("layout page size must be positive, got {width}x{height}")]
    InvalidPageSize { width: f32, height: f32 },
}
