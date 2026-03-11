use mr_pdf::{Color, Pdf, pt};
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/table_image.pdf")?;
    let writer = BufWriter::new(file);

    let mut pdf = Pdf::new(writer)?;

    pdf.text("Table with Images (Path & Base64)")
        .size(24.0)
        .align_center()
        .margin_bottom(20.0);

    // Small red dot as base64 for testing (1x1 red dot)
    let red_dot_base64 = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";

    pdf.table(|t| {
        t.widths(vec![pt(150.0), flex(1)]);

        t.header(vec!["Description", "Image"]);

        // 1. Image from Path
        t.row_builder(|r| {
            r.cell("Logo from path (demo.jpg)");
            r.cell_image("demo.jpg");
        });

        // 2. Image from Base64
        t.row_builder(|r| {
            r.cell("Small red dot from Base64");
            r.cell_image_base64(red_dot_base64);
        });

        t.header_style().bg_color(Color::Rgb(240, 240, 240));
    })?;

    pdf.finish()?;

    println!("Successfully wrote table_image.pdf to preview directory");
    Ok(())
}

fn flex(f: u32) -> mr_pdf::Size {
    mr_pdf::Size::Flex(f)
}
