pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    GraphitepdfErrors(#[from] graphitepdf_errors::GraphitePdfError),

    #[error(transparent)]
    Render(#[from] graphitepdf_render::Error),

    #[error("renderer backend error: {message}")]
    Backend { message: String },
}
