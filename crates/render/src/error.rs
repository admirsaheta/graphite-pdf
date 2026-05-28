pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    GraphitepdfErrors(#[from] graphitepdf_errors::GraphitePdfError),

    #[error(transparent)]
    Layout(#[from] graphitepdf_layout::Error),

    #[error(transparent)]
    Font(#[from] graphitepdf_font::Error),

    #[error(transparent)]
    Image(#[from] graphitepdf_image::Error),

    #[error(transparent)]
    Text(#[from] graphitepdf_textkit::Error),

    #[error("invalid color `{input}`")]
    InvalidColor { input: String },

    #[error("invalid transform `{input}`")]
    InvalidTransform { input: String },

    #[error("invalid dimension `{input}`")]
    InvalidDimension { input: String },

    #[error("could not resolve intrinsic dimensions for {kind}")]
    InvalidNaturalDimensions { kind: &'static str },

    #[error("SVG content is missing valid width and height information")]
    InvalidSvgDimensions,

    #[error("render backend error: {message}")]
    Backend { message: String },
}
