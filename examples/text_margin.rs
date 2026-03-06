use mr_pdf::Pdf;
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/text_margin.pdf")?;
    let writer = BufWriter::new(file);

    let mut pdf = Pdf::new(writer)?;

    pdf.text("Text with Margins Sample")
        .size(24.0)
        .align_center()
        .margin_bottom(20.0);

    pdf.text("This paragraph has a large margin top.")
        .margin_top(50.0)
        .margin_bottom(10.0);

    pdf.text("This paragraph is separated by the margin bottom of the previous one.")
        .size(14.0);

    pdf.text("Centered text with top and bottom margins")
        .align_center()
        .margin_top(30.0)
        .margin_bottom(30.0);

    pdf.text("The end of the margin demonstration.")
        .align_right();

    pdf.finish()?;

    println!("Successfully wrote text_margin.pdf to preview directory");
    Ok(())
}
