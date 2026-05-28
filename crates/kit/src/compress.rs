use crate::error::{GraphitePdfKitError, Result};
use flate2::{Compression, write::ZlibEncoder};

#[cfg(feature = "tracing")]
use tracing::instrument;

#[cfg_attr(feature = "tracing", instrument(skip(data)))]
pub fn flate_encode(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    std::io::Write::write_all(&mut encoder, data)
        .map_err(|e| GraphitePdfKitError::CompressionError(format!("Failed to write to encoder: {}", e)))?;
    encoder.finish()
        .map_err(|e| GraphitePdfKitError::CompressionError(format!("Failed to finish compression: {}", e)))
}
