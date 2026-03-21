use crate::{Align, Color, Pdf, TableBuilder, VAlign};
use crate::layout::table::{TableCell, Cell};
use std::collections::{HashMap, VecDeque};
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

/// A high-performance multiplexed table streamer that buffers rows to disk.
/// This allows interleaving rows for different tables while keeping RAM usage constant.
pub struct MultiplexedTable {
    temp_dir: PathBuf,
    keys: VecDeque<String>,
    writers: HashMap<String, BufWriter<File>>,
    render_order: Vec<String>,
    table_builder: TableBuilder,
    key_builders: HashMap<String, TableBuilder>,
}

impl MultiplexedTable {
    pub fn new(builder: TableBuilder) -> io::Result<Self> {
        let temp_dir = std::env::temp_dir().join(format!("mr_pdf_multi_{}", uuid_simple()));
        fs::create_dir_all(&temp_dir)?;

        Ok(Self {
            temp_dir,
            keys: VecDeque::new(),
            writers: HashMap::new(),
            render_order: Vec::new(),
            table_builder: builder,
            key_builders: HashMap::new(),
        })
    }

    /// Sets a specific TableBuilder for a given key.
    /// This allows different tables to have different headers, widths, and styles.
    pub fn set_builder(&mut self, key: &str, builder: TableBuilder) {
        self.key_builders.insert(key.to_string(), builder);
    }

    /// Shortcut to set only the header for a specific key.
    /// It clones the default builder and updates its header.
    pub fn header<I, T>(&mut self, key: &str, header: I)
    where
        I: IntoIterator<Item = T>,
        T: Into<TableCell>,
    {
        let mut builder = self.key_builders.get(key).cloned().unwrap_or_else(|| self.table_builder.clone());
        builder.header(header);
        self.key_builders.insert(key.to_string(), builder);
    }

    /// Appends a row to a specific table identified by key.
    pub fn insert<T>(&mut self, key: &str, row: Vec<T>) -> io::Result<()> 
    where T: Into<TableCell> {
        let key_str = key.to_string();
        if !self.writers.contains_key(&key_str) {
            let file_path = self.temp_dir.join(&key_str);
            let file = File::create(file_path)?;
            self.writers.insert(key_str.clone(), BufWriter::new(file));
            self.keys.push_back(key_str.clone());
        }

        let writer = self.writers.get_mut(&key_str).unwrap();
        let cells: Vec<TableCell> = row.into_iter().map(Into::into).collect();
        serialize_row(writer, &cells)?;
        Ok(())
    }

    /// Sets the rendering order for the tables.
    pub fn order(&mut self, keys: Vec<&str>) {
        self.render_order = keys.into_iter().map(|s| s.to_string()).collect();
    }

    /// Finalizes and renders all tables to the PDF in the specified order.
    pub fn render<W: Write>(mut self, pdf: &mut Pdf<W>) -> io::Result<()> {
        // Close all writers
        for (_, mut writer) in self.writers.drain() {
            let _ = writer.flush();
        }

        // Determine final order: Specified order first, then remaining keys in appearance order
        let mut final_order = self.render_order.clone();
        for key in &self.keys {
            if !final_order.contains(key) {
                final_order.push(key.clone());
            }
        }

        for key in final_order {
            let file_path = self.temp_dir.join(&key);
            if !file_path.exists() {
                continue;
            }

            // Start a new streaming table for this key
            let builder = self.key_builders.get(&key).unwrap_or(&self.table_builder);
            let mut streaming = builder.clone().start(pdf)?;
            
            let file = File::open(file_path)?;
            let mut reader = BufReader::new(file);

            while let Some(row) = deserialize_row(&mut reader)? {
                streaming.add_row(row)?;
            }
        }

        // Cleanup
        let _ = fs::remove_dir_all(&self.temp_dir);
        Ok(())
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("{:x}", start)
}

fn serialize_row<W: Write>(w: &mut W, row: &[TableCell]) -> io::Result<()> {
    w.write_all(&(row.len() as u32).to_le_bytes())?;
    for cell in row {
        serialize_cell(w, cell)?;
    }
    Ok(())
}

fn deserialize_row<R: Read>(r: &mut R) -> io::Result<Option<Vec<TableCell>>> {
    let mut len_buf = [0u8; 4];
    if r.read_exact(&mut len_buf).is_err() {
        return Ok(None);
    }
    let len = u32::from_le_bytes(len_buf) as usize;
    let mut row = Vec::with_capacity(len);
    for _ in 0..len {
        row.push(deserialize_cell(r)?);
    }
    Ok(Some(row))
}

fn serialize_cell<W: Write>(w: &mut W, cell: &TableCell) -> io::Result<()> {
    // Content type
    let type_byte = match &cell.content {
        Cell::Text(_) => 0u8,
        Cell::ImagePath(_) => 1u8,
        Cell::ImageBase64(_) => 2u8,
        #[cfg(feature = "qrcode")]
        Cell::QrCode(_) => 3u8,
    };
    w.write_all(&[type_byte])?;

    // Content string
    let content_str = match &cell.content {
        Cell::Text(s) => s,
        Cell::ImagePath(s) => s,
        Cell::ImageBase64(s) => s,
        #[cfg(feature = "qrcode")]
        Cell::QrCode(s) => s,
    };
    let bytes = content_str.as_bytes();
    w.write_all(&(bytes.len() as u32).to_le_bytes())?;
    w.write_all(bytes)?;

    // Spans
    w.write_all(&(cell.colspan as u32).to_le_bytes())?;
    w.write_all(&(cell.rowspan as u32).to_le_bytes())?;

    // Alignments
    w.write_all(&[match cell.align {
        None => 0,
        Some(Align::Left) => 1,
        Some(Align::Center) => 2,
        Some(Align::Right) => 3,
    }])?;
    w.write_all(&[match cell.valign {
        None => 0,
        Some(VAlign::Top) => 1,
        Some(VAlign::Center) => 2,
        Some(VAlign::Bottom) => 3,
    }])?;

    // Link
    match &cell.link {
        None => w.write_all(&[0])?,
        Some(l) => {
            w.write_all(&[1])?;
            let b = l.as_bytes();
            w.write_all(&(b.len() as u32).to_le_bytes())?;
            w.write_all(b)?;
        }
    }

    // BG Color
    match cell.bg_color {
        None => { w.write_all(&[0])?; },
        Some(Color::Rgb(r, g, b)) => {
            w.write_all(&[1, r, g, b])?;
        }
    }

    // Text Color
    match cell.text_color {
        None => { w.write_all(&[0])?; },
        Some(Color::Rgb(r, g, b)) => {
            w.write_all(&[1, r, g, b])?;
        }
    }

    // Font Size
    match cell.font_size {
        None => w.write_all(&[0])?,
        Some(fs) => {
            w.write_all(&[1])?;
            let fs_val: f64 = fs;
            w.write_all(&fs_val.to_le_bytes())?;
        }
    }

    Ok(())
}

fn deserialize_cell<R: Read>(r: &mut R) -> io::Result<TableCell> {
    let mut type_byte = [0u8; 1];
    r.read_exact(&mut type_byte)?;

    let mut len_buf = [0u8; 4];
    r.read_exact(&mut len_buf)?;
    let len = u32::from_le_bytes(len_buf) as usize;
    let mut bytes = vec![0u8; len];
    r.read_exact(&mut bytes)?;
    let content_str = String::from_utf8(bytes).map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid UTF8"))?;

    let content = match type_byte[0] {
        0 => Cell::Text(content_str),
        1 => Cell::ImagePath(content_str),
        2 => Cell::ImageBase64(content_str),
        #[cfg(feature = "qrcode")]
        3 => Cell::QrCode(content_str),
        _ => return Err(io::Error::new(io::ErrorKind::Other, "Invalid cell type")),
    };

    let mut span_buf = [0u8; 4];
    r.read_exact(&mut span_buf)?;
    let colspan = u32::from_le_bytes(span_buf) as usize;
    r.read_exact(&mut span_buf)?;
    let rowspan = u32::from_le_bytes(span_buf) as usize;

    let mut align_buf = [0u8; 1];
    r.read_exact(&mut align_buf)?;
    let align = match align_buf[0] {
        1 => Some(Align::Left),
        2 => Some(Align::Center),
        3 => Some(Align::Right),
        _ => None,
    };
    r.read_exact(&mut align_buf)?;
    let valign = match align_buf[0] {
        1 => Some(VAlign::Top),
        2 => Some(VAlign::Center),
        3 => Some(VAlign::Bottom),
        _ => None,
    };

    let mut has_link_buf = [0u8; 1];
    r.read_exact(&mut has_link_buf)?;
    let link = if has_link_buf[0] == 1 {
        r.read_exact(&mut len_buf)?;
        let llen = u32::from_le_bytes(len_buf) as usize;
        let mut lbytes = vec![0u8; llen];
        r.read_exact(&mut lbytes)?;
        Some(String::from_utf8(lbytes).map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid UTF8"))?)
    } else {
        None
    };

    let mut has_color_buf = [0u8; 1];
    r.read_exact(&mut has_color_buf)?;
    let bg_color = if has_color_buf[0] == 1 {
        let mut rgb = [0u8; 3];
        r.read_exact(&mut rgb)?;
        Some(Color::Rgb(rgb[0], rgb[1], rgb[2]))
    } else {
        None
    };

    r.read_exact(&mut has_color_buf)?;
    let text_color = if has_color_buf[0] == 1 {
        let mut rgb = [0u8; 3];
        r.read_exact(&mut rgb)?;
        Some(Color::Rgb(rgb[0], rgb[1], rgb[2]))
    } else {
        None
    };

    let mut has_fs_buf = [0u8; 1];
    r.read_exact(&mut has_fs_buf)?;
    let font_size = if has_fs_buf[0] == 1 {
        let mut fs_bytes = [0u8; 8];
        r.read_exact(&mut fs_bytes)?;
        Some(f64::from_le_bytes(fs_bytes))
    } else {
        None
    };

    Ok(TableCell {
        content,
        colspan,
        rowspan,
        align,
        valign,
        link,
        bg_color,
        text_color,
        font_size,
    })
}
