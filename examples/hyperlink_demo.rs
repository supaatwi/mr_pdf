use mr_pdf::{Color, Pdf};
use std::fs::File;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let file = File::create("preview/hyperlink_demo.pdf")?;
    let mut pdf = Pdf::new(BufWriter::new(file))?;

    pdf.text("Hyperlink Demo").size(24.0).align_center();
    pdf.advance_cursor(20.0);

    // 1. Direct Text Hyperlink
    pdf.text("1. This is a direct link to Google")
        .size(14.0)
        .link("https://www.google.com");

    pdf.advance_cursor(10.0);
    pdf.text("Click the text above to open Google.")
        .size(10.0)
        .color(Color::Rgb(100, 100, 100));

    pdf.advance_cursor(30.0);

    // 2. Table with Links
    pdf.text("2. Table with Links").size(16.0);
    pdf.table(|table| {
        table.widths(vec![150.0, 300.0]);
        table.header(vec!["Site", "Description"]);

        table.row_builder(|r| {
            r.cell("GitHub").link("https://github.com");
            r.cell("The place where world builds software.");
        });

        table.row_builder(|r| {
            r.cell("Rust Lang").link("https://www.rust-lang.org");
            r.cell("Empowering everyone to build reliable and efficient software.");
        });
    })?;

    pdf.finish()?;
    println!("Successfully wrote hyperlink_demo.pdf to preview directory");
    Ok(())
}
