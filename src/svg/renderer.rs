use crate::{Color, Pdf};
use roxmltree::Document;
use std::fs;
use std::io::Write;

pub struct SvgRenderer;

impl SvgRenderer {
    pub fn render<W: Write>(
        pdf: &mut Pdf<W>,
        path: &str,
        w: Option<f64>,
        h: Option<f64>,
    ) -> std::io::Result<()> {
        let content = fs::read_to_string(path)?;
        let doc = Document::parse(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let svg_root = doc.root_element();
        let view_box = svg_root.attribute("viewBox").unwrap_or("0 0 100 100");
        let coords: Vec<f64> = view_box
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse().ok())
            .collect();

        let (min_x, min_y, vb_w, vb_h) = if coords.len() >= 4 {
            (coords[0], coords[1], coords[2], coords[3])
        } else {
            (0.0, 0.0, 100.0, 100.0)
        };

        let (display_w, display_h) = match (w, h) {
            (Some(ww), Some(hh)) => (ww, hh),
            (Some(ww), None) => (ww, vb_h * (ww / vb_w)),
            (None, Some(hh)) => (vb_w * (hh / vb_h), hh),
            (None, None) => (100.0, vb_h * (100.0 / vb_w)),
        };

        let scale = display_w / vb_w;
        let (start_x, start_y) = pdf.cursor_pos();
        pdf.advance_cursor(display_h + 10.0);
        let base_y = start_y;

        for node in doc.descendants() {
            if node.is_element() {
                let fill = node.attribute("fill").unwrap_or("#000000");
                if fill == "none" {
                    continue;
                }
                let color = parse_color(fill);

                match node.tag_name().name() {
                    "rect" => {
                        let rx = (node.attribute("x").unwrap_or("0").parse().unwrap_or(0.0)
                            - min_x)
                            * scale;
                        let ry = (node.attribute("y").unwrap_or("0").parse().unwrap_or(0.0)
                            - min_y)
                            * scale;
                        let rw = node
                            .attribute("width")
                            .unwrap_or("0")
                            .parse()
                            .unwrap_or(0.0)
                            * scale;
                        let rh = node
                            .attribute("height")
                            .unwrap_or("0")
                            .parse()
                            .unwrap_or(0.0)
                            * scale;
                        pdf.set_fill_color(color)?;
                        pdf.fill_rect(start_x + rx, base_y - ry - rh, rw, rh)?;
                    }
                    "circle" => {
                        let cx = (node.attribute("cx").unwrap_or("0").parse().unwrap_or(0.0)
                            - min_x)
                            * scale;
                        let cy = (node.attribute("cy").unwrap_or("0").parse().unwrap_or(0.0)
                            - min_y)
                            * scale;
                        let r = node.attribute("r").unwrap_or("0").parse().unwrap_or(0.0) * scale;
                        pdf.set_fill_color(color)?;
                        pdf.fill_circle(start_x + cx, base_y - cy, r)?;
                    }
                    "path" => {
                        if let Some(d) = node.attribute("d") {
                            pdf.set_fill_color(color)?;
                            render_path(pdf, d, start_x, base_y, scale, min_x, min_y)?;
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

fn parse_color(hex: &str) -> Color {
    if hex.starts_with('#') {
        let h = hex.trim_start_matches('#');
        if h.len() == 6 {
            let r = u8::from_str_radix(&h[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&h[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&h[4..6], 16).unwrap_or(0);
            return Color::Rgb(r, g, b);
        } else if h.len() == 3 {
            let r = u8::from_str_radix(&h[0..1], 16).unwrap_or(0) * 17;
            let g = u8::from_str_radix(&h[1..2], 16).unwrap_or(0) * 17;
            let b = u8::from_str_radix(&h[2..3], 16).unwrap_or(0) * 17;
            return Color::Rgb(r, g, b);
        }
    }
    Color::Rgb(0, 0, 0)
}

fn render_path<W: Write>(
    pdf: &mut Pdf<W>,
    d: &str,
    offset_x: f64,
    offset_y: f64,
    scale: f64,
    min_x: f64,
    min_y: f64,
) -> std::io::Result<()> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for c in d.chars() {
        if c.is_alphabetic() {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
            tokens.push(c.to_string());
        } else if c.is_whitespace() || c == ',' {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
        } else if c == '-' {
            if !current.is_empty() {
                tokens.push(current.clone());
            }
            current = "-".to_string();
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }

    let stream = pdf.get_stream();
    let mut i = 0;
    let mut last_x = 0.0;
    let mut last_y = 0.0;
    let mut subpath_start_x = 0.0;
    let mut subpath_start_y = 0.0;
    let mut current_cmd = ' ';
    let mut has_path = false;

    while i < tokens.len() {
        let token = &tokens[i];
        if token.len() == 1 && token.chars().next().unwrap().is_alphabetic() {
            current_cmd = token.chars().next().unwrap();
            i += 1;
        }

        match current_cmd {
            'M' | 'm' => {
                let is_rel = current_cmd == 'm';
                let x = tokens[i].parse::<f64>().unwrap_or(0.0);
                let y = tokens[i + 1].parse::<f64>().unwrap_or(0.0);
                let (nx, ny) = if is_rel {
                    (last_x + x, last_y + y)
                } else {
                    (x, y)
                };

                let px = (nx - min_x) * scale;
                let py = (ny - min_y) * scale;
                stream.push_str(&format!("{:.2} {:.2} m\n", offset_x + px, offset_y - py));

                last_x = nx;
                last_y = ny;
                subpath_start_x = nx;
                subpath_start_y = ny;
                i += 2;
                has_path = true;
                current_cmd = if is_rel { 'l' } else { 'L' };
            }
            'L' | 'l' => {
                let is_rel = current_cmd == 'l';
                let x = tokens[i].parse::<f64>().unwrap_or(0.0);
                let y = tokens[i + 1].parse::<f64>().unwrap_or(0.0);
                let (nx, ny) = if is_rel {
                    (last_x + x, last_y + y)
                } else {
                    (x, y)
                };

                let px = (nx - min_x) * scale;
                let py = (ny - min_y) * scale;
                stream.push_str(&format!("{:.2} {:.2} l\n", offset_x + px, offset_y - py));
                last_x = nx;
                last_y = ny;
                i += 2;
            }
            'H' | 'h' => {
                let is_rel = current_cmd == 'h';
                let x = tokens[i].parse::<f64>().unwrap_or(0.0);
                let nx = if is_rel { last_x + x } else { x };
                let px = (nx - min_x) * scale;
                let py = (last_y - min_y) * scale;
                stream.push_str(&format!("{:.2} {:.2} l\n", offset_x + px, offset_y - py));
                last_x = nx;
                i += 1;
            }
            'V' | 'v' => {
                let is_rel = current_cmd == 'v';
                let y = tokens[i].parse::<f64>().unwrap_or(0.0);
                let ny = if is_rel { last_y + y } else { y };
                let px = (last_x - min_x) * scale;
                let py = (ny - min_y) * scale;
                stream.push_str(&format!("{:.2} {:.2} l\n", offset_x + px, offset_y - py));
                last_y = ny;
                i += 1;
            }
            'C' | 'c' => {
                let is_rel = current_cmd == 'c';
                let c1x = tokens[i].parse::<f64>().unwrap_or(0.0);
                let c1y = tokens[i + 1].parse::<f64>().unwrap_or(0.0);
                let c2x = tokens[i + 2].parse::<f64>().unwrap_or(0.0);
                let c2y = tokens[i + 3].parse::<f64>().unwrap_or(0.0);
                let x = tokens[i + 4].parse::<f64>().unwrap_or(0.0);
                let y = tokens[i + 5].parse::<f64>().unwrap_or(0.0);

                let (f1x, f1y, f2x, f2y, fx, fy) = if is_rel {
                    (
                        last_x + c1x,
                        last_y + c1y,
                        last_x + c2x,
                        last_y + c2y,
                        last_x + x,
                        last_y + y,
                    )
                } else {
                    (c1x, c1y, c2x, c2y, x, y)
                };

                let p1x = (f1x - min_x) * scale;
                let p1y = (f1y - min_y) * scale;
                let p2x = (f2x - min_x) * scale;
                let p2y = (f2y - min_y) * scale;
                let px = (fx - min_x) * scale;
                let py = (fy - min_y) * scale;

                stream.push_str(&format!(
                    "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                    offset_x + p1x,
                    offset_y - p1y,
                    offset_x + p2x,
                    offset_y - p2y,
                    offset_x + px,
                    offset_y - py
                ));
                last_x = fx;
                last_y = fy;
                i += 6;
            }
            'S' | 's' | 'Q' | 'q' | 'T' | 't' | 'A' | 'a' => {
                // Ignore for now but skip tokens to prevent infinite loop or corruption
                let count = match current_cmd.to_ascii_uppercase() {
                    'S' | 'Q' => 4,
                    'T' => 2,
                    'A' => 7,
                    _ => 0,
                };
                i += count;
            }
            'Z' | 'z' => {
                stream.push_str("h\n");
                last_x = subpath_start_x;
                last_y = subpath_start_y;
                i += 0;
                current_cmd = ' ';
            }
            _ => {
                i += 1;
            }
        }
    }
    if has_path {
        stream.push_str("f\n");
    }
    Ok(())
}
