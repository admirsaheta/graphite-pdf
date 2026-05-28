use crate::text::TextBuilder;
use crate::vector::{Canvas, Color};
use crate::page::PageSize;

#[cfg(feature = "tracing")]
use tracing::instrument;

#[derive(Clone, Debug, PartialEq)]
pub struct BorderStyle {
    pub color: Color,
    pub width: f64,
    pub enabled: bool,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            width: 1.0,
            enabled: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TableCell {
    pub content: Vec<u8>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub background: Option<Color>,
    pub border: BorderStyle,
    pub colspan: u32,
    pub rowspan: u32,
}

impl Default for TableCell {
    fn default() -> Self {
        Self {
            content: Vec::new(),
            width: None,
            height: None,
            background: None,
            border: Default::default(),
            colspan: 1,
            rowspan: 1,
        }
    }
}

impl TableCell {
    pub fn text(text: TextBuilder) -> Self {
        Self {
            content: text.finish(),
            ..Default::default()
        }
    }

    pub fn canvas(canvas: Canvas) -> Self {
        Self {
            content: canvas.finish(),
            ..Default::default()
        }
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    pub fn border(mut self, style: BorderStyle) -> Self {
        self.border = style;
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct TableRow {
    cells: Vec<TableCell>,
    height: Option<f64>,
}

impl TableRow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cell(mut self, cell: TableCell) -> Self {
        self.cells.push(cell);
        self
    }

    pub fn height(mut self, h: f64) -> Self {
        self.height = Some(h);
        self
    }
}

#[derive(Clone, Debug)]
pub struct TableBuilder {
    rows: Vec<TableRow>,
    x: f64,
    y: f64,
    column_widths: Vec<f64>,
}

impl TableBuilder {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            rows: Vec::new(),
            x,
            y,
            column_widths: Vec::new(),
        }
    }

    pub fn column_widths(mut self, widths: &[f64]) -> Self {
        self.column_widths = widths.to_vec();
        self
    }

    pub fn row(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }

    #[cfg_attr(feature = "tracing", instrument)]
    pub fn finish(self) -> Vec<u8> {
        let mut canvas = Canvas::new();
        let mut y = self.y;

        let num_cols = self.rows.iter().map(|row| row.cells.len()).max().unwrap_or(0);
        let col_widths = if !self.column_widths.is_empty() && self.column_widths.len() == num_cols {
            self.column_widths.clone()
        } else {
            let total_width = PageSize::A4.width - 144.0;
            vec![total_width / num_cols as f64; num_cols]
        };

        for row in &self.rows {
            let row_height = row.height.unwrap_or(30.0);
            let mut x = self.x;
            for (i, cell) in row.cells.iter().enumerate() {
                let cell_width = cell.width.unwrap_or(col_widths[i]);
                if let Some(bg) = cell.background {
                    canvas = canvas.fill_color(bg);
                    canvas = canvas.rect(x, y, cell_width, row_height);
                    canvas = canvas.fill();
                }
                if cell.border.enabled {
                    canvas = canvas.stroke_color(cell.border.color);
                    canvas = canvas.line_width(cell.border.width);
                    canvas = canvas.rect(x, y, cell_width, row_height);
                    canvas = canvas.stroke();
                }
                x += cell_width;
            }
            y -= row_height;
        }
        canvas.finish()
    }
}
