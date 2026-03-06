use mr_pdf::Pdf;
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    // Create the output file in the preview directory
    let file = File::create("preview/image_example.pdf")?;
    let writer = BufWriter::new(file);

    let mut pdf = Pdf::new(writer)?;

    // Add a title
    pdf.text("PDF with Image Example").size(24.0).align_center();

    // Add some description
    pdf.text("This example demonstrates how to embed a JPEG image into the PDF.")
        .size(12.0);

    // Add the logo image
    // logo.jpg is in the project root
    pdf.image("logo.jpg")
        .position(100.0, 500.0)
        .size(200.0, 150.0)
        .render()?;

    pdf.finish()?;

    println!("Successfully wrote image_example.pdf to preview directory");
    Ok(())
}
