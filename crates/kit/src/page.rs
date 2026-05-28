use crate::error::{GraphitePdfKitError, Result};
use std::fmt;

/// Page size in points (1 point = 1/72 inch).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageSize {
    pub width: f64,
    pub height: f64,
}

impl PageSize {
    /// Standard A0 size (2384 × 3370 points).
    pub const A0: Self = Self::new(2383.937, 3370.394);
    /// Standard A1 size (1684 × 2384 points).
    pub const A1: Self = Self::new(1683.780, 2383.937);
    /// Standard A2 size (1191 × 1684 points).
    pub const A2: Self = Self::new(1190.551, 1683.780);
    /// Standard A3 size (842 × 1191 points).
    pub const A3: Self = Self::new(841.890, 1190.551);
    /// Standard A4 size (595 × 842 points) (default).
    pub const A4: Self = Self::new(595.276, 841.890);
    /// Standard A5 size (420 × 595 points).
    pub const A5: Self = Self::new(420.472, 595.276);
    /// Standard A6 size (298 × 420 points).
    pub const A6: Self = Self::new(297.638, 420.472);
    /// Standard Letter size (612 × 792 points).
    pub const LETTER: Self = Self::new(612.0, 792.0);
    /// Standard Legal size (612 × 1008 points).
    pub const LEGAL: Self = Self::new(612.0, 1008.0);
    /// Standard Tabloid size (792 × 1224 points).
    pub const TABLOID: Self = Self::new(792.0, 1224.0);

    /// Creates a new page size with given width and height.
    pub const fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    /// Returns a landscape version of the page size (swaps width and height).
    pub fn landscape(&self) -> Self {
        Self::new(self.height, self.width)
    }

    /// Converts inches to points.
    pub fn from_inches(width_in: f64, height_in: f64) -> Result<Self> {
        if width_in <= 0.0 || height_in <= 0.0 {
            return Err(GraphitePdfKitError::InvalidPageSize(
                "Page dimensions must be positive".to_string(),
            ));
        }
        Ok(Self::new(width_in * 72.0, height_in * 72.0))
    }

    /// Converts millimeters to points.
    pub fn from_mm(width_mm: f64, height_mm: f64) -> Result<Self> {
        if width_mm <= 0.0 || height_mm <= 0.0 {
            return Err(GraphitePdfKitError::InvalidPageSize(
                "Page dimensions must be positive".to_string(),
            ));
        }
        const MM_PER_POINT: f64 = 25.4 / 72.0;
        Ok(Self::new(width_mm / MM_PER_POINT, height_mm / MM_PER_POINT))
    }

    /// Converts centimeters to points.
    pub fn from_cm(width_cm: f64, height_cm: f64) -> Result<Self> {
        Self::from_mm(width_cm * 10.0, height_cm * 10.0)
    }
}

impl TryFrom<(f64, f64)> for PageSize {
    type Error = GraphitePdfKitError;

    fn try_from(value: (f64, f64)) -> Result<Self> {
        if value.0 <= 0.0 || value.1 <= 0.0 {
            return Err(GraphitePdfKitError::InvalidPageSize(
                "Page width and height must be positive".to_string(),
            ));
        }
        Ok(Self::new(value.0, value.1))
    }
}

impl Default for PageSize {
    fn default() -> Self {
        Self::A4
    }
}

impl fmt::Display for PageSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} × {} pt", self.width, self.height)
    }
}

/// Page margins in points.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageMargins {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}

impl PageMargins {
    /// Creates new margins.
    pub const fn new(left: f64, right: f64, top: f64, bottom: f64) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Creates margins with the same value for all four sides.
    pub const fn all(value: f64) -> Self {
        Self::new(value, value, value, value)
    }

    /// No margins.
    pub const ZERO: Self = Self::all(0.0);

    /// Default 1 inch margins.
    pub const ONE_INCH: Self = Self::all(72.0);
}

impl Default for PageMargins {
    fn default() -> Self {
        Self::ONE_INCH
    }
}

impl fmt::Display for PageMargins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "left: {} pt, right: {} pt, top: {} pt, bottom: {} pt",
            self.left, self.right, self.top, self.bottom
        )
    }
}

impl From<f64> for PageMargins {
    fn from(value: f64) -> Self {
        Self::all(value)
    }
}

/// Page orientation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageOrientation {
    /// Portrait orientation (default).
    Portrait,
    /// Landscape orientation.
    Landscape,
}

impl Default for PageOrientation {
    fn default() -> Self {
        Self::Portrait
    }
}
