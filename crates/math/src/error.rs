pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    GraphitepdfErrors(#[from] graphitepdf_errors::GraphitePdfError),

    #[error(transparent)]
    Svg(#[from] graphitepdf_svg::Error),

    #[error("math render backend error: {0}")]
    MathBackend(String),

    #[error("invalid math dimension `{input}`")]
    InvalidDimension { input: String },

    #[error("math SVG is missing a valid viewBox")]
    InvalidViewBox,

    #[error("math backend returned an invalid SVG root")]
    InvalidSvgRoot,
}
