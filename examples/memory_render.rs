use mr_pdf::Pdf;
use std::fs;

fn main() -> std::io::Result<()> {
    // 1. Using the new convenience render method (returns Vec<u8>)
    let bytes = Pdf::render(|pdf| {
        pdf.set_title("Memory Render Example");

        pdf.text("This PDF was generated entirely in memory!")
            .size(20.0)
            .align_center()
            .margin_bottom(20.0);

        pdf.text("No intermediate file stream was used until the very end.")
            .size(12.0);

        Ok(())
    })?;

    // Now you have the bytes in a Vec<u8>
    println!("Generated PDF with {} bytes", bytes.len());

    // You can then save it to a file, send over network, etc.
    fs::create_dir_all("preview")?;
    fs::write("preview/memory_render.pdf", bytes)?;
    println!("Saved to preview/memory_render.pdf");

    // 2. Alternatively, using Pdf::memory() manually
    let mut pdf = Pdf::memory()?;
    pdf.text("Manual memory generation").size(14.0);
    let bytes2 = pdf.finish()?;

    println!("Manual version generated {} bytes", bytes2.len());

    Ok(())
}
