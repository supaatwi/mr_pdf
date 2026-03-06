use mr_pdf::{Orientation, PaperSize, Pdf};
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/paper_size.pdf")?;
    let writer = BufWriter::new(file);

    let mut pdf = Pdf::new(writer)?;

    // ── 1. Set Paper Size to A5 Landscape ──
    pdf.set_paper_size(PaperSize::A5);
    pdf.set_orientation(Orientation::Landscape);

    pdf.text("This is an A5 Landscape Page")
        .size(20.0)
        .align_center();

    pdf.text("The library now supports A3, A4, A5, and Custom sizes.")
        .size(12.0);

    // ── 2. Switch to A4 Portrait for the next page ──
    pdf.new_page()?;
    pdf.set_paper_size(PaperSize::A4);
    pdf.set_orientation(Orientation::Portrait);

    pdf.text("This is an A4 Portrait Page")
        .size(20.0)
        .align_center();

    pdf.text("You can change paper size and orientation per page by calling pdf.new_page() followed by the set methods.")
        .size(12.0);

    // ── 3. Custom Size Page ──
    pdf.new_page()?;
    pdf.set_paper_size(PaperSize::Custom(400.0, 400.0)); // Square 400x400 points

    pdf.text("Custom Square Page (400x400)")
        .size(16.0)
        .align_center();

    pdf.finish()?;

    println!("Successfully wrote paper_size.pdf to preview directory");
    Ok(())
}
