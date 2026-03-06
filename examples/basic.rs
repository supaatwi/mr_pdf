use mr_pdf::{Align, Pdf};
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/basic.pdf")?;
    let writer = BufWriter::new(file);

    let mut pdf = Pdf::new(writer)?;

    pdf.text("Report Title").size(24.0).align_center();

    pdf.text("This is an automatically generated PDF with streaming architecture. This is an automatically generated PDF with streaming architecture")
        .size(12.0)
        .align(Align::Left);

    pdf.finish()?;

    println!("Successfully wrote basic.pdf");
    Ok(())
}
