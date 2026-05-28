#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Pt(pub f32);

impl Pt {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub const fn value(self) -> f32 {
        self.0
    }
}

impl From<f32> for Pt {
    fn from(value: f32) -> Self {
        Self(value)
    }
}
