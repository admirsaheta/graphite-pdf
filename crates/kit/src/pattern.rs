use crate::vector::Color;

#[derive(Clone, Debug)]
pub enum Pattern {
    Solid(Color),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Tiling(TilingPattern),
}

#[derive(Clone, Debug)]
pub struct LinearGradient {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub stops: Vec<GradientStop>,
}

#[derive(Clone, Debug)]
pub struct RadialGradient {
    pub cx0: f64,
    pub cy0: f64,
    pub r0: f64,
    pub cx1: f64,
    pub cy1: f64,
    pub r1: f64,
    pub stops: Vec<GradientStop>,
}

#[derive(Clone, Debug)]
pub struct GradientStop {
    pub offset: f64,
    pub color: Color,
}

impl GradientStop {
    pub fn new(offset: f64, color: Color) -> Self {
        Self { offset, color }
    }
}

#[derive(Clone, Debug)]
pub struct TilingPattern {
    pub x_step: f64,
    pub y_step: f64,
    pub bbox: (f64, f64, f64, f64),
}
