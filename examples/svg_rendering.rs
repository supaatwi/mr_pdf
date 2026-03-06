use mr_pdf::{PaperSize, Pdf, SizeExt};
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/svg_demo.pdf")?;
    let mut pdf = Pdf::new(BufWriter::new(file))?;

    pdf.set_paper_size(PaperSize::A4);
    pdf.set_title("SVG Rendering Demo");

    pdf.text("SVG Rendering Example")
        .size(24.0)
        .align_center()
        .margin_bottom(30.0);

    // 1. Defualt size (100pt width)
    pdf.svg("demo.svg").render()?;

    pdf.advance_cursor(20.0);
    pdf.text("2. Custom Size (50% Width)")
        .size(14.0)
        .margin_bottom(10.0);

    // 2. Custom Size
    pdf.svg("demo.svg").width(50.0.pct()).render()?;

    pdf.advance_cursor(20.0);
    pdf.text("The above graphics were rendered with different sizes using the new builder API.")
        .size(10.0)
        .align_center();

    pdf.finish()?;
    println!("Successfully generated preview/svg_demo.pdf");
    Ok(())
}
