use crate::vector::Color;

#[cfg(feature = "tracing")]
use tracing::instrument;

/// Text alignment options.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlignment {
    /// Left-aligned (default).
    Left,
    /// Center-aligned.
    Center,
    /// Right-aligned.
    Right,
}

impl Default for TextAlignment {
    fn default() -> Self {
        Self::Left
    }
}

/// Text rendering mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextRenderingMode {
    /// Fill text (default).
    Fill,
    /// Stroke text.
    Stroke,
    /// Fill and stroke text.
    FillStroke,
    /// Invisible text (clip path).
    Invisible,
    /// Fill and add to clip path.
    FillClip,
    /// Stroke and add to clip path.
    StrokeClip,
    /// Fill, stroke, and add to clip path.
    FillStrokeClip,
    /// Add text to clip path.
    Clip,
}

impl Default for TextRenderingMode {
    fn default() -> Self {
        Self::Fill
    }
}

/// Font weight options.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontWeight {
    /// Thin (100).
    Thin,
    /// Extra light (200).
    ExtraLight,
    /// Light (300).
    Light,
    /// Normal (400, default).
    Normal,
    /// Medium (500).
    Medium,
    /// Semi-bold (600).
    SemiBold,
    /// Bold (700).
    Bold,
    /// Extra bold (800).
    ExtraBold,
    /// Black (900).
    Black,
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Normal
    }
}

/// Builder for constructing text content.
#[derive(Clone, Debug)]
pub struct TextBuilder {
    commands: Vec<String>,
}

impl TextBuilder {
    /// Creates a new text builder.
    pub fn new() -> Self {
        Self {
            commands: vec!["BT".to_string()],
        }
    }

    /// Sets the current font and font size.
    pub fn font(mut self, name: &str, size: f64) -> Self {
        self.commands.push(format!("/{} {} Tf", name, size));
        self
    }

    /// Moves to the next line.
    pub fn next_line(mut self, offset_x: f64, offset_y: f64) -> Self {
        self.commands.push(format!("{} {} Td", offset_x, offset_y));
        self
    }

    /// Moves to an absolute position.
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.commands.push(format!("{} {} Td", x, y));
        self
    }

    /// Sets the text matrix and text position matrix.
    pub fn text_matrix(mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        self.commands.push(format!("{} {} {} {} {} {} Tm", a, b, c, d, e, f));
        self
    }

    /// Adds text to the current position.
    pub fn text(mut self, text: &str) -> Self {
        let escaped = text
            .chars()
            .map(|c| match c {
                '(' => "\\(".to_string(),
                ')' => "\\)".to_string(),
                '\\' => "\\\\".to_string(),
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\t' => "\\t".to_string(),
                '\x08' => "\\b".to_string(),
                '\x0c' => "\\f".to_string(),
                _ => c.to_string(),
            })
            .collect::<String>();
        self.commands.push(format!("({}) Tj", escaped));
        self
    }

    /// Adds text and moves to the next line.
    pub fn text_line(mut self, text: &str) -> Self {
        let escaped = text
            .chars()
            .map(|c| match c {
                '(' => "\\(".to_string(),
                ')' => "\\)".to_string(),
                '\\' => "\\\\".to_string(),
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\t' => "\\t".to_string(),
                '\x08' => "\\b".to_string(),
                '\x0c' => "\\f".to_string(),
                _ => c.to_string(),
            })
            .collect::<String>();
        self.commands.push(format!("({}) '", escaped));
        self
    }

    /// Adds text with spacing between words and characters.
    pub fn text_line_spacing(mut self, text: &str, word_spacing: f64, char_spacing: f64) -> Self {
        let escaped = text
            .chars()
            .map(|c| match c {
                '(' => "\\(".to_string(),
                ')' => "\\)".to_string(),
                '\\' => "\\\\".to_string(),
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\t' => "\\t".to_string(),
                '\x08' => "\\b".to_string(),
                '\x0c' => "\\f".to_string(),
                _ => c.to_string(),
            })
            .collect::<String>();
        self.commands.push(format!("{} {} ({}) \"", word_spacing, char_spacing, escaped));
        self
    }

    /// Sets the character spacing.
    pub fn char_spacing(mut self, spacing: f64) -> Self {
        self.commands.push(format!("{} Tc", spacing));
        self
    }

    /// Sets the word spacing.
    pub fn word_spacing(mut self, spacing: f64) -> Self {
        self.commands.push(format!("{} Tw", spacing));
        self
    }

    /// Sets the horizontal text scaling (1.0 = normal).
    pub fn horizontal_scaling(mut self, scale: f64) -> Self {
        self.commands.push(format!("{} Tz", scale * 100.0));
        self
    }

    /// Sets the leading (line spacing).
    pub fn leading(mut self, leading: f64) -> Self {
        self.commands.push(format!("{} TL", leading));
        self
    }

    /// Sets the text rendering mode.
    pub fn rendering_mode(mut self, mode: TextRenderingMode) -> Self {
        let code = match mode {
            TextRenderingMode::Fill => 0,
            TextRenderingMode::Stroke => 1,
            TextRenderingMode::FillStroke => 2,
            TextRenderingMode::Invisible => 3,
            TextRenderingMode::FillClip => 4,
            TextRenderingMode::StrokeClip => 5,
            TextRenderingMode::FillStrokeClip => 6,
            TextRenderingMode::Clip => 7,
        };
        self.commands.push(format!("{} Tr", code));
        self
    }

    /// Sets the text rise (for superscripts/subscripts).
    pub fn text_rise(mut self, rise: f64) -> Self {
        self.commands.push(format!("{} Ts", rise));
        self
    }

    /// Sets the text color.
    pub fn set_color(mut self, color: Color) -> Self {
        self.commands.push(format!("{} {} {} rg", color.r, color.g, color.b));
        self
    }

    /// Applies a translation to the text.
    pub fn translate(mut self, tx: f64, ty: f64) -> Self {
        self.commands.push(format!("1 0 0 1 {} {} Tm", tx, ty));
        self
    }

    /// Applies a rotation to the text (in radians).
    pub fn rotate(mut self, angle: f64) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        self.commands.push(format!("{} {} {} {} 0 0 Tm", cos_a, sin_a, -sin_a, cos_a));
        self
    }

    /// Applies scaling to the text.
    pub fn scale(mut self, sx: f64, sy: f64) -> Self {
        self.commands.push(format!("{} 0 0 {} 0 0 Tm", sx, sy));
        self
    }

    /// Applies a skew transformation to the text.
    pub fn skew(mut self, ax: f64, ay: f64) -> Self {
        let tan_x = ax.tan();
        let tan_y = ay.tan();
        self.commands.push(format!("1 {} {} 1 0 0 Tm", tan_x, tan_y));
        self
    }

    /// Finishes the text block and returns the content bytes.
    #[cfg_attr(feature = "tracing", instrument)]
    pub fn finish(self) -> Vec<u8> {
        let mut cmds = self.commands;
        cmds.push("ET".to_string());
        let mut content = cmds.join("\n");
        content.push('\n');
        content.into_bytes()
    }

    /// Extends this text builder with another's commands.
    pub fn extend(mut self, other: TextBuilder) -> Self {
        self.commands.extend(other.commands);
        self
    }
}

impl Default for TextBuilder {
    fn default() -> Self {
        Self::new()
    }
}
