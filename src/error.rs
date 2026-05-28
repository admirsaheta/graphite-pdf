use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub type Result<T> = std::result::Result<T, GraphitePdfError>;

#[derive(Debug)]
pub enum GraphitePdfError {
    InvalidDocument(String),
    Layout(String),
    Render(String),
    UnsupportedFeature(&'static str),
}

impl Display for GraphitePdfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDocument(message) => write!(f, "invalid document: {message}"),
            Self::Layout(message) => write!(f, "layout error: {message}"),
            Self::Render(message) => write!(f, "render error: {message}"),
            Self::UnsupportedFeature(feature) => {
                write!(f, "unsupported feature: {feature}")
            }
        }
    }
}

impl Error for GraphitePdfError {}
