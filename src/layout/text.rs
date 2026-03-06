use crate::font::FontId;
use crate::{Align, Pdf};
use std::io::Write;

/// A text block represents a single piece of text with specific styling.
/// It is rendered to the PDF when the block is dropped (at the end of its scope).
pub struct TextBlock<'a, W: Write> {
    pdf: &'a mut Pdf<W>,
    text: String,
    font: Option<FontId>,
    size: f64,
    align: Align,
    max_width: Option<f64>,
    wrap: bool,
    margin_top: f64,
    margin_bottom: f64,
    link: Option<String>,
    color: Option<crate::Color>,
}

impl<'a, W: Write> TextBlock<'a, W> {
    pub fn new(pdf: &'a mut Pdf<W>, text: &str) -> Self {
        let font = pdf.current_font;
        Self {
            pdf,
            text: text.to_string(),
            font,
            size: 12.0,
            align: Align::Left,
            max_width: None,
            wrap: true,
            margin_top: 0.0,
            margin_bottom: 0.0,
            link: None,
            color: None,
        }
    }

    /// Sets the font size.
    pub fn size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }

    /// Sets the text alignment.
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Centers the text horizontally.
    pub fn align_center(self) -> Self {
        self.align(Align::Center)
    }

    /// Aligns the text to the right.
    pub fn align_right(self) -> Self {
        self.align(Align::Right)
    }

    /// Sets the maximum width for the text block.
    pub fn max_width(mut self, w: f64) -> Self {
        self.max_width = Some(w);
        self
    }

    /// Enables or disables word-wrapping.
    pub fn wrap(mut self, w: bool) -> Self {
        self.wrap = w;
        self
    }

    /// Adds a margin at the top of the text block.
    pub fn margin_top(mut self, m: f64) -> Self {
        self.margin_top = m;
        self
    }

    /// Adds a margin at the bottom of the text block.
    pub fn margin_bottom(mut self, m: f64) -> Self {
        self.margin_bottom = m;
        self
    }

    /// Sets the font to be used from the registered fonts.
    pub fn font(mut self, name: &str) -> Self {
        self.font = self.pdf.font_manager.get_font_id(name);
        self
    }

    /// Adds a clickable hyperlink to the text.
    pub fn link(mut self, url: &str) -> Self {
        self.link = Some(url.to_string());
        self
    }

    /// Sets the text color.
    pub fn color(mut self, color: crate::Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl<'a, W: Write> Drop for TextBlock<'a, W> {
    fn drop(&mut self) {
        let _ = self.pdf.ensure_page_pub();

        let margin = self.pdf.margin_pub();
        let available = self.max_width.unwrap_or(self.pdf.content_width());

        if self.margin_top > 0.0 {
            self.pdf.advance_cursor(self.margin_top);
        }

        match self.font {
            Some(font_id) => {
                let (ascent, descent) =
                    self.pdf.font_manager.get_ascent_descent(font_id, self.size);
                let line_h = ascent - descent;

                let lines = word_wrap_ttf(
                    &self.pdf.font_manager,
                    font_id,
                    &self.text,
                    self.size,
                    available,
                    self.wrap,
                );

                for (i, line) in lines.iter().enumerate() {
                    let _ = self.pdf.check_page_break(line_h);
                    let (x, y) = self.pdf.cursor_pos();

                    let text_w = self.pdf.font_manager.string_width(font_id, line, self.size);
                    let x_off = x_offset(self.align, x, margin, available, text_w);

                    let baseline = y - ascent;

                    if let Some(c) = &self.color {
                        let _ = self.pdf.set_fill_color(c.clone());
                    }

                    let encoded = self.pdf.font_manager.encode_text(font_id, line);
                    let s = self.pdf.get_stream();
                    s.push_str("BT\n");
                    s.push_str(&format!("/F{} {:.1} Tf\n", font_id.0, self.size));
                    s.push_str(&format!("{:.2} {:.2} Td\n", x_off, baseline));
                    s.push_str(&format!("{} Tj\n", encoded));
                    s.push_str("ET\n");

                    if self.color.is_some() {
                        let _ = self.pdf.set_fill_color(crate::Color::Rgb(0, 0, 0));
                    }

                    if let Some(url) = &self.link {
                        self.pdf.add_link(
                            (x_off, baseline + descent, x_off + text_w, baseline + ascent),
                            url,
                        );
                    }

                    self.pdf.advance_cursor(line_h * 1.3);

                    if i == lines.len() - 1 && self.margin_bottom > 0.0 {
                        self.pdf.advance_cursor(self.margin_bottom);
                    }
                }
            }

            None => {
                let ascent = self.size * 0.8;
                let descent = -self.size * 0.2;
                let line_h = ascent - descent;
                let char_w = self.size * 0.52;

                let lines = word_wrap_helvetica(&self.text, available, char_w, self.wrap);

                for (i, line) in lines.iter().enumerate() {
                    let _ = self.pdf.check_page_break(line_h);
                    let (x, y) = self.pdf.cursor_pos();

                    let text_w = line.len() as f64 * char_w;
                    let x_off = x_offset(self.align, x, margin, available, text_w);
                    let baseline = y - ascent;

                    if let Some(c) = &self.color {
                        let _ = self.pdf.set_fill_color(c.clone());
                    }

                    let escaped = escape_pdf_str(line);
                    let s = self.pdf.get_stream();
                    s.push_str("BT\n");
                    s.push_str(&format!("/FBuiltin {:.1} Tf\n", self.size));
                    s.push_str(&format!("{:.2} {:.2} Td\n", x_off, baseline));
                    s.push_str(&format!("({}) Tj\n", escaped));
                    s.push_str("ET\n");

                    if self.color.is_some() {
                        let _ = self.pdf.set_fill_color(crate::Color::Rgb(0, 0, 0));
                    }

                    if let Some(url) = &self.link {
                        self.pdf.add_link(
                            (x_off, baseline + descent, x_off + text_w, baseline + ascent),
                            url,
                        );
                    }

                    self.pdf.advance_cursor(line_h * 1.3);

                    if i == lines.len() - 1 && self.margin_bottom > 0.0 {
                        self.pdf.advance_cursor(self.margin_bottom);
                    }
                }
            }
        }
    }
}

fn x_offset(align: Align, cursor_x: f64, margin: f64, available: f64, text_w: f64) -> f64 {
    match align {
        Align::Left => cursor_x,
        Align::Center => margin + (available - text_w) / 2.0,
        Align::Right => (margin + available - text_w).max(margin),
    }
}

fn word_wrap_ttf(
    fm: &crate::font::FontManager,
    font_id: crate::font::FontId,
    text: &str,
    size: f64,
    available: f64,
    do_wrap: bool,
) -> Vec<String> {
    if !do_wrap {
        return vec![text.to_string()];
    }
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let candidate = if current.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current, word)
        };
        if fm.string_width(font_id, &candidate, size) > available && !current.is_empty() {
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

fn word_wrap_helvetica(text: &str, available: f64, char_w: f64, do_wrap: bool) -> Vec<String> {
    if !do_wrap {
        return vec![text.to_string()];
    }
    let max_chars = (available / char_w).floor() as usize;
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let candidate = if current.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current, word)
        };
        if candidate.len() > max_chars && !current.is_empty() {
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

pub fn escape_pdf_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        match c {
            '(' => out.push_str("\\("),
            ')' => out.push_str("\\)"),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            c => out.push(c),
        }
    }
    out
}
