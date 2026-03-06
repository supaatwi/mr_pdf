use crate::layout::text::escape_pdf_str;
use crate::{Align, Color, Pdf, Size, VAlign};
use std::io::Write;

const CELL_PADDING: f64 = 5.0;

/// Configuration for table visual appearance.
#[derive(Clone, Debug)]
pub struct TableStyle {
    pub bg_color: Option<Color>,
    pub text_color: Option<Color>,
    pub font_size: f64,
}

/// Border style options for tables.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TableBorderStyle {
    #[default]
    Full,
    None,
    Ghost,
    HeaderOnly,
}

impl TableStyle {
    pub fn new(font_size: f64) -> Self {
        Self {
            bg_color: None,
            text_color: None,
            font_size,
        }
    }

    /// Sets the background color for cells.
    pub fn bg_color(&mut self, color: Color) -> &mut Self {
        self.bg_color = Some(color);
        self
    }

    /// Sets the text color for cells.
    pub fn text_color(&mut self, color: Color) -> &mut Self {
        self.text_color = Some(color);
        self
    }

    /// Sets the font size for text in cells.
    pub fn font_size(&mut self, size: f64) -> &mut Self {
        self.font_size = size;
        self
    }
}

/// Default alignment configurations for a column.
#[derive(Clone, Debug)]
pub struct ColumnConfig {
    pub align: Align,
    pub valign: VAlign,
}

impl Default for ColumnConfig {
    fn default() -> Self {
        Self {
            align: Align::Left,
            valign: VAlign::Top,
        }
    }
}

#[derive(Clone)]
pub enum Cell {
    Text(String),
    ImagePath(String),
    ImageBase64(String),
}

/// Represents a single cell in a table row.
#[derive(Clone)]
pub struct TableCell {
    pub content: Cell,
    pub colspan: usize,
    pub align: Option<Align>,
    pub valign: Option<VAlign>,
    pub link: Option<String>,
}

impl TableCell {
    /// Creates a text cell.
    pub fn text(s: &str) -> Self {
        Self {
            content: Cell::Text(s.to_string()),
            colspan: 1,
            align: None,
            valign: None,
            link: None,
        }
    }

    /// Creates an image cell from a file path.
    pub fn image(path: &str) -> Self {
        Self {
            content: Cell::ImagePath(path.to_string()),
            colspan: 1,
            align: None,
            valign: None,
            link: None,
        }
    }

    /// Creates an image cell from a Base64 string.
    pub fn image_base64(b64: &str) -> Self {
        Self {
            content: Cell::ImageBase64(b64.to_string()),
            colspan: 1,
            align: None,
            valign: None,
            link: None,
        }
    }

    /// Sets how many columns this cell should span.
    pub fn with_span(mut self, n: usize) -> Self {
        self.colspan = n.max(1);
        self
    }

    /// Overrides the horizontal alignment for this cell.
    pub fn align(mut self, a: Align) -> Self {
        self.align = Some(a);
        self
    }

    /// Overrides the vertical alignment for this cell.
    pub fn valign(mut self, v: VAlign) -> Self {
        self.valign = Some(v);
        self
    }

    /// Adds a clickable hyperlink to the cell.
    pub fn link(mut self, url: &str) -> Self {
        self.link = Some(url.to_string());
        self
    }
}

impl From<&str> for TableCell {
    fn from(s: &str) -> Self {
        Self::text(s)
    }
}

impl From<String> for TableCell {
    fn from(s: String) -> Self {
        Self {
            content: Cell::Text(s),
            colspan: 1,
            align: None,
            valign: None,
            link: None,
        }
    }
}

/// Fluent builder for creating table rows.
pub struct RowBuilder {
    pub cells: Vec<TableCell>,
}

impl RowBuilder {
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    /// Adds a text cell.
    pub fn cell(&mut self, text: &str) -> &mut Self {
        self.cells.push(TableCell::text(text));
        self
    }

    /// Adds an image cell.
    pub fn cell_image(&mut self, path: &str) -> &mut Self {
        self.cells.push(TableCell::image(path));
        self
    }

    /// Adds an image cell from Base64.
    pub fn cell_image_base64(&mut self, b64: &str) -> &mut Self {
        self.cells.push(TableCell::image_base64(b64));
        self
    }

    /// Sets the colspan for the most recently added cell.
    pub fn span(&mut self, n: usize) -> &mut Self {
        if let Some(last) = self.cells.last_mut() {
            last.colspan = n.max(1);
        }
        self
    }

    /// Sets the horizontal alignment for the most recently added cell.
    pub fn align(&mut self, a: Align) -> &mut Self {
        if let Some(last) = self.cells.last_mut() {
            last.align = Some(a);
        }
        self
    }

    /// Sets the vertical alignment for the most recently added cell.
    pub fn valign(&mut self, v: VAlign) -> &mut Self {
        if let Some(last) = self.cells.last_mut() {
            last.valign = Some(v);
        }
        self
    }

    /// Adds a hyperlink to the most recently added cell.
    pub fn link(&mut self, url: &str) -> &mut Self {
        if let Some(last) = self.cells.last_mut() {
            last.link = Some(url.to_string());
        }
        self
    }
}

/// Builder for complex PDF tables.
pub struct TableBuilder {
    widths: Vec<Size>,
    header: Vec<TableCell>,
    rows: Vec<Vec<TableCell>>,
    repeat_header: bool,
    column_configs: Vec<ColumnConfig>,
    header_style: TableStyle,
    row_style: TableStyle,
    border_style: TableBorderStyle,
    zebra_color: Option<Color>,
}

impl Default for TableBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TableBuilder {
    pub fn new() -> Self {
        Self {
            widths: Vec::new(),
            header: Vec::new(),
            rows: Vec::new(),
            repeat_header: false,
            column_configs: Vec::new(),
            header_style: TableStyle::new(11.0),
            row_style: TableStyle::new(11.0),
            border_style: TableBorderStyle::Full,
            zebra_color: None,
        }
    }

    /// Defines column widths. Can use points, percentages, or flex.
    pub fn widths<I, T>(&mut self, widths: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Size>,
    {
        self.widths = widths.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the table header cells.
    pub fn header<I, T>(&mut self, header: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<TableCell>,
    {
        self.header = header.into_iter().map(Into::into).collect();
        self
    }

    /// Adds a simple row of text cells.
    pub fn row<I, T>(&mut self, row: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<TableCell>,
    {
        self.rows.push(row.into_iter().map(Into::into).collect());
        self
    }

    /// Adds a row using the RowBuilder closure for complex cell config.
    pub fn row_builder<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut RowBuilder),
    {
        let mut builder = RowBuilder::new();
        f(&mut builder);
        self.rows.push(builder.cells);
        self
    }

    /// Configures default horizontal alignment for a specific column.
    pub fn column_align(&mut self, col: usize, align: Align) -> &mut Self {
        while self.column_configs.len() <= col {
            self.column_configs.push(ColumnConfig::default());
        }
        self.column_configs[col].align = align;
        self
    }

    /// Configures default vertical alignment for a specific column.
    pub fn column_valign(&mut self, col: usize, valign: VAlign) -> &mut Self {
        while self.column_configs.len() <= col {
            self.column_configs.push(ColumnConfig::default());
        }
        self.column_configs[col].valign = valign;
        self
    }

    /// Returns a mutable reference to the header style configuration.
    pub fn header_style(&mut self) -> &mut TableStyle {
        &mut self.header_style
    }

    /// Returns a mutable reference to the body row style configuration.
    pub fn row_style(&mut self) -> &mut TableStyle {
        &mut self.row_style
    }

    /// Enables header repetition on new pages.
    pub fn repeat_header(&mut self, repeat: bool) -> &mut Self {
        self.repeat_header = repeat;
        self
    }

    /// Sets the table border style.
    pub fn border(&mut self, style: TableBorderStyle) -> &mut Self {
        self.border_style = style;
        self
    }

    /// Enables zebra striping with the specified background color for even rows.
    pub fn zebra(&mut self, color: Color) -> &mut Self {
        self.zebra_color = Some(color);
        self
    }

    pub fn build(self) -> Table {
        Table {
            widths: self.widths,
            header: self.header,
            rows: self.rows,
            repeat_header: self.repeat_header,
            column_configs: self.column_configs,
            header_style: self.header_style,
            row_style: self.row_style,
            border_style: self.border_style,
            zebra_color: self.zebra_color,
        }
    }
}

pub struct Table {
    widths: Vec<Size>,
    header: Vec<TableCell>,
    rows: Vec<Vec<TableCell>>,
    repeat_header: bool,
    column_configs: Vec<ColumnConfig>,
    header_style: TableStyle,
    row_style: TableStyle,
    border_style: TableBorderStyle,
    zebra_color: Option<Color>,
}

fn measure<W: Write>(pdf: &Pdf<W>, text: &str, font_size: f64) -> f64 {
    match pdf.current_font {
        Some(fid) => pdf.font_manager.string_width(fid, text, font_size),
        None => text.len() as f64 * font_size * 0.52,
    }
}

fn wrap<W: Write>(pdf: &Pdf<W>, text: &str, col_width: f64, font_size: f64) -> Vec<String> {
    let available = (col_width - CELL_PADDING * 2.0).max(1.0);
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let candidate = if current.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current, word)
        };
        if measure(pdf, &candidate, font_size) > available && !current.is_empty() {
            lines.push(current);
            current = word.to_string();
        } else {
            current = candidate;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

fn spanned_width(widths: &[f64], start_col: usize, colspan: usize, default_w: f64) -> f64 {
    (0..colspan)
        .map(|c| widths.get(start_col + c).copied().unwrap_or(default_w))
        .sum()
}

fn row_height<W: Write>(
    pdf: &Pdf<W>,
    row: &[TableCell],
    widths: &[f64],
    default_w: f64,
    font_size: f64,
) -> f64 {
    let line_height = font_size * 1.3;
    let mut col_idx = 0;
    let max_lines = row
        .iter()
        .map(|tc| {
            let w = spanned_width(widths, col_idx, tc.colspan, default_w);
            col_idx += tc.colspan;
            match &tc.content {
                Cell::Text(t) => wrap(pdf, t, w, font_size).len(),
                Cell::ImagePath(_) => 1,
                Cell::ImageBase64(_) => 1,
            }
        })
        .max()
        .unwrap_or(1);

    max_lines as f64 * line_height + CELL_PADDING * 2.0
}

impl Table {
    pub fn render<W: Write>(&self, pdf: &mut Pdf<W>) -> std::io::Result<()> {
        let available_full_w = pdf.content_width();

        let resolved_widths: Vec<f64> = self
            .widths
            .iter()
            .map(|w| match w {
                Size::Points(p) => *p,
                Size::Percent(pct) => (pct / 100.0) * available_full_w,
                Size::Flex(_) => (1.0 / self.widths.len() as f64) * available_full_w,
            })
            .collect();

        let default_w = resolved_widths.first().copied().unwrap_or(100.0);

        let draw_row = |pdf: &mut Pdf<W>,
                        row: &[TableCell],
                        h: f64,
                        style: &TableStyle,
                        widths: &[f64],
                        is_header: bool,
                        row_idx: usize|
         -> std::io::Result<()> {
            let (start_x, top_y) = pdf.cursor_pos();
            let bottom_y = top_y - h;
            let line_height = style.font_size * 1.3;

            let mut x = start_x;
            let mut col_idx = 0;

            let total_w: f64 = (0..row.iter().map(|c| c.colspan).sum())
                .map(|i| resolved_widths.get(i).copied().unwrap_or(default_w))
                .sum();

            // Background color (Style BG or Zebra)
            let mut final_bg = style.bg_color.clone();
            if !is_header && final_bg.is_none() {
                if let Some(zc) = &self.zebra_color {
                    if row_idx % 2 == 1 {
                        final_bg = Some(zc.clone());
                    }
                }
            }

            if let Some(bg) = final_bg {
                pdf.set_fill_color(bg)?;
                pdf.fill_rect(start_x, bottom_y, total_w, h)?;
            }

            for tc in row {
                let w = spanned_width(widths, col_idx, tc.colspan, default_w);
                let config = self
                    .column_configs
                    .get(col_idx)
                    .cloned()
                    .unwrap_or_default();
                col_idx += tc.colspan;

                let align = tc.align.unwrap_or(config.align);
                let valign = tc.valign.unwrap_or(config.valign);

                // Borders
                pdf.set_stroke_color(Color::Rgb(0, 0, 0))?;
                match self.border_style {
                    TableBorderStyle::Full => {
                        pdf.rect(x, bottom_y, w, h)?;
                    }
                    TableBorderStyle::Ghost => {
                        pdf.line(x, bottom_y, x + w, bottom_y)?;
                    }
                    TableBorderStyle::HeaderOnly => {
                        if is_header {
                            pdf.line(x, bottom_y, x + w, bottom_y)?;
                        }
                    }
                    TableBorderStyle::None => {}
                }

                match &tc.content {
                    Cell::Text(text) => {
                        let lines = wrap(pdf, text, w, style.font_size);
                        let total_text_h = lines.len() as f64 * line_height;
                        let v_shift = match valign {
                            VAlign::Top => 0.0,
                            VAlign::Center => {
                                (h - 2.0 * CELL_PADDING - total_text_h).max(0.0) / 2.0
                            }
                            VAlign::Bottom => (h - 2.0 * CELL_PADDING - total_text_h).max(0.0),
                        };

                        let baseline_adj = style.font_size * 0.2;
                        let mut current_y =
                            top_y - CELL_PADDING - v_shift - line_height + baseline_adj;

                        if let Some(text_color) = &style.text_color {
                            pdf.set_fill_color(text_color.clone())?;
                        } else {
                            pdf.set_fill_color(Color::Rgb(0, 0, 0))?;
                        }

                        for line in lines {
                            let line_w = measure(pdf, &line, style.font_size);
                            let h_shift = match align {
                                Align::Left => 0.0,
                                Align::Center => (w - 2.0 * CELL_PADDING - line_w).max(0.0) / 2.0,
                                Align::Right => (w - 2.0 * CELL_PADDING - line_w).max(0.0),
                            };

                            let tx = x + CELL_PADDING + h_shift;

                            match pdf.current_font {
                                Some(fid) => {
                                    let encoded = pdf.font_manager.encode_text(fid, &line);
                                    let s = pdf.get_stream();
                                    s.push_str("BT\n");
                                    s.push_str(&format!("/F{} {:.1} Tf\n", fid.0, style.font_size));
                                    s.push_str(&format!("{:.2} {:.2} Td\n", tx, current_y));
                                    s.push_str(&format!("{} Tj\n", encoded));
                                    s.push_str("ET\n");
                                }
                                None => {
                                    let escaped = escape_pdf_str(&line);
                                    let s = pdf.get_stream();
                                    s.push_str("BT\n");
                                    s.push_str(&format!("/FBuiltin {:.1} Tf\n", style.font_size));
                                    s.push_str(&format!("{:.2} {:.2} Td\n", tx, current_y));
                                    s.push_str(&format!("({}) Tj\n", escaped));
                                    s.push_str("ET\n");
                                }
                            }

                            if let Some(url) = &tc.link {
                                pdf.add_link(
                                    (tx, current_y, tx + line_w, current_y + style.font_size),
                                    url,
                                );
                            }

                            current_y -= line_height;
                        }
                    }
                    Cell::ImagePath(path) => {
                        let pad = 3.0;
                        let iw = w - pad * 2.0;
                        let ih = h - pad * 2.0;
                        if iw > 0.0 && ih > 0.0 {
                            pdf.image(path)
                                .position(x + pad, bottom_y + pad)
                                .size(iw, ih)
                                .render()?;
                        }
                    }
                    Cell::ImageBase64(b64) => {
                        let pad = 3.0;
                        let iw = w - pad * 2.0;
                        let ih = h - pad * 2.0;
                        if iw > 0.0 && ih > 0.0 {
                            pdf.image_base64(b64)
                                .position(x + pad, bottom_y + pad)
                                .size(iw, ih)
                                .render()?;
                        }
                    }
                }

                if let Some(url) = &tc.link {
                    pdf.add_link((x, bottom_y, x + w, bottom_y + h), url);
                }

                x += w;
            }

            pdf.advance_cursor(h);
            Ok(())
        };

        if !self.header.is_empty() {
            let h = row_height(
                pdf,
                &self.header,
                &resolved_widths,
                default_w,
                self.header_style.font_size,
            );
            pdf.check_page_break(h)?;
            draw_row(
                pdf,
                &self.header,
                h,
                &self.header_style,
                &resolved_widths,
                true,
                0,
            )?;
        }

        for (i, row) in self.rows.iter().enumerate() {
            let h = row_height(
                pdf,
                row,
                &resolved_widths,
                default_w,
                self.row_style.font_size,
            );
            let pre_y = pdf.cursor_pos().1;
            pdf.check_page_break(h)?;
            let post_y = pdf.cursor_pos().1;

            if post_y > pre_y && self.repeat_header && !self.header.is_empty() {
                let hh = row_height(
                    pdf,
                    &self.header,
                    &resolved_widths,
                    default_w,
                    self.header_style.font_size,
                );
                draw_row(
                    pdf,
                    &self.header,
                    hh,
                    &self.header_style,
                    &resolved_widths,
                    true,
                    0,
                )?;
            }

            draw_row(pdf, row, h, &self.row_style, &resolved_widths, false, i)?;
        }

        Ok(())
    }
}
