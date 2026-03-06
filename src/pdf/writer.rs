use crate::Metadata;
use crate::font::FontManager;
use crate::layout::text::escape_pdf_str;
use std::io::Write;

pub struct LinkAnnotation {
    pub rect: (f64, f64, f64, f64),
    pub url: String,
}

/// Low-level PDF writer that handles object generation, cross-references, and streaming.
pub struct PdfWriter<W: Write> {
    pub writer: W,
    offsets: Vec<u64>,
    pages: Vec<u32>,
    pub next_object_id: u32,
    pos: u64,
    catalog_id: u32,
    pages_id: u32,
    resources_id: u32,
    info_id: u32,
    builtin_font_id: u32,
    xobjects: Vec<u32>,
    page_annots: Vec<(u32, Vec<LinkAnnotation>)>,
}

impl<W: Write> PdfWriter<W> {
    pub fn new(writer: W) -> std::io::Result<Self> {
        let mut slf = Self {
            writer,
            offsets: vec![0],
            pages: Vec::new(),
            next_object_id: 1,
            pos: 0,
            catalog_id: 0,
            pages_id: 0,
            resources_id: 0,
            info_id: 0,
            builtin_font_id: 0,
            xobjects: Vec::new(),
            page_annots: Vec::new(),
        };
        slf.write_raw(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n")?;

        slf.catalog_id = slf.alloc_id();
        slf.pages_id = slf.alloc_id();
        slf.resources_id = slf.alloc_id();
        slf.info_id = slf.alloc_id();
        slf.builtin_font_id = slf.alloc_id();

        let bfid = slf.builtin_font_id;
        slf.start_obj(bfid)?;
        slf.write_raw(
            b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>\n",
        )?;
        slf.end_obj()?;

        Ok(slf)
    }

    pub fn write_raw(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.writer.write_all(data)?;
        self.pos += data.len() as u64;
        Ok(())
    }

    pub fn alloc_id(&mut self) -> u32 {
        let id = self.next_object_id;
        self.next_object_id += 1;
        self.offsets.push(0);
        id
    }

    pub fn start_obj(&mut self, id: u32) -> std::io::Result<()> {
        self.offsets[id as usize] = self.pos;
        let s = format!("{} 0 obj\n", id);
        self.write_raw(s.as_bytes())?;
        Ok(())
    }

    pub fn end_obj(&mut self) -> std::io::Result<()> {
        self.write_raw(b"endobj\n")?;
        Ok(())
    }

    /// Adds a new page dictionary to the document.
    pub fn add_page(&mut self, width: f64, height: f64) -> std::io::Result<(u32, u32)> {
        let page_id = self.alloc_id();
        let content_id = self.alloc_id();
        let annots_id = self.alloc_id();
        self.pages.push(page_id);
        self.page_annots.push((annots_id, Vec::new()));

        self.start_obj(page_id)?;
        let s = format!(
            "<< /Type /Page\n   /Parent {} 0 R\n   /Resources {} 0 R\n   /MediaBox [0 0 {:.2} {:.2}]\n   /Contents {} 0 R\n   /Annots {} 0 R\n>>\n",
            self.pages_id, self.resources_id, width, height, content_id, annots_id
        );
        self.write_raw(s.as_bytes())?;
        self.end_obj()?;

        Ok((page_id, content_id))
    }

    /// Adds a hyperlink to the current page.
    pub fn add_link(&mut self, rect: (f64, f64, f64, f64), url: &str) {
        if let Some((_, annots)) = self.page_annots.last_mut() {
            annots.push(LinkAnnotation {
                rect,
                url: url.to_string(),
            });
        }
    }

    /// Appends a content stream to a specific object.
    pub fn write_content_stream(&mut self, content_id: u32, content: &str) -> std::io::Result<()> {
        self.start_obj(content_id)?;
        let s = format!("<< /Length {} >>\nstream\n", content.len());
        self.write_raw(s.as_bytes())?;
        self.write_raw(content.as_bytes())?;
        self.write_raw(b"\nendstream\n")?;
        self.end_obj()?;
        Ok(())
    }

    pub fn register_xobject(&mut self, xobj_id: u32) {
        self.xobjects.push(xobj_id);
    }

    /// Finalizes the PDF by writing cross-reference tables and trailers.
    pub fn finish(
        mut self,
        font_manager: &mut FontManager,
        metadata: &Metadata,
    ) -> std::io::Result<W> {
        let font_resources = font_manager.embed_fonts(&mut self)?;

        let annots_to_write = std::mem::take(&mut self.page_annots);
        for (annots_id, links) in annots_to_write {
            let mut link_ids = Vec::new();
            for link in links {
                let id = self.alloc_id();
                link_ids.push(id);
                self.start_obj(id)?;
                let esc_url = escape_pdf_str(&link.url);
                let s = format!(
                    "<< /Type /Annot /Subtype /Link /Rect [{:.2} {:.2} {:.2} {:.2}] /Border [0 0 0] /A << /Type /Action /S /URI /URI ({}) >> >>\n",
                    link.rect.0, link.rect.1, link.rect.2, link.rect.3, esc_url
                );
                self.write_raw(s.as_bytes())?;
                self.end_obj()?;
            }

            self.start_obj(annots_id)?;
            let mut s = String::from("[ ");
            for lid in link_ids {
                s.push_str(&format!("{} 0 R ", lid));
            }
            s.push_str("]\n");
            self.write_raw(s.as_bytes())?;
            self.end_obj()?;
        }

        self.start_obj(self.resources_id)?;
        self.write_raw(b"<< ")?;
        let bfid = self.builtin_font_id;
        let s = format!(
            "/Font << /FBuiltin {} 0 R {} >> ",
            bfid,
            if font_resources.is_empty() {
                String::new()
            } else {
                font_resources
            }
        );
        self.write_raw(s.as_bytes())?;
        let xobjects = self.xobjects.clone();
        if !xobjects.is_empty() {
            self.write_raw(b"/XObject << ")?;
            for xobj_id in &xobjects {
                self.write_raw(format!("/Im{} {} 0 R ", xobj_id, xobj_id).as_bytes())?;
            }
            self.write_raw(b">> ")?;
        }
        self.write_raw(b">>\n")?;
        self.end_obj()?;

        self.start_obj(self.pages_id)?;
        let mut kids = String::new();
        for pid in &self.pages {
            kids.push_str(&format!("{} 0 R ", pid));
        }
        let s = format!(
            "<< /Type /Pages /Count {} /Kids [{}] >>\n",
            self.pages.len(),
            kids
        );
        self.write_raw(s.as_bytes())?;
        self.end_obj()?;

        self.start_obj(self.catalog_id)?;
        let s = format!("<< /Type /Catalog /Pages {} 0 R >>\n", self.pages_id);
        self.write_raw(s.as_bytes())?;
        self.end_obj()?;

        let xref_pos = self.pos;
        let s = format!("xref\n0 {}\n", self.offsets.len());
        self.write_raw(s.as_bytes())?;
        self.write_raw(b"0000000000 65535 f \n")?;
        for i in 1..self.offsets.len() {
            let s = format!("{:010} 00000 n \n", self.offsets[i]);
            self.write_raw(s.as_bytes())?;
        }

        self.start_obj(self.info_id)?;
        self.write_raw(b"<< ")?;
        if let Some(t) = &metadata.title {
            let esc_t = escape_pdf_str(t);
            self.write_raw(format!("/Title ({}) ", esc_t).as_bytes())?;
        }
        if let Some(a) = &metadata.author {
            let esc_a = escape_pdf_str(a);
            self.write_raw(format!("/Author ({}) ", esc_a).as_bytes())?;
        }
        if let Some(s) = &metadata.subject {
            let esc_s = escape_pdf_str(s);
            self.write_raw(format!("/Subject ({}) ", esc_s).as_bytes())?;
        }
        self.write_raw(b">>\n")?;
        self.end_obj()?;

        let s = format!(
            "trailer\n<< /Size {} /Root {} 0 R /Info {} 0 R >>\nstartxref\n{}\n%%EOF\n",
            self.offsets.len(),
            self.catalog_id,
            self.info_id,
            xref_pos
        );
        self.write_raw(s.as_bytes())?;

        Ok(self.writer)
    }
}
