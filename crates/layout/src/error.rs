pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    GraphitepdfErrors(#[from] graphitepdf_errors::GraphitePdfError),

    #[error(transparent)]
    Font(#[from] graphitepdf_font::Error),

    #[error(transparent)]
    Image(#[from] graphitepdf_image::Error),

    #[error(transparent)]
    Math(#[from] graphitepdf_math::Error),

    #[error(transparent)]
    Svg(#[from] graphitepdf_svg::Error),

    #[error(transparent)]
    Text(#[from] graphitepdf_textkit::Error),

    #[error("layout document must contain at least one page")]
    EmptyDocument,

    #[error("layout page size must be positive, got {width}x{height}")]
    InvalidPageSize { width: f32, height: f32 },

    #[error("could not resolve intrinsic dimensions for {kind}")]
    InvalidNaturalDimensions { kind: &'static str },

    #[error("image source nodes require an explicit height until async asset loading is wired in")]
    UnresolvedAssetDimensions { kind: &'static str },

    #[error("SVG content is missing valid width and height information")]
    InvalidSvgDimensions,

    #[error("invalid dimension `{input}`")]
    InvalidDimension { input: String },
}
