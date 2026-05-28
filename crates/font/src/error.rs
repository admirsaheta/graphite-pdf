pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    GraphitepdfErrors(#[from] graphitepdf_errors::GraphitePdfError),

    #[error("invalid font source: {message}")]
    InvalidFontSource { message: String },

    #[error("font family not registered: {family}")]
    UnknownFontFamily { family: String },

    #[error("font style `{style}` is not registered for family `{family}`")]
    UnknownFontStyle { family: String, style: String },

    #[error("font weight `{weight}` is not registered for family `{family}` and style `{style}`")]
    UnknownFontWeight {
        family: String,
        style: String,
        weight: u16,
    },

    #[error("invalid font weight `{weight}`; expected a value between 1 and 1000")]
    InvalidFontWeight { weight: u16 },

    #[error("invalid data URI: {message}")]
    InvalidDataUri { message: String },

    #[error("unsupported remote font URL scheme `{scheme}`")]
    UnsupportedRemoteScheme { scheme: String },

    #[error("failed to load local font from `{path}`: {message}")]
    LocalFontLoad { path: String, message: String },

    #[error("failed to load remote font from `{url}`: {message}")]
    RemoteFontLoad { url: String, message: String },
}
