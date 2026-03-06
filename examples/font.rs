use mr_pdf::Pdf;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/font.pdf")?;
    let mut pdf = Pdf::stream(file)?;

    pdf.text("Hello PDF").size(24.0).align_center();
    pdf.text("Page 1 content").size(12.0);

    pdf.register_font("Sarabun", "font/Sarabun-Regular.ttf")?;
    pdf.text("สวัสดี PDF").size(18.0);

    pdf.finish()?;

    println!("Successfully generated 'font.pdf'");

    Ok(())
}
