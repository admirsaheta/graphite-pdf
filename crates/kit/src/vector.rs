use std::fmt;

/// Represents an RGB color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    pub const fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from((r, g, b): (f64, f64, f64)) -> Self {
        Self { r, g, b }
    }
}

/// Line cap style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

/// Line join style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

/// Canvas for building vector graphics paths.
#[derive(Debug, Clone)]
pub struct Canvas {
    buffer: Vec<u8>,
}

impl Canvas {
    /// Creates a new canvas.
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// Moves to a point without drawing a line.
    pub fn move_to(mut self, x: f64, y: f64) -> Self {
        self.buffer.extend(format!("{} {} m\n", x, y).as_bytes());
        self
    }

    /// Draws a line from the current point to a new point.
    pub fn line_to(mut self, x: f64, y: f64) -> Self {
        self.buffer.extend(format!("{} {} l\n", x, y).as_bytes());
        self
    }

    /// Draws a cubic Bezier curve.
    pub fn curve_to(mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> Self {
        self.buffer
            .extend(format!("{} {} {} {} {} {} c\n", x1, y1, x2, y2, x3, y3).as_bytes());
        self
    }

    /// Draws a rectangle.
    pub fn rect(mut self, x: f64, y: f64, width: f64, height: f64) -> Self {
        self.buffer
            .extend(format!("{} {} {} {} re\n", x, y, width, height).as_bytes());
        self
    }

    /// Sets the stroke color.
    pub fn stroke_color(mut self, color: Color) -> Self {
        self.buffer
            .extend(format!("{} {} {} RG\n", color.r, color.g, color.b).as_bytes());
        self
    }

    /// Sets the fill color.
    pub fn fill_color(mut self, color: Color) -> Self {
        self.buffer
            .extend(format!("{} {} {} rg\n", color.r, color.g, color.b).as_bytes());
        self
    }

    /// Sets the line width.
    pub fn line_width(mut self, width: f64) -> Self {
        self.buffer.extend(format!("{} w\n", width).as_bytes());
        self
    }

    /// Closes the current path.
    pub fn close_path(mut self) -> Self {
        self.buffer.extend(b"h\n");
        self
    }

    /// Fills the current path.
    pub fn fill(mut self) -> Self {
        self.buffer.extend(b"f\n");
        self
    }

    /// Strokes the current path.
    pub fn stroke(mut self) -> Self {
        self.buffer.extend(b"S\n");
        self
    }

    /// Fills and strokes the current path.
    pub fn fill_stroke(mut self) -> Self {
        self.buffer.extend(b"B\n");
        self
    }

    /// Finishes the canvas and returns the PDF content stream bytes.
    pub fn finish(self) -> Vec<u8> {
        self.buffer
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<[u8]> for Canvas {
    fn as_ref(&self) -> &[u8] {
        &self.buffer
    }
}

impl fmt::Write for Canvas {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buffer.extend(s.as_bytes());
        Ok(())
    }
}
