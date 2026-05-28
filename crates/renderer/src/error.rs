pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Layout(#[from] graphitepdf_layout::Error),

    #[error(transparent)]
    Render(#[from] graphitepdf_render::Error),

    #[error(transparent)]
    Kit(#[from] graphitepdf_kit::GraphitePdfKitError),

    #[error("renderer backend error: {message}")]
    Backend { message: String },
}
