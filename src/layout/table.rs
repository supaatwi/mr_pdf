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
    pub rowspan: usize,
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
            rowspan: 1,
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
            rowspan: 1,
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
            rowspan: 1,
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

    /// Sets how many rows this cell should span.
    pub fn with_rowspan(mut self, n: usize) -> Self {
        self.rowspan = n.max(1);
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
            rowspan: 1,
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

impl Default for RowBuilder {
    fn default() -> Self {
        Self::new()
    }
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

    /// Sets the rowspan for the most recently added cell.
    pub fn rowspan(&mut self, n: usize) -> &mut Self {
        if let Some(last) = self.cells.last_mut() {
            last.rowspan = n.max(1);
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
    header: Vec<Vec<TableCell>>,
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

    /// Sets the table header cells (replaces existing header rows).
    pub fn header<I, T>(&mut self, header: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<TableCell>,
    {
        self.header = vec![header.into_iter().map(Into::into).collect()];
        self
    }

    /// Adds a row to the table header.
    pub fn header_row<I, T>(&mut self, header: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<TableCell>,
    {
        self.header.push(header.into_iter().map(Into::into).collect());
        self
    }

    /// Adds a header row using the RowBuilder closure.
    pub fn header_row_builder<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut RowBuilder),
    {
        let mut builder = RowBuilder::new();
        f(&mut builder);
        self.header.push(builder.cells);
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

    /// Initializes a StreamingTable that writes directly to the PDF without buffering all rows.
    pub fn start<'a, W: Write>(self, pdf: &'a mut Pdf<W>) -> std::io::Result<StreamingTable<'a, W>> {
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

        let num_cols = resolved_widths.len();
        let default_w = resolved_widths.first().copied().unwrap_or(100.0);
        let total_table_w: f64 = resolved_widths.iter().copied().sum();

        let mut st = StreamingTable {
            _widths: self.widths, // Kept if needed later
            resolved_widths,
            num_cols,
            default_w,
            total_table_w,
            header: self.header,
            repeat_header: self.repeat_header,
            column_configs: self.column_configs,
            header_style: self.header_style,
            row_style: self.row_style,
            border_style: self.border_style,
            zebra_color: self.zebra_color,
            pdf,
            row_count: 0,
            top_y: 0.0,
        };

        // Initialize and draw the header on the very first page
        st.draw_header()?;
        st.top_y = st.pdf.cursor_pos().1;

        Ok(st)
    }
}

pub struct StreamingTable<'a, W: Write> {
    _widths: Vec<Size>,
    resolved_widths: Vec<f64>,
    num_cols: usize,
    default_w: f64,
    total_table_w: f64,
    header: Vec<Vec<TableCell>>,
    repeat_header: bool,
    column_configs: Vec<ColumnConfig>,
    header_style: TableStyle,
    row_style: TableStyle,
    border_style: TableBorderStyle,
    zebra_color: Option<Color>,

    pdf: &'a mut Pdf<W>,
    row_count: usize,
    top_y: f64,
}

impl<'a, W: Write> StreamingTable<'a, W> {
    /// Adds a single row using a builder closure and immediately renders it.
    pub fn row<F>(&mut self, f: F) -> std::io::Result<()>
    where
        F: FnOnce(&mut RowBuilder),
    {
        let mut row_builder = RowBuilder::new();
        f(&mut row_builder);
        self.add_row(row_builder.cells)
    }

    /// Adds a single row manually configured.
    pub fn add_row(&mut self, row: Vec<TableCell>) -> std::io::Result<()> {
        // Evaluate the height needed for this specific row.
        // NOTE: StreamingTable processes one row at a time. Multi-row body rowspans are not supported within StreamingTable.
        let rows = [row];
        let (placements, heights) = process_rows(
            self.pdf,
            &rows,
            self.num_cols,
            &self.resolved_widths,
            self.default_w,
            self.row_style.font_size,
        );
        let h = heights.first().copied().unwrap_or(0.0);

        self.pdf.check_page_break(h)?;
        let pos_y = self.pdf.cursor_pos().1;

        // Header repetition on new pages
        if pos_y > self.top_y && self.repeat_header && !self.header.is_empty() {
            self.draw_header()?;
            self.top_y = self.pdf.cursor_pos().1;
        }

        let start_x = self.pdf.cursor_pos().0;
        let row_bottom = self.top_y - h;

        // Background / Zebra coloring
        let mut final_bg = self.row_style.bg_color;
        if final_bg.is_none()
            && let Some(zc) = &self.zebra_color
                && self.row_count % 2 == 1 {
                    final_bg = Some(*zc);
                }
        if let Some(bg) = final_bg {
            self.pdf.set_fill_color(bg)?;
            self.pdf.fill_rect(start_x, row_bottom, self.total_table_w, h)?;
        }

        for p in &placements {
            let x = start_x
                + (0..p.start_col)
                    .map(|c| self.resolved_widths.get(c).copied().unwrap_or(self.default_w))
                    .sum::<f64>();
            let bottom = self.top_y - h;
            let w = spanned_width(&self.resolved_widths, p.start_col, p.span_w, self.default_w);

            let config = self.column_configs.get(p.start_col).cloned().unwrap_or_default();

            draw_cell_content(
                self.pdf,
                p.cell,
                x,
                bottom,
                w,
                h,
                &self.row_style,
                &config,
                false,
                self.border_style,
            )?;
        }

        self.pdf.advance_cursor(h);
        self.top_y = self.pdf.cursor_pos().1;
        self.row_count += 1;
        
        Ok(())
    }

    fn draw_header(&mut self) -> std::io::Result<()> {
        if self.header.is_empty() {
            return Ok(());
        }

        let (header_placements, header_heights) = process_rows(
            self.pdf,
            &self.header,
            self.num_cols,
            &self.resolved_widths,
            self.default_w,
            self.header_style.font_size,
        );

        if header_heights.is_empty() {
            return Ok(());
        }

        let total_h: f64 = header_heights.iter().sum();
        self.pdf.check_page_break(total_h)?;

        let (start_x, top_y) = self.pdf.cursor_pos();
        let current_y = top_y;

        if let Some(bg) = &self.header_style.bg_color {
            self.pdf.set_fill_color(*bg)?;
            self.pdf.fill_rect(start_x, current_y - total_h, self.total_table_w, total_h)?;
        }

        for p in &header_placements {
            let x = start_x
                + (0..p.start_col)
                    .map(|c| self.resolved_widths.get(c).copied().unwrap_or(self.default_w))
                    .sum::<f64>();
            let cell_top_y = top_y
                - (0..p.start_row)
                    .map(|r| header_heights[r])
                    .sum::<f64>();
            let cell_h = (0..p.span_h)
                .map(|r| header_heights[p.start_row + r])
                .sum::<f64>();
            let bottom = cell_top_y - cell_h;
            let w = spanned_width(&self.resolved_widths, p.start_col, p.span_w, self.default_w);

            let config = self.column_configs.get(p.start_col).cloned().unwrap_or_default();

            draw_cell_content(
                self.pdf,
                p.cell,
                x,
                bottom,
                w,
                cell_h,
                &self.header_style,
                &config,
                true,
                self.border_style,
            )?;
        }

        self.pdf.advance_cursor(total_h);
        Ok(())
    }
}

pub struct Table {
    widths: Vec<Size>,
    header: Vec<Vec<TableCell>>,
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

    for explicit_line in text.split('\n') {
        let mut current = String::new();
        let words: Vec<&str> = explicit_line.split_whitespace().collect();

        if words.is_empty() {
            lines.push(String::new());
            continue;
        }

        for word in words {
            // First check if the word ITSELF is too long to fit on a line
            let mut word_parts = Vec::new();
            if measure(pdf, word, font_size) > available {
                // Try to break at '/' or '-'
                let mut p_curr = String::new();
                for ch in word.chars() {
                    let cand = format!("{}{}", p_curr, ch);
                    if measure(pdf, &cand, font_size) > available && !p_curr.is_empty() {
                        // Overflow! See if we can break here
                        word_parts.push(p_curr);
                        p_curr = ch.to_string();
                    } else if ch == '/' || ch == '-' {
                        word_parts.push(cand);
                        p_curr = String::new();
                    } else {
                        p_curr = cand;
                    }
                }
                if !p_curr.is_empty() {
                    word_parts.push(p_curr);
                }
            } else {
                word_parts.push(word.to_string());
            }

            for (idx, part) in word_parts.iter().enumerate() {
                let candidate = if current.is_empty() {
                    part.clone()
                } else if idx == 0 {
                    // It's the first part of a NEW word, so it needs a space
                    format!("{} {}", current, part)
                } else {
                    // It's a subsequent part of the SAME broken word, NO space
                    format!("{}{}", current, part)
                };
                
                if measure(pdf, &candidate, font_size) > available && !current.is_empty() {
                    if idx == 0 {
                        // Overflow on a new word, push the old line
                        lines.push(current);
                        current = part.clone();
                    } else {
                        // Overflow while building the same broken word,
                        // this means the piece before the '-' or '/' plus `current` 
                        // fits, but adding this next piece exceeds the limit.
                        // So push what we have.
                        lines.push(current);
                        current = part.clone();
                    }
                } else {
                    current = candidate;
                }
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
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

struct GridCell<'a> {
    cell: &'a TableCell,
    start_row: usize,
    start_col: usize,
    span_w: usize,
    span_h: usize,
}

fn process_rows<'a, W: Write>(
    pdf: &Pdf<W>,
    rows: &'a [Vec<TableCell>],
    num_cols: usize,
    resolved_widths: &[f64],
    default_w: f64,
    font_size: f64,
) -> (Vec<GridCell<'a>>, Vec<f64>) {
    let mut occupied = vec![vec![false; num_cols]; rows.len()];
    let mut placements = Vec::new();

    for (r, row) in rows.iter().enumerate() {
        let mut c = 0;
        for tc in row {
            while c < num_cols && occupied[r][c] {
                c += 1;
            }
            if c >= num_cols {
                break;
            }

            let span_w = tc.colspan.min(num_cols - c);
            let span_h = tc.rowspan;

            if r + span_h > occupied.len() {
                occupied.resize(r + span_h, vec![false; num_cols]);
            }

            placements.push(GridCell {
                cell: tc,
                start_row: r,
                start_col: c,
                span_w,
                span_h,
            });

            for rr in 0..span_h {
                for cc in 0..span_w {
                    occupied[r + rr][c + cc] = true;
                }
            }
        }
    }

    let min_row_h = font_size * 1.3 + CELL_PADDING * 2.0;
    let mut row_heights = vec![min_row_h; occupied.len()];

    for p in &placements {
        if p.span_h == 1 {
            let w = spanned_width(resolved_widths, p.start_col, p.span_w, default_w);
            let lines = match &p.cell.content {
                Cell::Text(t) => wrap(pdf, t, w, font_size).len(),
                _ => 1,
            };
            let h = lines as f64 * (font_size * 1.3) + CELL_PADDING * 2.0;
            if h > row_heights[p.start_row] {
                row_heights[p.start_row] = h;
            }
        }
    }

    for p in &placements {
        if p.span_h > 1 {
            let w = spanned_width(resolved_widths, p.start_col, p.span_w, default_w);
            let lines = match &p.cell.content {
                Cell::Text(t) => wrap(pdf, t, w, font_size).len(),
                _ => 1,
            };
            let needed_h = lines as f64 * (font_size * 1.3) + CELL_PADDING * 2.0;
            let current_h: f64 = (0..p.span_h).map(|i| row_heights[p.start_row + i]).sum();

            if needed_h > current_h {
                let extra_per_row = (needed_h - current_h) / (p.span_h as f64);
                for i in 0..p.span_h {
                    row_heights[p.start_row + i] += extra_per_row;
                }
            }
        }
    }

    (placements, row_heights)
}

fn draw_cell_content<W: Write>(
    pdf: &mut Pdf<W>,
    tc: &TableCell,
    x: f64,
    bottom_y: f64,
    w: f64,
    h: f64,
    style: &TableStyle,
    config: &ColumnConfig,
    is_header: bool,
    border_style: TableBorderStyle,
) -> std::io::Result<()> {
    let top_y = bottom_y + h;
    let line_height = style.font_size * 1.3;

    let align = tc.align.unwrap_or(config.align);
    let valign = tc.valign.unwrap_or(config.valign);

    pdf.set_stroke_color(Color::Rgb(0, 0, 0))?;
    match border_style {
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
                VAlign::Center => (h - 2.0 * CELL_PADDING - total_text_h).max(0.0) / 2.0,
                VAlign::Bottom => (h - 2.0 * CELL_PADDING - total_text_h).max(0.0),
            };

            let baseline_adj = style.font_size * 0.2;
            let mut current_y = top_y - CELL_PADDING - v_shift - line_height + baseline_adj;

            if let Some(text_color) = &style.text_color {
                pdf.set_fill_color(*text_color)?;
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
                    pdf.add_link((tx, current_y, tx + line_w, current_y + style.font_size), url);
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

    Ok(())
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

        let num_cols = resolved_widths.len();
        let default_w = resolved_widths.first().copied().unwrap_or(100.0);
        let total_table_w: f64 = resolved_widths.iter().copied().sum();

        let (header_placements, header_heights) = process_rows(
            pdf,
            &self.header,
            num_cols,
            &resolved_widths,
            default_w,
            self.header_style.font_size,
        );

        let (body_placements, body_heights) = process_rows(
            pdf,
            &self.rows,
            num_cols,
            &resolved_widths,
            default_w,
            self.row_style.font_size,
        );

        let draw_header = |pdf: &mut Pdf<W>| -> std::io::Result<()> {
            if header_heights.is_empty() {
                return Ok(());
            }

            let total_h: f64 = header_heights.iter().sum();
            pdf.check_page_break(total_h)?;

            let (start_x, top_y) = pdf.cursor_pos();
            let current_y = top_y;

            if let Some(bg) = &self.header_style.bg_color {
                pdf.set_fill_color(*bg)?;
                pdf.fill_rect(start_x, current_y - total_h, total_table_w, total_h)?;
            }

            for p in &header_placements {
                let x = start_x
                    + (0..p.start_col)
                        .map(|c| resolved_widths.get(c).copied().unwrap_or(default_w))
                        .sum::<f64>();
                let cell_top_y = top_y
                    - (0..p.start_row)
                        .map(|r| header_heights[r])
                        .sum::<f64>();
                let cell_h = (0..p.span_h)
                    .map(|r| header_heights[p.start_row + r])
                    .sum::<f64>();
                let bottom = cell_top_y - cell_h;
                let w = spanned_width(&resolved_widths, p.start_col, p.span_w, default_w);

                let config = self.column_configs.get(p.start_col).cloned().unwrap_or_default();

                draw_cell_content(
                    pdf,
                    p.cell,
                    x,
                    bottom,
                    w,
                    cell_h,
                    &self.header_style,
                    &config,
                    true,
                    self.border_style,
                )?;
            }

            pdf.advance_cursor(total_h);
            Ok(())
        };

        if !self.header.is_empty() {
            draw_header(pdf)?;
        }

        let mut top_y = pdf.cursor_pos().1;

        for r in 0..body_heights.len() {
            let h = body_heights[r];
            pdf.check_page_break(h)?;
            
            let pos_y = pdf.cursor_pos().1;
            if pos_y > top_y && self.repeat_header && !self.header.is_empty() {
                draw_header(pdf)?;
                top_y = pdf.cursor_pos().1;
            }

            let start_x = pdf.cursor_pos().0;
            let row_bottom = top_y - h;

            let mut final_bg = self.row_style.bg_color;
            if final_bg.is_none()
                && let Some(zc) = &self.zebra_color
                    && r % 2 == 1 {
                        final_bg = Some(*zc);
                    }

            if let Some(bg) = final_bg {
                pdf.set_fill_color(bg)?;
                pdf.fill_rect(start_x, row_bottom, total_table_w, h)?;
            }

            for p in &body_placements {
                if p.start_row == r {
                    let x = start_x
                        + (0..p.start_col)
                            .map(|c| resolved_widths.get(c).copied().unwrap_or(default_w))
                            .sum::<f64>();
                    let cell_h = (0..p.span_h)
                        .map(|ri| body_heights[p.start_row + ri])
                        .sum::<f64>();
                    let bottom = top_y - cell_h;
                    let w = spanned_width(&resolved_widths, p.start_col, p.span_w, default_w);

                    let config = self.column_configs.get(p.start_col).cloned().unwrap_or_default();

                    draw_cell_content(
                        pdf,
                        p.cell,
                        x,
                        bottom,
                        w,
                        cell_h,
                        &self.row_style,
                        &config,
                        false,
                        self.border_style,
                    )?;
                }
            }

            top_y -= h;
            pdf.advance_cursor(h);
        }

        Ok(())
    }
}
